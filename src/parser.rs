use super::ast;
use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::files;
use pest::iterators::{Pair, Pairs};
use pest::{Parser, Token};
use std::iter::{Filter, Peekable};

// Generate the PDL parser.
// TODO: use #[grammar = "pdl.pest"]
// currently not possible because CARGO_MANIFEST_DIR is not set
// in soong environment.
#[derive(pest_derive::Parser)]
#[grammar_inline = r#"
WHITESPACE = _{ " " | "\n" }
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
size_modifier = @{
    ("+"|"-"|"*"|"/") ~ (digit|"+"|"-"|"*"|"/")+
}

endianness_declaration = { "little_endian_packets" | "big_endian_packets" }

enum_tag = { identifier ~ "=" ~ integer }
enum_tag_list = { enum_tag ~ ("," ~ enum_tag)* ~ ","? }
enum_declaration = {
    "enum" ~ identifier ~ ":" ~ integer ~ "{" ~
        enum_tag_list ~
    "}"
}

constraint = { identifier ~ "=" ~ (identifier|integer) }
constraint_list = { constraint ~ ("," ~ constraint)* }

checksum_field = { "_checksum_start_" ~ "(" ~ identifier ~ ")" }
padding_field = { "_padding_" ~ "[" ~ integer ~ "]" }
size_field = { "_size_" ~ "(" ~ (identifier|payload_identifier|body_identifier)  ~ ")" ~ ":" ~ integer }
count_field = { "_count_" ~ "(" ~ identifier ~ ")" ~ ":" ~ integer }
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
group_field = { identifier ~ ("{" ~ constraint_list ~ "}")? }

field = _{
    checksum_field |
    padding_field |
    size_field |
    count_field |
    body_field |
    payload_field |
    fixed_field |
    reserved_field |
    array_field |
    scalar_field |
    typedef_field |
    group_field
}
field_list = { field ~ ("," ~ field)* ~ ","? }

packet_declaration = {
   "packet" ~ identifier ~
        (":" ~ identifier)? ~
           ("(" ~ constraint_list ~ ")")? ~
    "{" ~
        field_list? ~
    "}"
}

struct_declaration = {
    "struct" ~ identifier ~
        (":" ~ identifier)? ~
           ("(" ~ constraint_list ~ ")")? ~
    "{" ~
        field_list? ~
    "}"
}

group_declaration = {
    "group" ~ identifier ~ "{" ~ field_list ~ "}"
}

checksum_declaration = {
    "checksum" ~ identifier ~ ":" ~ integer ~ string
}

custom_field_declaration = {
    "custom_field" ~ identifier ~ (":" ~ integer)? ~ string
}

test_case = { string }
test_case_list = _{ test_case ~ ("," ~ test_case)* ~ ","? }
test_declaration = {
    "test" ~ identifier ~ "{" ~
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

grammar = {
    SOI ~
    endianness_declaration? ~
    declaration* ~
    EOI
}
"#]
pub struct PDLParser;

type Node<'i> = Pair<'i, Rule>;
type NodeIterator<'i> = Peekable<Filter<Pairs<'i, Rule>, fn(&Node<'i>) -> bool>>;
type Context<'a> = (ast::FileId, &'a Vec<usize>);

trait Helpers<'i> {
    fn children(self) -> NodeIterator<'i>;
    fn as_loc(&self, context: &Context) -> ast::SourceRange;
    fn as_string(&self) -> String;
    fn as_usize(&self) -> Result<usize, String>;
}

impl<'i> Helpers<'i> for Node<'i> {
    fn children(self) -> NodeIterator<'i> {
        self.into_inner().filter((|n| n.as_rule() != Rule::COMMENT) as fn(&Self) -> bool).peekable()
    }

    fn as_loc(&self, context: &Context) -> ast::SourceRange {
        let span = self.as_span();
        ast::SourceRange {
            file: context.0,
            start: ast::SourceLocation::new(span.start_pos().pos(), context.1),
            end: ast::SourceLocation::new(span.end_pos().pos(), context.1),
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

fn expect<'i>(iter: &mut NodeIterator<'i>, rule: Rule) -> Result<Node<'i>, String> {
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

fn parse_string(iter: &mut NodeIterator<'_>) -> Result<String, String> {
    expect(iter, Rule::string).map(|n| n.as_string())
}

fn parse_atomic_expr(iter: &mut NodeIterator<'_>, context: &Context) -> Result<ast::Expr, String> {
    match iter.next() {
        Some(n) if n.as_rule() == Rule::identifier => {
            Ok(ast::Expr::Identifier { loc: n.as_loc(context), name: n.as_string() })
        }
        Some(n) if n.as_rule() == Rule::integer => {
            Ok(ast::Expr::Integer { loc: n.as_loc(context), value: n.as_usize()? })
        }
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

fn parse_size_modifier_opt(iter: &mut NodeIterator<'_>) -> Option<String> {
    maybe(iter, Rule::size_modifier).map(|n| n.as_string())
}

fn parse_endianness(node: Node<'_>, context: &Context) -> Result<ast::Endianness, String> {
    if node.as_rule() != Rule::endianness_declaration {
        err_unexpected_rule(Rule::endianness_declaration, node.as_rule())
    } else {
        Ok(ast::Endianness {
            loc: node.as_loc(context),
            value: match node.as_str() {
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
        let value = parse_atomic_expr(&mut children, context)?;
        Ok(ast::Constraint { id, loc, value })
    }
}

fn parse_constraint_list_opt(
    iter: &mut NodeIterator<'_>,
    context: &Context,
) -> Result<Vec<ast::Constraint>, String> {
    maybe(iter, Rule::constraint_list)
        .map_or(Ok(vec![]), |n| n.children().map(|n| parse_constraint(n, context)).collect())
}

fn parse_enum_tag(node: Node<'_>, context: &Context) -> Result<ast::Tag, String> {
    if node.as_rule() != Rule::enum_tag {
        err_unexpected_rule(Rule::enum_tag, node.as_rule())
    } else {
        let loc = node.as_loc(context);
        let mut children = node.children();
        let id = parse_identifier(&mut children)?;
        let value = parse_integer(&mut children)?;
        Ok(ast::Tag { id, loc, value })
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
    let rule = node.as_rule();
    let mut children = node.children();
    Ok(match rule {
        Rule::checksum_field => {
            let field_id = parse_identifier(&mut children)?;
            ast::Field::Checksum { loc, field_id }
        }
        Rule::padding_field => {
            let width = parse_integer(&mut children)?;
            ast::Field::Padding { loc, width }
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
            ast::Field::Size { loc, field_id, width }
        }
        Rule::count_field => {
            let field_id = parse_identifier(&mut children)?;
            let width = parse_integer(&mut children)?;
            ast::Field::Count { loc, field_id, width }
        }
        Rule::body_field => ast::Field::Body { loc },
        Rule::payload_field => {
            let size_modifier = parse_size_modifier_opt(&mut children);
            ast::Field::Payload { loc, size_modifier }
        }
        Rule::fixed_field => {
            let (tag_id, value) = parse_identifier_or_integer(&mut children)?;
            let (enum_id, width) = parse_identifier_or_integer(&mut children)?;
            ast::Field::Fixed { loc, enum_id, tag_id, width, value }
        }
        Rule::reserved_field => {
            let width = parse_integer(&mut children)?;
            ast::Field::Reserved { loc, width }
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
            ast::Field::Array { loc, id, type_id, width, size, size_modifier }
        }
        Rule::scalar_field => {
            let id = parse_identifier(&mut children)?;
            let width = parse_integer(&mut children)?;
            ast::Field::Scalar { loc, id, width }
        }
        Rule::typedef_field => {
            let id = parse_identifier(&mut children)?;
            let type_id = parse_identifier(&mut children)?;
            ast::Field::Typedef { loc, id, type_id }
        }
        Rule::group_field => {
            let group_id = parse_identifier(&mut children)?;
            let constraints = parse_constraint_list_opt(&mut children, context)?;
            ast::Field::Group { loc, group_id, constraints }
        }
        _ => return Err(format!("expected rule *_field, got {:?}", rule)),
    })
}

fn parse_field_list<'i>(
    iter: &mut NodeIterator<'i>,
    context: &Context,
) -> Result<Vec<ast::Field>, String> {
    expect(iter, Rule::field_list)
        .and_then(|n| n.children().map(|n| parse_field(n, context)).collect())
}

fn parse_field_list_opt<'i>(
    iter: &mut NodeIterator<'i>,
    context: &Context,
) -> Result<Vec<ast::Field>, String> {
    maybe(iter, Rule::field_list)
        .map_or(Ok(vec![]), |n| n.children().map(|n| parse_field(n, context)).collect())
}

fn parse_grammar(root: Node<'_>, context: &Context) -> Result<ast::Grammar, String> {
    let mut toplevel_comments = vec![];
    let mut grammar = ast::Grammar::new(context.0);

    let mut comment_start = vec![];
    for token in root.clone().tokens() {
        match token {
            Token::Start { rule: Rule::COMMENT, pos } => comment_start.push(pos),
            Token::End { rule: Rule::COMMENT, pos } => {
                let start_pos = comment_start.pop().unwrap();
                grammar.comments.push(ast::Comment {
                    loc: ast::SourceRange {
                        file: context.0,
                        start: ast::SourceLocation::new(start_pos.pos(), context.1),
                        end: ast::SourceLocation::new(pos.pos(), context.1),
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
            Rule::endianness_declaration => {
                grammar.endianness = Some(parse_endianness(node, context)?)
            }
            Rule::checksum_declaration => {
                let mut children = node.children();
                let id = parse_identifier(&mut children)?;
                let width = parse_integer(&mut children)?;
                let function = parse_string(&mut children)?;
                grammar.declarations.push(ast::Decl::Checksum { id, loc, function, width })
            }
            Rule::custom_field_declaration => {
                let mut children = node.children();
                let id = parse_identifier(&mut children)?;
                let width = parse_integer_opt(&mut children)?;
                let function = parse_string(&mut children)?;
                grammar.declarations.push(ast::Decl::CustomField { id, loc, function, width })
            }
            Rule::enum_declaration => {
                let mut children = node.children();
                let id = parse_identifier(&mut children)?;
                let width = parse_integer(&mut children)?;
                let tags = parse_enum_tag_list(&mut children, context)?;
                grammar.declarations.push(ast::Decl::Enum { id, loc, width, tags })
            }
            Rule::packet_declaration => {
                let mut children = node.children();
                let id = parse_identifier(&mut children)?;
                let parent_id = parse_identifier_opt(&mut children)?;
                let constraints = parse_constraint_list_opt(&mut children, context)?;
                let fields = parse_field_list_opt(&mut children, context)?;
                grammar.declarations.push(ast::Decl::Packet {
                    id,
                    loc,
                    parent_id,
                    constraints,
                    fields,
                })
            }
            Rule::struct_declaration => {
                let mut children = node.children();
                let id = parse_identifier(&mut children)?;
                let parent_id = parse_identifier_opt(&mut children)?;
                let constraints = parse_constraint_list_opt(&mut children, context)?;
                let fields = parse_field_list_opt(&mut children, context)?;
                grammar.declarations.push(ast::Decl::Struct {
                    id,
                    loc,
                    parent_id,
                    constraints,
                    fields,
                })
            }
            Rule::group_declaration => {
                let mut children = node.children();
                let id = parse_identifier(&mut children)?;
                let fields = parse_field_list(&mut children, context)?;
                grammar.declarations.push(ast::Decl::Group { id, loc, fields })
            }
            Rule::test_declaration => {}
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    grammar.comments.append(&mut toplevel_comments);
    Ok(grammar)
}

/// Parse a PDL grammar text.
/// The grammar is added to the compilation database under the
/// provided name.
pub fn parse_inline(
    sources: &mut ast::SourceDatabase,
    name: String,
    source: String,
) -> Result<ast::Grammar, Diagnostic<ast::FileId>> {
    let root = PDLParser::parse(Rule::grammar, &source)
        .map_err(|e| {
            Diagnostic::error()
                .with_message(format!("failed to parse input file '{}': {}", &name, e))
        })?
        .next()
        .unwrap();
    let line_starts: Vec<_> = files::line_starts(&source).collect();
    let file = sources.add(name, source.clone());
    parse_grammar(root, &(file, &line_starts)).map_err(|e| Diagnostic::error().with_message(e))
}

/// Parse a new source file.
/// The source file is fully read and added to the compilation database.
/// Returns the constructed AST, or a descriptive error message in case
/// of syntax error.
pub fn parse_file(
    sources: &mut ast::SourceDatabase,
    name: String,
) -> Result<ast::Grammar, Diagnostic<ast::FileId>> {
    let source = std::fs::read_to_string(&name).map_err(|e| {
        Diagnostic::error().with_message(format!("failed to read input file '{}': {}", &name, e))
    })?;
    parse_inline(sources, name, source)
}
