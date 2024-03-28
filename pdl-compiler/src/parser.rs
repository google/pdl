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

use crate::ast;
use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::files;
use pest::iterators::{Pair, Pairs};
use pest::{Parser, Token};
use std::iter::{Filter, Peekable};

// Generate the PDL parser.
//
// TODO:
// - use #[grammar = "pdl.pest"]
//   currently not possible because CARGO_MANIFEST_DIR is not set
//   in soong environment.
// - use silent atomic rules for keywords like
//   ENUM = @{ "enum" ~ WHITESPACE }
//   currently not implemented in pest:
//   https://github.com/pest-parser/pest/issues/520
#[derive(pest_derive::Parser)]
#[grammar_inline = r#"
WHITESPACE = _{ " " | "\n" | "\r" | "\t" }
COMMENT = { block_comment | line_comment }

block_comment = { "/*" ~ (!"*/" ~ ANY)* ~ "*/" }
line_comment = { "//" ~ (!"\n" ~ ANY)* }

alpha = { 'a'..'z' | 'A'..'Z' }
digit = { '0'..'9' }
hexdigit = { digit | 'a'..'f' | 'A'..'F' }
alphanum = { alpha | digit | "_" }

identifier = @{ alpha ~ alphanum* }
payload_identifier = @{ "_payload_" }
body_identifier = @{ "_body_" }
intvalue = @{ digit+ }
hexvalue = @{ ("0x"|"0X") ~ hexdigit+ }
integer = @{ hexvalue | intvalue }
string = @{ "\"" ~ (!"\"" ~ ANY)* ~ "\"" }
size_modifier = @{ "+" ~ intvalue }

ENUM = @{ "enum" ~ WHITESPACE }
PACKET = @{ "packet" ~ WHITESPACE }
STRUCT = @{ "struct" ~ WHITESPACE }
GROUP = @{ "group" ~ WHITESPACE }
CHECKSUM = @{ "checksum" ~ WHITESPACE }
CUSTOM_FIELD = @{ "custom_field" ~ WHITESPACE }
TEST = @{ "test" ~ WHITESPACE }

endianness_declaration = ${ ("little_endian_packets" | "big_endian_packets") ~ WHITESPACE }

enum_value = { identifier ~ "=" ~ integer }
enum_value_list = { enum_value ~ ("," ~ enum_value)* ~ ","? }
enum_range = {
    identifier ~ "=" ~ integer ~ ".." ~ integer ~ ("{" ~
        enum_value_list ~
    "}")?
}
enum_other = { identifier ~ "=" ~ ".." }
enum_tag = { enum_range | enum_value | enum_other }
enum_tag_list = { enum_tag ~ ("," ~ enum_tag)* ~ ","? }
enum_declaration = {
    ENUM ~ identifier ~ ":" ~ integer ~ "{" ~
        enum_tag_list ~
    "}"
}

constraint = { identifier ~ "=" ~ (identifier|integer) }
constraint_list = { constraint ~ ("," ~ constraint)* }

checksum_field = { "_checksum_start_" ~ "(" ~ identifier ~ ")" }
padding_field = { "_padding_" ~ "[" ~ integer ~ "]" }
size_field = { "_size_" ~ "(" ~ (identifier|payload_identifier|body_identifier)  ~ ")" ~ ":" ~ integer }
count_field = { "_count_" ~ "(" ~ identifier ~ ")" ~ ":" ~ integer }
elementsize_field = { "_elementsize_" ~ "(" ~ identifier ~ ")" ~ ":" ~ integer }
body_field = @{ "_body_" }
payload_field = { "_payload_" ~ (":" ~ "[" ~ size_modifier ~ "]")? }
fixed_field = { "_fixed_" ~ "=" ~ (
    (integer ~ ":" ~ integer) |
    (identifier ~ ":" ~ identifier)
)}
reserved_field = { "_reserved_" ~ ":" ~ integer }
array_field = { identifier ~ ":" ~ (integer|identifier) ~
    "[" ~ (size_modifier|integer)? ~ "]"
}
scalar_field = { identifier ~ ":" ~ integer }
typedef_field = { identifier ~ ":" ~ identifier }
group_field = { identifier ~ ("{" ~ constraint_list? ~ "}")? }

field_desc = _{
    checksum_field |
    padding_field |
    size_field |
    count_field |
    elementsize_field |
    body_field |
    payload_field |
    fixed_field |
    reserved_field |
    array_field |
    scalar_field |
    typedef_field |
    group_field
}
field = { field_desc ~ ("if" ~ constraint)? }
field_list = { field ~ ("," ~ field)* ~ ","? }

packet_declaration = {
   PACKET ~ identifier ~
        (":" ~ identifier)? ~
           ("(" ~ constraint_list ~ ")")? ~
    "{" ~
        field_list? ~
    "}"
}

struct_declaration = {
    STRUCT ~ identifier ~
        (":" ~ identifier)? ~
           ("(" ~ constraint_list ~ ")")? ~
    "{" ~
        field_list? ~
    "}"
}

group_declaration = {
    GROUP ~ identifier ~ "{" ~ field_list ~ "}"
}

checksum_declaration = {
    CHECKSUM ~ identifier ~ ":" ~ integer ~ string
}

custom_field_declaration = {
    CUSTOM_FIELD ~ identifier ~ (":" ~ integer)? ~ string
}

test_case = { string }
test_case_list = _{ test_case ~ ("," ~ test_case)* ~ ","? }
test_declaration = {
    TEST ~ identifier ~ "{" ~
        test_case_list ~
    "}"
}

declaration = _{
    enum_declaration |
    packet_declaration |
    struct_declaration |
    group_declaration |
    checksum_declaration |
    custom_field_declaration |
    test_declaration
}

file = {
    SOI ~
    endianness_declaration ~
    declaration* ~
    EOI
}
"#]
pub struct PDLParser;

type Node<'i> = Pair<'i, Rule>;
type NodeIterator<'i> = Peekable<Filter<Pairs<'i, Rule>, fn(&Node<'i>) -> bool>>;
struct Context<'a> {
    file: ast::FileId,
    line_starts: &'a Vec<usize>,
    key: std::cell::Cell<usize>,
}

trait Helpers<'i> {
    fn children(self) -> NodeIterator<'i>;
    fn as_loc(&self, context: &Context) -> ast::SourceRange;
    fn as_string(&self) -> String;
    fn as_usize(&self) -> Result<usize, String>;
}

impl<'a> Context<'a> {
    fn key(&self) -> usize {
        self.key.replace(self.key.get() + 1)
    }
}

impl<'i> Helpers<'i> for Node<'i> {
    fn children(self) -> NodeIterator<'i> {
        self.into_inner().filter((|n| n.as_rule() != Rule::COMMENT) as fn(&Self) -> bool).peekable()
    }

    fn as_loc(&self, context: &Context) -> ast::SourceRange {
        let span = self.as_span();
        ast::SourceRange {
            file: context.file,
            start: ast::SourceLocation::new(span.start_pos().pos(), context.line_starts),
            end: ast::SourceLocation::new(span.end_pos().pos(), context.line_starts),
        }
    }

    fn as_string(&self) -> String {
        self.as_str().to_owned()
    }

    fn as_usize(&self) -> Result<usize, String> {
        let text = self.as_str();
        if let Some(num) = text.strip_prefix("0x") {
            usize::from_str_radix(num, 16)
                .map_err(|_| format!("cannot convert '{}' to usize", self.as_str()))
        } else {
            #[allow(clippy::from_str_radix_10)]
            usize::from_str_radix(text, 10)
                .map_err(|_| format!("cannot convert '{}' to usize", self.as_str()))
        }
    }
}

fn err_unexpected_rule<T>(expected: Rule, found: Rule) -> Result<T, String> {
    Err(format!("expected rule {:?}, got {:?}", expected, found))
}

fn err_missing_rule<T>(expected: Rule) -> Result<T, String> {
    Err(format!("expected rule {:?}, got nothing", expected))
}

fn expect<'i>(iter: &mut impl Iterator<Item = Node<'i>>, rule: Rule) -> Result<Node<'i>, String> {
    match iter.next() {
        Some(node) if node.as_rule() == rule => Ok(node),
        Some(node) => err_unexpected_rule(rule, node.as_rule()),
        None => err_missing_rule(rule),
    }
}

fn maybe<'i>(iter: &mut NodeIterator<'i>, rule: Rule) -> Option<Node<'i>> {
    iter.next_if(|n| n.as_rule() == rule)
}

fn parse_identifier(iter: &mut NodeIterator<'_>) -> Result<String, String> {
    expect(iter, Rule::identifier).map(|n| n.as_string())
}

fn parse_integer(iter: &mut NodeIterator<'_>) -> Result<usize, String> {
    expect(iter, Rule::integer).and_then(|n| n.as_usize())
}

fn parse_identifier_opt(iter: &mut NodeIterator<'_>) -> Result<Option<String>, String> {
    Ok(maybe(iter, Rule::identifier).map(|n| n.as_string()))
}

fn parse_integer_opt(iter: &mut NodeIterator<'_>) -> Result<Option<usize>, String> {
    maybe(iter, Rule::integer).map(|n| n.as_usize()).transpose()
}

fn parse_identifier_or_integer(
    iter: &mut NodeIterator<'_>,
) -> Result<(Option<String>, Option<usize>), String> {
    match iter.next() {
        Some(n) if n.as_rule() == Rule::identifier => Ok((Some(n.as_string()), None)),
        Some(n) if n.as_rule() == Rule::integer => Ok((None, Some(n.as_usize()?))),
        Some(n) => Err(format!(
            "expected rule {:?} or {:?}, got {:?}",
            Rule::identifier,
            Rule::integer,
            n.as_rule()
        )),
        None => {
            Err(format!("expected rule {:?} or {:?}, got nothing", Rule::identifier, Rule::integer))
        }
    }
}

fn parse_string<'i>(iter: &mut impl Iterator<Item = Node<'i>>) -> Result<String, String> {
    expect(iter, Rule::string)
        .map(|n| n.as_str())
        .and_then(|s| s.strip_prefix('"').ok_or_else(|| "expected \" prefix".to_owned()))
        .and_then(|s| s.strip_suffix('"').ok_or_else(|| "expected \" suffix".to_owned()))
        .map(|s| s.to_owned())
}

fn parse_size_modifier_opt(iter: &mut NodeIterator<'_>) -> Option<String> {
    maybe(iter, Rule::size_modifier).map(|n| n.as_string())
}

fn parse_endianness(node: Node<'_>, context: &Context) -> Result<ast::Endianness, String> {
    if node.as_rule() != Rule::endianness_declaration {
        err_unexpected_rule(Rule::endianness_declaration, node.as_rule())
    } else {
        Ok(ast::Endianness {
            loc: node.as_loc(context),
            value: match node.as_str().trim() {
                "little_endian_packets" => ast::EndiannessValue::LittleEndian,
                "big_endian_packets" => ast::EndiannessValue::BigEndian,
                _ => unreachable!(),
            },
        })
    }
}

fn parse_constraint(node: Node<'_>, context: &Context) -> Result<ast::Constraint, String> {
    if node.as_rule() != Rule::constraint {
        err_unexpected_rule(Rule::constraint, node.as_rule())
    } else {
        let loc = node.as_loc(context);
        let mut children = node.children();
        let id = parse_identifier(&mut children)?;
        let (tag_id, value) = parse_identifier_or_integer(&mut children)?;
        Ok(ast::Constraint { id, loc, value, tag_id })
    }
}

fn parse_constraint_list_opt(
    iter: &mut NodeIterator<'_>,
    context: &Context,
) -> Result<Vec<ast::Constraint>, String> {
    maybe(iter, Rule::constraint_list)
        .map_or(Ok(vec![]), |n| n.children().map(|n| parse_constraint(n, context)).collect())
}

fn parse_enum_value(node: Node<'_>, context: &Context) -> Result<ast::TagValue, String> {
    if node.as_rule() != Rule::enum_value {
        err_unexpected_rule(Rule::enum_value, node.as_rule())
    } else {
        let loc = node.as_loc(context);
        let mut children = node.children();
        let id = parse_identifier(&mut children)?;
        let value = parse_integer(&mut children)?;
        Ok(ast::TagValue { id, loc, value })
    }
}

fn parse_enum_value_list_opt(
    iter: &mut NodeIterator<'_>,
    context: &Context,
) -> Result<Vec<ast::TagValue>, String> {
    maybe(iter, Rule::enum_value_list)
        .map_or(Ok(vec![]), |n| n.children().map(|n| parse_enum_value(n, context)).collect())
}

fn parse_enum_range(node: Node<'_>, context: &Context) -> Result<ast::TagRange, String> {
    if node.as_rule() != Rule::enum_range {
        err_unexpected_rule(Rule::enum_range, node.as_rule())
    } else {
        let loc = node.as_loc(context);
        let mut children = node.children();
        let id = parse_identifier(&mut children)?;
        let start = parse_integer(&mut children)?;
        let end = parse_integer(&mut children)?;
        let tags = parse_enum_value_list_opt(&mut children, context)?;
        Ok(ast::TagRange { id, loc, range: start..=end, tags })
    }
}

fn parse_enum_other(node: Node<'_>, context: &Context) -> Result<ast::TagOther, String> {
    if node.as_rule() != Rule::enum_other {
        err_unexpected_rule(Rule::enum_other, node.as_rule())
    } else {
        let loc = node.as_loc(context);
        let mut children = node.children();
        let id = parse_identifier(&mut children)?;
        Ok(ast::TagOther { id, loc })
    }
}

fn parse_enum_tag(node: Node<'_>, context: &Context) -> Result<ast::Tag, String> {
    if node.as_rule() != Rule::enum_tag {
        err_unexpected_rule(Rule::enum_tag, node.as_rule())
    } else {
        match node.children().next() {
            Some(node) if node.as_rule() == Rule::enum_value => {
                Ok(ast::Tag::Value(parse_enum_value(node, context)?))
            }
            Some(node) if node.as_rule() == Rule::enum_range => {
                Ok(ast::Tag::Range(parse_enum_range(node, context)?))
            }
            Some(node) if node.as_rule() == Rule::enum_other => {
                Ok(ast::Tag::Other(parse_enum_other(node, context)?))
            }
            Some(node) => Err(format!(
                "expected rule {:?} or {:?}, got {:?}",
                Rule::enum_value,
                Rule::enum_range,
                node.as_rule()
            )),
            None => Err(format!(
                "expected rule {:?} or {:?}, got nothing",
                Rule::enum_value,
                Rule::enum_range
            )),
        }
    }
}

fn parse_enum_tag_list(
    iter: &mut NodeIterator<'_>,
    context: &Context,
) -> Result<Vec<ast::Tag>, String> {
    expect(iter, Rule::enum_tag_list)
        .and_then(|n| n.children().map(|n| parse_enum_tag(n, context)).collect())
}

fn parse_field(node: Node<'_>, context: &Context) -> Result<ast::Field, String> {
    let loc = node.as_loc(context);
    let mut children = node.children();
    let desc = children.next().unwrap();
    let cond = children.next();
    let rule = desc.as_rule();
    let mut children = desc.children();
    Ok(ast::Field {
        loc,
        key: context.key(),
        cond: cond.map(|constraint| parse_constraint(constraint, context)).transpose()?,
        desc: match rule {
            Rule::checksum_field => {
                let field_id = parse_identifier(&mut children)?;
                ast::FieldDesc::Checksum { field_id }
            }
            Rule::padding_field => {
                let size = parse_integer(&mut children)?;
                ast::FieldDesc::Padding { size }
            }
            Rule::size_field => {
                let field_id = match children.next() {
                    Some(n) if n.as_rule() == Rule::identifier => n.as_string(),
                    Some(n) if n.as_rule() == Rule::payload_identifier => n.as_string(),
                    Some(n) if n.as_rule() == Rule::body_identifier => n.as_string(),
                    Some(n) => err_unexpected_rule(Rule::identifier, n.as_rule())?,
                    None => err_missing_rule(Rule::identifier)?,
                };
                let width = parse_integer(&mut children)?;
                ast::FieldDesc::Size { field_id, width }
            }
            Rule::count_field => {
                let field_id = parse_identifier(&mut children)?;
                let width = parse_integer(&mut children)?;
                ast::FieldDesc::Count { field_id, width }
            }
            Rule::elementsize_field => {
                let field_id = parse_identifier(&mut children)?;
                let width = parse_integer(&mut children)?;
                ast::FieldDesc::ElementSize { field_id, width }
            }
            Rule::body_field => ast::FieldDesc::Body,
            Rule::payload_field => {
                let size_modifier = parse_size_modifier_opt(&mut children);
                ast::FieldDesc::Payload { size_modifier }
            }
            Rule::fixed_field => match children.next() {
                Some(n) if n.as_rule() == Rule::integer => {
                    let value = n.as_usize()?;
                    let width = parse_integer(&mut children)?;
                    ast::FieldDesc::FixedScalar { width, value }
                }
                Some(n) if n.as_rule() == Rule::identifier => {
                    let tag_id = n.as_string();
                    let enum_id = parse_identifier(&mut children)?;
                    ast::FieldDesc::FixedEnum { enum_id, tag_id }
                }
                _ => unreachable!(),
            },
            Rule::reserved_field => {
                let width = parse_integer(&mut children)?;
                ast::FieldDesc::Reserved { width }
            }
            Rule::array_field => {
                let id = parse_identifier(&mut children)?;
                let (type_id, width) = parse_identifier_or_integer(&mut children)?;
                let (size, size_modifier) = match children.next() {
                    Some(n) if n.as_rule() == Rule::integer => (Some(n.as_usize()?), None),
                    Some(n) if n.as_rule() == Rule::size_modifier => (None, Some(n.as_string())),
                    Some(n) => {
                        return Err(format!(
                            "expected rule {:?} or {:?}, got {:?}",
                            Rule::integer,
                            Rule::size_modifier,
                            n.as_rule()
                        ))
                    }
                    None => (None, None),
                };
                ast::FieldDesc::Array { id, type_id, width, size, size_modifier }
            }
            Rule::scalar_field => {
                let id = parse_identifier(&mut children)?;
                let width = parse_integer(&mut children)?;
                ast::FieldDesc::Scalar { id, width }
            }
            Rule::typedef_field => {
                let id = parse_identifier(&mut children)?;
                let type_id = parse_identifier(&mut children)?;
                ast::FieldDesc::Typedef { id, type_id }
            }
            Rule::group_field => {
                let group_id = parse_identifier(&mut children)?;
                let constraints = parse_constraint_list_opt(&mut children, context)?;
                ast::FieldDesc::Group { group_id, constraints }
            }
            _ => return Err(format!("expected rule *_field, got {:?}", rule)),
        },
    })
}

fn parse_field_list(iter: &mut NodeIterator, context: &Context) -> Result<Vec<ast::Field>, String> {
    expect(iter, Rule::field_list)
        .and_then(|n| n.children().map(|n| parse_field(n, context)).collect())
}

fn parse_field_list_opt(
    iter: &mut NodeIterator,
    context: &Context,
) -> Result<Vec<ast::Field>, String> {
    maybe(iter, Rule::field_list)
        .map_or(Ok(vec![]), |n| n.children().map(|n| parse_field(n, context)).collect())
}

fn parse_toplevel(root: Node<'_>, context: &Context) -> Result<ast::File, String> {
    let mut toplevel_comments = vec![];
    let mut file = ast::File::new(context.file);

    let mut comment_start = vec![];
    for token in root.clone().tokens() {
        match token {
            Token::Start { rule: Rule::COMMENT, pos } => comment_start.push(pos),
            Token::End { rule: Rule::COMMENT, pos } => {
                let start_pos = comment_start.pop().unwrap();
                file.comments.push(ast::Comment {
                    loc: ast::SourceRange {
                        file: context.file,
                        start: ast::SourceLocation::new(start_pos.pos(), context.line_starts),
                        end: ast::SourceLocation::new(pos.pos(), context.line_starts),
                    },
                    text: start_pos.span(&pos).as_str().to_owned(),
                })
            }
            _ => (),
        }
    }

    for node in root.children() {
        let loc = node.as_loc(context);
        let rule = node.as_rule();
        match rule {
            Rule::endianness_declaration => file.endianness = parse_endianness(node, context)?,
            Rule::checksum_declaration => {
                let mut children = node.children();
                expect(&mut children, Rule::CHECKSUM)?;
                let id = parse_identifier(&mut children)?;
                let width = parse_integer(&mut children)?;
                let function = parse_string(&mut children)?;
                file.declarations.push(ast::Decl {
                    loc,
                    key: context.key(),
                    desc: ast::DeclDesc::Checksum { id, function, width },
                })
            }
            Rule::custom_field_declaration => {
                let mut children = node.children();
                expect(&mut children, Rule::CUSTOM_FIELD)?;
                let id = parse_identifier(&mut children)?;
                let width = parse_integer_opt(&mut children)?;
                let function = parse_string(&mut children)?;
                file.declarations.push(ast::Decl {
                    loc,
                    key: context.key(),
                    desc: ast::DeclDesc::CustomField { id, function, width },
                })
            }
            Rule::enum_declaration => {
                let mut children = node.children();
                expect(&mut children, Rule::ENUM)?;
                let id = parse_identifier(&mut children)?;
                let width = parse_integer(&mut children)?;
                let tags = parse_enum_tag_list(&mut children, context)?;
                file.declarations.push(ast::Decl {
                    loc,
                    key: context.key(),
                    desc: ast::DeclDesc::Enum { id, width, tags },
                })
            }
            Rule::packet_declaration => {
                let mut children = node.children();
                expect(&mut children, Rule::PACKET)?;
                let id = parse_identifier(&mut children)?;
                let parent_id = parse_identifier_opt(&mut children)?;
                let constraints = parse_constraint_list_opt(&mut children, context)?;
                let fields = parse_field_list_opt(&mut children, context)?;
                file.declarations.push(ast::Decl {
                    loc,
                    key: context.key(),
                    desc: ast::DeclDesc::Packet { id, parent_id, constraints, fields },
                })
            }
            Rule::struct_declaration => {
                let mut children = node.children();
                expect(&mut children, Rule::STRUCT)?;
                let id = parse_identifier(&mut children)?;
                let parent_id = parse_identifier_opt(&mut children)?;
                let constraints = parse_constraint_list_opt(&mut children, context)?;
                let fields = parse_field_list_opt(&mut children, context)?;
                file.declarations.push(ast::Decl {
                    loc,
                    key: context.key(),
                    desc: ast::DeclDesc::Struct { id, parent_id, constraints, fields },
                })
            }
            Rule::group_declaration => {
                let mut children = node.children();
                expect(&mut children, Rule::GROUP)?;
                let id = parse_identifier(&mut children)?;
                let fields = parse_field_list(&mut children, context)?;
                file.declarations.push(ast::Decl {
                    loc,
                    key: context.key(),
                    desc: ast::DeclDesc::Group { id, fields },
                })
            }
            Rule::test_declaration => {}
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    file.comments.append(&mut toplevel_comments);
    file.max_key = context.key();
    Ok(file)
}

/// Parse PDL source code from a string.
///
/// The file is added to the compilation database under the provided
/// name.
pub fn parse_inline(
    sources: &mut ast::SourceDatabase,
    name: &str,
    source: String,
) -> Result<ast::File, Diagnostic<ast::FileId>> {
    let root = PDLParser::parse(Rule::file, &source)
        .map_err(|e| {
            Diagnostic::error()
                .with_message(format!("failed to parse input file '{}': {}", name, e))
        })?
        .next()
        .unwrap();
    let line_starts: Vec<_> = files::line_starts(&source).collect();
    let file = sources.add(name.to_owned(), source.clone());
    parse_toplevel(root, &Context { file, line_starts: &line_starts, key: std::cell::Cell::new(0) })
        .map_err(|e| Diagnostic::error().with_message(e))
}

/// Parse a new source file.
///
/// The source file is fully read and added to the compilation
/// database. Returns the constructed AST, or a descriptive error
/// message in case of syntax error.
pub fn parse_file(
    sources: &mut ast::SourceDatabase,
    name: &str,
) -> Result<ast::File, Diagnostic<ast::FileId>> {
    let source = std::fs::read_to_string(name).map_err(|e| {
        Diagnostic::error().with_message(format!("failed to read input file '{}': {}", name, e))
    })?;
    parse_inline(sources, name, source)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn endianness_is_set() {
        // The file starts out with a placeholder little-endian value.
        // This tests that we update it while parsing.
        let mut db = ast::SourceDatabase::new();
        let file = parse_inline(&mut db, "stdin", String::from("  big_endian_packets  ")).unwrap();
        assert_eq!(file.endianness.value, ast::EndiannessValue::BigEndian);
        assert_ne!(file.endianness.loc, ast::SourceRange::default());
    }

    #[test]
    fn test_parse_string_bare() {
        let mut pairs = PDLParser::parse(Rule::string, r#""test""#).unwrap();

        assert_eq!(parse_string(&mut pairs).as_deref(), Ok("test"));
        assert_eq!(pairs.next(), None, "pairs is empty");
    }

    #[test]
    fn test_parse_string_space() {
        let mut pairs = PDLParser::parse(Rule::string, r#""test with space""#).unwrap();

        assert_eq!(parse_string(&mut pairs).as_deref(), Ok("test with space"));
        assert_eq!(pairs.next(), None, "pairs is empty");
    }

    #[test]
    #[should_panic] /* This is not supported */
    fn test_parse_string_escape() {
        let mut pairs = PDLParser::parse(Rule::string, r#""\"test\"""#).unwrap();

        assert_eq!(parse_string(&mut pairs).as_deref(), Ok(r#""test""#));
        assert_eq!(pairs.next(), None, "pairs is empty");
    }

    #[test]
    fn test_no_whitespace_between_keywords() {
        // Validate that the parser rejects inputs where whitespaces
        // are not applied between alphabetical keywords and identifiers.
        let mut db = ast::SourceDatabase::new();
        assert!(parse_inline(
            &mut db,
            "test",
            r#"
            little_endian_packetsstructx{foo:8}
            "#
            .to_owned()
        )
        .is_err());

        let result = parse_inline(
            &mut db,
            "test",
            r#"
            little_endian_packets
            struct x { foo:8 }
            "#
            .to_owned(),
        );
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
