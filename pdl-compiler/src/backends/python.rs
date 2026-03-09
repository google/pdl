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

//! Python compiler backend.

use crate::{analyzer, ast};

#[derive(Default)]
struct CodeBlock {
    pub lines: Vec<String>,
}

impl CodeBlock {
    fn append(&mut self, code: String) {
        let code = code.trim_start_matches('\n');
        self.lines.extend(code.trim_end().lines().map(String::from));
    }

    fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}

fn indent(s: &str, level: usize) -> String {
    let prefix = "    ".repeat(level);
    s.lines()
        .map(|line| if line.is_empty() { line.to_string() } else { format!("{}{}", prefix, line) })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Construct a mask of the required width.
/// Width can be arbitrarily large.
fn mask(width: usize) -> String {
    let mut mask = "0x".to_string();
    if !width.is_multiple_of(4) {
        mask += &format!("{:x}", (1 << (width % 4)) - 1);
    }
    for _ in 0..width / 4 {
        mask += "f";
    }
    mask
}

fn generate_prelude() -> String {
    r#"from dataclasses import dataclass, field, fields
from typing import Optional, List, Tuple, Union
import enum
import inspect


class DecodeError(Exception):
    pass


class EnumValueError(DecodeError):
    def __init__(self, packet_name: str, field_name: str, enum_name: str, value: int):
        self.packet_name = packet_name
        self.field_name = field_name
        self.enum_name = enum_name
        self.value = value
        super().__init__(f"invalid {packet_name}.{field_name} value: {value} is not a valid {enum_name} value")


class FixedValueError(DecodeError):
    def __init__(self, packet_name: str, expected_value: int, actual_value: int):
        self.packet_name = packet_name
        self.expected_value = expected_value
        self.actual_value = actual_value
        super().__init__(f"invalid {packet_name} fixed value: expected {expected_value}, got {actual_value}")


class ConstraintValueError(DecodeError):
    def __init__(self, packet_name: str, field_name: int, expected_value: int, actual_value: int):
        self.packet_name = packet_name
        self.field_name = field_name
        self.expected_value = expected_value
        self.actual_value = actual_value
        super().__init__(f"invalid {packet_name}.{field_name} value: expected {expected_value}, got {actual_value}")


class LengthError(DecodeError):
    def __init__(self, packet_name: str, expected_size: int, actual_size: int):
        self.packet_name = packet_name
        self.expected_size = expected_size
        self.actual_size = actual_size
        super().__init__(f"invalid {packet_name} input size: expected {expected_size}, got {actual_size}")


class ArraySizeError(DecodeError):
    def __init__(self, packet_name: str, field_name: int, array_size: int, element_size: int):
        self.packet_name = packet_name
        self.field_name = field_name
        self.array_size = array_size
        self.element_size = element_size
        super().__init__(f"invalid {packet_name}.{field_name} size: {array_size} is not a multiple of the element size {element_size}")


class TrailingBytesError(DecodeError):
    def __init__(self, packet_name: str, trailing_size: int):
        self.packet_name = packet_name
        self.trailing_size = trailing_size
        super().__init__(f"unexpected {packet_name} parsing remainder of size {trailing_size}")


@dataclass
class Packet:
    payload: Optional[bytes] = field(repr=False, default_factory=bytes, compare=False)

    @classmethod
    def parse_all(cls, span: bytes) -> 'Packet':
        packet, remain = getattr(cls, 'parse')(span)
        if len(remain) > 0:
            raise TrailingBytesError(cls.__name__, len(remain))
        return packet

    @property
    def size(self) -> int:
        return 0

    def show(self, prefix: str = '') -> None:
        print(f'{self.__class__.__name__}')

        def print_val(p: str, pp: str, name: str, align: int, typ: object, val: object) -> None:
            if name == 'payload':
                pass

            # Scalar fields.
            elif typ is int:
                print(f'{p}{name:{align}} = {val} (0x{val:x})')

            # Byte fields.
            elif typ is bytes:
                print(f'{p}{name:{align}} = [', end='')
                line = ''
                n_pp = ''
                for (idx, b) in enumerate(val):
                    if idx > 0 and idx % 8 == 0:
                        print(f'{n_pp}{line}')
                        line = ''
                        n_pp = pp + (' ' * (align + 4))
                    line += f' {b:02x}'
                print(f'{n_pp}{line} ]')

            # Enum fields.
            elif inspect.isclass(typ) and issubclass(typ, enum.IntEnum):
                print(f'{p}{name:{align}} = {typ.__name__}::{val.name} (0x{val:x})')

            # Struct fields.
            elif inspect.isclass(typ) and issubclass(typ, globals().get('Packet')):
                print(f'{p}{name:{align}} = ', end='')
                val.show(prefix=pp)

            # Array fields.
            elif getattr(typ, '__origin__', None) is list:
                print(f'{p}{name:{align}}')
                last = len(val) - 1
                align = 5
                for (idx, elt) in enumerate(val):
                    n_p  = pp + ('├── ' if idx != last else '└── ')
                    n_pp = pp + ('│   ' if idx != last else '    ')
                    print_val(n_p, n_pp, f'[{idx}]', align, typ.__args__[0], val[idx])

            # Custom fields.
            elif inspect.isclass(typ):
                print(f'{p}{name:{align}} = {repr(val)}')

            else:
                print(f'{p}{name:{align}} = ##{typ}##')

        last = len(fields(self)) - 1
        align = max((len(f.name) for f in fields(self) if f.name != 'payload'), default=0)

        for (idx, f) in enumerate(fields(self)):
            p  = prefix + ('├── ' if idx != last else '└── ')
            pp = prefix + ('│   ' if idx != last else '    ')
            val = getattr(self, f.name)

            print_val(p, pp, f.name, align, f.type, val)
"#
    .to_string()
}

pub fn generate(
    sources: &ast::SourceDatabase,
    file: &ast::File,
    custom_type_location: Option<&str>,
    exclude_declarations: &[String],
) -> String {
    let mut code = String::new();
    let source = sources.get(file.file).expect("could not read source");
    let scope = analyzer::Scope::new(file).unwrap();
    let schema = analyzer::Schema::new(file);

    code.push_str(&format!("# File generated from {}, with the command:\n", source.name()));
    code.push_str("#  pdlc ...\n");
    code.push_str("# /!\\ Do not edit by hand.\n");

    let mut custom_types = Vec::new();
    for decl in &file.declarations {
        if let Some(id) = decl.id() {
            if exclude_declarations.contains(&id.to_string()) {
                continue;
            }
        }

        match &decl.desc {
            ast::DeclDesc::CustomField { id, .. } | ast::DeclDesc::Checksum { id, .. } => {
                custom_types.push(id.clone());
            }
            _ => {}
        }
    }

    if !custom_types.is_empty() {
        if let Some(location) = custom_type_location {
            code.push_str(&format!("\nfrom {} import {}\n", location, custom_types.join(", ")));
        }
    }

    code.push_str(&generate_prelude());

    for decl in &file.declarations {
        if let Some(id) = decl.id() {
            if exclude_declarations.contains(&id.to_string()) {
                continue;
            }
        }

        match &decl.desc {
            ast::DeclDesc::CustomField { id, .. } => {
                code.push_str(&format!("\nif (not callable(getattr({}, 'parse', None)) or\n", id));
                code.push_str(&format!("    not callable(getattr({}, 'parse_all', None))):\n", id));
                code.push_str(&format!(
                    "    raise Exception('The custom field type {} does not implement the parse method')\n",
                    id
                ));
            }
            ast::DeclDesc::Checksum { id, .. } => {
                code.push_str(&format!("\nif not callable({}):\n", id));
                code.push_str(&format!("    raise Exception('{} is not callable')\n", id));
            }
            _ => {}
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
            }
            ast::DeclDesc::Packet { parent_id: None, .. }
            | ast::DeclDesc::Struct { parent_id: None, .. } => {
                code.push_str(&generate_toplevel_packet_declaration(&scope, &schema, file, decl));
            }
            ast::DeclDesc::Packet { .. } | ast::DeclDesc::Struct { .. } => {
                code.push_str(&generate_derived_packet_declaration(&scope, &schema, file, decl));
            }
            _ => {}
        }
    }

    code
}

fn generate_enum_declaration(id: &str, tags: &[ast::Tag], _width: usize) -> String {
    let mut tag_decls = Vec::new();
    for tag in tags {
        if let ast::Tag::Value(t) = tag {
            tag_decls.push(format!("{} = {:#x}", t.id, t.value));
        }
    }

    let is_open = tags.iter().any(|t| matches!(t, ast::Tag::Other(_)));
    let mut unknown_handler = Vec::new();
    if is_open {
        unknown_handler.push("return v".to_string());
    } else {
        for tag in tags {
            if let ast::Tag::Range(t) = tag {
                unknown_handler.push(format!(
                    "if v >= {:#x} and v <= {:#x}:",
                    t.range.start(),
                    t.range.end()
                ));
                unknown_handler.push("    return v".to_string());
            }
        }
        unknown_handler.push(format!(r#"raise EnumValueError("", "", "{id}", v)"#));
    }

    format!(
        r#"
class {enum_name}(enum.IntEnum):
{tag_decls}

    @staticmethod
    def from_int(v: int) -> Union[int, '{enum_name}']:
        try:
            return {enum_name}(v)
        except ValueError:
{unknown_handler}
"#,
        enum_name = id,
        tag_decls = indent(&tag_decls.join("\n"), 1),
        unknown_handler = indent(&unknown_handler.join("\n"), 3)
    )
}

fn generate_packet_field_declarations<'a>(
    scope: &'a analyzer::Scope<'a>,
    decl: &ast::Decl,
) -> Vec<String> {
    let mut field_decls = Vec::new();
    for field in decl.fields() {
        if field.cond.is_some() {
            match &field.desc {
                ast::FieldDesc::Scalar { id, .. } => {
                    field_decls
                        .push(format!("{id}: Optional[int] = field(kw_only=True, default=None)"));
                }
                ast::FieldDesc::Typedef { id, type_id, .. } => {
                    field_decls.push(format!(
                        "{id}: Optional[{type_id}] = field(kw_only=True, default=None)",
                    ));
                }
                _ => unreachable!(),
            }
        } else {
            match &field.desc {
                ast::FieldDesc::Scalar { id, .. } => {
                    field_decls.push(format!("{id}: int = field(kw_only=True, default=0)"));
                }
                ast::FieldDesc::Typedef { id, type_id, .. } => {
                    let type_decl = scope.typedef.get(type_id.as_str()).unwrap();
                    match &type_decl.desc {
                        ast::DeclDesc::Enum { tags, .. } => {
                            field_decls.push(match tags.first() {
                                Some(ast::Tag::Range(t)) => {
                                    format!(
                                        "{id}: {type_id} = field(kw_only=True, default={})",
                                        t.range.start())
                                }
                                Some(ast::Tag::Value(t)) => {
                                    format!(
                                        "{id}: {type_id} = field(kw_only=True, default={type_id}.{})",
                                        t.id)
                                }
                                Some(_) => todo!(),
                                None => unreachable!(),
                            });
                        }
                        ast::DeclDesc::Checksum { .. } => {
                            field_decls
                                .push(format!("{id}: int = field(kw_only=True, default=0)",));
                        }
                        ast::DeclDesc::Struct { .. } | ast::DeclDesc::CustomField { .. } => {
                            field_decls.push(format!(
                                "{id}: {type_id} = field(kw_only=True, default_factory={type_id})",
                            ));
                        }
                        _ => unreachable!(),
                    }
                }
                ast::FieldDesc::Array { id, width: Some(8), .. } => {
                    field_decls.push(format!(
                        "{id}: bytearray = field(kw_only=True, default_factory=bytearray)",
                    ));
                }
                ast::FieldDesc::Array { id, width: Some(_), .. } => {
                    field_decls.push(format!(
                        "{id}: List[int] = field(kw_only=True, default_factory=list)",
                    ));
                }
                ast::FieldDesc::Array { id, width: None, type_id: Some(type_id), .. } => {
                    field_decls.push(format!(
                        "{id}: List[{type_id}] = field(kw_only=True, default_factory=list)",
                    ));
                }
                // Handled via presence of optional fields
                ast::FieldDesc::Flag { .. }
                | ast::FieldDesc::Padding { .. }
                | ast::FieldDesc::Reserved { .. }
                | ast::FieldDesc::Checksum { .. }
                | ast::FieldDesc::FixedScalar { .. }
                | ast::FieldDesc::FixedEnum { .. }
                | ast::FieldDesc::ElementSize { .. }
                | ast::FieldDesc::Size { .. }
                | ast::FieldDesc::Count { .. }
                | ast::FieldDesc::Payload { .. }
                | ast::FieldDesc::Body => (),
                _ => unreachable!(),
            }
        }
    }
    field_decls
}

fn generate_toplevel_packet_declaration<'a>(
    scope: &'a analyzer::Scope<'a>,
    schema: &analyzer::Schema,
    file: &ast::File,
    decl: &ast::Decl,
) -> String {
    let packet_name = decl.id().unwrap();
    let field_decls = generate_packet_field_declarations(scope, decl);
    let serializer = generate_toplevel_packet_serializer(scope, schema, file, decl);
    let parser = generate_packet_parser(scope, schema, file, decl);
    let encoded_size = generate_packet_size_property(scope, schema, decl);
    let post_init = generate_packet_post_init(scope, decl);

    format!(
        r#"
@dataclass
class {packet_name}(Packet):
{field_decls}

    def __post_init__(self) -> None:
{post_init}

    @staticmethod
    def parse(span: bytes) -> Tuple['{packet_name}', bytes]:
{parser}

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
{serializer}

    @property
    def encoded_size(self) -> int:
{encoded_size}
"#,
        field_decls = indent(&field_decls.join("\n"), 1),
        post_init = indent(&post_init.join("\n"), 2),
        parser = indent(&parser.join("\n"), 2),
        serializer = indent(&serializer.join("\n"), 2),
        encoded_size = indent(&encoded_size.join("\n"), 2)
    )
}

fn generate_derived_packet_declaration<'a>(
    scope: &'a analyzer::Scope<'a>,
    schema: &analyzer::Schema,
    file: &ast::File,
    decl: &ast::Decl,
) -> String {
    let packet_name = decl.id().unwrap();
    let parent_name = decl.parent_id().unwrap();
    let field_decls = generate_packet_field_declarations(scope, decl);
    let serializer = generate_derived_packet_serializer(scope, schema, file, decl);
    let parser = generate_packet_parser(scope, schema, file, decl);
    let encoded_size = generate_packet_size_property(scope, schema, decl);
    let post_init = generate_packet_post_init(scope, decl);

    format!(
        r#"
@dataclass
class {packet_name}({parent_name}):
{field_decls}

    def __post_init__(self) -> None:
{post_init}

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['{packet_name}', bytes]:
{parser}

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
{serializer}

    @property
    def encoded_size(self) -> int:
{encoded_size}
"#,
        field_decls = indent(&field_decls.join("\n"), 1),
        post_init = indent(&post_init.join("\n"), 2),
        parser = indent(&parser.join("\n"), 2),
        serializer = indent(&serializer.join("\n"), 2),
        encoded_size = indent(&encoded_size.join("\n"), 2)
    )
}

fn generate_packet_post_init<'a>(scope: &'a analyzer::Scope<'a>, decl: &ast::Decl) -> Vec<String> {
    let mut constraints = Vec::new();
    for parent in scope.iter_parents(decl) {
        for constraint in parent.constraints() {
            if constraints.iter().any(|c: &ast::Constraint| c.id == constraint.id) {
                continue;
            }
            constraints.push(constraint.clone());
        }
    }
    for constraint in decl.constraints() {
        if constraints.iter().any(|c: &ast::Constraint| c.id == constraint.id) {
            continue;
        }
        constraints.push(constraint.clone());
    }

    if constraints.is_empty() {
        return vec!["pass".to_string()];
    }

    let mut code = Vec::new();
    for c in constraints {
        if let Some(value) = c.value {
            code.push(format!("self.{} = {}", c.id, value));
        } else if let Some(tag_id) = &c.tag_id {
            let field = scope.iter_fields(decl).find(|f| f.id() == Some(&c.id)).unwrap();
            if let ast::FieldDesc::Typedef { type_id, .. } = &field.desc {
                code.push(format!("self.{} = {}.{}", c.id, type_id, tag_id));
            }
        }
    }
    code
}

fn generate_packet_size_property<'a>(
    scope: &'a analyzer::Scope<'a>,
    schema: &analyzer::Schema,
    decl: &ast::Decl,
) -> Vec<String> {
    let mut constant_width = 0;
    let mut variable_width = Vec::new();

    for field in decl.fields() {
        if field.cond.is_some() {
            match &field.desc {
                ast::FieldDesc::Scalar { id: field_id, width, .. } => {
                    variable_width
                        .push(format!("(0 if self.{field_id} is None else {})", width / 8));
                }
                ast::FieldDesc::Typedef { id: field_id, type_id, .. } => {
                    let type_decl = scope.typedef.get(type_id.as_str()).unwrap();
                    match &type_decl.desc {
                        ast::DeclDesc::Enum { width, .. } => {
                            variable_width
                                .push(format!("(0 if self.{field_id} is None else {})", width / 8));
                        }
                        _ => {
                            variable_width.push(format!(
                                "(0 if self.{field_id} is None else self.{field_id}.encoded_size)",
                            ));
                        }
                    }
                }
                _ => {}
            }
            continue;
        }

        if let Some(padded_size) = schema.padded_size(field.key) {
            constant_width += padded_size;
            continue;
        }

        if let analyzer::Size::Static(w) = schema.field_size(field.key) {
            constant_width += w;
            continue;
        }

        match &field.desc {
            ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body => {
                variable_width.push("len(self.payload)".to_string());
            }
            ast::FieldDesc::Typedef { id: field_id, .. } => {
                variable_width.push(format!("self.{}.encoded_size", field_id));
            }
            ast::FieldDesc::Array { id: field_id, width: Some(8), .. } => {
                variable_width.push(format!("len(self.{field_id})"));
            }
            ast::FieldDesc::Array { id: field_id, width: Some(width), .. } => {
                variable_width.push(format!("len(self.{field_id}) * {}", width / 8));
            }
            ast::FieldDesc::Array { id: field_id, width: None, type_id: Some(type_id), .. } => {
                let type_decl = scope.typedef.get(type_id.as_str()).unwrap();
                match &type_decl.desc {
                    ast::DeclDesc::Enum { width, .. } => {
                        variable_width.push(format!("len(self.{field_id}) * {}", width / 8));
                    }
                    _ => {
                        variable_width.push(format!("sum([elt.encoded_size for elt in self.{field_id}])",));
                    }
                }
            }
            _ => {}
        }
    }

    match (variable_width.as_slice(), constant_width / 8) {
        ([], c) => vec![format!("return {c}")],
        ([v], 0) => vec![format!("return {v}")],
        ([v], c) => vec![format!("return {v} + {c}")],
        (v, 0) => {
            let mut res = vec!["return (".to_string()];
            res.push(indent(&v.join(" +\n"), 1));
            res.push(")".to_string());
            res
        }
        (v, c) => {
            let mut res = vec![format!("return {c} + (")];
            res.push(indent(&v.join(" +\n"), 1));
            res.push(")".to_string());
            res
        }
    }
}

fn generate_toplevel_packet_serializer<'a>(
    scope: &'a analyzer::Scope<'a>,
    schema: &'a analyzer::Schema,
    file: &ast::File,
    decl: &ast::Decl,
) -> Vec<String> {
    let mut serializer = FieldSerializer::new(scope, schema, file.endianness.value);
    for field in decl.fields() {
        serializer.serialize(decl, field);
    }
    let mut code = vec!["_span = bytearray()".to_string()];
    code.extend(serializer.code);
    code.push("return bytes(_span)".to_string());
    code
}

fn generate_derived_packet_serializer<'a>(
    scope: &'a analyzer::Scope<'a>,
    schema: &'a analyzer::Schema,
    file: &ast::File,
    decl: &ast::Decl,
) -> Vec<String> {
    let parent = scope.get_parent(decl).unwrap();
    let mut serializer = FieldSerializer::new(scope, schema, file.endianness.value);
    for field in decl.fields() {
        serializer.serialize(decl, field);
    }
    let mut code = vec!["_span = bytearray()".to_string()];
    code.extend(serializer.code);
    code.push(format!("return {}.serialize(self, payload = bytes(_span))", parent.id().unwrap()));
    code
}

fn get_specialized_children<'a>(file: &'a ast::File, decl: &'a ast::Decl) -> Vec<&'a ast::Decl> {
    let mut children = Vec::new();
    for d in &file.declarations {
        if d.parent_id() == decl.id() {
            let is_alias = d
                .fields()
                .all(|f| matches!(f.desc, ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body));
            if is_alias {
                children.extend(get_specialized_children(file, d));
            } else {
                children.push(d);
            }
        }
    }
    children
}

/// Generate the parse() function for a toplevel Packet or Struct declaration.
fn generate_packet_parser<'a>(
    scope: &'a analyzer::Scope<'a>,
    schema: &'a analyzer::Schema,
    file: &ast::File,
    decl: &ast::Decl,
) -> Vec<String> {
    let mut code = if scope.get_parent(decl).is_none() {
        vec!["fields = {'payload': None}".to_string()]
    } else {
        vec![]
    };

    let packet_name = decl.id().unwrap();

    for c in scope.iter_constraints(decl) {
        match c {
            ast::Constraint { id, value: Some(v), .. } => {
                code.push(format!("if fields['{id}'] != {v}:"));
                code.push(format!(r#"    raise ConstraintValueError("{packet_name}", "{id}", {v}, fields['{id}'])"#));
            }
            ast::Constraint { id, tag_id: Some(tag_id), .. } => {
                let field = scope.iter_fields(decl).find(|f| f.id() == Some(&c.id)).unwrap();
                if let ast::FieldDesc::Typedef { type_id, .. } = &field.desc {
                    code.push(format!("if fields['{id}'] != {type_id}.{tag_id}:"));
                    code.push(format!(r#"    raise ConstraintValueError("{packet_name}", "{id}", int({type_id}.{tag_id}), int(fields['{id}']))"#));
                }
            }
            _ => unreachable!(),
        }
    }

    // Parse fields iteratively.
    let mut parser = FieldParser::new(scope, schema, file, decl);
    for field in decl.fields() {
        parser.parse(field);
    }
    parser.done();

    // Specialize to child packets.
    let children = get_specialized_children(file, decl);
    let mut specialization = CodeBlock::default();

    // Try parsing every child packet successively until one is
    // successfully parsed. Return a parsing error if none is valid.
    // Return parent packet if no child packet matches.
    // TODO: order child packets by decreasing size in case no constraint
    // is given for specialization.
    match &children.as_slice() {
        [] => (),
        [child] => {
            let child_name = child.id().unwrap();
            specialization.append(format!(
                r#"
try:
    child, remainder = {child_name}.parse(fields, payload)
    if remainder:
        raise TrailingBytesError("{child_name}", len(remainder))
    return child, span
except DecodeError:
    pass
                        "#
            ));
        }
        _ => {
            let children_classes = children
                .iter()
                .map(|child| child.id().unwrap())
                .collect::<Vec<_>>()
                .join(",\n            ");
            specialization.append(format!(
                r#"
for cls in [{children_classes}]:
    try:
        child, remainder = cls.parse(fields, payload)
        if remainder:
            raise TrailingBytesError(cls.__name__, len(remainder))
        return child, span
    except DecodeError:
        pass
                        "#
            ));
        }
    }

    code.extend(parser.code.lines);
    code.extend(specialization.lines);
    code.push(format!("return {}(**fields), span", decl.id().unwrap()));
    code
}

struct FieldParser<'a> {
    scope: &'a analyzer::Scope<'a>,
    schema: &'a analyzer::Schema,
    file: &'a ast::File,
    decl: &'a ast::Decl,
    offset: usize,
    shift: usize,
    chunk: Vec<(usize, usize, &'a ast::Field)>,
    unchecked_code: CodeBlock,
    code: CodeBlock,
}

impl<'a> FieldParser<'a> {
    fn new(
        scope: &'a analyzer::Scope<'a>,
        schema: &'a analyzer::Schema,
        file: &'a ast::File,
        decl: &'a ast::Decl,
    ) -> Self {
        Self {
            scope,
            schema,
            file,
            decl,
            offset: 0,
            shift: 0,
            chunk: Vec::new(),
            unchecked_code: CodeBlock::default(),
            code: CodeBlock::default(),
        }
    }

    fn unchecked_append(&mut self, code: String) {
        self.unchecked_code.append(code);
    }

    fn do_append(&mut self, code: String) {
        self.code.append(code)
    }

    fn append(&mut self, code: String) {
        self.check_code();
        self.code.append(code);
    }

    fn check_size(&mut self, size: String) {
        let packet_name = self.decl.id().unwrap();
        self.code.append(format!(
            r#"
if len(span) < {size}:
    raise LengthError("{packet_name}", {size}, len(span))
            "#
        ));
    }

    fn check_code(&mut self) {
        if !self.unchecked_code.is_empty() {
            assert!(self.chunk.is_empty());
            let unchecked_code = std::mem::take(&mut self.unchecked_code.lines);
            let offset = self.offset;
            self.check_size(offset.to_string());
            self.code.lines.extend(unchecked_code);
        }
    }

    fn consume_span(&mut self, keep: usize) {
        if self.offset > 0 {
            self.check_code();
            let offset = self.offset;
            self.code.append(format!("span = span[{}:]", offset - keep));
            self.offset = 0;
        }
    }

    fn parse_bit_field(&mut self, field: &'a ast::Field) {
        let analyzer::Size::Static(width) = self.schema.field_size(field.key) else {
            unreachable!()
        };

        // Add to current chunk.
        self.chunk.push((self.shift, width, field));
        self.shift += width;

        // Wait for more fields if not on a byte boundary.
        if !self.shift.is_multiple_of(8) {
            return;
        }

        // Parse the backing integer using the configured endiannes,
        // extract field values.
        let size = self.shift / 8;
        let end_offset = self.offset + size;
        let byteorder = match self.file.endianness.value {
            ast::EndiannessValue::LittleEndian => "little",
            ast::EndiannessValue::BigEndian => "big",
        };

        let value = if size == 1 {
            format!("span[{}]", self.offset)
        } else {
            self.unchecked_append(format!(
                "value_ = int.from_bytes(span[{}:{}], byteorder='{byteorder}')",
                self.offset, end_offset
            ));
            "value_".to_string()
        };

        let packet_name = self.decl.id().unwrap();
        let chunk = std::mem::take(&mut self.chunk);

        for (shift, width, field) in chunk {
            let v = if shift == 0 && width == self.shift {
                value.clone()
            } else {
                format!("({value} >> {shift}) & {}", mask(width))
            };

            match &field.desc {
                ast::FieldDesc::Scalar { id, .. } => {
                    self.unchecked_append(format!("fields['{}'] = {}", id, v));
                }
                ast::FieldDesc::FixedScalar { value, .. } => {
                    self.unchecked_append(format!(
                        r#"
if {v} != {value:#x}:
    raise FixedValueError("{packet_name}", {v}, {value})
                        "#
                    ));
                }
                ast::FieldDesc::FixedEnum { enum_id, tag_id, .. } => {
                    self.unchecked_append(format!(
                        r#"
if {v} != {enum_id}.{tag_id}:
    raise FixedValueError("{packet_name}", {v}, int({enum_id}.{tag_id}))
                        "#
                    ));
                }
                ast::FieldDesc::Typedef { id, type_id, .. } => {
                    self.unchecked_append(format!("fields['{id}'] = {type_id}.from_int({v})"));
                }
                ast::FieldDesc::Size { field_id, .. } => {
                    self.unchecked_append(format!("{field_id}_size = {v}"));
                }
                ast::FieldDesc::Count { field_id, .. } => {
                    self.unchecked_append(format!("{field_id}_count = {v}"));
                }
                ast::FieldDesc::ElementSize { .. } => {
                    todo!()
                }
                ast::FieldDesc::Reserved { .. } => {}
                ast::FieldDesc::Flag { id, .. } => {
                    self.unchecked_append(format!("{id} = {v}"));
                }
                _ => unreachable!(),
            }
        }

        self.offset = end_offset;
        self.shift = 0;
    }

    fn parse_checksum_field(&mut self, id: &str) {
        let value_field = self.decl.fields().find(|f| f.id() == Some(id)).unwrap();
        let type_id = match &value_field.desc {
            ast::FieldDesc::Typedef { type_id, .. } => type_id,
            _ => unreachable!(),
        };

        let value_size = self.schema.field_size(value_field.key).static_().unwrap() / 8;

        let packet_name = self.decl.id().unwrap();
        let mut offset_from_start: isize = -1;
        let mut offset_from_end: isize = -1;
        let mut found_start = false;
        let mut found_value = false;

        for f in self.decl.fields() {
            if let ast::FieldDesc::Checksum { field_id } = &f.desc {
                if field_id == id {
                    found_start = true;
                    offset_from_start = 0;
                    continue;
                }
            }
            if f == value_field {
                found_value = true;
                offset_from_end = 0;
                continue;
            }
            if found_start && !found_value {
                match self.schema.field_size(f.key) {
                    analyzer::Size::Static(w) => {
                        if offset_from_start != -1 {
                            offset_from_start += w as isize;
                        }
                    }
                    _ => {
                        offset_from_start = -1;
                    }
                }
            }
            if found_value {
                match self.schema.field_size(f.key) {
                    analyzer::Size::Static(w) => {
                        if offset_from_end != -1 {
                            offset_from_end += w as isize;
                        }
                    }
                    _ => {
                        offset_from_end = -1;
                    }
                }
            }
        }

        let type_decl = self.scope.typedef.get(type_id.as_str()).unwrap();
        let function = match &type_decl.desc {
            ast::DeclDesc::Checksum { function, .. } => function,
            _ => unreachable!(),
        };

        let byteorder = match self.file.endianness.value {
            ast::EndiannessValue::LittleEndian => "little",
            ast::EndiannessValue::BigEndian => "big",
        };

        if offset_from_start != -1 {
            let offset_bytes = offset_from_start / 8;
            let start_offset = offset_bytes + value_size as isize;

            self.unchecked_append(format!(
                r#"
if len(span) < {start_offset}:
    raise LengthError("{packet_name}", {start_offset}, len(span))
                "#
            ));

            let value = if value_size > 1 {
                format!(
                    "int.from_bytes(span[{offset_bytes}:{start_offset}], byteorder='{byteorder}')"
                )
            } else {
                format!("span[{offset_bytes}]")
            };

            self.unchecked_append(format!(
                r#"
{id} = {value}
fields['{id}'] = {id}
computed_{id} = {function}(span[:{offset_bytes}])
if computed_{id} != {id}:
    raise Exception(f'Invalid checksum computation: {{computed_{id}}} != {{{id}}}')
                "#,
            ));
        } else if offset_from_end != -1 {
            let offset_bytes = offset_from_end / 8;
            let end_offset = offset_bytes + value_size as isize;

            self.unchecked_append(format!(
                r#"
if len(span) < {end_offset}:
    raise LengthError("{packet_name}", {end_offset}, len(span))
                "#,
            ));

            let value = if value_size > 1 {
                format!(
                    "int.from_bytes(span[-{end_offset}:-{}], byteorder='{byteorder}')",
                    if offset_bytes == 0 { "".to_string() } else { offset_bytes.to_string() },
                )
            } else {
                format!("span[-{end_offset}]")
            };

            self.unchecked_append(format!(
                r#"
{id} = {value}
fields['{id}'] = {id}
computed_{id} = {function}(span[:-{end_offset}])
if computed_{id} != {id}:
    raise Exception(f'Invalid checksum computation: {{computed_{id}}} != {{{id}}}')
                "#
            ));
        }
    }

    fn parse_typedef_field(&mut self, field: &'a ast::Field) {
        if self.shift != 0 {
            panic!("Typedef field does not start on an octet boundary");
        }
        let (id, type_id) = match &field.desc {
            ast::FieldDesc::Typedef { id, type_id, .. } => (id, type_id),
            _ => unreachable!(),
        };

        match self.schema.field_size(field.key) {
            analyzer::Size::Static(w) => {
                let size = w / 8;
                let start_offset = self.offset;
                let end_offset = self.offset + size;
                let type_decl = self.scope.typedef.get(type_id.as_str()).unwrap();
                if matches!(type_decl.desc, ast::DeclDesc::Checksum { .. }) {
                    // Handled by parse_checksum_start.
                } else {
                    self.unchecked_append(format!(
                        "fields['{id}'] = {type_id}.parse_all(span[{start_offset}:{end_offset}])",
                    ));
                }
                self.offset = end_offset;
            }
            _ => {
                self.consume_span(0);
                self.append(format!(
                    r#"
{id}, span = {type_id}.parse(span)
fields['{id}'] = {id}
                    "#
                ));
            }
        }
    }

    fn parse_payload_field(&mut self, field: &'a ast::Field) {
        let packet_name = self.decl.id().unwrap();
        self.consume_span(0);
        let id = match &field.desc {
            ast::FieldDesc::Payload { .. } => "_payload_",
            ast::FieldDesc::Body => "_body_",
            _ => unreachable!(),
        };

        let size_field = self.decl.fields().find(|f| match &f.desc {
            ast::FieldDesc::Size { field_id, .. } => field_id == id,
            _ => false,
        });

        if let Some(_f) = size_field {
            if let ast::FieldDesc::Payload { size_modifier: Some(modifier), .. } = &field.desc {
                self.append(format!("{}_size -= {}", id, modifier));
            }
            self.append(format!(
                r#"
if len(span) < {id}_size:
    raise LengthError("{packet_name}", {id}_size, len(span))
payload = span[:{id}_size]
span = span[{id}_size:]
                "#
            ));
        } else {
            let mut offset_from_end = 0;
            let mut found = false;
            for f in self.decl.fields() {
                if f == field {
                    found = true;
                    continue;
                }
                if found {
                    if let analyzer::Size::Static(w) = self.schema.field_size(f.key) {
                        offset_from_end += w;
                    }
                }
            }

            if offset_from_end == 0 {
                self.append(
                    r#"
payload = span
span = bytes([])
                    "#
                    .to_string(),
                );
            } else {
                let offset_bytes = offset_from_end / 8;
                self.append(format!(
                    r#"
if len(span) < {offset_bytes}:
    raise LengthError("{packet_name}", {offset_bytes}, len(span))
payload = span[:-{offset_bytes}]
span = span[-{offset_bytes}:]
                    "#
                ));
            }
        }
        self.append("fields['payload'] = payload".to_string());
    }

    fn parse_array_field(&mut self, field: &'a ast::Field) {
        let ast::FieldDesc::Array { id, size_modifier, .. } = &field.desc else {
            return;
        };

        let element_size = analyzer::element_size(self.scope, self.schema, self.decl, field);
        let array_size = analyzer::array_size(self.decl, field);
        let padded_size = self.schema.padded_size(field.key);
        let packet_name = self.decl.id().unwrap();

        // Shift the span to reset the offset to 0.
        self.consume_span(0);

        // Apply the size modifier.
        if let Some(size_modifier) = size_modifier {
            self.append(format!("{id}_size = {id}_size - {size_modifier}"));
        }

        // Parse from the padded array if padding is present.
        if let Some(padded_size) = padded_size {
            let padded_size = padded_size / 8;
            self.check_size(format!("{padded_size}"));
            self.append(format!("remaining_span = span[{padded_size}:]"));
            self.append(format!("span = span[:{padded_size}]"));
        }

        use analyzer::{ArraySize, ElementSize};
        match (element_size, array_size) {
            (ElementSize::Static(element_size), ArraySize::StaticCount(count)) => {
                let total_size = element_size * count;
                self.check_size(format!("{total_size}"));
                self.append(format!("{id} = []"));
                self.append(format!("for n in range({count}):"));

                let element_span = if element_size == 1 {
                    "span[n:n + 1]".to_string()
                } else {
                    format!("span[n * {element_size}:(n + 1) * {element_size}]")
                };

                self.parse_array_element_static(field, element_span);
                self.append(format!("fields['{id}'] = {id}"));
                self.append(format!("span = span[{total_size}:]"));
            }

            (ElementSize::Static(1), ArraySize::DynamicCount)
            | (ElementSize::Static(1), ArraySize::DynamicSize) => {
                let count =
                    if matches!(array_size, ArraySize::DynamicSize) { "size" } else { "count" };
                self.check_size(format!("{id}_{count}"));
                self.append(format!("{id} = []"));
                self.append(format!("for n in range({id}_{count}):"));

                self.parse_array_element_static(field, "span[n:n + 1]".to_string());
                self.append(format!("fields['{id}'] = {id}"));
                self.append(format!("span = span[{id}_{count}:]"));
            }

            (ElementSize::Static(element_size), ArraySize::DynamicCount) => {
                self.check_size(format!("{element_size} * {id}_count"));
                self.append(format!("{id} = []"));
                self.append(format!("for n in range({id}_count):"));

                let element_span = if element_size == 1 {
                    "span[n:n + 1]".to_string()
                } else {
                    format!("span[n * {element_size}:(n + 1) * {element_size}]")
                };

                self.parse_array_element_static(field, element_span);
                self.append(format!("fields['{id}'] = {id}"));
                self.append(format!("span = span[{id}_count * {element_size}:]"));
            }

            (ElementSize::Static(element_size), ArraySize::DynamicSize) => {
                self.check_size(format!("{id}_size"));
                self.append(format!("if {id}_size % {element_size} != 0:"));
                self.append(
                    format!(r#"    raise ArraySizeError("{packet_name}", "{id}", {id}_size, {element_size})"#)
                );
                self.append(format!("{id}_count = int({id}_size / {element_size})"));
                self.append(format!("{id} = []"));
                self.append(format!("for n in range({id}_count):"));

                let element_span = if element_size == 1 {
                    "span[n:n + 1]".to_string()
                } else {
                    format!("span[n * {element_size}:(n + 1) * {element_size}]")
                };

                self.parse_array_element_static(field, element_span);
                self.append(format!("fields['{id}'] = {id}"));
                self.append(format!("span = span[{id}_size:]"));
            }

            (ElementSize::Static(1), ArraySize::Unknown) => {
                self.append(format!("{id} = []"));
                self.append("for n in range(len(span)):".to_string());
                self.parse_array_element_static(field, "span[n:n + 1]".to_string());
                self.append(format!("fields['{id}'] = {id}"));
                self.append("span = bytes()".to_string());
            }

            (ElementSize::Static(element_size), ArraySize::Unknown) => {
                self.append(format!("if len(span) % {element_size} != 0:"));
                self.append(
                    format!(r#"    raise ArraySizeError("{packet_name}", "{id}", len(span), {element_size})"#)
                );
                self.append(format!("{id}_count = int(len(span) / {element_size})"));
                self.append(format!("{id} = []"));
                self.append(format!("for n in range({id}_count):"));
                self.parse_array_element_static(
                    field,
                    format!("span[n * {element_size}:(n + 1) * {element_size}]"),
                );
                self.append(format!("fields['{id}'] = {id}"));
                self.append("span = bytes()".to_string());
            }

            (ElementSize::Dynamic, ArraySize::StaticCount(_count)) => todo!(),
            (ElementSize::Dynamic, ArraySize::DynamicCount) => todo!(),
            (ElementSize::Dynamic, ArraySize::DynamicSize) => todo!(),
            (ElementSize::Dynamic, ArraySize::Unknown) => todo!(),

            (ElementSize::Unknown, ArraySize::StaticCount(count)) => {
                self.append(format!("{id} = []"));
                self.append(format!("for n in range({count}):"));
                self.parse_array_element_dynamic(field, "span".to_string());
                self.append(format!("fields['{id}'] = {id}"));
            }

            (ElementSize::Unknown, ArraySize::DynamicCount) => {
                self.append(format!("{id} = []"));
                self.append(format!("for n in range({id}_count):"));
                self.parse_array_element_dynamic(field, "span".to_string());
                self.append(format!("fields['{id}'] = {id}"));
            }

            (ElementSize::Unknown, ArraySize::DynamicSize) => {
                self.check_size(format!("{id}_size"));
                self.append(format!("array_span = span[:{id}_size]"));
                self.append(format!("{id} = []"));
                self.append("while len(array_span) > 0:".to_string());
                self.parse_array_element_dynamic(field, "array_span".to_string());
                self.append(format!("fields['{id}'] = {id}"));
                self.append(format!("span = span[{id}_size:]"));
            }

            (ElementSize::Unknown, ArraySize::Unknown) => {
                self.append(format!("{} = []", id));
                self.append("while len(span) > 0:".to_string());
                self.parse_array_element_dynamic(field, "span".to_string());
                self.append(format!("fields['{}'] = {}", id, id));
            }
        }

        if padded_size.is_some() {
            self.append("span = remaining_span".to_string());
        }
    }

    fn parse_array_element_static(&mut self, field: &'a ast::Field, span: String) {
        let byteorder = match self.file.endianness.value {
            ast::EndiannessValue::LittleEndian => "little",
            ast::EndiannessValue::BigEndian => "big",
        };
        match &field.desc {
            ast::FieldDesc::Array { id, type_id: None, .. } => {
                self.do_append(format!(
                    "    {id}.append(int.from_bytes({span}, byteorder='{byteorder}'))"
                ));
            }
            ast::FieldDesc::Array { id, type_id: Some(type_id), .. } => {
                match &self.scope.typedef.get(type_id).unwrap().desc {
                    ast::DeclDesc::Enum { .. } => {
                        let value = format!("int.from_bytes({span}, byteorder='{byteorder}')");
                        self.do_append(format!("    {id}.append({type_id}.from_int({value}))"));
                    }
                    _ => {
                        self.do_append(format!("    {id}.append({type_id}.parse_all({span}))"));
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn parse_array_element_dynamic(&mut self, field: &'a ast::Field, span: String) {
        let ast::FieldDesc::Array { id, type_id: Some(type_id), .. } = &field.desc else {
            unreachable!()
        };
        self.do_append(format!("    _elt, {span} = {type_id}.parse({span})"));
        self.do_append(format!("    {id}.append(_elt)"));
    }

    fn parse_optional_field(&mut self, field: &'a ast::Field) {
        self.consume_span(0);
        let cond = field.cond.as_ref().unwrap();
        let id = field.id().unwrap();
        let packet_name = self.decl.id().unwrap();
        let byteorder = match self.file.endianness.value {
            ast::EndiannessValue::LittleEndian => "little",
            ast::EndiannessValue::BigEndian => "big",
        };

        match &field.desc {
            ast::FieldDesc::Scalar { width, .. } => {
                self.append(format!("if {} == {}:", cond.id, cond.value.unwrap()));
                self.append(format!("    if len(span) < {}:", width / 8));
                self.append(format!(
                    r#"        raise LengthError("{packet_name}", {}, len(span))"#,
                    width / 8
                ));
                self.append(format!(
                    "    fields['{}'] = int.from_bytes(span[:{}], byteorder='{}')",
                    id,
                    width / 8,
                    byteorder
                ));
                self.append(format!("    span = span[{}:]", width / 8));
            }
            ast::FieldDesc::Typedef { type_id, .. } => {
                let type_decl = self.scope.typedef.get(type_id.as_str()).unwrap();
                match &type_decl.desc {
                    ast::DeclDesc::Enum { width, .. } => {
                        self.append(format!("if {} == {}:", cond.id, cond.value.unwrap()));
                        self.append(format!("    if len(span) < {}:", width / 8));
                        self.append(format!(
                            r#"        raise LengthError("{packet_name}", {}, len(span))"#,
                            width / 8
                        ));
                        self.append(format!(
                            "    fields['{}'] = {}(int.from_bytes(span[:{}], byteorder='{}'))",
                            id,
                            type_id,
                            width / 8,
                            byteorder
                        ));
                        self.append(format!("    span = span[{}:]", width / 8));
                    }
                    _ => {
                        self.append(format!("if {} == {}:", cond.id, cond.value.unwrap()));
                        self.append(format!("    {}, span = {}.parse(span)", id, type_id));
                        self.append(format!("    fields['{}'] = {}", id, id));
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn parse(&mut self, field: &'a ast::Field) {
        if field.cond.is_some() {
            self.parse_optional_field(field);
            return;
        }

        match &field.desc {
            ast::FieldDesc::Scalar { .. }
            | ast::FieldDesc::FixedScalar { .. }
            | ast::FieldDesc::FixedEnum { .. }
            | ast::FieldDesc::Reserved { .. }
            | ast::FieldDesc::Size { .. }
            | ast::FieldDesc::Count { .. }
            | ast::FieldDesc::ElementSize { .. }
            | ast::FieldDesc::Flag { .. } => self.parse_bit_field(field),
            ast::FieldDesc::Checksum { field_id } => self.parse_checksum_field(field_id),
            ast::FieldDesc::Typedef { type_id, .. } => {
                let type_decl = self.scope.typedef.get(type_id.as_str()).unwrap();
                match &type_decl.desc {
                    ast::DeclDesc::Enum { .. } => self.parse_bit_field(field),
                    _ => self.parse_typedef_field(field),
                }
            }
            ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body => {
                self.parse_payload_field(field)
            }
            ast::FieldDesc::Array { .. } => self.parse_array_field(field),
            _ => {}
        }
    }

    fn done(&mut self) {
        self.consume_span(0);
    }
}

struct FieldSerializer<'a> {
    scope: &'a analyzer::Scope<'a>,
    schema: &'a analyzer::Schema,
    byteorder: &'static str,
    shift: usize,
    values: Vec<String>,
    code: Vec<String>,
}

impl<'a> FieldSerializer<'a> {
    fn new(
        scope: &'a analyzer::Scope<'a>,
        schema: &'a analyzer::Schema,
        byteorder: ast::EndiannessValue,
    ) -> Self {
        Self {
            scope,
            schema,
            byteorder: match byteorder {
                ast::EndiannessValue::LittleEndian => "little",
                ast::EndiannessValue::BigEndian => "big",
            },
            shift: 0,
            values: Vec::new(),
            code: Vec::new(),
        }
    }

    fn append(&mut self, line: String) {
        self.code.push(line);
    }

    fn encode_int(&mut self, value: &str, size: usize) {
        if size == 1 {
            self.append(format!("_span.append({value})"))
        } else {
            let byteorder = self.byteorder;
            self.append(format!(
                "_span.extend(int.to_bytes({value}, length={size}, byteorder='{byteorder}'))"
            ))
        }
    }

    fn pack_bit_fields(&mut self) {
        let size = self.shift / 8;

        if self.values.is_empty() {
            // This condition is true when the bit fields are all reserved fields.
            self.append(format!("_span.extend([0] * {})", size));
        } else if self.values.len() == 1 {
            self.encode_int(&self.values[0].clone(), size);
        } else {
            self.append("_value = (".to_string());
            self.append("    ".to_string() + &self.values.join(" |\n    "));
            self.append(")".to_string());
            self.encode_int("_value", size);
        }

        self.shift = 0;
        self.values.clear();
    }

    fn serialize_bit_field(&mut self, decl: &ast::Decl, field: &ast::Field) {
        let analyzer::Size::Static(width) = self.schema.field_size(field.key) else {
            unreachable!()
        };
        let decl_id = decl.id().unwrap();
        let shift = self.shift;

        match &field.desc {
            ast::FieldDesc::Scalar { id, .. } => {
                let max_value = mask(width);
                self.append(format!("if self.{id} > {max_value}:"));
                self.append(format!("    raise ValueError(\"Invalid scalar value {decl_id}::{id}: {{self.{id}}} > {max_value}\")"));
                self.values.push(format!("(self.{} << {})", id, shift));
            }
            ast::FieldDesc::FixedScalar { value, .. } => {
                self.values.push(format!("({:#x} << {})", value, shift));
            }
            ast::FieldDesc::FixedEnum { enum_id, tag_id, .. } => {
                self.values.push(format!("({}.{} << {})", enum_id, tag_id, shift));
            }
            ast::FieldDesc::Typedef { id, .. } => {
                self.values.push(format!("(self.{} << {})", id, shift));
            }
            ast::FieldDesc::Reserved { .. } => {}
            ast::FieldDesc::Size { field_id, .. } => {
                let max_size = mask(width);
                let value_field = self
                    .scope
                    .iter_fields(decl)
                    .find(|field| match &field.desc {
                        ast::FieldDesc::Payload { .. } => field_id == "_payload_",
                        ast::FieldDesc::Body => field_id == "_body_",
                        _ => field.id() == Some(field_id),
                    })
                    .unwrap();

                let size = match &value_field.desc {
                    ast::FieldDesc::Payload { size_modifier: Some(size_modifier) } => {
                        self.append(format!(
                            "_payload_size = len(payload or self.payload or []) + {size_modifier}"
                        ));
                        "_payload_size".to_string()
                    }
                    ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body => {
                        self.append(
                            "_payload_size = len(payload or self.payload or [])".to_string(),
                        );
                        "_payload_size".to_string()
                    }
                    ast::FieldDesc::Array { size_modifier, .. } => {
                        let array_size = match analyzer::element_size(
                            self.scope,
                            self.schema,
                            decl,
                            value_field,
                        ) {
                            analyzer::ElementSize::Static(1) => {
                                format!("len(self.{field_id})")
                            }
                            analyzer::ElementSize::Static(size) => {
                                format!("len(self.{field_id}) * {size}")
                            }
                            analyzer::ElementSize::Dynamic | analyzer::ElementSize::Unknown => {
                                format!("sum(elt.encoded_size for elt in self.{field_id})")
                            }
                        };
                        let size_modifier = if let Some(size_modifier) = size_modifier {
                            format!("{size_modifier} + ")
                        } else {
                            String::new()
                        };
                        self.append(format!("{field_id}_size = {size_modifier}{array_size}"));
                        format!("{field_id}_size")
                    }
                    _ => unreachable!(),
                };

                self.append(format!("if {size} > {max_size}:"));
                self.append(format!("    raise ValueError(\"Invalid size value {decl_id}::{field_id}: {{{size}}} > {max_size}\")"));
                self.values.push(format!("({size} << {shift})"));
            }
            ast::FieldDesc::Count { field_id, .. } => {
                let max_count = mask(width);
                self.append(format!("if len(self.{field_id}) > {max_count}:"));
                self.append(format!("    raise ValueError(\"Invalid count value {decl_id}::{field_id}: {{len(self.{field_id})}} > {max_count}\")"));
                self.values.push(format!("(len(self.{field_id}) << {shift})"));
            }
            ast::FieldDesc::ElementSize { .. } => {
                todo!()
            }
            ast::FieldDesc::Flag { id: _, optional_field_ids, .. } => {
                let optional_field_id = &optional_field_ids[0].0;
                let value_present = optional_field_ids[0].1;
                let value_absent = if value_present == 0 { 1 } else { 0 };
                self.values.push(format!(
                    "(({} if self.{} is None else {}) << {})",
                    value_absent, optional_field_id, value_present, shift
                ));
            }
            _ => unreachable!(),
        };

        self.shift += width;
        if self.shift.is_multiple_of(8) {
            self.pack_bit_fields();
        }
    }

    fn serialize_checksum_field(&mut self, _field: &ast::Field) {
        self.append("_checksum_start = len(_span)".to_string());
    }

    fn serialize_typedef_field(&mut self, field: &ast::Field) {
        let (id, type_id) = match &field.desc {
            ast::FieldDesc::Typedef { id, type_id, .. } => (id, type_id),
            _ => unreachable!(),
        };
        let type_decl = self.scope.typedef.get(type_id.as_str()).unwrap();
        if let ast::DeclDesc::Checksum { function, width, .. } = &type_decl.desc {
            let size = width / 8;
            self.append(format!("_checksum = {}(_span[_checksum_start:])", function));
            if size == 1 {
                self.append("_span.append(_checksum)".to_string());
            } else {
                self.append(format!(
                    "_span.extend(int.to_bytes(_checksum, length={}, byteorder='{}'))",
                    size, self.byteorder
                ));
            }
        } else {
            self.append(format!("_span.extend(self.{}.serialize())", id));
        }
    }

    fn serialize_payload_field(&mut self, _field: &ast::Field) {
        //self.pack_bit_fields();
        self.append("_span.extend(payload or self.payload or [])".to_string());
    }

    fn serialize_array_field(&mut self, field: &ast::Field) {
        let id = field.id().unwrap();
        let padded_size = self.schema.padded_size(field.key);
        if padded_size.is_some() {
            self.append(format!("_{}_start = len(_span)", id));
        }

        match &field.desc {
            ast::FieldDesc::Array { id, width: Some(8), .. } => {
                self.append(format!("_span.extend(self.{id})"));
            }
            ast::FieldDesc::Array { id, width: Some(width), .. } => {
                self.append(format!("for elt in self.{id}:"));
                self.append(format!(
                    "    _span.extend(int.to_bytes(elt, length={}, byteorder='{}'))",
                    width / 8,
                    self.byteorder
                ));
            }
            ast::FieldDesc::Array { id, type_id: Some(type_id), .. } => {
                self.append(format!("for elt in self.{id}:"));
                match &self.scope.typedef[type_id].desc {
                    ast::DeclDesc::Enum { width: 8, .. } => {
                        self.append("    _span.append(int(elt))".to_string());
                    }
                    ast::DeclDesc::Enum { width, .. } => {
                        self.append(format!(
                            "    _span.extend(int.to_bytes(elt, length={}, byteorder='{}'))",
                            width / 8,
                            self.byteorder
                        ));
                    }
                    _ => {
                        self.append("    _span.extend(elt.serialize())".to_string());
                    }
                }
            }
            _ => unreachable!(),
        }

        if let Some(ps) = padded_size {
            let ps_bytes = ps / 8;
            self.append(format!("_span.extend([0] * ({} - len(_span) + _{}_start))", ps_bytes, id));
        }
    }

    fn serialize_optional_field(&mut self, field: &ast::Field) {
        let id = field.id().unwrap();

        match &field.desc {
            ast::FieldDesc::Scalar { width, .. } => {
                self.append(format!("if self.{} is not None:", id));
                self.append(format!(
                    "    _span.extend(int.to_bytes(self.{}, length={}, byteorder='{}'))",
                    id,
                    width / 8,
                    self.byteorder
                ));
            }
            ast::FieldDesc::Typedef { type_id, .. } => {
                let type_decl = self.scope.typedef.get(type_id.as_str()).unwrap();
                match &type_decl.desc {
                    ast::DeclDesc::Enum { width, .. } => {
                        self.append(format!("if self.{} is not None:", id));
                        self.append(format!(
                            "    _span.extend(int.to_bytes(self.{}, length={}, byteorder='{}'))",
                            id,
                            width / 8,
                            self.byteorder
                        ));
                    }
                    _ => {
                        self.append(format!("if self.{} is not None:", id));
                        self.append(format!("    _span.extend(self.{}.serialize())", id));
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn serialize(&mut self, decl: &ast::Decl, field: &ast::Field) {
        if field.cond.is_some() {
            self.serialize_optional_field(field);
            return;
        }

        match &field.desc {
            ast::FieldDesc::Scalar { .. }
            | ast::FieldDesc::FixedScalar { .. }
            | ast::FieldDesc::FixedEnum { .. }
            | ast::FieldDesc::Reserved { .. }
            | ast::FieldDesc::Size { .. }
            | ast::FieldDesc::Count { .. }
            | ast::FieldDesc::ElementSize { .. }
            | ast::FieldDesc::Flag { .. } => self.serialize_bit_field(decl, field),
            ast::FieldDesc::Checksum { .. } => self.serialize_checksum_field(field),
            ast::FieldDesc::Typedef { type_id, .. } => {
                let type_decl = self.scope.typedef.get(type_id.as_str()).unwrap();
                match &type_decl.desc {
                    ast::DeclDesc::Enum { .. } => self.serialize_bit_field(decl, field),
                    _ => self.serialize_typedef_field(field),
                }
            }
            ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body => {
                self.serialize_payload_field(field)
            }
            ast::FieldDesc::Array { .. } => self.serialize_array_field(field),
            _ => {}
        }
    }
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
            Some("tests.custom_types"),
            &[
                "Packet_Array_Field_VariableElementSize_ConstantSize".to_string(),
                "Packet_Array_Field_VariableElementSize_VariableSize".to_string(),
                "Packet_Array_Field_VariableElementSize_VariableCount".to_string(),
                "Packet_Array_Field_VariableElementSize_UnknownSize".to_string(),
            ],
        );
        assert_snapshot_eq(&format!("tests/generated/python/le_backend.py"), &actual_code);
    }
}
