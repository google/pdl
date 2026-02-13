// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! C++ compiler backend.

use crate::{analyzer, ast};
use heck::ToUpperCamelCase;
use std::collections::HashSet;

fn indent(s: &str, level: usize) -> String {
    let prefix = "    ".repeat(level);
    s.lines()
        .map(|line| if line.is_empty() { line.to_string() } else { format!("{}{}", prefix, line) })
        .collect::<Vec<_>>()
        .join("\n")
}

fn to_pascal_case(s: &str) -> String {
    s.to_upper_camel_case()
}

fn mask(width: usize) -> String {
    format!("{:#x}", (1u128 << width) - 1)
}

fn deref(var: Option<&str>, id: &str) -> String {
    match var {
        Some(v) => format!("{}.{}", v, id),
        None => id.to_string(),
    }
}

fn get_cxx_scalar_type(width: usize) -> String {
    for n in [8, 16, 32, 64] {
        if width <= n {
            return format!("uint{}_t", n);
        }
    }
    panic!("PDL type does not fit on non-extended scalar types: width = {}", width);
}

pub fn generate(
    sources: &ast::SourceDatabase,
    file: &ast::File,
    namespace: Option<&str>,
    include_headers: &[String],
    using_namespaces: &[String],
    exclude_declarations: &[String],
) -> String {
    let mut code = String::new();
    let source = sources.get(file.file).expect("could not read source");
    let scope = analyzer::Scope::new(file).unwrap();
    let schema = analyzer::Schema::new(file);

    code.push_str(&format!("// File generated from {}, with the command:\n", source.name()));
    code.push_str("//  pdlc ...\n");
    code.push_str("// /!\\ Do not edit by hand\n\n");

    code.push_str("#pragma once\n\n");
    code.push_str("#include <cstdint>\n");
    code.push_str("#include <string>\n");
    code.push_str("#include <optional>\n");
    code.push_str("#include <utility>\n");
    code.push_str("#include <vector>\n");
    code.push_str("#include <array>\n");
    code.push_str("#include <numeric>\n\n");
    code.push_str("#include <packet_runtime.h>\n\n");

    for header in include_headers {
        code.push_str(&format!("#include <{}>\n", header));
    }
    if !include_headers.is_empty() {
        code.push('\n');
    }

    for ns in using_namespaces {
        code.push_str(&format!("using namespace {};\n", ns));
    }
    if !using_namespaces.is_empty() {
        code.push('\n');
    }

    code.push_str("#ifndef _ASSERT_VALID\n");
    code.push_str("#ifdef ASSERT\n");
    code.push_str("#define _ASSERT_VALID ASSERT\n");
    code.push_str("#else\n");
    code.push_str("#include <cassert>\n");
    code.push_str("#define _ASSERT_VALID assert\n");
    code.push_str("#endif  // ASSERT\n");
    code.push_str("#endif  // !_ASSERT_VALID\n\n");

    if let Some(ns) = namespace {
        code.push_str(&format!("namespace {} {{\n", ns));
    }

    // Forward declarations
    for decl in &file.declarations {
        if let Some(id) = decl.id() {
            if exclude_declarations.contains(&id.to_string()) {
                continue;
            }
            if matches!(decl.desc, ast::DeclDesc::Packet { .. }) {
                code.push_str(&format!("class {}View;\n", id));
            }
        }
    }

    for decl in &file.declarations {
        if let Some(id) = decl.id() {
            if exclude_declarations.contains(&id.to_string()) {
                continue;
            }
        }

        match &decl.desc {
            ast::DeclDesc::Enum { id, tags, width } => {
                code.push_str(&generate_enum_declaration(id, tags, *width));
                code.push_str(&generate_enum_to_text(id, tags));
            }
            ast::DeclDesc::Packet { .. } => {
                code.push_str(&generate_packet_view(&scope, &schema, decl));
                code.push_str(&generate_packet_builder(&scope, &schema, decl));
            }
            ast::DeclDesc::Struct { .. } => {
                code.push_str(&generate_struct_declaration(&scope, &schema, decl));
            }
            _ => {}
        }
    }

    if let Some(ns) = namespace {
        code.push_str(&format!("}}  // {}\n", ns));
    }

    code
}

fn generate_enum_declaration(id: &str, tags: &[ast::Tag], width: usize) -> String {
    let enum_type = get_cxx_scalar_type(width);
    let mut tag_decls = Vec::new();
    for tag in tags {
        if let ast::Tag::Value(t) = tag {
            tag_decls.push(format!("{} = {:#x},", t.id, t.value));
        }
    }

    format!(
        "\nenum class {} : {} {{\n{}\n}};\n",
        id,
        enum_type,
        indent(&tag_decls.join("\n"), 1)
    )
}

fn generate_enum_to_text(id: &str, tags: &[ast::Tag]) -> String {
    let mut tag_cases = Vec::new();
    for tag in tags {
        if let ast::Tag::Value(t) = tag {
            tag_cases.push(format!("case {}::{}: return \"{}\";", id, t.id, t.id));
        }
    }

    format!(
        r#"
inline std::string {id}Text({id} tag) {{
    switch (tag) {{
{tag_cases}
        default:
            return std::string("Unknown {id}: " +
                   std::to_string(static_cast<uint64_t>(tag)));
    }}
}}
"#,
        id = id,
        tag_cases = indent(&tag_cases.join("\n"), 2)
    )
}

fn get_unconstrained_parent_fields<'a>(
    scope: &analyzer::Scope<'a>,
    decl: &'a ast::Decl,
) -> Vec<&'a ast::Field> {
    let mut constraints = HashSet::new();
    for parent in scope.iter_parents_and_self(decl) {
        for constraint in parent.constraints() {
            constraints.insert(constraint.id.clone());
        }
    }

    let mut fields = Vec::new();
    let parents: Vec<_> = scope.iter_parents(decl).collect();
    for parent in parents.into_iter().rev() {
        for field in parent.fields() {
            if let Some(id) = field.id() {
                if !constraints.contains(id) {
                    match &field.desc {
                        ast::FieldDesc::Scalar { .. }
                        | ast::FieldDesc::Array { .. }
                        | ast::FieldDesc::Typedef { .. }
                        | ast::FieldDesc::Payload { .. }
                        | ast::FieldDesc::Body => {
                            fields.push(field);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    fields
}

fn get_all_parent_fields<'a>(
    scope: &analyzer::Scope<'a>,
    decl: &'a ast::Decl,
) -> Vec<&'a ast::Field> {
    let mut fields = Vec::new();
    let parents: Vec<_> = scope.iter_parents(decl).collect();
    for parent in parents.into_iter().rev() {
        for field in parent.fields() {
            match &field.desc {
                ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body => {}
                _ => {
                    fields.push(field);
                }
            }
        }
    }
    fields
}

struct FieldParser<'a> {
    scope: &'a analyzer::Scope<'a>,
    schema: &'a analyzer::Schema,
    endianness: ast::EndiannessValue,
    offset: usize,
    shift: usize,
    chunk: Vec<(usize, usize, &'a ast::Field)>,
    chunk_nr: usize,
    unchecked_code: Vec<String>,
    code: Vec<String>,
    is_cond_for: HashSet<String>,
    target_prefix: String,
    extract_arrays: bool,
    decl: &'a ast::Decl,
}

impl<'a> FieldParser<'a> {
    fn new(
        scope: &'a analyzer::Scope<'a>,
        schema: &'a analyzer::Schema,
        endianness: ast::EndiannessValue,
        target_prefix: &str,
        extract_arrays: bool,
        decl: &'a ast::Decl,
    ) -> Self {
        Self {
            scope,
            schema,
            endianness,
            offset: 0,
            shift: 0,
            chunk: Vec::new(),
            chunk_nr: 0,
            unchecked_code: Vec::new(),
            code: Vec::new(),
            is_cond_for: HashSet::new(),
            target_prefix: target_prefix.to_string(),
            extract_arrays,
            decl,
        }
    }

    fn unchecked_append(&mut self, line: String) {
        self.unchecked_code.push(line);
    }

    fn append(&mut self, line: String) {
        assert!(self.unchecked_code.is_empty());
        self.code.push(line);
    }

    fn check_size(&mut self, size: &str) {
        self.append(format!("if (span.size() < {}) {{", size));
        self.append("    return false;".to_string());
        self.append("}".to_string());
    }

    fn check_code(&mut self) {
        if !self.unchecked_code.is_empty() {
            assert!(self.chunk.is_empty());
            let unchecked_code = std::mem::take(&mut self.unchecked_code);
            let offset = self.offset;
            self.check_size(&offset.to_string());
            self.code.extend(unchecked_code);
            self.offset = 0;
        }
    }

    fn parse_bit_field(&mut self, field: &'a ast::Field) {
        let width = self.schema.field_size(field.key).static_().unwrap();
        self.chunk.push((self.shift, width, field));
        self.shift += width;

        if !self.shift.is_multiple_of(8) {
            return;
        }

        let size = self.shift / 8;
        let backing_type = get_cxx_scalar_type(self.shift);
        let byteorder = match self.endianness {
            ast::EndiannessValue::LittleEndian => "le",
            ast::EndiannessValue::BigEndian => "be",
        };

        let should_skip_value = self.chunk.iter().all(|(_, _, f)| matches!(f.desc, ast::FieldDesc::Reserved { .. }));
        if should_skip_value {
            self.unchecked_append(format!("span.skip({}); // skip reserved fields", size));
            self.offset += size;
            self.shift = 0;
            self.chunk.clear();
            return;
        }

        let value = if self.chunk.len() > 1 {
            let v = format!("chunk{}", self.chunk_nr);
            self.unchecked_append(format!(
                "{} {} = span.read_{}<{}, {}>();",
                backing_type, v, byteorder, backing_type, size
            ));
            self.chunk_nr += 1;
            v
        } else {
            format!("span.read_{}<{}, {}>()", byteorder, backing_type, size)
        };

        let chunk = std::mem::take(&mut self.chunk);
        for (shift, width, field) in chunk.iter() {
            let v = if chunk.len() == 1 && *shift == 0 {
                value.clone()
            } else {
                format!("({} >> {}) & {}", value, shift, mask(*width))
            };

            let is_cond = field.id().map(|id| self.is_cond_for.contains(id)).unwrap_or(false);

            match &field.desc {
                ast::FieldDesc::Scalar { id, .. } => {
                    if is_cond {
                        self.unchecked_append(format!("uint8_t {} = {};", id, v));
                    } else {
                        self.unchecked_append(format!("{}{}_ = {};", self.target_prefix, id, v));
                    }
                }
                ast::FieldDesc::FixedScalar { value: fixed_value, .. } => {
                    self.unchecked_append(format!("if (static_cast<uint64_t>({}) != {:#x}) {{", v, fixed_value));
                    self.unchecked_append("    return false;".to_string());
                    self.unchecked_append("}".to_string());
                }
                ast::FieldDesc::FixedEnum { enum_id, tag_id, .. } => {
                    self.unchecked_append(format!("if ({}({}) != {}::{}) {{", enum_id, v, enum_id, tag_id));
                    self.unchecked_append("    return false;".to_string());
                    self.unchecked_append("}".to_string());
                }
                ast::FieldDesc::Typedef { id, type_id, .. } => {
                    let type_decl = self.scope.typedef.get(type_id).unwrap();
                    if matches!(type_decl.desc, ast::DeclDesc::Enum { .. }) {
                        self.unchecked_append(format!("{}{}_ = {}({});", self.target_prefix, id, type_id, v));
                    } else {
                        self.unchecked_append(format!("{}{}_ = {};", self.target_prefix, id, v));
                    }
                }
                ast::FieldDesc::Size { field_id, .. } => {
                    let field_name = if field_id == "_payload_" || field_id == "_body_" { "payload" } else { field_id };
                    self.unchecked_append(format!("{}{}_size_ = {};", self.target_prefix, field_name, v));
                }
                ast::FieldDesc::Count { field_id, .. } => {
                    self.unchecked_append(format!("{}{}_count_ = {};", self.target_prefix, field_id, v));
                }
                ast::FieldDesc::ElementSize { field_id, .. } => {
                    let field_name = if field_id == "_payload_" || field_id == "_body_" { "payload" } else { field_id };
                    self.unchecked_append(format!("{}{}_element_size_ = {};", self.target_prefix, field_name, v));
                }
                ast::FieldDesc::Flag { id, .. } => {
                    self.unchecked_append(format!("uint8_t {} = {};", id, v));
                }
                ast::FieldDesc::Reserved { .. } => {}
                _ => panic!("Unsupported bit field type: {:?}", field.desc),
            }
        }

        self.offset += size;
        self.shift = 0;
    }

    fn parse_typedef_field(&mut self, field: &'a ast::Field, id: &str, type_id: &str) {
        if self.shift != 0 {
            panic!("Typedef field does not start on an octet boundary");
        }
        self.check_code();

        let type_decl = self.scope.typedef.get(type_id).unwrap();
        if let ast::DeclDesc::Enum { width, .. } = &type_decl.desc {
            let ty = get_cxx_scalar_type(*width);
            let byteorder = match self.endianness {
                ast::EndiannessValue::LittleEndian => "le",
                ast::EndiannessValue::BigEndian => "be",
            };
            self.append(format!("if (span.size() < {}) return false;", width / 8));
            self.append(format!("{}{}_ = static_cast<{}>(span.read_{}<{}, {}>());", self.target_prefix, id, type_id, byteorder, ty, width / 8));
            return;
        }

        let field_size = self.schema.field_size(field.key);
        if let analyzer::Size::Unknown = field_size {
            let trailing_size = self.get_trailing_size(field);
            if trailing_size > 0 {
                self.append(format!("if (span.size() < {}) return false;", trailing_size / 8));
                let size = format!("span.size() - {}", trailing_size / 8);
                self.append(format!("pdl::packet::slice {0}_span = span.subrange(0, {1});", id, size));
                self.append(format!("if (!{0}::Parse({1}_span, &{2}{1}_)) return false;", type_id, id, self.target_prefix));
                self.append(format!("span.skip({0}_span.size());", id));
            } else {
                self.append(format!("if (!{}::Parse(span, &{}{}_)) return false;", type_id, self.target_prefix, id));
            }
        } else {
            self.append(format!("if (!{}::Parse(span, &{}{}_)) return false;", type_id, self.target_prefix, id));
        }
    }

    fn parse_optional_field(&mut self, field: &'a ast::Field) {
        self.check_code();
        let cond = field.cond.as_ref().unwrap();
        let byteorder = match self.endianness {
            ast::EndiannessValue::LittleEndian => "le",
            ast::EndiannessValue::BigEndian => "be",
        };

        match &field.desc {
            ast::FieldDesc::Scalar { id, width } => {
                let backing_type = get_cxx_scalar_type(*width);
                let size = width / 8;
                let cond_value = cond.value.unwrap();
                self.append(format!("if ({} == {}) {{", cond.id, cond_value));
                self.append(format!("    if (span.size() < {}) {{", size));
                self.append("        return false;".to_string());
                self.append("    }".to_string());
                self.append(format!(
                    "    {}{}_ = span.read_{}<{}, {}>();",
                    self.target_prefix, id, byteorder, backing_type, size
                ));
                self.append("}".to_string());
            }
            ast::FieldDesc::Typedef { id, type_id, .. } => {
                let type_decl = self.scope.typedef.get(type_id).unwrap();
                let cond_value = cond.value.unwrap();
                if let ast::DeclDesc::Enum { width, .. } = &type_decl.desc {
                    let backing_type = get_cxx_scalar_type(*width);
                    let size = width / 8;
                    self.append(format!("if ({} == {}) {{", cond.id, cond_value));
                    self.append(format!("    if (span.size() < {}) {{", size));
                    self.append("        return false;".to_string());
                    self.append("    }".to_string());
                    self.append(format!(
                        "    {}{}_ = {}(span.read_{}<{}, {}>());",
                        self.target_prefix, id, type_id, byteorder, backing_type, size
                    ));
                    self.append("}".to_string());
                } else {
                    self.append(format!("if ({} == {}) {{", cond.id, cond_value));
                    self.append(format!("    auto& opt_output = {}{}_.emplace();", self.target_prefix, id));
                    self.append(format!("    if (!{}::Parse(span, &opt_output)) {{", type_id));
                    self.append("        return false;".to_string());
                    self.append("    }".to_string());
                    self.append("}".to_string());
                }
            }
            _ => panic!("Unsupported optional field type"),
        }
    }

    fn parse_array_field_lite(&mut self, field: &'a ast::Field, id: &str, _type_id: Option<&str>, _width: Option<usize>, _size: Option<usize>) {
        self.check_code();
        match self.schema.field_size(field.key) {
            analyzer::Size::Static(bits) => {
                let size = bits / 8;
                self.check_size(&size.to_string());
                self.append(format!("{}{}_ = span.subrange(0, {});", self.target_prefix, id, size));
                self.append(format!("span.skip({});", size));
            }
            _ => {
                let mut size_expr = "".to_string();
                let mut count_expr = "".to_string();

                for f in self.decl.fields() {
                    match &f.desc {
                        ast::FieldDesc::Size { field_id, .. } if field_id == id => {
                            let field_name = if field_id == "_payload_" || field_id == "_body_" { "payload" } else { field_id };
                            size_expr = format!("{}{}_size_", self.target_prefix, field_name);
                        }
                        ast::FieldDesc::Count { field_id, .. } if field_id == id => {
                            count_expr = format!("{}{}_count_", self.target_prefix, field_id);
                        }
                        _ => {}
                    }
                }

                if !size_expr.is_empty() {
                    let inverted_modifier = match &field.desc {
                        ast::FieldDesc::Array { size_modifier: Some(m), .. } => {
                            if let Some(stripped) = m.strip_prefix('+') { format!("- {}", stripped) }
                            else if let Some(stripped) = m.strip_prefix('-') { format!("+ {}", stripped) }
                            else { m.clone() }
                        }
                        _ => String::new(),
                    };
                    let actual_size = if inverted_modifier.is_empty() { size_expr } else { format!("({} {})", size_expr, inverted_modifier) };
                    self.append(format!("if (span.size() < {}) return false;", actual_size));
                    self.append(format!("{}{}_ = span.subrange(0, {});", self.target_prefix, id, actual_size));
                    self.append(format!("span.skip({});", actual_size));
                } else if !count_expr.is_empty() {
                    if let ast::FieldDesc::Array { width: Some(w), .. } = &field.desc {
                        let total_size = format!("{} * {}", count_expr, w / 8);
                        self.append(format!("if (span.size() < {}) return false;", total_size));
                        self.append(format!("{}{}_ = span.subrange(0, {});", self.target_prefix, id, total_size));
                        self.append(format!("span.skip({});", total_size));
                    } else {
                         // Full parse needed
                         self.append(format!("{}{}_ = span;", self.target_prefix, id));
                         self.append("span.clear();".to_string());
                    }
                } else {
                    let trailing_size = self.get_trailing_size(field);
                    if trailing_size > 0 {
                        self.append(format!("if (span.size() < {}) return false;", trailing_size / 8));
                        let size = format!("span.size() - {}", trailing_size / 8);
                        self.append(format!("{}{}_ = span.subrange(0, {});", self.target_prefix, id, size));
                        self.append(format!("span.skip({});", size));
                    } else {
                        self.append(format!("{}{}_ = span;", self.target_prefix, id));
                        self.append("span.clear();".to_string());
                    }
                }
            }
        }
    }

    fn get_trailing_size(&self, field: &'a ast::Field) -> usize {
        let mut trailing_size = 0;
        let mut found = false;
        for f in self.decl.fields() {
            if found {
                match self.schema.field_size(f.key) {
                    analyzer::Size::Static(bits) => trailing_size += bits,
                    _ => panic!("Multiple unknown size fields in {}", self.decl.id().unwrap_or("unknown")),
                }
            }
            if f.key == field.key {
                found = true;
            }
        }
        trailing_size
    }

    fn parse_payload_field_lite(&mut self, _field: &'a ast::Field, _is_body: bool) {
        if self.shift != 0 {
            panic!("Payload field does not start on an octet boundary");
        }
        self.check_code();
        let mut size_expr = "".to_string();
        for f in self.decl.fields() {
            if let ast::FieldDesc::Size { field_id, .. } = &f.desc {
                if field_id == "_payload_" || field_id == "_body_" {
                    size_expr = format!("{}{}_size_", self.target_prefix, "payload");
                }
            }
        }

        if !size_expr.is_empty() {
            let inverted_modifier = match &_field.desc {
                ast::FieldDesc::Payload { size_modifier: Some(m), .. } => {
                    if let Some(stripped) = m.strip_prefix('+') { format!("- {}", stripped) }
                    else if let Some(stripped) = m.strip_prefix('-') { format!("+ {}", stripped) }
                    else { m.clone() }
                }
                _ => String::new(),
            };
            let actual_size = if inverted_modifier.is_empty() { size_expr } else { format!("({} {})", size_expr, inverted_modifier) };
            self.append(format!("if (span.size() < {}) return false;", actual_size));
            self.append(format!("{}payload_ = span.subrange(0, {});", self.target_prefix, actual_size));
            self.append(format!("span.skip({});", actual_size));
        } else {
            let trailing_size = self.get_trailing_size(_field);
            if trailing_size > 0 {
                 self.append(format!("if (span.size() < {}) return false;", trailing_size / 8));
                 let size = format!("span.size() - {}", trailing_size / 8);
                 self.append(format!("{}payload_ = span.subrange(0, {});", self.target_prefix, size));
                 self.append(format!("span.skip({});", size));
            } else {
                 self.append(format!("{}payload_ = span;", self.target_prefix));
                 self.append("span.clear();".to_string());
            }
        }
    }

    fn parse_array_field_full(&mut self, _field: &'a ast::Field, id: &str, type_id: Option<&str>, width: Option<usize>, size: Option<usize>) {
        self.check_code();
        let byteorder = match self.endianness {
            ast::EndiannessValue::LittleEndian => "le",
            ast::EndiannessValue::BigEndian => "be",
        };

        if let Some(s) = size {
            self.append(format!("for (int n = 0; n < {}; n++) {{", s));
            if let Some(tid) = type_id {
                 let td = self.scope.typedef.get(tid).unwrap();
                 match &td.desc {
                     ast::DeclDesc::Enum { width, .. } => {
                         let backing_type = get_cxx_scalar_type(*width);
                         self.append(format!("    if (span.size() < {}) return false;", width / 8));
                         self.append(format!("    {0}{1}_[n] = {2}(span.read_{3}<{4}, {5}>());", self.target_prefix, id, tid, byteorder, backing_type, width / 8));
                     }
                     _ => { self.append(format!("    if (!{}::Parse(span, &{}{}_[n])) return false;", tid, self.target_prefix, id)); }
                 }
            } else {
                let element_width = width.unwrap();
                let backing_type = get_cxx_scalar_type(element_width);
                self.append(format!("    if (span.size() < {}) return false;", element_width / 8));
                self.append(format!("    {0}{1}_[n] = span.read_{2}<{3}, {4}>();", self.target_prefix, id, byteorder, backing_type, element_width / 8));
            }
            self.append("}".to_string());
        } else {
            let mut count_expr = "".to_string();
            let mut size_expr = "".to_string();
            for f in self.decl.fields() {
                match &f.desc {
                    ast::FieldDesc::Size { field_id, .. } if field_id == id => {
                        let field_name = if field_id == "_payload_" || field_id == "_body_" { "payload" } else { field_id };
                        size_expr = format!("{}{}_size_", self.target_prefix, field_name);
                    }
                    ast::FieldDesc::Count { field_id, .. } if field_id == id => {
                        count_expr = format!("{}{}_count_", self.target_prefix, field_id);
                    }
                    _ => {}
                }
            }

            if !count_expr.is_empty() {
                self.append(format!("for (size_t n = 0; n < {}; n++) {{", count_expr));
            } else if !size_expr.is_empty() {
                let inverted_modifier = match &_field.desc {
                    ast::FieldDesc::Array { size_modifier: Some(m), .. } => {
                        if let Some(stripped) = m.strip_prefix('+') { format!("- {}", stripped) }
                        else if let Some(stripped) = m.strip_prefix('-') { format!("+ {}", stripped) }
                        else { m.clone() }
                    }
                    _ => String::new(),
                };
                let actual_size = if inverted_modifier.is_empty() { size_expr } else { format!("({} {})", size_expr, inverted_modifier) };
                self.append(format!("size_t limit = (span.size() > {}) ? (span.size() - {}) : 0;", actual_size, actual_size));
                self.append("while (span.size() > limit) {".to_string());
            } else {
                self.append("while (span.size() > 0) {".to_string());
            }

            if let Some(tid) = type_id {
                 let td = self.scope.typedef.get(tid).unwrap();
                 match &td.desc {
                     ast::DeclDesc::Enum { width, .. } => {
                         let backing_type = get_cxx_scalar_type(*width);
                         self.append(format!("    if (span.size() < {}) return false;", width / 8));
                         self.append(format!("    {}{}_.push_back({}(span.read_{}<{}, {}>()));", self.target_prefix, id, tid, byteorder, backing_type, width / 8));
                     }
                     _ => {
                         self.append(format!("    {} element;", tid));
                         self.append(format!("    if (!{}::Parse(span, &element)) return false;", tid));
                         self.append(format!("    {}{}_.emplace_back(std::move(element));", self.target_prefix, id));
                     }
                 }
            } else {
                let element_width = width.unwrap();
                let backing_type = get_cxx_scalar_type(element_width);
                self.append(format!("    if (span.size() < {}) return false;", element_width / 8));
                self.append(format!("    {}{}_.push_back(span.read_{}<{}, {}>());", self.target_prefix, id, byteorder, backing_type, element_width / 8));
            }
            self.append("}".to_string());
        }
    }

    fn parse(&mut self, field: &'a ast::Field) {
        if field.cond.is_some() {
            self.parse_optional_field(field);
        } else if self.scope.is_bitfield(field) {
            self.parse_bit_field(field);
        } else {
            self.check_code();
            match &field.desc {
                ast::FieldDesc::Padding { .. } => {}
                ast::FieldDesc::Array { id, type_id, width, size, .. } => {
                    let padded_size = self.schema.padded_size(field.key);
                    if padded_size.is_some() {
                        self.append(format!("size_t {0}_start_size = span.size();", id));
                    }
                    if !self.extract_arrays {
                         self.parse_array_field_lite(field, id, type_id.as_deref(), *width, *size);
                    } else {
                         self.parse_array_field_full(field, id, type_id.as_deref(), *width, *size);
                    }
                    if let Some(ps) = padded_size {
                        let ps_bytes = ps / 8;
                        self.append(format!("if ({0}_start_size - span.size() < {1}) {{", id, ps_bytes));
                        self.append(format!("    if (span.size() < {1} - ({0}_start_size - span.size())) return false;", id, ps_bytes));
                        self.append(format!("    span.skip({1} - ({0}_start_size - span.size()));", id, ps_bytes));
                        self.append("}".to_string());
                    }
                }
                ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body => {
                    self.parse_payload_field_lite(field, matches!(field.desc, ast::FieldDesc::Body));
                }
                ast::FieldDesc::Scalar { id, width } => {
                    let ty = get_cxx_scalar_type(*width);
                    let byteorder = match self.endianness {
                        ast::EndiannessValue::LittleEndian => "le",
                        ast::EndiannessValue::BigEndian => "be",
                    };
                    self.append(format!("if (span.size() < {}) return false;", width / 8));
                    self.append(format!("{}{}_ = span.read_{}<{}, {}>();", self.target_prefix, id, byteorder, ty, width / 8));
                }
                ast::FieldDesc::Size { field_id, width, .. } => {
                    let ty = get_cxx_scalar_type(*width);
                    let field_name = if field_id == "_payload_" || field_id == "_body_" { "payload" } else { field_id };
                    let byteorder = match self.endianness {
                        ast::EndiannessValue::LittleEndian => "le",
                        ast::EndiannessValue::BigEndian => "be",
                    };
                    self.append(format!("if (span.size() < {}) return false;", width / 8));
                    self.append(format!("{}{}_size_ = span.read_{}<{}, {}>();", self.target_prefix, field_name, byteorder, ty, width / 8));
                }
                ast::FieldDesc::Count { field_id, width, .. } => {
                    let ty = get_cxx_scalar_type(*width);
                    let byteorder = match self.endianness {
                        ast::EndiannessValue::LittleEndian => "le",
                        ast::EndiannessValue::BigEndian => "be",
                    };
                    self.append(format!("if (span.size() < {}) return false;", width / 8));
                    self.append(format!("{}{}_count_ = span.read_{}<{}, {}>();", self.target_prefix, field_id, byteorder, ty, width / 8));
                }
                ast::FieldDesc::ElementSize { field_id, width, .. } => {
                    let ty = get_cxx_scalar_type(*width);
                    let field_name = if field_id == "_payload_" || field_id == "_body_" { "payload" } else { field_id };
                    let byteorder = match self.endianness {
                        ast::EndiannessValue::LittleEndian => "le",
                        ast::EndiannessValue::BigEndian => "be",
                    };
                    self.append(format!("if (span.size() < {}) return false;", width / 8));
                    self.append(format!("{}{}_element_size_ = span.read_{}<{}, {}>();", self.target_prefix, field_name, byteorder, ty, width / 8));
                }
                ast::FieldDesc::FixedScalar { width, value } => {
                    let ty = get_cxx_scalar_type(*width);
                    let byteorder = match self.endianness {
                        ast::EndiannessValue::LittleEndian => "le",
                        ast::EndiannessValue::BigEndian => "be",
                    };
                    self.append(format!("if (span.size() < {}) return false;", width / 8));
                    self.append(format!("if (span.read_{}<{}, {}>() != {:#x}) return false;", byteorder, ty, width / 8, value));
                }
                ast::FieldDesc::FixedEnum { enum_id, tag_id, .. } => {
                    let width = match &self.scope.typedef[enum_id].desc {
                        ast::DeclDesc::Enum { width, .. } => *width,
                        _ => unreachable!(),
                    };
                    let ty = get_cxx_scalar_type(width);
                    let byteorder = match self.endianness {
                        ast::EndiannessValue::LittleEndian => "le",
                        ast::EndiannessValue::BigEndian => "be",
                    };
                    self.append(format!("if (span.size() < {}) return false;", width / 8));
                    self.append(format!("if (span.read_{}<{}, {}>() != static_cast<{}>( {}::{} )) return false;", byteorder, ty, width / 8, ty, enum_id, tag_id));
                }
                ast::FieldDesc::Typedef { id, type_id, .. } => {
                    self.parse_typedef_field(field, id, type_id);
                }
                _ => {}
            }
        }
    }

    fn done(&mut self) {
        self.check_code();
    }
}

struct FieldSerializer<'a> {
    scope: &'a analyzer::Scope<'a>,
    schema: &'a analyzer::Schema,
    endianness: ast::EndiannessValue,
    shift: usize,
    values: Vec<(String, usize)>,
    code: Vec<String>,
    indent_level: usize,
}

impl<'a> FieldSerializer<'a> {
    fn new(
        scope: &'a analyzer::Scope<'a>,
        schema: &'a analyzer::Schema,
        endianness: ast::EndiannessValue,
    ) -> Self {
        Self {
            scope,
            schema,
            endianness,
            shift: 0,
            values: Vec::new(),
            code: Vec::new(),
            indent_level: 0,
        }
    }

    fn indent(&mut self) {
        self.indent_level += 1;
    }

    fn unindent(&mut self) {
        self.indent_level -= 1;
    }

    fn append(&mut self, line: &str) {
        let prefix = "    ".repeat(self.indent_level);
        for l in line.lines() {
            self.code.push(format!("{}{}", prefix, l));
        }
    }

    fn get_payload_field_size(&self, var: Option<&str>, field_id: &str, decl: &'a ast::Decl) -> String {
        let parent_constraints = self.scope.iter_parents_and_self(decl).flat_map(|d| d.constraints()).collect::<Vec<_>>();
        let get_field_size_expr = |f: &ast::Field| -> String {
            let fid = f.id();
            let is_constrained = fid.map(|fid| parent_constraints.iter().any(|c| c.id == fid)).unwrap_or(false);
            if is_constrained {
                 return format!("{}", self.schema.field_size(f.key).static_().unwrap() / 8);
            }

            match &f.desc {
                ast::FieldDesc::Scalar { width, .. } => {
                    if let Some(cond) = &f.cond {
                        let cond_field = self.scope.iter_fields(decl).find(|f| f.id() == Some(&cond.id)).expect("Cond field not found");
                        let _cond_field_var = format!("{0}.has_value() ? ...", cond_field.id().unwrap()); // This is getting complex
                        // Actually, I'll just use a simpler approach for now.
                        format!("(({}_.has_value()) ? {} : 0)", f.id().unwrap(), width / 8)
                    } else {
                        format!("{}", width / 8)
                    }
                }
                ast::FieldDesc::Typedef { id, type_id, .. } => {
                    let type_decl = self.scope.typedef.get(type_id).unwrap();
                    let width = match &type_decl.desc {
                        ast::DeclDesc::Enum { width, .. } => *width,
                        _ => 0,
                    };
                    if f.cond.is_some() {
                         if width > 0 {
                              format!("(({}_.has_value()) ? {} : 0)", id, width / 8)
                         } else {
                              format!("(({0}_.has_value()) ? {0}_->GetSize() : 0)", id)
                         }
                    } else if width > 0 {
                        format!("{}", width / 8)
                    } else {
                        format!("{}_.GetSize()", id)
                    }
                }
                ast::FieldDesc::Array { id, width, type_id, .. } => {
                    let element_size = if let Some(w) = width {
                        format!("{}", w / 8)
                    } else {
                        let tid = type_id.as_ref().unwrap();
                        let td = self.scope.typedef.get(tid).unwrap();
                        match &td.desc {
                            ast::DeclDesc::Enum { width, .. } => format!("{}", width / 8),
                            _ => "element.GetSize()".to_string(),
                        }
                    };

                    if element_size.contains("GetSize") {
                        format!("std::accumulate({0}_.begin(), {0}_.end(), static_cast<size_t>(0), [](size_t s, auto const& element) {{ return s + {1}; }})", id, element_size)
                    } else {
                        format!("({0}_.size() * {1})", id, element_size)
                    }
                }
                ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body => {
                     format!("{}.size()", deref(var, "payload_"))
                }
                _ => format!("{}", self.schema.field_size(f.key).static_().unwrap_or(0) / 8),
            }
        };

        if field_id == "_payload_" || field_id == "_body_" {
            let size_modifier = self.scope.iter_fields(decl).find_map(|f| match &f.desc {
                ast::FieldDesc::Payload { size_modifier, .. } => size_modifier.as_ref(),
                _ => None,
            });

            let has_local_payload = decl.fields().any(|f| matches!(f.desc, ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body));
            let size_expr = if has_local_payload {
                format!("{}.size()", deref(var, "payload_"))
            } else {
                let local_fields = decl.fields().map(get_field_size_expr).collect::<Vec<_>>();
                if local_fields.is_empty() {
                    "0".to_string()
                } else {
                    local_fields.join(" + ")
                }
            };

            if let Some(m) = size_modifier {
                return format!("({} {})", size_expr, m);
            }
            return size_expr;
        }
        let f = self.scope.iter_fields(decl).find(|f| f.id() == Some(field_id))
            .unwrap_or_else(|| panic!("Field {} not found in {}", field_id, decl.id().unwrap_or("unknown")));
        match &f.desc {
            ast::FieldDesc::Array { .. } => {
                let size_expr = get_field_size_expr(f);
                let size_modifier = match &f.desc {
                    ast::FieldDesc::Array { size_modifier, .. } => size_modifier.as_ref(),
                    _ => None,
                };
                if let Some(m) = size_modifier {
                    return format!("({} {})", size_expr, m);
                }
                size_expr
            }
            _ => format!("{}.GetSize()", deref(var, &format!("{}_", f.id().unwrap()))),
        }
    }

    fn pack_bit_fields(&mut self) {
        assert_eq!(self.shift % 8, 0);
        let size = self.shift / 8;
        let backing_type = get_cxx_scalar_type(self.shift);
        let byteorder = match self.endianness {
            ast::EndiannessValue::LittleEndian => "le",
            ast::EndiannessValue::BigEndian => "be",
        };

        if self.values.is_empty() {
            self.append(&format!(
                "pdl::packet::Builder::write_{}<{}, {}>(output, 0);",
                byteorder, backing_type, size
            ));
        } else {
            let packed_val = self.values.iter()
                .map(|(v, s)| {
                    if *s == 0 {
                        format!("(static_cast<{}>({}))", backing_type, v)
                    } else {
                        format!("(static_cast<{}>({}) << {})", backing_type, v, s)
                    }
                })
                .collect::<Vec<_>>()
                .join(" | ");
            self.append(&format!(
                "pdl::packet::Builder::write_{}<{}, {}>(output, {});",
                byteorder, backing_type, size, packed_val
            ));
        }

        self.shift = 0;
        self.values.clear();
    }

    fn serialize(&mut self, field: &'a ast::Field, decl: &'a ast::Decl, var: Option<&str>) {
        let parent_constraints = self.scope.iter_parents_and_self(decl).flat_map(|d| d.constraints()).collect::<Vec<_>>();
        let get_field_expr = |f: &ast::Field| -> String {
            if let Some(fid) = f.id() {
                if let Some(constraint) = parent_constraints.iter().find(|c| c.id == fid) {
                    if let Some(tag_id) = &constraint.tag_id {
                         let type_id = match &f.desc {
                             ast::FieldDesc::Typedef { type_id, .. } => Some(type_id.as_str()),
                             ast::FieldDesc::Array { type_id, .. } => type_id.as_deref(),
                             _ => None,
                         };
                         if let Some(type_id) = type_id {
                             return format!("{}::{}", type_id, tag_id);
                         }
                    }
                    return format!("{:#x}", constraint.value.unwrap());
                }
            }
            match &f.desc {
                ast::FieldDesc::Flag { optional_field_ids, .. } => {
                    let (opt_id, val_present) = &optional_field_ids[0];
                    let val_absent = if *val_present == 0 { 1 } else { 0 };
                    format!("({0}.has_value() ? {1} : {2})", deref(var, &format!("{}_", opt_id)), val_present, val_absent)
                }
                _ => deref(var, &format!("{}_", f.id().unwrap())),
            }
        };

        let field_var = field.id().map(|_| get_field_expr(field));
        let byteorder = match self.endianness {
            ast::EndiannessValue::LittleEndian => "le",
            ast::EndiannessValue::BigEndian => "be",
        };

        if let Some(cond) = &field.cond {
            let cond_field = self.scope.iter_fields(decl).find(|f| f.id() == Some(&cond.id)).expect("Cond field not found");
            let cond_field_var = get_field_expr(cond_field);
            let cond_val = cond.value.unwrap();
            self.append(&format!("if ({} == {}) {{", cond_field_var, cond_val));
            self.indent();
            match &field.desc {
                 ast::FieldDesc::Scalar { width, .. } => {
                      let ty = get_cxx_scalar_type(*width);
                      self.append(&format!("pdl::packet::Builder::write_{}<{}, {}>(output, *{});", byteorder, ty, width / 8, field_var.as_ref().unwrap()));
                 }
                 ast::FieldDesc::Typedef { type_id, .. } => {
                      let td = self.scope.typedef.get(type_id).unwrap();
                      if let ast::DeclDesc::Enum { width, .. } = &td.desc {
                           let ty = get_cxx_scalar_type(*width);
                           self.append(&format!("pdl::packet::Builder::write_{}<{}, {}>(output, static_cast<{}>(*{}));", byteorder, ty, width / 8, ty, field_var.as_ref().unwrap()));
                      } else {
                           self.append(&format!("{0}->Serialize(output);", field_var.as_ref().unwrap()));
                      }
                 }
                 _ => {}
            }
            self.unindent();
            self.append("}");
        } else if self.scope.is_bitfield(field) {
            let width = self.schema.field_size(field.key).static_().unwrap();
            let shift = self.shift;
            match &field.desc {
                ast::FieldDesc::Scalar { .. } => {
                    self.values.push((format!("{} & {}", field_var.unwrap(), mask(width)), shift));
                }
                ast::FieldDesc::FixedScalar { value, .. } => {
                    self.values.push((format!("{:#x}", value), shift));
                }
                ast::FieldDesc::FixedEnum { enum_id, tag_id, .. } => {
                    self.values.push((format!("{}::{}", enum_id, tag_id), shift));
                }
                ast::FieldDesc::Typedef { id: _, type_id, .. } => {
                    let type_decl = self.scope.typedef.get(type_id).unwrap();
                    if matches!(type_decl.desc, ast::DeclDesc::Enum { .. }) {
                        self.values.push((format!("static_cast<{}>({})", get_cxx_scalar_type(width), field_var.unwrap()), shift));
                    } else {
                        self.values.push((field_var.unwrap(), shift));
                    }
                }
                ast::FieldDesc::Size { field_id, .. } => {
                    let field_name = if field_id == "_payload_" || field_id == "_body_" { "payload" } else { field_id };
                    let size_expr = self.get_payload_field_size(var, field_id, decl);
                    self.append(&format!("size_t {field_name}_size = {size_expr};"));
                    self.values.push((format!("{}_size", field_name), shift));
                }
                ast::FieldDesc::Count { field_id, .. } => {
                    let f = self.scope.iter_fields(decl).find(|f| f.id() == Some(field_id)).expect("Field not found");
                    self.values.push((format!("{}.size()", get_field_expr(f)), shift));
                }
                ast::FieldDesc::ElementSize { field_id, .. } => {
                    let field_name = if field_id == "_payload_" || field_id == "_body_" { "payload" } else { field_id };
                    self.append(&format!("size_t {field_name}_element_size = 0; // TODO"));
                    self.values.push((format!("{}_element_size", field_name), shift));
                }
                ast::FieldDesc::Flag { .. } => {
                    self.values.push((field_var.unwrap(), shift));
                }
                _ => {}
            }
            self.shift += width;
            if self.shift.is_multiple_of(8) {
                self.pack_bit_fields();
            }
        } else {
            match &field.desc {
                ast::FieldDesc::Padding { .. } => {}
                ast::FieldDesc::Array { id, type_id, width, .. } => {
                    let padded_size = self.schema.padded_size(field.key);
                    if padded_size.is_some() {
                        self.append(&format!("size_t {0}_start = output.size();", id));
                    }
                    if let Some(v) = field_var {
                        self.append(&format!("for (auto const& element : {}) {{", v));
                        self.indent();
                        if let Some(tid) = type_id {
                            let td = self.scope.typedef.get(tid).unwrap();
                            match &td.desc {
                                ast::DeclDesc::Enum { width, .. } => {
                                    let backing_type = get_cxx_scalar_type(*width);
                                    self.append(&format!("pdl::packet::Builder::write_{}<{}, {}>(output, static_cast<{}>(element));", byteorder, backing_type, width / 8, backing_type));
                                }
                                _ => self.append("element.Serialize(output);"),
                            }
                        } else {
                            let element_width = width.unwrap();
                            let backing_type = get_cxx_scalar_type(element_width);
                            self.append(&format!("pdl::packet::Builder::write_{}<{}, {}>(output, static_cast<{}>(element));", byteorder, backing_type, element_width / 8, backing_type));
                        }
                        self.unindent();
                        self.append("}");
                    }
                    if let Some(ps) = padded_size {
                        let ps_bytes = ps / 8;
                        self.append(&format!("if (output.size() - {0}_start < {1}) {{", id, ps_bytes));
                        self.append(&format!("    output.resize({0}_start + {1}, 0);", id, ps_bytes));
                        self.append("}");
                    }
                }
                ast::FieldDesc::Scalar { width, .. } => {
                    let ty = get_cxx_scalar_type(*width);
                    self.append(&format!("pdl::packet::Builder::write_{}<{}, {}>(output, {});", byteorder, ty, width / 8, field_var.unwrap()));
                }
                ast::FieldDesc::Typedef { type_id, .. } => {
                    let td = self.scope.typedef.get(type_id).unwrap();
                    if let ast::DeclDesc::Enum { width, .. } = &td.desc {
                        let ty = get_cxx_scalar_type(*width);
                        self.append(&format!("pdl::packet::Builder::write_{}<{}, {}>(output, static_cast<{}>( {}));", byteorder, ty, width / 8, ty, field_var.unwrap()));
                    } else {
                        self.append(&format!("{}.Serialize(output);", field_var.unwrap()));
                    }
                }
                ast::FieldDesc::Size { field_id, width, .. } => {
                    let ty = get_cxx_scalar_type(*width);
                    let size_expr = self.get_payload_field_size(var, field_id, decl);
                    self.append(&format!("pdl::packet::Builder::write_{}<{}, {}>(output, static_cast<{}>( {}));", byteorder, ty, width / 8, ty, size_expr));
                }
                ast::FieldDesc::Count { field_id, width, .. } => {
                    let ty = get_cxx_scalar_type(*width);
                    let f = self.scope.iter_fields(decl).find(|f| f.id() == Some(field_id)).expect("Field not found");
                    self.append(&format!("pdl::packet::Builder::write_{}<{}, {}>(output, static_cast<{}>( {}.size() ));", byteorder, ty, width / 8, ty, get_field_expr(f)));
                }
                ast::FieldDesc::ElementSize { width, .. } => {
                    let ty = get_cxx_scalar_type(*width);
                    self.append(&format!("pdl::packet::Builder::write_{}<{}, {}>(output, static_cast<{}>( 0 )); // TODO", byteorder, ty, width / 8, ty));
                }
                ast::FieldDesc::FixedScalar { width, value } => {
                    let ty = get_cxx_scalar_type(*width);
                    self.append(&format!("pdl::packet::Builder::write_{}<{}, {}>(output, {:#x});", byteorder, ty, width / 8, value));
                }
                ast::FieldDesc::FixedEnum { enum_id, tag_id, .. } => {
                    let td = self.scope.typedef.get(enum_id).unwrap();
                    if let ast::DeclDesc::Enum { width, .. } = &td.desc {
                        let ty = get_cxx_scalar_type(*width);
                        self.append(&format!("pdl::packet::Builder::write_{}<{}, {}>(output, static_cast<{}>( {}::{} ));", byteorder, ty, width / 8, ty, enum_id, tag_id));
                    }
                }
                ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body => {
                     self.append(&format!("output.insert(output.end(), {0}.begin(), {0}.end());", deref(var, "payload_")));
                }
                _ => {}
            }
        }
    }
}

fn generate_packet_view(scope: &analyzer::Scope, schema: &analyzer::Schema, decl: &ast::Decl) -> String {
    let id = decl.id().unwrap();
    let parent_id = decl.parent_id();
    let parent_class = parent_id.map(|p| format!("{}View", p)).unwrap_or_else(|| "pdl::packet::slice".to_string());
    let bytes_initializer = if parent_id.is_some() { "parent.bytes_" } else { "parent" };
    let endianness = scope.file.endianness.value;
    let byteorder = match endianness {
        ast::EndiannessValue::LittleEndian => "le",
        ast::EndiannessValue::BigEndian => "be",
    };

    let mut is_cond_for = HashSet::new();
    for field in decl.fields() {
        if let Some(cond) = &field.cond {
            is_cond_for.insert(cond.id.clone());
        }
    }

    let mut field_members = Vec::new();
    let mut field_accessors = Vec::new();
    let all_fields = get_all_parent_fields(scope, decl).into_iter().chain(decl.fields()).collect::<Vec<_>>();

    let parent_constraints = scope.iter_parents_and_self(decl).flat_map(|d| d.constraints()).collect::<Vec<_>>();
    for field in &all_fields {
        let fid = field.id();
        let constraint = fid.and_then(|fid| parent_constraints.iter().find(|c| c.id == fid));
        let is_constrained = constraint.is_some();

        if is_constrained {
            // Constrained fields still get accessors returning their constant values.
            let fid = field.id().unwrap();
            let accessor_name = to_pascal_case(fid);
            let constraint = constraint.unwrap();
            match &field.desc {
                ast::FieldDesc::Typedef { type_id, .. } => {
                    let type_decl = scope.typedef.get(type_id).unwrap();
                    if let ast::DeclDesc::Enum { .. } = &type_decl.desc {
                        field_accessors.push(format!("    {} Get{}() const {{ return {}::{}; }}\n", type_id, accessor_name, type_id, constraint.tag_id.as_ref().unwrap()));
                    } else {
                         field_accessors.push(format!("    {} Get{}() const {{ return {}; }}\n", type_id, accessor_name, constraint.value.unwrap()));
                    }
                }
                ast::FieldDesc::Scalar { width, .. } => {
                    let ty = get_cxx_scalar_type(*width);
                    field_accessors.push(format!("    {} Get{}() const {{ return {}; }}\n", ty, accessor_name, constraint.value.unwrap()));
                }
                _ => {}
            }
            continue;
        }

        if let Some(fid) = field.id() {
            if is_cond_for.contains(fid) {
                continue;
            }
        }

        match &field.desc {
            ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body => {
                field_members.push("pdl::packet::slice payload_;".to_string());
                field_accessors.push(indent("std::vector<uint8_t> GetPayload() const {\n    _ASSERT_VALID(valid_);\n    return payload_.bytes();\n}\n", 1));
            }
            ast::FieldDesc::Array { id, type_id, width, size, .. } => {
                field_members.push(format!("pdl::packet::slice {}_;", id));
                let accessor_name = to_pascal_case(id);
                let element_type = type_id.as_deref().map(|t| t.to_string()).unwrap_or_else(|| get_cxx_scalar_type(width.unwrap()));
                let array_type = if let Some(s) = size {
                    format!("std::array<{}, {}>", element_type, s)
                } else {
                    format!("std::vector<{}>", element_type)
                };

                let mut accessor_code = Vec::new();
                accessor_code.push(format!("pdl::packet::slice span = {}_;", id));
                if let Some(s) = size {
                    accessor_code.push(format!("{} elements;", array_type));
                    accessor_code.push(format!("for (int n = 0; n < {}; n++) {{", s));
                    if let Some(tid) = type_id {
                         let td = scope.typedef.get(tid).unwrap();
                         match &td.desc {
                             ast::DeclDesc::Enum { width, .. } => {
                                 let backing_type = get_cxx_scalar_type(*width);
                                 accessor_code.push(format!("    elements[n] = {}(span.read_{}<{}, {}>());", tid, byteorder, backing_type, width / 8));
                             }
                             _ => { accessor_code.push(format!("    {}::Parse(span, &elements[n]);", tid)); }
                         }
                    } else {
                        let element_width = width.unwrap();
                        let backing_type = get_cxx_scalar_type(element_width);
                        accessor_code.push(format!("    elements[n] = span.read_{}<{}, {}>();", byteorder, backing_type, element_width / 8));
                    }
                    accessor_code.push("}".to_string());
                } else {
                    accessor_code.push(format!("{} elements;", array_type));
                    let mut count_limit = "span.size() > 0".to_string();
                    for f in &all_fields {
                         if let ast::FieldDesc::Count { field_id, .. } = &f.desc {
                              if field_id == id {
                                   count_limit = format!("elements.size() < {}_count_", id);
                                   break;
                              }
                         }
                    }

                    if let Some(tid) = type_id {
                         let td = scope.typedef.get(tid).unwrap();
                         match &td.desc {
                             ast::DeclDesc::Enum { width, .. } => {
                                 let backing_type = get_cxx_scalar_type(*width);
                                 accessor_code.push(format!("while ({} && span.size() >= {}) {{", count_limit, width / 8));
                                 accessor_code.push(format!("    elements.push_back({}(span.read_{}<{}, {}>()));", tid, byteorder, backing_type, width / 8));
                                 accessor_code.push("}".to_string());
                             }
                             _ => {
                                 accessor_code.push(format!("while ({}) {{", count_limit));
                                 accessor_code.push(format!("    {} element;", tid));
                                 accessor_code.push(format!("    if (!{}::Parse(span, &element)) break;", tid));
                                 accessor_code.push("    elements.emplace_back(std::move(element));".to_string());
                                 accessor_code.push("}".to_string());
                             }
                         }
                    } else {
                        let element_width = width.unwrap();
                        let backing_type = get_cxx_scalar_type(element_width);
                        accessor_code.push(format!("while ({} && span.size() >= {}) {{", count_limit, element_width / 8));
                        accessor_code.push(format!("    elements.push_back(span.read_{}<{}, {}>());", byteorder, backing_type, element_width / 8));
                        accessor_code.push("}".to_string());
                    }
                }
                accessor_code.push("return elements;".to_string());

                field_accessors.push(format!(
                    "    {} Get{}() const {{\n        _ASSERT_VALID(valid_);\n{}\n    }}\n",
                    array_type, accessor_name, indent(&accessor_code.join("\n"), 2)
                ));
            }
            ast::FieldDesc::Scalar { id, width } => {
                let ty = get_cxx_scalar_type(*width);
                let accessor_name = to_pascal_case(id);
                if field.cond.is_some() {
                    field_members.push(format!("std::optional<{}> {}_;", ty, id));
                    field_accessors.push(format!("    std::optional<{}> Get{}() const {{ _ASSERT_VALID(valid_); return {}_; }}\n", ty, accessor_name, id));
                } else {
                    field_members.push(format!("{} {}_;", ty, id));
                    field_accessors.push(format!("    {} Get{}() const {{ _ASSERT_VALID(valid_); return {}_; }}\n", ty, accessor_name, id));
                }
            }
            ast::FieldDesc::Typedef { id, type_id, .. } => {
                let ty = type_id;
                let accessor_name = to_pascal_case(id);
                if field.cond.is_some() {
                    field_members.push(format!("std::optional<{}> {}_;", ty, id));
                    field_accessors.push(format!("    std::optional<{}> Get{}() const {{ _ASSERT_VALID(valid_); return {}_; }}\n", ty, accessor_name, id));
                } else {
                    let type_decl = scope.typedef.get(type_id).unwrap();
                    if let ast::DeclDesc::Enum { tags, .. } = &type_decl.desc {
                        field_members.push(format!("{} {}_{{{}::{}}};", ty, id, ty, tags[0].id()));
                        field_accessors.push(format!("    {} Get{}() const {{ _ASSERT_VALID(valid_); return {}_; }}\n", ty, accessor_name, id));
                    } else {
                        field_members.push(format!("{} {}_;", ty, id));
                        field_accessors.push(format!("    {} const& Get{}() const {{ _ASSERT_VALID(valid_); return {}_; }}\n", ty, accessor_name, id));
                    }
                }
            }
            ast::FieldDesc::Size { field_id, width, .. } => {
                 let ty = get_cxx_scalar_type(*width);
                 let field_name = if field_id == "_payload_" || field_id == "_body_" { "payload" } else { field_id };
                 field_members.push(format!("{} {}_size_ {{0}};", ty, field_name));
            }
            ast::FieldDesc::Count { field_id, width, .. } => {
                 let ty = get_cxx_scalar_type(*width);
                 field_members.push(format!("{} {}_count_ {{0}};", ty, field_id));
            }
            ast::FieldDesc::ElementSize { field_id, width, .. } => {
                 let ty = get_cxx_scalar_type(*width);
                 let field_name = if field_id == "_payload_" || field_id == "_body_" { "payload" } else { field_id };
                 field_members.push(format!("{} {}_element_size_ {{0}};", ty, field_name));
            }
            _ => {}
        }
    }

    let mut field_parsers = Vec::new();
    if parent_id.is_some() {
        field_parsers.push("// Check validity of parent packet.".to_string());
        field_parsers.push("if (!parent.IsValid()) { return false; }".to_string());
        let unconstrained_parent_fields = get_unconstrained_parent_fields(scope, decl);
        if !unconstrained_parent_fields.is_empty() {
            field_parsers.push("// Copy parent field values.".to_string());
            for f in unconstrained_parent_fields {
                if let Some(id) = f.id() {
                    field_parsers.push(format!("{}_ = parent.{}_;", id, id));
                }
            }
        }
    }

    if decl.fields().next().is_some() {
        field_parsers.push("// Parse packet field values.".to_string());
        let span = if parent_id.is_some() { "parent.payload_" } else { "parent" };
        field_parsers.push(format!("pdl::packet::slice span = {};", span));

        let mut parser = FieldParser::new(scope, schema, endianness, "", false, decl);
        parser.is_cond_for = is_cond_for;
        for f in decl.fields() {
            parser.parse(f);
        }
        parser.done();
        field_parsers.extend(parser.code);
    }
    field_parsers.push("return true;".to_string());

    let friend_classes = scope.iter_children(decl)
        .filter_map(|child| child.id().map(|id| format!("friend class {}View;", id)))
        .collect::<Vec<_>>();

    format!(
        r#"
class {id}View {{
public:
    static {id}View Create({parent_class} const& parent) {{
        return {id}View(parent);
    }}

{field_accessors}
    std::string ToString() const {{ return ""; }}

    bool IsValid() const {{
        return valid_;
    }}

    pdl::packet::slice bytes() const {{
        return bytes_;
    }}

protected:
    explicit {id}View({parent_class} const& parent)
          : bytes_({bytes_initializer}) {{
        valid_ = Parse(parent);
    }}

    bool Parse({parent_class} const& parent) {{
{field_parsers}
    }}

    bool valid_{{false}};
    pdl::packet::slice bytes_;
{field_members}

{friend_classes}
}};
"#,
        id = id,
        parent_class = parent_class,
        bytes_initializer = bytes_initializer,
        field_parsers = indent(&field_parsers.join("\n"), 2),
        field_members = indent(&field_members.join("\n"), 1),
        field_accessors = field_accessors.join("\n"),
        friend_classes = indent(&friend_classes.join("\n"), 1)
    )
}
fn generate_packet_builder(scope: &analyzer::Scope, schema: &analyzer::Schema, decl: &ast::Decl) -> String {
    let id = decl.id().unwrap();
    let class_name = format!("{}Builder", id);
    let endianness = scope.file.endianness.value;

    let mut field_members = Vec::new();
    let mut constructor_params: Vec<String> = Vec::new();
    let mut constructor_inits = Vec::new();

    let parent_constraints = scope.iter_parents_and_self(decl).flat_map(|d| d.constraints()).collect::<Vec<_>>();
    let all_fields = get_all_parent_fields(scope, decl).into_iter().chain(decl.fields()).collect::<Vec<_>>();

    for field in &all_fields {
        let fid = field.id();
        let is_constrained = fid.map(|fid| parent_constraints.iter().any(|c| c.id == fid)).unwrap_or(false);
        if is_constrained {
            continue;
        }

        match &field.desc {
            ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body => {
                field_members.push("std::vector<uint8_t> payload_;".to_string());
                constructor_params.push("std::vector<uint8_t> payload".to_string());
                constructor_inits.push("payload_(std::move(payload))".to_string());
            }
            ast::FieldDesc::Array { id, width, type_id, size, .. } => {
                let element_type = type_id.as_deref().map(|t| t.to_string()).unwrap_or_else(|| get_cxx_scalar_type(width.unwrap()));
                if let Some(s) = size {
                    field_members.push(format!("std::array<{}, {}> {}_;", element_type, s, id));
                    constructor_params.push(format!("std::array<{}, {}> {}", element_type, s, id));
                } else {
                    field_members.push(format!("std::vector<{}> {}_;", element_type, id));
                    constructor_params.push(format!("std::vector<{}> {}", element_type, id));
                }
                constructor_inits.push(format!("{0}_(std::move({0}))", id));
            }
            ast::FieldDesc::Scalar { id, width } => {
                let ty = get_cxx_scalar_type(*width);
                if field.cond.is_some() {
                    field_members.push(format!("std::optional<{}> {}_;", ty, id));
                    constructor_params.push(format!("std::optional<{}> {}", ty, id));
                } else {
                    field_members.push(format!("{} {}_{{0}};", ty, id));
                    constructor_params.push(format!("{} {}", ty, id));
                }
                constructor_inits.push(format!("{0}_(std::move({0}))", id));
            }
            ast::FieldDesc::Typedef { id, type_id, .. } => {
                let ty = type_id;
                if field.cond.is_some() {
                    field_members.push(format!("std::optional<{}> {}_;", ty, id));
                    constructor_params.push(format!("std::optional<{}> {}", ty, id));
                } else {
                    let type_decl = scope.typedef.get(type_id).unwrap();
                    if let ast::DeclDesc::Enum { tags, .. } = &type_decl.desc {
                        field_members.push(format!("{} {}_{{{}::{}}};", ty, id, ty, tags[0].id()));
                    } else {
                        field_members.push(format!("{} {}_;", ty, id));
                    }
                    constructor_params.push(format!("{} {}", ty, id));
                }
                constructor_inits.push(format!("{0}_(std::move({0}))", id));
            }
            ast::FieldDesc::Size { field_id, width, .. } => {
                let ty = get_cxx_scalar_type(*width);
                let field_name = if field_id == "_payload_" || field_id == "_body_" { "payload" } else { field_id };
                field_members.push(format!("{} {}_size_ {{0}};", ty, field_name));
            }
            ast::FieldDesc::Count { field_id, width, .. } => {
                let ty = get_cxx_scalar_type(*width);
                field_members.push(format!("{} {}_count_ {{0}};", ty, field_id));
            }
            ast::FieldDesc::ElementSize { field_id, width, .. } => {
                let ty = get_cxx_scalar_type(*width);
                let field_name = if field_id == "_payload_" || field_id == "_body_" { "payload" } else { field_id };
                field_members.push(format!("{} {}_element_size_ {{0}};", ty, field_name));
            }
            _ => {}
        }
    }

    let mut serializer = FieldSerializer::new(scope, schema, endianness);
    for f in &all_fields {
        serializer.serialize(f, decl, None);
    }
    let field_serializers = serializer.code;

    let mut variable_widths = Vec::new();
    let mut static_bits = 0;

    let get_field_expr = |f: &ast::Field| -> String {
        if let Some(fid) = f.id() {
            if let Some(constraint) = parent_constraints.iter().find(|c| c.id == fid) {
                return format!("{:#x}", constraint.value.unwrap());
            }
        }
        match &f.desc {
            ast::FieldDesc::Flag { optional_field_ids, .. } => {
                let (opt_id, val_present) = &optional_field_ids[0];
                let val_absent = if *val_present == 0 { 1 } else { 0 };
                format!("({0}_.has_value() ? {1} : {2})", opt_id, val_present, val_absent)
            }
            _ => format!("{}_", f.id().unwrap()),
        }
    };

    for f in &all_fields {
        let field_size = schema.field_size(f.key);
        match &f.desc {
            ast::FieldDesc::Scalar { width, .. } => {
                if let Some(cond) = &f.cond {
                    let cond_field = scope.iter_fields(decl).find(|f| f.id() == Some(&cond.id)).expect("Cond field not found");
                    let cond_field_var = get_field_expr(cond_field);
                    variable_widths.push(format!("(({} == {}) ? {} : 0)", cond_field_var, cond.value.unwrap(), width / 8));
                } else {
                    static_bits += width;
                }
            }
            ast::FieldDesc::Typedef { id: _, type_id, .. } => {
                let type_decl = scope.typedef.get(type_id).unwrap();
                let width = match &type_decl.desc {
                    ast::DeclDesc::Enum { width, .. } => *width,
                    _ => 0,
                };
                if let Some(cond) = &f.cond {
                    let cond_field = scope.iter_fields(decl).find(|f| f.id() == Some(&cond.id)).expect("Cond field not found");
                    let cond_field_var = get_field_expr(cond_field);
                    if width > 0 {
                         variable_widths.push(format!("(({} == {}) ? {} : 0)", cond_field_var, cond.value.unwrap(), width / 8));
                    } else {
                         variable_widths.push(format!("(({} == {}) ? {}_->GetSize() : 0)", cond_field_var, cond.value.unwrap(), f.id().unwrap()));
                    }
                } else if width > 0 {
                    static_bits += width;
                } else {
                    variable_widths.push(format!("{}_.GetSize()", f.id().unwrap()));
                }
            }
            ast::FieldDesc::Array { id, width, type_id, .. } => {
                let padded_size = schema.padded_size(f.key);
                let array_size = if let Some(w) = width {
                    format!("({0}_.size() * {1})", id, w / 8)
                } else if let Some(tid) = type_id {
                    let td = scope.typedef.get(tid).unwrap();
                    match &td.desc {
                        ast::DeclDesc::Enum { width, .. } => format!("({0}_.size() * {1})", id, width / 8),
                        _ => format!("std::accumulate({0}_.begin(), {0}_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) {{ return s + e.GetSize(); }})", id),
                    }
                } else {
                    format!("std::accumulate({0}_.begin(), {0}_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) {{ return s + e.GetSize(); }})", id)
                };

                if let Some(ps) = padded_size {
                    variable_widths.push(format!("std::max<size_t>({}, {})", array_size, ps / 8));
                } else {
                    variable_widths.push(array_size);
                }
            }
            ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body => {
                 variable_widths.push("payload_.size()".to_string());
            }
            _ => {
                if let analyzer::Size::Static(bits) = field_size {
                    static_bits += bits;
                }
            }
        }
    }

    let size_expr = if variable_widths.is_empty() {
        format!("return {};", static_bits / 8)
    } else if static_bits > 0 {
        format!("return {} + ({});", static_bits / 8, variable_widths.join(" + "))
    } else {
        format!("return {};", variable_widths.join(" + "))
    };

    let constructor = if constructor_params.is_empty() {
        format!("    {0}() = default;", class_name)
    } else {
        format!("    {0}() = default;\n    explicit {0}({1}) : {2} {{}}", class_name, constructor_params.join(", "), constructor_inits.join(", "))
    };

    format!(
        r#"
class {class_name} : public pdl::packet::Builder {{
public:
    ~{class_name}() override = default;
{constructor}
    {class_name}({class_name} const&) = default;
    {class_name}({class_name}&&) = default;
    {class_name}& operator=({class_name} const&) = default;

    void Serialize(std::vector<uint8_t>& output) const override {{
{field_serializers}
    }}

    size_t GetSize() const override {{
        {size_expr}
    }}

    std::string ToString() const {{ return ""; }}

{field_members}
}};
"#,
        class_name = class_name,
        constructor = constructor,
        field_serializers = indent(&field_serializers.join("\n"), 2),
        size_expr = size_expr,
        field_members = indent(&field_members.join("\n"), 1)
    )
}
fn generate_struct_declaration(scope: &analyzer::Scope, schema: &analyzer::Schema, decl: &ast::Decl) -> String {
    let id = decl.id().unwrap();
    let mut field_members = Vec::new();
    let mut constructor_params: Vec<String> = Vec::new();
    let mut constructor_inits = Vec::new();
    let endianness = scope.file.endianness.value;

    for field in decl.fields() {
         match &field.desc {
            ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body => {
                field_members.push("std::vector<uint8_t> payload_;".to_string());
                constructor_params.push("std::vector<uint8_t> payload".to_string());
                constructor_inits.push("payload_(std::move(payload))".to_string());
            }
            ast::FieldDesc::Array { id, width, type_id, size, .. } => {
                let element_type = type_id.as_deref().map(|t| t.to_string()).unwrap_or_else(|| get_cxx_scalar_type(width.unwrap()));
                if let Some(s) = size {
                    field_members.push(format!("std::array<{}, {}> {}_;", element_type, s, id));
                    constructor_params.push(format!("std::array<{}, {}> {}", element_type, s, id));
                } else {
                    field_members.push(format!("std::vector<{}> {}_;", element_type, id));
                    constructor_params.push(format!("std::vector<{}> {}", element_type, id));
                }
                constructor_inits.push(format!("{0}_(std::move({0}))", id));
            }
            ast::FieldDesc::Scalar { id, width } => {
                let ty = get_cxx_scalar_type(*width);
                if field.cond.is_some() {
                    field_members.push(format!("std::optional<{}> {}_;", ty, id));
                    constructor_params.push(format!("std::optional<{}> {}", ty, id));
                } else {
                    field_members.push(format!("{} {}_{{0}};", ty, id));
                    constructor_params.push(format!("{} {}", ty, id));
                }
                constructor_inits.push(format!("{0}_(std::move({0}))", id));
            }
            ast::FieldDesc::Typedef { id, type_id, .. } => {
                let ty = type_id;
                if field.cond.is_some() {
                    field_members.push(format!("std::optional<{}> {}_;", ty, id));
                    constructor_params.push(format!("std::optional<{}> {}", ty, id));
                } else {
                    let type_decl = scope.typedef.get(type_id).unwrap();
                    if let ast::DeclDesc::Enum { tags, .. } = &type_decl.desc {
                        field_members.push(format!("{} {}_{{{}::{}}};", ty, id, ty, tags[0].id()));
                    } else {
                        field_members.push(format!("{} {}_;", ty, id));
                    }
                    constructor_params.push(format!("{} {}", ty, id));
                }
                constructor_inits.push(format!("{0}_(std::move({0}))", id));
            }
            ast::FieldDesc::Size { field_id, width, .. } => {
                let ty = get_cxx_scalar_type(*width);
                let field_name = if field_id == "_payload_" || field_id == "_body_" { "payload" } else { field_id };
                field_members.push(format!("{} {}_size_ {{0}};", ty, field_name));
            }
            ast::FieldDesc::Count { field_id, width, .. } => {
                let ty = get_cxx_scalar_type(*width);
                field_members.push(format!("{} {}_count_ {{0}};", ty, field_id));
            }
            ast::FieldDesc::ElementSize { field_id, width, .. } => {
                let ty = get_cxx_scalar_type(*width);
                let field_name = if field_id == "_payload_" || field_id == "_body_" { "payload" } else { field_id };
                field_members.push(format!("{} {}_element_size_ {{0}};", ty, field_name));
            }
            _ => {}
        }
    }

    let mut field_parsers = Vec::new();
    field_parsers.push("pdl::packet::slice span = parent_span;".to_string());

    let mut parser = FieldParser::new(scope, schema, endianness, "output->", true, decl);
    for f in decl.fields() {
        parser.parse(f);
    }
    parser.done();
    field_parsers.extend(parser.code);
    field_parsers.push("parent_span = span;".to_string());
    field_parsers.push("return true;".to_string());

    let mut serializer = FieldSerializer::new(scope, schema, endianness);
    for f in decl.fields() {
        serializer.serialize(f, decl, None);
    }
    let field_serializers = serializer.code;

    let mut variable_widths = Vec::new();
    let mut static_bits = 0;

    let get_field_expr = |f: &ast::Field| -> String {
        match &f.desc {
            ast::FieldDesc::Flag { optional_field_ids, .. } => {
                let (opt_id, val_present) = &optional_field_ids[0];
                let val_absent = if *val_present == 0 { 1 } else { 0 };
                format!("({0}_.has_value() ? {1} : {2})", opt_id, val_present, val_absent)
            }
            _ => format!("{}_", f.id().unwrap()),
        }
    };

    for f in decl.fields() {
        let field_size = schema.field_size(f.key);
        match &f.desc {
            ast::FieldDesc::Scalar { width, .. } => {
                if let Some(cond) = &f.cond {
                    let cond_field = scope.iter_fields(decl).find(|f| f.id() == Some(&cond.id)).expect("Cond field not found");
                    let cond_field_var = get_field_expr(cond_field);
                    variable_widths.push(format!("(({} == {}) ? {} : 0)", cond_field_var, cond.value.unwrap(), width / 8));
                } else {
                    static_bits += width;
                }
            }
            ast::FieldDesc::Typedef { id: _, type_id, .. } => {
                let type_decl = scope.typedef.get(type_id).unwrap();
                let width = match &type_decl.desc {
                    ast::DeclDesc::Enum { width, .. } => *width,
                    _ => 0,
                };
                if let Some(cond) = &f.cond {
                    let cond_field = scope.iter_fields(decl).find(|f| f.id() == Some(&cond.id)).expect("Cond field not found");
                    let cond_field_var = get_field_expr(cond_field);
                    if width > 0 {
                         variable_widths.push(format!("(({} == {}) ? {} : 0)", cond_field_var, cond.value.unwrap(), width / 8));
                    } else {
                         variable_widths.push(format!("(({} == {}) ? {}_->GetSize() : 0)", cond_field_var, cond.value.unwrap(), f.id().unwrap()));
                    }
                } else if width > 0 {
                    static_bits += width;
                } else {
                    variable_widths.push(format!("{}_.GetSize()", f.id().unwrap()));
                }
            }
            ast::FieldDesc::Array { id, width, type_id, .. } => {
                let padded_size = schema.padded_size(f.key);
                let array_size = if let Some(w) = width {
                    format!("({0}_.size() * {1})", id, w / 8)
                } else if let Some(tid) = type_id {
                    let td = scope.typedef.get(tid).unwrap();
                    match &td.desc {
                        ast::DeclDesc::Enum { width, .. } => format!("({0}_.size() * {1})", id, width / 8),
                        _ => format!("std::accumulate({0}_.begin(), {0}_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) {{ return s + e.GetSize(); }})", id),
                    }
                } else {
                    format!("std::accumulate({0}_.begin(), {0}_.end(), static_cast<size_t>(0), [](size_t s, auto const& e) {{ return s + e.GetSize(); }})", id)
                };

                if let Some(ps) = padded_size {
                    variable_widths.push(format!("std::max<size_t>({}, {})", array_size, ps / 8));
                } else {
                    variable_widths.push(array_size);
                }
            }
            ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body => {
                 variable_widths.push("payload_.size()".to_string());
            }
            _ => {
                if let analyzer::Size::Static(bits) = field_size {
                    static_bits += bits;
                }
            }
        }
    }

    let size_expr = if variable_widths.is_empty() {
        format!("return {};", static_bits / 8)
    } else if static_bits > 0 {
        format!("return {} + ({});", static_bits / 8, variable_widths.join(" + "))
    } else {
        format!("return {};", variable_widths.join(" + "))
    };

    let constructor = if constructor_params.is_empty() {
        format!("    {0}() = default;", id)
    } else {
        format!("    {0}() = default;\n    explicit {0}({1}) : {2} {{}}", id, constructor_params.join(", "), constructor_inits.join(", "))
    };

    format!(
        r#"
class {id} : public pdl::packet::Builder {{
public:
    ~{id}() override = default;
{constructor}
    {id}({id} const&) = default;
    {id}({id}&&) = default;
    {id}& operator=({id} const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, {id}* output) {{
{struct_parse_code}
    }}

    void Serialize(std::vector<uint8_t>& output) const override {{
{field_serializers}
    }}

    size_t GetSize() const override {{
        {size_expr}
    }}

    std::string ToString() const {{ return ""; }}

{field_members}
}};
"#,
        id = id,
        constructor = constructor,
        struct_parse_code = indent(&field_parsers.join("\n"), 2),
        field_serializers = indent(&field_serializers.join("\n"), 2),
        size_expr = size_expr,
        field_members = indent(&field_members.join("\n"), 1)
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::assert_snapshot_eq;
    use crate::{analyzer, ast, parser};

    #[test]
    fn test_canonical() {
        let mut db = ast::SourceDatabase::new();
        let input_file = "tests/canonical/le_test_file.pdl";
        let file = parser::parse_file(&mut db, input_file).unwrap();
        let file = analyzer::analyze(&file).unwrap();
        let actual_code = generate(
            &db,
            &file,
            Some("le_backend"),
            &[],
            &[],
            &[
                "Packet_Custom_Field_ConstantSize".to_string(),
                "Packet_Custom_Field_VariableSize".to_string(),
                "Packet_Checksum_Field_FromStart".to_string(),
                "Packet_Checksum_Field_FromEnd".to_string(),
                "Packet_Array_Field_VariableElementSize_ConstantSize".to_string(),
                "Packet_Array_Field_VariableElementSize_VariableSize".to_string(),
                "Packet_Array_Field_VariableElementSize_VariableCount".to_string(),
                "Packet_Array_Field_VariableElementSize_UnknownSize".to_string(),
                "Struct_Custom_Field_ConstantSize".to_string(),
                "Struct_Custom_Field_VariableSize".to_string(),
                "Struct_Checksum_Field_FromStart".to_string(),
                "Struct_Checksum_Field_FromEnd".to_string(),
                "Struct_Custom_Field_ConstantSize_".to_string(),
                "Struct_Custom_Field_VariableSize_".to_string(),
                "Struct_Checksum_Field_FromStart_".to_string(),
                "Struct_Checksum_Field_FromEnd_".to_string(),
                "PartialParent5".to_string(),
                "PartialChild5_A".to_string(),
                "PartialChild5_B".to_string(),
                "PartialParent12".to_string(),
                "PartialChild12_A".to_string(),
                "PartialChild12_B".to_string(),
            ],
        );
        assert_snapshot_eq("tests/generated/cxx/le_backend.h", &actual_code);
    }
}
