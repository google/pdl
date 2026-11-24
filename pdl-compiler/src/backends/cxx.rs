// Copyright (C) 2026 Google LLC
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

use crate::{analyzer, ast, backends::common};
use heck::ToUpperCamelCase;
use std::collections::{HashMap, HashSet};

// Return the maximum value for the scalar type.
fn scalar_max(width: usize) -> usize {
    if width >= usize::BITS as usize { usize::MAX } else { (1 << width) - 1 }
}

fn indent(s: &str, level: usize) -> String {
    let prefix = "    ".repeat(level);
    s.lines()
        .map(|line| if line.is_empty() { line.to_string() } else { format!("{}{}", prefix, line) })
        .collect::<Vec<_>>()
        .join("\n")
}

fn mask(width: usize) -> String {
    format!("{:#x}", (1u128 << width) - 1)
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

    code.push_str(&format!(
        r#"// File generated from {source_name}, with the command
//   pdlc --output-format cxx {source_name}
// /!\ Do not edit by hand

#pragma once

#include <cstdint>
#include <string>
#include <optional>
#include <utility>
#include <vector>
#include <array>
#include <numeric>

#include <packet_runtime.h>
"#,
        source_name = source.name()
    ));

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

    code.push_str(
        r#"
#ifndef _ASSERT_VALID
#ifdef ASSERT
#define _ASSERT_VALID ASSERT
#else
#include <cassert>
#define _ASSERT_VALID assert
#endif  // ASSERT
#endif  // !_ASSERT_VALID

"#,
    );

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

    // Packets and structs must be generated in dependency order: C++ types
    // must be defined before they are used. `analyzer::analyze` reorders
    // declarations according to this constraint.
    for decl in &file.declarations {
        if let Some(id) = decl.id() {
            if exclude_declarations.contains(&id.to_string()) {
                continue;
            }
        }
        match &decl.desc {
            ast::DeclDesc::Enum { id, tags, width } => {
                code.push_str(&generate_enum_declaration(id, tags, *width));
                code.push_str(&generate_enum_to_string(id, tags, *width));
                code.push_str(&generate_enum_is_valid(id, tags, *width));
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

// Generate the default enum value.
// The default value is the first tag of the enum that is not the default
// (`Other`) tag. If the first tag identifies a range, then the first value
// of the range is used. If the enum declares no tag other than the default
// tag, the default tag is used.
fn default_enum_value(type_id: &str, tags: &[ast::Tag]) -> String {
    match tags.iter().find(|tag| !matches!(tag, ast::Tag::Other(_))) {
        Some(ast::Tag::Value(ast::TagValue { id, .. })) => format!("{type_id}::{id}"),
        Some(ast::Tag::Range(ast::TagRange { tags, .. })) if !tags.is_empty() => {
            format!("{type_id}::{}", &tags[0].id)
        }
        Some(ast::Tag::Range(ast::TagRange { range, .. })) => {
            format!("{type_id}({})", range.start())
        }
        // `find` skips `Tag::Other`, so the remaining case is an enum whose
        // only declared tag is the default tag.
        _ => format!("{type_id}(0)"),
    }
}

fn generate_enum_declaration(id: &str, tags: &[ast::Tag], width: usize) -> String {
    let enum_type = get_cxx_scalar_type(width);
    let mut tag_decls = Vec::new();
    for tag in tags {
        match tag {
            ast::Tag::Value(t) => tag_decls.push(format!("{} = {:#x},", t.id, t.value)),
            ast::Tag::Range(t) => {
                // Generate enum tags for named values in the range.
                // TBD: optionally generate values for the enum start and end values.
                for t in &t.tags {
                    tag_decls.push(format!("{} = {:#x},", t.id, t.value));
                }
            }
            // C++ enums are open, the default tag is not declared.
            ast::Tag::Other(_) => (),
        }
    }

    format!(
        r#"
enum class {id} : {enum_type} {{
{tag_declarations}
}};
"#,
        tag_declarations = indent(&tag_decls.join("\n"), 1)
    )
}

// Generate the string conversion method for an enum declaration.
fn generate_enum_to_string(id: &str, tags: &[ast::Tag], width: usize) -> String {
    let mut tag_cases = Vec::new();
    let mut range_matches = Vec::new();
    let mut other = "UNKNOWN";
    let enum_type = get_cxx_scalar_type(width);
    for tag in tags {
        match tag {
            ast::Tag::Value(t) => {
                tag_cases.push(format!("case {}::{}: return \"{}\";", id, t.id, t.id))
            }
            ast::Tag::Range(t) => {
                tag_cases.extend(
                    t.tags.iter().map(|t| format!("case {}::{}: return \"{}\";", id, t.id, t.id)),
                );
                if !common::is_complete_enum_range(t) {
                    range_matches.push(format!(
                        r#"if ({enum_type}(tag) >= {start} && {enum_type}(tag) <= {end}) {{
    return std::string("{id}(" + std::to_string({enum_type}(tag)) + ")");
}}"#,
                        start = t.range.start(),
                        end = t.range.end(),
                        id = t.id,
                        enum_type = enum_type,
                    ))
                }
            }
            ast::Tag::Other(t) => {
                other = &t.id;
            }
        }
    }

    format!(
        r#"
inline std::string {id}ToString({id} tag) {{
    switch (tag) {{
{tag_cases}
        default:
            break;
    }}
{range_matches}
    return std::string("{other}(" +
           std::to_string({enum_type}(tag)) + ")");
}}
"#,
        tag_cases = indent(&tag_cases.join("\n"), 2),
        range_matches = indent(&range_matches.join("\n"), 1),
        other = other,
        enum_type = enum_type,
    )
}

/// Generate the validation function for enum values.
fn generate_enum_is_valid(id: &str, tags: &[ast::Tag], width: usize) -> String {
    if common::is_open_enum(tags) || common::is_complete_enum(tags, scalar_max(width)) {
        return String::new();
    }

    let backing_type = get_cxx_scalar_type(width);
    let has_ranges = tags.iter().any(|t| matches!(t, ast::Tag::Range(_)));

    if has_ranges {
        let condition = tags
            .iter()
            .map(|tag| match tag {
                ast::Tag::Value(t) => format!("value == {:#x}", t.value),
                ast::Tag::Range(t) => {
                    format!("({:#x} <= value && value <= {:#x})", t.range.start(), t.range.end())
                }
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()
            .join("\n        || ");

        format!(
            r#"
inline bool IsValid{id}({backing_type} value) {{
    return {condition};
}}
"#,
        )
    } else {
        let tag_cases = tags
            .iter()
            .map(|tag| match tag {
                ast::Tag::Value(t) => format!("case {:#x}:", t.value),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()
            .join("\n        ");

        format!(
            r#"
inline bool IsValid{id}({backing_type} value) {{
    switch (value) {{
        {tag_cases}
            return true;
        default:
            return false;
    }}
}}
"#,
        )
    }
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

        let should_skip_value =
            self.chunk.iter().all(|(_, _, f)| matches!(f.desc, ast::FieldDesc::Reserved { .. }));
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
                    self.unchecked_append(format!(
                        "if (static_cast<uint64_t>({}) != {:#x}) {{",
                        v, fixed_value
                    ));
                    self.unchecked_append("    return false;".to_string());
                    self.unchecked_append("}".to_string());
                }
                ast::FieldDesc::FixedEnum { enum_id, tag_id, .. } => {
                    self.unchecked_append(format!(
                        "if ({}({}) != {}::{}) {{",
                        enum_id, v, enum_id, tag_id
                    ));
                    self.unchecked_append("    return false;".to_string());
                    self.unchecked_append("}".to_string());
                }
                ast::FieldDesc::Typedef { id, type_id, .. } => {
                    let type_decl = self.scope.typedef.get(type_id).unwrap();
                    match &type_decl.desc {
                        ast::DeclDesc::Enum { tags, width, .. }
                            if common::is_open_enum(tags)
                                || common::is_complete_enum(tags, scalar_max(*width)) =>
                        {
                            self.unchecked_append(format!(
                                "{}{}_ = {}({});",
                                self.target_prefix, id, type_id, v
                            ));
                        }
                        ast::DeclDesc::Enum { .. } => {
                            // The raw value is named after the field: multiple enum
                            // fields can be extracted from the same chunk.
                            self.unchecked_append(format!("auto raw_{id} = {v};"));
                            self.unchecked_append(format!("if (!IsValid{type_id}(raw_{id})) {{"));
                            self.unchecked_append("   return false;".to_string());
                            self.unchecked_append("}".to_string());
                            self.unchecked_append(format!(
                                "{}{}_ = {}(raw_{id});",
                                self.target_prefix, id, type_id
                            ));
                        }
                        _ => {
                            self.unchecked_append(format!(
                                "{}{}_ = {};",
                                self.target_prefix, id, v
                            ));
                        }
                    }
                }
                ast::FieldDesc::Size { field_id, .. } => {
                    let field_name = if field_id == "_payload_" || field_id == "_body_" {
                        "payload"
                    } else {
                        field_id
                    };
                    self.unchecked_append(format!("size_t {}_size_ = {};", field_name, v));
                }
                ast::FieldDesc::Count { field_id, .. } => {
                    self.unchecked_append(format!(
                        "{}{}_count_ = {};",
                        self.target_prefix, field_id, v
                    ));
                }
                ast::FieldDesc::ElementSize { field_id, .. } => {
                    let field_name = if field_id == "_payload_" || field_id == "_body_" {
                        "payload"
                    } else {
                        field_id
                    };
                    self.unchecked_append(format!(
                        "{}{}_element_size_ = {};",
                        self.target_prefix, field_name, v
                    ));
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

        let field_size = self.schema.field_size(field.key);
        if let analyzer::Size::Unknown = field_size {
            let trailing_size = self.get_trailing_size(field).unwrap_or(0);
            if trailing_size > 0 {
                self.append(format!("if (span.size() < {}) return false;", trailing_size / 8));
                self.append(format!("size_t {id}_size = span.size() - {};", trailing_size / 8));
                self.append(
                    format!("pdl::packet::slice {id}_span = span.subrange(0, {id}_size);",),
                );
                self.append(format!(
                    "if (!{type_id}::Parse({id}_span, &{}{id}_)) return false;",
                    self.target_prefix
                ));
                // The bytes remaining after parsing the typedef field are added
                // back to the input span.
                self.append(format!("span.skip({id}_size - {id}_span.size());"));
            } else {
                self.append(format!(
                    "if (!{type_id}::Parse(span, &{}{id}_)) return false;",
                    self.target_prefix
                ));
            }
        } else {
            self.append(format!(
                "if (!{type_id}::Parse(span, &{}{id}_)) return false;",
                self.target_prefix,
            ));
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
                if let ast::DeclDesc::Enum { width, tags, .. } = &type_decl.desc {
                    let backing_type = get_cxx_scalar_type(*width);
                    let is_open = common::is_open_enum(tags);
                    let size = width / 8;
                    self.append(format!("if ({} == {}) {{", cond.id, cond_value));
                    self.append(format!("    if (span.size() < {}) {{", size));
                    self.append("        return false;".to_string());
                    self.append("    }".to_string());
                    if is_open {
                        self.append(format!(
                            "    {}{}_ = {}(span.read_{}<{}, {}>());",
                            self.target_prefix, id, type_id, byteorder, backing_type, size
                        ));
                        self.append("}".to_string());
                    } else {
                        self.append(format!(
                            "    auto raw_value = span.read_{}<{}, {}>();",
                            byteorder, backing_type, size
                        ));
                        self.append(format!("    if (!IsValid{type_id}(raw_value)) {{"));
                        self.append("       return false;".to_string());
                        self.append("    }".to_string());
                        self.append(format!(
                            "    {}{}_ = {}(raw_value);",
                            self.target_prefix, id, type_id
                        ));
                        self.append("}".to_string());
                    }
                } else {
                    self.append(format!("if ({} == {}) {{", cond.id, cond_value));
                    self.append(format!(
                        "    auto& opt_output = {}{}_.emplace();",
                        self.target_prefix, id
                    ));
                    self.append(format!("    if (!{}::Parse(span, &opt_output)) {{", type_id));
                    self.append("        return false;".to_string());
                    self.append("    }".to_string());
                    self.append("}".to_string());
                }
            }
            _ => panic!("Unsupported optional field type"),
        }
    }

    fn parse_array_field_lite(
        &mut self,
        field: &'a ast::Field,
        id: &str,
        type_id: Option<&str>,
        size_modifier: Option<&str>,
    ) {
        self.check_code();

        let element_size = analyzer::element_size(self.scope, self.schema, self.decl, field);
        let array_size = analyzer::array_size(self.decl, field);

        // Apply the size modifier.
        if let Some(size_modifier) = size_modifier {
            self.append(format!("{id}_size_ = {id}_size_ - {size_modifier};"));
        }

        // TODO element validation
        use analyzer::{ArraySize, ElementSize};
        match (element_size, array_size) {
            (ElementSize::Static(element_size), ArraySize::StaticCount(count)) => {
                let total_size = element_size * count;
                self.check_size(&format!("{total_size}"));
                self.append(format!("{id}_ = span.subrange(0, {total_size});"));
                self.append(format!("span.skip({total_size});"));
            }

            (ElementSize::Static(element_size), ArraySize::DynamicCount) => {
                self.check_size(&format!("{element_size} * {id}_count_"));
                self.append(format!("{id}_ = span.subrange(0, {element_size} * {id}_count_);"));
                self.append(format!("span.skip({element_size} * {id}_count_);"));
            }

            (ElementSize::Static(element_size), ArraySize::DynamicSize) => {
                self.check_size(&format!("{id}_size_"));
                self.append(format!("if (({id}_size_ % {element_size}) != 0) {{"));
                self.append("    return false;".to_string());
                self.append("}".to_string());
                self.append(format!("{id}_ = span.subrange(0, {id}_size_);"));
                self.append(format!("span.skip({id}_size_);"));
            }

            (ElementSize::Static(element_size), ArraySize::Unknown) => {
                self.append(format!("if ((span.size() % {element_size}) != 0) {{"));
                self.append("    return false;".to_string());
                self.append("}".to_string());
                self.append(format!("{id}_ = span;"));
                self.append("span.clear();".to_string());
            }

            (ElementSize::Dynamic, ArraySize::StaticCount(count)) => {
                self.append(format!("if ({id}_element_size_ == 0) {{"));
                self.append("    return false;".to_string());
                self.append("}".to_string());
                self.check_size(&format!("{id}_element_size_ * {count}"));
                self.append(format!("{id}_ = span.subrange(0, {id}_element_size_ * {count});"));
                self.append(format!("span.skip({id}_element_size_ * {count});"));
            }

            (ElementSize::Dynamic, ArraySize::DynamicCount) => {
                self.append(format!("if ({id}_element_size_ == 0) {{"));
                self.append("    return false;".to_string());
                self.append("}".to_string());
                self.check_size(&format!("{id}_element_size_ * {id}_count_"));
                self.append(format!("{id}_ = span.subrange(0, {id}_element_size_ * {id}_count_);"));
                self.append(format!("span.skip({id}_element_size_ * {id}_count_);"));
            }

            (ElementSize::Dynamic, ArraySize::DynamicSize) => {
                self.check_size(&format!("{id}_size_"));
                self.append(format!(
                    "if ({id}_element_size_ == 0 || ({id}_size_ % {id}_element_size_) != 0) {{"
                ));
                self.append("    return false;".to_string());
                self.append("}".to_string());
                self.append(format!("{id}_ = span.subrange(0, {id}_size_);"));
                self.append(format!("span.skip({id}_size_);"));
            }

            (ElementSize::Dynamic, ArraySize::Unknown) => {
                self.append(format!(
                    "if ({id}_element_size_ == 0 || (span.size() % {id}_element_size_) != 0) {{"
                ));
                self.append("    return false;".to_string());
                self.append("}".to_string());
                self.append(format!("{id}_ = span;"));
                self.append("span.clear();".to_string());
            }

            (ElementSize::Unknown, ArraySize::StaticCount(count)) => {
                let tid = type_id.unwrap();
                self.append(format!("{id}_ = span;"));
                self.append(format!("for (size_t n = 0; n < {count}; n++) {{"));
                self.append(format!("    {tid} out;"));
                self.append(format!("    if (!{tid}::Parse(span, &out)) {{"));
                self.append("        return false;".to_string());
                self.append("    }".to_string());
                self.append("}".to_string());
                self.append(format!("{id}_ = {id}_.subrange(0, {id}_.size() - span.size());"));
            }

            (ElementSize::Unknown, ArraySize::DynamicCount) => {
                let tid = type_id.unwrap();
                self.append(format!("{id}_ = span;"));
                self.append(format!("for (size_t n = 0; n < {id}_count_; n++) {{"));
                self.append(format!("    {tid} out;"));
                self.append(format!("    if (!{tid}::Parse(span, &out)) {{"));
                self.append("        return false;".to_string());
                self.append("    }".to_string());
                self.append("}".to_string());
                self.append(format!("{id}_ = {id}_.subrange(0, {id}_.size() - span.size());"));
            }

            (ElementSize::Unknown, ArraySize::DynamicSize) => {
                self.check_size(&format!("{id}_size_"));
                self.append(format!("{id}_ = span.subrange(0, {id}_size_);"));
                self.append(format!("span.skip({id}_size_);"));
            }

            (ElementSize::Unknown, ArraySize::Unknown) => {
                let tid = type_id.unwrap();
                self.append(format!("{id}_ = span;"));
                self.append("while (span.size() > 0) {".to_string());
                self.append(format!("    {tid} out;"));
                self.append(format!("    if (!{tid}::Parse(span, &out)) {{"));
                self.append("        return false;".to_string());
                self.append("    }".to_string());
                self.append("}".to_string());
                self.append(format!("{id}_ = {id}_.subrange(0, {id}_.size() - span.size());"));
            }
        }
    }

    /// Return the size in bits of the fields following `field`,
    /// or `None` if one of the following fields does not have a static size.
    fn get_trailing_size(&self, field: &'a ast::Field) -> Option<usize> {
        let mut trailing_size = 0;
        let mut found = false;
        for f in self.decl.fields() {
            if found {
                match self.schema.field_size(f.key) {
                    analyzer::Size::Static(bits) => trailing_size += bits,
                    _ => return None,
                }
            }
            if f.key == field.key {
                found = true;
            }
        }
        Some(trailing_size)
    }

    fn parse_payload_field_lite(&mut self, _field: &'a ast::Field) {
        assert_eq!(self.shift, 0, "Payload field does not start on an octet boundary");
        self.check_code();

        // Packet views store the payload as a slice, structs store the
        // payload as a byte vector.
        let bytes = if self.extract_arrays { ".bytes()" } else { "" };
        let mut size_expr = "".to_string();
        for f in self.decl.fields() {
            if let ast::FieldDesc::Size { field_id, .. } = &f.desc {
                if field_id == "_payload_" || field_id == "_body_" {
                    size_expr = "payload_size_".to_owned();
                }
            }
        }

        if !size_expr.is_empty() {
            let inverted_modifier = match &_field.desc {
                ast::FieldDesc::Payload { size_modifier: Some(m), .. } => {
                    if let Some(stripped) = m.strip_prefix('+') {
                        format!("- {}", stripped)
                    } else if let Some(stripped) = m.strip_prefix('-') {
                        format!("+ {}", stripped)
                    } else {
                        m.clone()
                    }
                }
                _ => String::new(),
            };
            let actual_size = if inverted_modifier.is_empty() {
                size_expr
            } else {
                format!("({} {})", size_expr, inverted_modifier)
            };
            self.append(format!("if (span.size() < {}) return false;", actual_size));
            self.append(format!(
                "{}payload_ = span.subrange(0, {}){};",
                self.target_prefix, actual_size, bytes
            ));
            self.append(format!("span.skip({});", actual_size));
        } else {
            let trailing_size = self.get_trailing_size(_field).unwrap_or(0);
            if trailing_size > 0 {
                self.append(format!("if (span.size() < {}) return false;", trailing_size / 8));
                self.append(format!("size_t payload_size = span.size() - {};", trailing_size / 8));
                self.append(format!(
                    "{}payload_ = span.subrange(0, payload_size){};",
                    self.target_prefix, bytes
                ));
                self.append("span.skip(payload_size);".to_string());
            } else {
                self.append(format!("{}payload_ = span{};", self.target_prefix, bytes));
                self.append("span.clear();".to_string());
            }
        }
    }

    fn parse_array_field_full(
        &mut self,
        field: &'a ast::Field,
        id: &str,
        type_id: Option<&str>,
        size_modifier: Option<&str>,
    ) {
        self.check_code();

        let element_size = analyzer::element_size(self.scope, self.schema, self.decl, field);
        let array_size = analyzer::array_size(self.decl, field);

        // Apply the size modifier.
        if let Some(size_modifier) = size_modifier {
            self.append(format!("{id}_size_ = {id}_size_ - {size_modifier};"));
        }

        use analyzer::{ArraySize, ElementSize};
        match (element_size, array_size) {
            (ElementSize::Static(element_size), ArraySize::StaticCount(count)) => {
                let total_size = element_size * count;
                self.check_size(&format!("{total_size}"));
                self.append(format!("for (size_t n = 0; n < {count}; n++) {{"));
                let out = self.parse_array_element(field, "span".to_string());
                self.append(format!("    output->{id}_[n] = {out};"));
                self.append("}".to_string());
            }

            (ElementSize::Static(element_size), ArraySize::DynamicCount) => {
                self.check_size(&format!("{element_size} * output->{id}_count_"));
                self.append(format!("for (size_t n = 0; n < output->{id}_count_; n++) {{"));
                let out = self.parse_array_element(field, "span".to_string());
                self.append(format!("    output->{id}_.push_back({out});"));
                self.append("}".to_string());
            }

            (ElementSize::Static(element_size), ArraySize::DynamicSize) => {
                self.check_size(&format!("{id}_size_"));
                self.append(format!("if (({id}_size_ % {element_size}) != 0) {{"));
                self.append("    return false;".to_string());
                self.append("}".to_string());
                self.append(format!("auto {id}_count_ = {id}_size_ / {element_size};"));
                self.append(format!("for (size_t n = 0; n < {id}_count_; n++) {{"));
                let out = self.parse_array_element(field, "span".to_string());
                self.append(format!("    output->{id}_.push_back({out});"));
                self.append("}".to_string());
            }

            (ElementSize::Static(element_size), ArraySize::Unknown) => {
                self.append(format!("if ((span.size() % {element_size}) != 0) {{"));
                self.append("    return false;".to_string());
                self.append("}".to_string());
                self.append(format!("auto {id}_count_ = span.size() / {element_size};"));
                self.append(format!("for (size_t n = 0; n < {id}_count_; n++) {{"));
                let out = self.parse_array_element(field, "span".to_string());
                self.append(format!("    output->{id}_.push_back({out});"));
                self.append("}".to_string());
            }

            (ElementSize::Dynamic, ArraySize::StaticCount(count)) => {
                self.check_size(&format!("output->{id}_element_size_ * {count}"));
                self.append(format!("for (size_t n = 0; n < {count}; n++) {{"));
                self.append(format!("    auto element_span = span.subrange(n * output->{id}_element_size_, output->{id}_element_size_);"));
                let out = self.parse_array_element(field, "element_span".to_string());
                self.append(format!("    output->{id}_[n] = {out};"));
                self.append("}".to_string());
                self.append(format!("span.skip(output->{id}_element_size_ * {count});"));
            }

            (ElementSize::Dynamic, ArraySize::DynamicCount) => {
                self.check_size(&format!("output->{id}_element_size_ * output->{id}_count_"));
                self.append(format!("for (size_t n = 0; n < output->{id}_count_; n++) {{"));
                self.append(format!("    auto element_span = span.subrange(n * output->{id}_element_size_, output->{id}_element_size_);"));
                let out = self.parse_array_element(field, "element_span".to_string());
                self.append(format!("    output->{id}_[n] = {out};"));
                self.append("}".to_string());
                self.append(format!(
                    "span.skip(output->{id}_element_size_ * output->{id}_count_);"
                ));
            }

            (ElementSize::Dynamic, ArraySize::DynamicSize) => {
                self.check_size(&format!("{id}_size_"));
                self.append(format!(
                    "if (output->{id}_element_size_ == 0 || ({id}_size_ % output->{id}_element_size_) != 0) {{"
                ));
                self.append("    return false;".to_string());
                self.append("}".to_string());
                self.append(format!("auto {id}_count_ = {id}_size_ / output->{id}_element_size_;"));
                self.append(format!("for (size_t n = 0; n < {id}_count_; n++) {{"));
                self.append(format!("    auto element_span = span.subrange(n * output->{id}_element_size_, output->{id}_element_size_);"));
                let out = self.parse_array_element(field, "element_span".to_string());
                self.append(format!("    output->{id}_[n] = {out};"));
                self.append("}".to_string());
                self.append(format!("span.skip({id}_size_);"));
            }

            (ElementSize::Dynamic, ArraySize::Unknown) => {
                self.append(format!("if (output->{id}_element_size_ == 0 || (span.size() % output->{id}_element_size_) != 0) {{"));
                self.append("    return false;".to_string());
                self.append("}".to_string());
                self.append(format!(
                    "auto {id}_count_ = span.size() / output->{id}_element_size_;"
                ));
                self.append(format!("for (size_t n = 0; n < {id}_count_; n++) {{"));
                self.append(format!("    auto element_span = span.subrange(n * output->{id}_element_size_, output->{id}_element_size_);"));
                let out = self.parse_array_element(field, "element_span".to_string());
                self.append(format!("    output->{id}_[n] = {out};"));
                self.append("}".to_string());
                self.append("span.clear();".to_string());
            }

            (ElementSize::Unknown, ArraySize::StaticCount(count)) => {
                let tid = type_id.unwrap();
                self.append(format!("for (size_t n = 0; n < {count}; n++) {{"));
                self.append(format!("    {tid} out;"));
                self.append(format!("    if (!{tid}::Parse(span, &out)) {{"));
                self.append("        return false;".to_string());
                self.append("    }".to_string());
                self.append(format!("    output->{id}_[n] = std::move(out);"));
                self.append("}".to_string());
            }

            (ElementSize::Unknown, ArraySize::DynamicCount) => {
                let tid = type_id.unwrap();
                self.append(format!("for (size_t n = 0; n < output->{id}_count_; n++) {{"));
                self.append(format!("    {tid} out;"));
                self.append(format!("    if (!{tid}::Parse(span, &out)) {{"));
                self.append("        return false;".to_string());
                self.append("    }".to_string());
                self.append(format!("    output->{id}_.push_back(std::move(out));"));
                self.append("}".to_string());
            }

            (ElementSize::Unknown, ArraySize::DynamicSize) => {
                let tid = type_id.unwrap();
                self.check_size(&format!("{id}_size_"));
                self.append(format!("auto {id}_span = span.subrange(0, {id}_size_);"));
                self.append(format!("while ({id}_span.size() > 0) {{"));
                self.append(format!("    {tid} out;"));
                self.append(format!("    if (!{tid}::Parse({id}_span, &out)) {{"));
                self.append("        return false;".to_string());
                self.append("    }".to_string());
                self.append(format!("    output->{id}_.push_back(std::move(out));"));
                self.append("}".to_string());
                self.append(format!("span.skip({id}_size_);"));
            }

            (ElementSize::Unknown, ArraySize::Unknown) => {
                let tid = type_id.unwrap();
                self.append("while (span.size() > 0) {".to_string());
                self.append(format!("    {tid} out;"));
                self.append(format!("    if (!{tid}::Parse(span, &out)) {{"));
                self.append("        return false;".to_string());
                self.append("    }".to_string());
                self.append(format!("    output->{id}_.push_back(std::move(out));"));
                self.append("}".to_string());
            }
        }
    }

    fn parse_array_element(&mut self, field: &'a ast::Field, span: String) -> String {
        let byteorder = match self.endianness {
            ast::EndiannessValue::LittleEndian => "le",
            ast::EndiannessValue::BigEndian => "be",
        };
        match &field.desc {
            ast::FieldDesc::Array { width: Some(width), type_id: None, .. } => {
                let element_size = *width / 8;
                let backing_type = get_cxx_scalar_type(*width);
                format!("{span}.read_{byteorder}<{backing_type}, {element_size}>()")
            }
            ast::FieldDesc::Array { type_id: Some(type_id), .. } => {
                match &self.scope.typedef.get(type_id).unwrap().desc {
                    ast::DeclDesc::Enum { width, tags, .. } => {
                        let element_size = *width / 8;
                        let backing_type = get_cxx_scalar_type(*width);
                        let raw_value =
                            format!("{span}.read_{byteorder}<{backing_type}, {element_size}>()");
                        if tags.iter().any(|t| matches!(t, ast::Tag::Other(_))) {
                            format!("{type_id}({raw_value})")
                        } else {
                            self.append(format!("    auto raw_value = {raw_value};"));
                            self.append(format!("    if (!IsValid{type_id}(raw_value)) {{"));
                            self.append("        return false;".to_string());
                            self.append("    }".to_string());
                            format!("{type_id}(raw_value)")
                        }
                    }
                    _ => {
                        self.append(format!("    {type_id} out;"));
                        self.append(format!("    if (!{type_id}::Parse({span}, &out)) {{"));
                        self.append("        return false;".to_string());
                        self.append("    }".to_string());
                        "out".to_string()
                    }
                }
            }
            _ => unreachable!(),
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
                ast::FieldDesc::Array { id, type_id, size_modifier, .. } => {
                    let padded_size = self.schema.padded_size(field.key);
                    if padded_size.is_some() {
                        self.append(format!("size_t {id}_start_size = span.size();"));
                    }
                    if !self.extract_arrays {
                        self.parse_array_field_lite(
                            field,
                            id,
                            type_id.as_deref(),
                            size_modifier.as_deref(),
                        );
                    } else {
                        self.parse_array_field_full(
                            field,
                            id,
                            type_id.as_deref(),
                            size_modifier.as_deref(),
                        );
                    }
                    if let Some(padded_size) = padded_size {
                        let padding_bytes = padded_size / 8;
                        self.append(format!(
                            "if ({id}_start_size - span.size() < {padding_bytes}) {{",
                        ));
                        self.append(format!("    if (span.size() < {padding_bytes} - ({id}_start_size - span.size())) return false;"));
                        self.append(format!(
                            "    span.skip({padding_bytes} - ({id}_start_size - span.size()));",
                        ));
                        self.append("}".to_string());
                    }
                }
                ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body => {
                    self.parse_payload_field_lite(field);
                }
                ast::FieldDesc::Scalar { id, width } => {
                    let ty = get_cxx_scalar_type(*width);
                    let byteorder = match self.endianness {
                        ast::EndiannessValue::LittleEndian => "le",
                        ast::EndiannessValue::BigEndian => "be",
                    };
                    self.append(format!("if (span.size() < {}) return false;", width / 8));
                    self.append(format!(
                        "{}{id}_ = span.read_{byteorder}<{ty}, {}>();",
                        self.target_prefix,
                        width / 8
                    ));
                }
                ast::FieldDesc::Size { field_id, width, .. } => {
                    let ty = get_cxx_scalar_type(*width);
                    let field_name = if field_id == "_payload_" || field_id == "_body_" {
                        "payload"
                    } else {
                        field_id
                    };
                    let byteorder = match self.endianness {
                        ast::EndiannessValue::LittleEndian => "le",
                        ast::EndiannessValue::BigEndian => "be",
                    };
                    self.append(format!("if (span.size() < {}) return false;", width / 8));
                    self.append(format!(
                        "{}{field_name}_size_ = span.read_{byteorder}<{ty}, {}>();",
                        self.target_prefix,
                        width / 8
                    ));
                }
                ast::FieldDesc::Count { field_id, width, .. } => {
                    let ty = get_cxx_scalar_type(*width);
                    let byteorder = match self.endianness {
                        ast::EndiannessValue::LittleEndian => "le",
                        ast::EndiannessValue::BigEndian => "be",
                    };
                    self.append(format!("if (span.size() < {}) return false;", width / 8));
                    self.append(format!(
                        "{}{field_id}_count_ = span.read_{byteorder}<{ty}, {}>();",
                        self.target_prefix,
                        width / 8
                    ));
                }
                ast::FieldDesc::ElementSize { field_id, width, .. } => {
                    let ty = get_cxx_scalar_type(*width);
                    let field_name = if field_id == "_payload_" || field_id == "_body_" {
                        "payload"
                    } else {
                        field_id
                    };
                    let byteorder = match self.endianness {
                        ast::EndiannessValue::LittleEndian => "le",
                        ast::EndiannessValue::BigEndian => "be",
                    };
                    self.append(format!("if (span.size() < {}) return false;", width / 8));
                    self.append(format!(
                        "{}{field_name}_element_size_ = span.read_{byteorder}<{ty}, {}>();",
                        self.target_prefix,
                        width / 8
                    ));
                }
                ast::FieldDesc::FixedScalar { width, value } => {
                    let ty = get_cxx_scalar_type(*width);
                    let byteorder = match self.endianness {
                        ast::EndiannessValue::LittleEndian => "le",
                        ast::EndiannessValue::BigEndian => "be",
                    };
                    self.append(format!("if (span.size() < {}) return false;", width / 8));
                    self.append(format!(
                        "if (span.read_{byteorder}<{ty}, {}>() != {value:#x}) return false;",
                        width / 8,
                    ));
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
                    self.append(format!(
                        "if (span.read_{byteorder}<{ty}, {}>() != static_cast<{ty}>( {enum_id}::{tag_id} )) return false;",
                        width / 8,
                    ));
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

/// Keeps the necessary context for the encoder implementation.
struct Encoder<'a> {
    scope: &'a analyzer::Scope<'a>,
    schema: &'a analyzer::Schema,
    endianness: ast::EndiannessValue,
    constraints: HashMap<String, ast::Constraint>,
    shift: usize,
    values: Vec<(String, usize)>,
    code: Vec<String>,
    indent_level: usize,
}

/// Represents the computed size of a packet,
/// composed of constant and variable size fields.
#[derive(Default)]
struct RuntimeSize {
    constant: usize,
    variable: Vec<String>,
}

impl std::ops::AddAssign<RuntimeSize> for RuntimeSize {
    fn add_assign(&mut self, other: RuntimeSize) {
        self.constant += other.constant;
        self.variable.extend_from_slice(&other.variable)
    }
}

impl std::ops::AddAssign<String> for RuntimeSize {
    fn add_assign(&mut self, other: String) {
        self.variable.push(other)
    }
}

impl std::ops::AddAssign<usize> for RuntimeSize {
    fn add_assign(&mut self, other: usize) {
        self.constant += other
    }
}

impl<'a> Encoder<'a> {
    fn new(
        scope: &'a analyzer::Scope<'a>,
        schema: &'a analyzer::Schema,
        endianness: ast::EndiannessValue,
        constraints: HashMap<String, ast::Constraint>,
    ) -> Self {
        Self {
            scope,
            schema,
            endianness,
            constraints,
            shift: 0,
            values: Vec::new(),
            code: Vec::new(),
            indent_level: 0,
        }
    }

    fn byteorder(&self) -> &'static str {
        match self.endianness {
            ast::EndiannessValue::LittleEndian => "le",
            ast::EndiannessValue::BigEndian => "be",
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

    fn encode(&mut self, decl: &'a ast::Decl, children: &[&'a ast::Decl]) {
        for field in decl.fields() {
            self.encode_field(decl, children, field);
        }
    }

    /// Compute the partial size of the packet `decl` counting only the fields
    /// immediatly in the declaration and children declarations.
    fn compute_partial_packet_size(
        &self,
        decl: &'a ast::Decl,
        children: &[&'a ast::Decl],
    ) -> RuntimeSize {
        // Accumulate the size of all fields in the packet declaration.
        let mut packet_size = RuntimeSize::default();
        for f in decl.fields() {
            if let Some(padded_size) = self.schema.padded_size(f.key) {
                packet_size += padded_size;
                continue;
            }
            match &f.desc {
                ast::FieldDesc::Scalar { id, width, .. } if f.cond.is_some() => {
                    packet_size += format!("({id}_.has_value() ? {} : 0)", *width / 8)
                }
                ast::FieldDesc::Typedef { id, type_id, .. } => {
                    let type_decl = self.scope.typedef.get(type_id).unwrap();
                    match (self.schema.decl_size(type_decl.key), &f.cond) {
                        (analyzer::Size::Static(width), Some(_)) => {
                            packet_size += format!("({id}_.has_value() ? {} : 0)", width / 8)
                        }
                        (analyzer::Size::Static(width), None) => packet_size += width,
                        (_, Some(_)) => {
                            packet_size += format!("({id}_.has_value() ? {id}_->GetSize() : 0)")
                        }
                        (_, None) => packet_size += format!("{id}_.GetSize()"),
                    }
                }
                ast::FieldDesc::Array { id, .. } => {
                    let element_size = analyzer::element_size(self.scope, self.schema, decl, f);
                    let array_size = analyzer::array_size(decl, f);

                    use analyzer::{ArraySize, ElementSize};
                    match (element_size, array_size) {
                        (ElementSize::Static(width), ArraySize::StaticCount(count)) => {
                            packet_size += 8 * width * count
                        }
                        (ElementSize::Static(width), _) => {
                            packet_size += format!("{width} * {id}_.size()")
                        }
                        _ => {
                            packet_size += format!(
                                "std::accumulate({id}_.begin(), {id}_.end(), size_t(0), [](size_t s, auto const& elt) {{ return s + elt.GetSize(); }})"
                            )
                        }
                    }
                }
                ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body => {
                    if children.is_empty() {
                        packet_size += "payload_.size()".to_owned()
                    } else {
                        packet_size += self.compute_partial_packet_size(children[0], &children[1..])
                    }
                }
                // Remaining fields are all bitfields with a static bit size,
                // or simply markers dropped during encoding.
                _ => packet_size += self.schema.field_size(f.key).static_().unwrap_or(0),
            }
        }

        packet_size
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
                "pdl::packet::Builder::write_{byteorder}<{backing_type}, {size}>(output, 0);",
            ));
        } else {
            let packed_val = self
                .values
                .iter()
                .map(|(v, s)| {
                    if *s == 0 {
                        format!("(static_cast<{backing_type}>({v}))")
                    } else {
                        format!("(static_cast<{backing_type}>({v}) << {s})")
                    }
                })
                .collect::<Vec<_>>()
                .join(" | ");
            self.append(&format!(
                "pdl::packet::Builder::write_{byteorder}<{backing_type}, {size}>(output, {packed_val});",
            ));
        }

        self.shift = 0;
        self.values.clear();
    }

    /// Encode an optional field `field`.
    fn encode_optional_field(&mut self, field: &'a ast::Field) {
        let id = field.id().unwrap();
        let byteorder = self.byteorder();

        self.append(&format!("if ({id}_.has_value()) {{"));
        self.indent();

        match &field.desc {
            ast::FieldDesc::Scalar { width, .. } => {
                let ty = get_cxx_scalar_type(*width);
                if !matches!(*width, 8 | 16 | 32 | 64) {
                    self.append(&format!("if (*{id}_ > {}) {{", mask(*width)));
                    self.append("    return false;");
                    self.append("}");
                }
                self.append(&format!(
                    "pdl::packet::Builder::write_{byteorder}<{ty}, {}>(output, *{id}_);",
                    width / 8,
                ));
            }
            ast::FieldDesc::Typedef { type_id, .. } => {
                let td = self.scope.typedef.get(type_id).unwrap();
                if let ast::DeclDesc::Enum { width, .. } = &td.desc {
                    let ty = get_cxx_scalar_type(*width);
                    self.append(&format!(
                        "pdl::packet::Builder::write_{byteorder}<{ty}, {}>(output, {ty}(*{id}_));",
                        width / 8,
                    ));
                } else {
                    self.append(&format!("if (!{id}_->Serialize(output)) {{"));
                    self.append("    return false;");
                    self.append("}");
                }
            }
            _ => unreachable!(),
        }

        self.unindent();
        self.append("}");
    }

    /// Encode a bit field `field` of the packet declaration `decl`.
    /// `children` lists the chosen specialization for the current packet.
    fn encode_bit_field(
        &mut self,
        decl: &'a ast::Decl,
        children: &[&'a ast::Decl],
        field: &'a ast::Field,
    ) {
        let width = self.schema.field_size(field.key).static_().unwrap();
        let shift = self.shift;

        match &field.desc {
            ast::FieldDesc::Scalar { id, .. } => {
                if let Some(constraint) = self.constraints.get(id) {
                    self.values.push((format!("{}", constraint.value.unwrap()), shift));
                } else {
                    if !matches!(width, 8 | 16 | 32 | 64) {
                        self.append(&format!("if ({id}_ > {}) {{", mask(width)));
                        self.append("    return false;");
                        self.append("}");
                    }
                    self.values.push((format!("{id}_"), shift));
                }
            }
            ast::FieldDesc::FixedScalar { value, .. } => {
                self.values.push((format!("{:#x}", value), shift));
            }
            ast::FieldDesc::FixedEnum { enum_id, tag_id, .. } => {
                self.values.push((format!("{}::{}", enum_id, tag_id), shift));
            }
            ast::FieldDesc::Typedef { id, type_id, .. } => {
                let type_decl = self.scope.typedef.get(type_id).unwrap();
                if let Some(constraint) = self.constraints.get(id) {
                    self.values.push((
                        format!("{type_id}::{}", constraint.tag_id.as_ref().unwrap()),
                        shift,
                    ));
                } else if let ast::DeclDesc::Enum { tags, .. } = &type_decl.desc {
                    // Verify that the proposed enum value is actually valid for the
                    // given enum declaration. The caller could have cast any value into
                    // the member.
                    let ty = get_cxx_scalar_type(width);
                    if !matches!(width, 8 | 16 | 32 | 64) {
                        self.append(&format!("if ({ty}({id}_) > {}) {{", mask(width)));
                        self.append("    return false;");
                        self.append("}");
                    }
                    if !common::is_complete_enum(tags, scalar_max(width))
                        && !common::is_open_enum(tags)
                    {
                        self.append(&format!("if (!IsValid{type_id}({ty}({id}_))) {{"));
                        self.append("    return false;");
                        self.append("}");
                    }
                    self.values.push((format!("{ty}({id}_)"), shift));
                }
            }
            ast::FieldDesc::Size { field_id, width, .. }
                if field_id == "_payload_" || field_id == "_body_" =>
            {
                let size_var = format!("{}_payload_size", decl.id().unwrap());
                let size_expr = if children.is_empty() {
                    "payload_.size()".to_owned()
                } else {
                    match self.compute_partial_packet_size(children[0], &children[1..]) {
                        RuntimeSize { constant: 0, variable } if variable.is_empty() => {
                            "0".to_owned()
                        }
                        RuntimeSize { constant, variable } if variable.is_empty() => {
                            format!("{}", constant / 8)
                        }
                        RuntimeSize { constant: 0, variable } => variable.join(" + "),
                        RuntimeSize { constant, variable } => {
                            format!("{} + {}", constant / 8, variable.join(" + "))
                        }
                    }
                };

                self.append(&format!("size_t {size_var} = {size_expr};"));

                let size_modifier = decl.fields().find_map(|f| match &f.desc {
                    ast::FieldDesc::Payload { size_modifier, .. } => size_modifier.as_ref(),
                    _ => None,
                });

                if let Some(modifier) = size_modifier {
                    self.append(&format!("{size_var} = {size_var} {modifier};"))
                }

                if *width < 64 {
                    self.append(&format!("if ({size_var} > {}) {{", mask(*width)));
                    self.append("    return false;");
                    self.append("}");
                }

                self.values.push((size_var, shift));
            }
            ast::FieldDesc::Size { field_id, width, .. } => {
                let array_field = self
                    .scope
                    .iter_fields(decl)
                    .find(|f| f.id() == Some(field_id))
                    .expect("Field not found");

                match analyzer::element_size(self.scope, self.schema, decl, array_field) {
                    analyzer::ElementSize::Static(element_size) => {
                        self.append(&format!(
                            "size_t {field_id}_size = {element_size} * {field_id}_.size();"
                        ));
                    }

                    analyzer::ElementSize::Dynamic | analyzer::ElementSize::Unknown => {
                        self.append(&format!("size_t {field_id}_size = 0;"));
                        self.append(&format!("for (auto const& elt : {field_id}_) {{"));
                        self.append(&format!("    {field_id}_size += elt.GetSize();"));
                        self.append("}");
                    }
                }

                if let ast::FieldDesc::Array { size_modifier: Some(modifier), .. } =
                    &array_field.desc
                {
                    self.append(&format!("{field_id}_size = {field_id}_size {modifier};"));
                }

                if *width < 64 {
                    self.append(&format!("if ({field_id}_size > {}) {{", mask(*width)));
                    self.append("    return false;");
                    self.append("}");
                }

                self.values.push((format!("{field_id}_size"), shift));
            }
            ast::FieldDesc::Count { field_id, width, .. } => {
                // The count field can only apply to an array field.
                // The code is generated under this assumption.

                if *width < 64 {
                    self.append(&format!("if ({field_id}_.size() > {}) {{", mask(*width)));
                    self.append("    return false;");
                    self.append("}");
                }

                self.values.push((format!("{field_id}_.size()"), shift));
            }
            ast::FieldDesc::ElementSize { field_id, width, .. } => {
                // The element_size field can only apply to an array field.
                // The code is generated under this assumption.

                self.append(&format!("size_t {field_id}_element_size = {field_id}_.empty() ? 0 : {field_id}_[0].GetSize();"));
                self.append(&format!("for (auto const& elt : {field_id}_) {{"));
                self.append(&format!("    if (elt.GetSize() != {field_id}_element_size) {{"));
                self.append("        return false;");
                self.append("    }");
                self.append("}");

                if *width < 64 {
                    self.append(&format!("if ({field_id}_element_size > {}) {{", mask(*width)));
                    self.append("    return false;");
                    self.append("}");
                }

                self.values.push((format!("{field_id}_element_size"), shift));
            }
            ast::FieldDesc::Flag { id, .. } => {
                let flag_value = |field: &'_ ast::Field| {
                    let id = field.id().unwrap();
                    let cond = field.cond.as_ref().unwrap().value.unwrap();
                    format!("{id}_.has_value() ? {} : {}", cond, 1 - cond)
                };

                // Get a list of all fields that use this condition flag.
                // Verify that the proposed encoding is consistent.
                let ty = get_cxx_scalar_type(width);
                let optional_fields = decl
                    .fields()
                    .filter(|f| matches!(f.cond, Some(ast::Constraint { id: ref cid, .. }) if cid == id))
                    .collect::<Vec<_>>();
                self.append(&format!("{ty} {id} = {};", flag_value(optional_fields[0])));
                for optional_field in optional_fields.iter() {
                    self.append(&format!("if ({id} != ({})) {{", flag_value(optional_field)));
                    self.append("    return false;");
                    self.append("}");
                }
                self.values.push((id.to_owned(), shift));
            }
            ast::FieldDesc::Reserved { .. } => (),
            _ => unreachable!(),
        }

        self.shift += width;
        if self.shift.is_multiple_of(8) {
            self.pack_bit_fields();
        }
    }

    /// Encoder a single field `field` of the packet declaration `decl`.
    /// `children` lists the chosen specialization for the current packet.
    fn encode_field(
        &mut self,
        decl: &'a ast::Decl,
        children: &[&'a ast::Decl],
        field: &'a ast::Field,
    ) {
        let byteorder = self.byteorder();
        match &field.desc {
            _ if field.cond.is_some() => self.encode_optional_field(field),
            _ if self.scope.is_bitfield(field) => self.encode_bit_field(decl, children, field),
            ast::FieldDesc::Array { id, type_id, width, .. } => {
                let padded_size = self.schema.padded_size(field.key);
                if padded_size.is_some() {
                    self.append(&format!("size_t {id}_start = output.size();"));
                }

                self.append(&format!("for (auto const& element : {id}_) {{"));
                self.indent();

                if let Some(tid) = type_id {
                    let td = self.scope.typedef.get(tid).unwrap();
                    match &td.desc {
                        ast::DeclDesc::Enum { width, .. } => {
                            let ty = get_cxx_scalar_type(*width);
                            self.append(&format!("pdl::packet::Builder::write_{byteorder}<{ty}, {}>(output, {ty}(element));", width / 8));
                        }
                        _ => {
                            self.append("if (!element.Serialize(output)) {");
                            self.append("    return false;");
                            self.append("}");
                        }
                    }
                } else {
                    let element_width = width.unwrap();
                    let backing_type = get_cxx_scalar_type(element_width);
                    self.append(&format!("pdl::packet::Builder::write_{byteorder}<{}, {}>(output, static_cast<{}>(element));", backing_type, element_width / 8, backing_type));
                }

                self.unindent();
                self.append("}");

                if let Some(ps) = padded_size {
                    let ps_bytes = ps / 8;
                    self.append(&format!("if (output.size() - {0}_start > {1}) {{", id, ps_bytes));
                    self.append("    return false;");
                    self.append("}");
                    self.append(&format!("if (output.size() - {0}_start < {1}) {{", id, ps_bytes));
                    self.append(&format!("    output.resize({0}_start + {1}, 0);", id, ps_bytes));
                    self.append("}");
                }
            }
            ast::FieldDesc::Typedef { id, .. } => {
                self.append(&format!("if (!{id}_.Serialize(output)) {{"));
                self.append("    return false;");
                self.append("}");
            }
            ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body => {
                if children.is_empty() {
                    // The current declaration corresponds to the packet being generated.
                    // The payload is encoded as a builder field in this case.
                    self.append("output.insert(output.end(), payload_.begin(), payload_.end());");
                } else {
                    // Serialize the children fields in place of the parent payload.
                    self.encode(children[0], &children[1..])
                }
            }
            // Padding declarations are checked in array and payload code generation.
            ast::FieldDesc::Padding { .. } => (),
            // Checksum fields are not supported by the C++ backend;
            // the checksum start marker does not produce any output.
            ast::FieldDesc::Checksum { .. } => (),
            _ => unreachable!(),
        }
    }
}

fn generate_packet_view(
    scope: &analyzer::Scope,
    schema: &analyzer::Schema,
    decl: &ast::Decl,
) -> String {
    let id = decl.id().unwrap();
    let parent_id = decl.parent_id();
    let parent_class =
        parent_id.map(|p| format!("{}View", p)).unwrap_or_else(|| "pdl::packet::slice".to_string());
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
    let all_fields =
        get_all_parent_fields(scope, decl).into_iter().chain(decl.fields()).collect::<Vec<_>>();

    let parent_constraints =
        scope.iter_parents_and_self(decl).flat_map(|d| d.constraints()).collect::<Vec<_>>();
    for field in &all_fields {
        let fid = field.id();
        let constraint = fid.and_then(|fid| parent_constraints.iter().find(|c| c.id == fid));
        let is_constrained = constraint.is_some();

        if is_constrained {
            // Constrained fields still get accessors returning their constant values.
            let fid = field.id().unwrap();
            let accessor_name = fid.to_upper_camel_case();
            let constraint = constraint.unwrap();
            match &field.desc {
                ast::FieldDesc::Typedef { type_id, .. } => {
                    field_accessors.push(format!(
                        "    {} Get{}() const {{ return {}::{}; }}\n",
                        type_id,
                        accessor_name,
                        type_id,
                        constraint.tag_id.as_ref().unwrap()
                    ));
                }
                ast::FieldDesc::Scalar { width, .. } => {
                    let ty = get_cxx_scalar_type(*width);
                    field_accessors.push(format!(
                        "    {} Get{}() const {{ return {}; }}\n",
                        ty,
                        accessor_name,
                        constraint.value.unwrap()
                    ));
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
                let accessor_name = id.to_upper_camel_case();
                let element_type = type_id
                    .as_deref()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| get_cxx_scalar_type(width.unwrap()));
                let array_type = if let Some(s) = size {
                    format!("std::array<{}, {}>", element_type, s)
                } else {
                    format!("std::vector<{}>", element_type)
                };

                let is_dynamic_element_size = matches!(
                    analyzer::element_size(scope, schema, decl, field),
                    analyzer::ElementSize::Dynamic
                );
                let mut accessor_code = Vec::new();
                accessor_code.push(format!("pdl::packet::slice span = {}_;", id));
                if let Some(s) = size {
                    accessor_code.push(format!("{} elements;", array_type));
                    accessor_code.push(format!("for (int n = 0; n < {}; n++) {{", s));
                    if is_dynamic_element_size {
                        let tid = type_id.as_deref().unwrap();
                        accessor_code.push(format!(
                            "    auto element_span = span.subrange(0, {}_element_size_);",
                            id
                        ));
                        accessor_code
                            .push(format!("    {}::Parse(element_span, &elements[n]);", tid));
                        accessor_code.push(format!("    span.skip({}_element_size_);", id));
                    } else if let Some(tid) = type_id {
                        let td = scope.typedef.get(tid).unwrap();
                        match &td.desc {
                            ast::DeclDesc::Enum { width, .. } => {
                                let backing_type = get_cxx_scalar_type(*width);
                                accessor_code.push(format!(
                                    "    elements[n] = {}(span.read_{}<{}, {}>());",
                                    tid,
                                    byteorder,
                                    backing_type,
                                    width / 8
                                ));
                            }
                            _ => {
                                accessor_code
                                    .push(format!("    {}::Parse(span, &elements[n]);", tid));
                            }
                        }
                    } else {
                        let element_width = width.unwrap();
                        let backing_type = get_cxx_scalar_type(element_width);
                        accessor_code.push(format!(
                            "    elements[n] = span.read_{}<{}, {}>();",
                            byteorder,
                            backing_type,
                            element_width / 8
                        ));
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

                    if is_dynamic_element_size {
                        let tid = type_id.as_deref().unwrap();
                        accessor_code.push(format!("while ({}) {{", count_limit));
                        accessor_code.push(format!(
                            "    auto element_span = span.subrange(0, {}_element_size_);",
                            id
                        ));
                        accessor_code.push(format!("    {} element;", tid));
                        accessor_code.push(format!(
                            "    if (!{}::Parse(element_span, &element)) break;",
                            tid
                        ));
                        accessor_code
                            .push("    elements.emplace_back(std::move(element));".to_string());
                        accessor_code.push(format!("    span.skip({}_element_size_);", id));
                        accessor_code.push("}".to_string());
                    } else if let Some(tid) = type_id {
                        let td = scope.typedef.get(tid).unwrap();
                        match &td.desc {
                            ast::DeclDesc::Enum { width, .. } => {
                                let backing_type = get_cxx_scalar_type(*width);
                                accessor_code.push(format!(
                                    "while ({} && span.size() >= {}) {{",
                                    count_limit,
                                    width / 8
                                ));
                                accessor_code.push(format!(
                                    "    elements.push_back({}(span.read_{}<{}, {}>()));",
                                    tid,
                                    byteorder,
                                    backing_type,
                                    width / 8
                                ));
                                accessor_code.push("}".to_string());
                            }
                            _ => {
                                accessor_code.push(format!("while ({}) {{", count_limit));
                                accessor_code.push(format!("    {} element;", tid));
                                accessor_code.push(format!(
                                    "    if (!{}::Parse(span, &element)) break;",
                                    tid
                                ));
                                accessor_code.push(
                                    "    elements.emplace_back(std::move(element));".to_string(),
                                );
                                accessor_code.push("}".to_string());
                            }
                        }
                    } else {
                        let element_width = width.unwrap();
                        let backing_type = get_cxx_scalar_type(element_width);
                        accessor_code.push(format!(
                            "while ({} && span.size() >= {}) {{",
                            count_limit,
                            element_width / 8
                        ));
                        accessor_code.push(format!(
                            "    elements.push_back(span.read_{}<{}, {}>());",
                            byteorder,
                            backing_type,
                            element_width / 8
                        ));
                        accessor_code.push("}".to_string());
                    }
                }
                accessor_code.push("return elements;".to_string());

                field_accessors.push(format!(
                    "    {} Get{}() const {{\n        _ASSERT_VALID(valid_);\n{}\n    }}\n",
                    array_type,
                    accessor_name,
                    indent(&accessor_code.join("\n"), 2)
                ));
            }
            ast::FieldDesc::Scalar { id, width } => {
                let ty = get_cxx_scalar_type(*width);
                let accessor_name = id.to_upper_camel_case();
                if field.cond.is_some() {
                    field_members.push(format!("std::optional<{}> {}_;", ty, id));
                    field_accessors.push(format!("    std::optional<{}> Get{}() const {{ _ASSERT_VALID(valid_); return {}_; }}\n", ty, accessor_name, id));
                } else {
                    field_members.push(format!("{} {}_;", ty, id));
                    field_accessors.push(format!(
                        "    {} Get{}() const {{ _ASSERT_VALID(valid_); return {}_; }}\n",
                        ty, accessor_name, id
                    ));
                }
            }
            ast::FieldDesc::Typedef { id, type_id, .. } => {
                let ty = type_id;
                let accessor_name = id.to_upper_camel_case();
                if field.cond.is_some() {
                    field_members.push(format!("std::optional<{}> {}_;", ty, id));
                    field_accessors.push(format!("    std::optional<{}> Get{}() const {{ _ASSERT_VALID(valid_); return {}_; }}\n", ty, accessor_name, id));
                } else {
                    let type_decl = scope.typedef.get(type_id).unwrap();
                    if let ast::DeclDesc::Enum { tags, .. } = &type_decl.desc {
                        field_members.push(format!(
                            "{} {}_{{{}}};",
                            ty,
                            id,
                            default_enum_value(ty, tags)
                        ));
                        field_accessors.push(format!(
                            "    {} Get{}() const {{ _ASSERT_VALID(valid_); return {}_; }}\n",
                            ty, accessor_name, id
                        ));
                    } else {
                        field_members.push(format!("{} {}_;", ty, id));
                        field_accessors.push(format!("    {} const& Get{}() const {{ _ASSERT_VALID(valid_); return {}_; }}\n", ty, accessor_name, id));
                    }
                }
            }
            // Size fields are never required in the generated views.
            // The slice already contains the length indication.
            ast::FieldDesc::Size { .. } => {}
            ast::FieldDesc::Count { field_id, width, .. } => {
                let ty = get_cxx_scalar_type(*width);
                field_members.push(format!("{} {}_count_ {{0}};", ty, field_id));
            }
            ast::FieldDesc::ElementSize { field_id, width, .. } => {
                let ty = get_cxx_scalar_type(*width);
                let field_name = if field_id == "_payload_" || field_id == "_body_" {
                    "payload"
                } else {
                    field_id
                };
                field_members.push(format!("{} {}_element_size_ {{0}};", ty, field_name));
            }
            _ => {}
        }
    }

    let mut field_parsers = Vec::new();
    if parent_id.is_some() {
        field_parsers.push("if (!parent.IsValid()) { return false; }".to_string());
        let unconstrained_parent_fields = get_unconstrained_parent_fields(scope, decl);
        if !unconstrained_parent_fields.is_empty() {
            for f in unconstrained_parent_fields {
                if let Some(id) = f.id() {
                    field_parsers.push(format!("{}_ = parent.{}_;", id, id));
                }
            }
        }
        // Copy the size, count, and element size values of the parent array
        // fields, which are used by the array accessors.
        for f in get_all_parent_fields(scope, decl) {
            let member = match &f.desc {
                ast::FieldDesc::Size { field_id, .. }
                    if field_id != "_payload_" && field_id != "_body_" =>
                {
                    format!("{field_id}_size_")
                }
                ast::FieldDesc::Count { field_id, .. } => format!("{field_id}_count_"),
                ast::FieldDesc::ElementSize { field_id, .. } => format!("{field_id}_element_size_"),
                _ => continue,
            };
            field_parsers.push(format!("{member} = parent.{member};"));
        }
    }

    // The trailing byte check is only added when the parent packet has a payload field,
    // or the current packet is a root packet. In other cases there is no actual parsing
    // done in the method, just added field constraints.
    let trailing_size_check = match scope.get_parent(decl) {
        Some(parent) => parent
            .fields()
            .any(|f| matches!(f.desc, ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body)),
        None => true,
    };

    if trailing_size_check {
        let span = if parent_id.is_some() { "parent.payload_" } else { "parent" };
        field_parsers.push(format!("pdl::packet::slice span = {};", span));
    }

    if decl.fields().next().is_some() {
        let mut parser = FieldParser::new(scope, schema, endianness, "", false, decl);
        parser.is_cond_for = is_cond_for;
        for f in decl.fields() {
            parser.parse(f);
        }
        parser.done();
        field_parsers.extend(parser.code);
    }

    if trailing_size_check {
        field_parsers.push("if (span.size() > 0) {".to_string());
        field_parsers.push("    return false;".to_string());
        field_parsers.push("}".to_string());
    }

    field_parsers.push("return true;".to_string());

    let friend_classes = scope
        .iter_children(decl)
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
fn generate_packet_builder(
    scope: &analyzer::Scope,
    schema: &analyzer::Schema,
    decl: &ast::Decl,
) -> String {
    let id = decl.id().unwrap();
    let class_name = format!("{}Builder", id);
    let endianness = scope.file.endianness.value;

    let mut field_members = Vec::new();
    let mut constructor_params: Vec<String> = Vec::new();
    let mut constructor_inits = Vec::new();

    let parent_constraints = scope
        .iter_parents_and_self(decl)
        .flat_map(|d| d.constraints())
        .map(|c| (c.id.to_string(), c.clone()))
        .collect::<HashMap<_, _>>();

    let all_fields =
        get_all_parent_fields(scope, decl).into_iter().chain(decl.fields()).collect::<Vec<_>>();

    for field in &all_fields {
        let fid = field.id();
        let is_constrained = fid.map(|fid| parent_constraints.contains_key(fid)).unwrap_or(false);
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
                let element_type = type_id
                    .as_deref()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| get_cxx_scalar_type(width.unwrap()));
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
                        field_members.push(format!(
                            "{} {}_{{{}}};",
                            ty,
                            id,
                            default_enum_value(ty, tags)
                        ));
                    } else {
                        field_members.push(format!("{} {}_;", ty, id));
                    }
                    constructor_params.push(format!("{} {}", ty, id));
                }
                constructor_inits.push(format!("{0}_(std::move({0}))", id));
            }
            // Size, count, and element size values are computed
            // when serializing the packet.
            _ => {}
        }
    }

    let mut all_decls = scope.iter_parents_and_self(decl).collect::<Vec<_>>();
    all_decls.reverse();

    let mut serializer = Encoder::new(scope, schema, endianness, parent_constraints.clone());
    serializer.encode(all_decls[0], &all_decls[1..]);
    serializer.append("return true;");

    let encoded_size = match serializer.compute_partial_packet_size(all_decls[0], &all_decls[1..]) {
        RuntimeSize { constant: 0, variable } if variable.is_empty() => "0".to_owned(),
        RuntimeSize { constant: 0, variable } => variable.join(" + "),
        RuntimeSize { constant, variable } if variable.is_empty() => format!("{}", constant / 8),
        RuntimeSize { constant, variable } => {
            format!("{} + {}", constant / 8, variable.join(" + "))
        }
    };

    let field_serializers = serializer.code;
    let constructor = if constructor_params.is_empty() {
        format!("    {0}() = default;", class_name)
    } else {
        format!(
            "    {0}() = default;\n    explicit {0}({1}) : {2} {{}}",
            class_name,
            constructor_params.join(", "),
            constructor_inits.join(", ")
        )
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

    bool Serialize(std::vector<uint8_t>& output) const override {{
{field_serializers}
    }}

    size_t GetSize() const override {{
        return {encoded_size};
    }}

    std::string ToString() const {{ return ""; }}

{field_members}
}};
"#,
        class_name = class_name,
        constructor = constructor,
        field_serializers = indent(&field_serializers.join("\n"), 2),
        field_members = indent(&field_members.join("\n"), 1)
    )
}
fn generate_struct_declaration(
    scope: &analyzer::Scope,
    schema: &analyzer::Schema,
    decl: &ast::Decl,
) -> String {
    let id = decl.id().unwrap();
    let mut field_members = Vec::new();
    let mut constructor_params: Vec<String> = Vec::new();
    let mut constructor_inits = Vec::new();
    let endianness = scope.file.endianness.value;

    let parent_constraints = scope
        .iter_parents_and_self(decl)
        .flat_map(|d| d.constraints())
        .map(|c| (c.id.to_string(), c.clone()))
        .collect::<HashMap<_, _>>();

    for field in decl.fields() {
        match &field.desc {
            ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body => {
                field_members.push("std::vector<uint8_t> payload_;".to_string());
                constructor_params.push("std::vector<uint8_t> payload".to_string());
                constructor_inits.push("payload_(std::move(payload))".to_string());
            }
            ast::FieldDesc::Array { id, width, type_id, size, .. } => {
                let element_type = type_id
                    .as_deref()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| get_cxx_scalar_type(width.unwrap()));
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
                        field_members.push(format!(
                            "{} {}_{{{}}};",
                            ty,
                            id,
                            default_enum_value(ty, tags)
                        ));
                    } else {
                        field_members.push(format!("{} {}_;", ty, id));
                    }
                    constructor_params.push(format!("{} {}", ty, id));
                }
                constructor_inits.push(format!("{0}_(std::move({0}))", id));
            }
            // Size fields are never required in the generated views.
            // The slice already contains the length indication.
            ast::FieldDesc::Size { .. } => {}
            ast::FieldDesc::Count { field_id, width, .. } => {
                let ty = get_cxx_scalar_type(*width);
                field_members.push(format!("{} {}_count_ {{0}};", ty, field_id));
            }
            ast::FieldDesc::ElementSize { field_id, width, .. } => {
                let ty = get_cxx_scalar_type(*width);
                let field_name = if field_id == "_payload_" || field_id == "_body_" {
                    "payload"
                } else {
                    field_id
                };
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

    let mut all_decls = scope.iter_parents_and_self(decl).collect::<Vec<_>>();
    all_decls.reverse();
    let mut serializer = Encoder::new(scope, schema, endianness, parent_constraints);
    serializer.encode(all_decls[0], &all_decls[1..]);
    serializer.append("return true;");

    let encoded_size = match serializer.compute_partial_packet_size(all_decls[0], &all_decls[1..]) {
        RuntimeSize { constant: 0, variable } if variable.is_empty() => "0".to_owned(),
        RuntimeSize { constant: 0, variable } => variable.join(" + "),
        RuntimeSize { constant, variable } if variable.is_empty() => format!("{}", constant / 8),
        RuntimeSize { constant, variable } => {
            format!("{} + {}", constant / 8, variable.join(" + "))
        }
    };

    let field_serializers = serializer.code;
    let constructor = if !constructor_params.is_empty() {
        format!(
            "    explicit {id}({constructor_parameters}) : {initializer_list} {{}}",
            constructor_parameters = constructor_params.join(", "),
            initializer_list = constructor_inits.join(", ")
        )
    } else {
        String::new()
    };

    format!(
        r#"
class {id} : public pdl::packet::Builder {{
public:
    ~{id}() override = default;
    {id}() = default;
    {id}({id} const&) = default;
    {id}({id}&&) = default;
{constructor}
    {id}& operator=({id} const&) = default;

    static bool Parse(pdl::packet::slice& parent_span, {id}* output) {{
{struct_parse_code}
    }}

    bool Serialize(std::vector<uint8_t>& output) const override {{
{field_serializers}
    }}

    size_t GetSize() const override {{
        return {encoded_size};
    }}

    std::string ToString() const {{ return ""; }}

{field_members}
}};
"#,
        struct_parse_code = indent(&field_parsers.join("\n"), 2),
        field_serializers = indent(&field_serializers.join("\n"), 2),
        field_members = indent(&field_members.join("\n"), 1)
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::assert_snapshot_eq;
    use crate::{analyzer, ast, parser};

    #[test]
    fn test_canonical_le() {
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
                "Struct_Custom_Field_ConstantSize".to_string(),
                "Struct_Custom_Field_VariableSize".to_string(),
                "Struct_Checksum_Field_FromStart".to_string(),
                "Struct_Checksum_Field_FromEnd".to_string(),
                "Struct_Custom_Field_ConstantSize_".to_string(),
                "Struct_Custom_Field_VariableSize_".to_string(),
                "Struct_Checksum_Field_FromStart_".to_string(),
                "Struct_Checksum_Field_FromEnd_".to_string(),
            ],
        );
        assert_snapshot_eq("tests/generated/cxx/le_backend.h", &actual_code);
    }

    #[test]
    fn test_canonical_be() {
        let mut db = ast::SourceDatabase::new();
        let input_file = "tests/canonical/le_test_file.pdl";
        let source = std::fs::read_to_string(input_file).unwrap();
        let source = source.replace("little_endian_packets", "big_endian_packets");
        let file =
            parser::parse_inline(&mut db, "tests/canonical/be_test_file.pdl", source).unwrap();
        let file = analyzer::analyze(&file).unwrap();
        let actual_code = generate(
            &db,
            &file,
            Some("be_backend"),
            &[],
            &[],
            &[
                "Packet_Custom_Field_ConstantSize".to_string(),
                "Packet_Custom_Field_VariableSize".to_string(),
                "Packet_Checksum_Field_FromStart".to_string(),
                "Packet_Checksum_Field_FromEnd".to_string(),
                "Struct_Custom_Field_ConstantSize".to_string(),
                "Struct_Custom_Field_VariableSize".to_string(),
                "Struct_Checksum_Field_FromStart".to_string(),
                "Struct_Checksum_Field_FromEnd".to_string(),
                "Struct_Custom_Field_ConstantSize_".to_string(),
                "Struct_Custom_Field_VariableSize_".to_string(),
                "Struct_Checksum_Field_FromStart_".to_string(),
                "Struct_Checksum_Field_FromEnd_".to_string(),
            ],
        );
        assert_snapshot_eq("tests/generated/cxx/be_backend.h", &actual_code);
    }
}
