use codespan_reporting::diagnostic;
use codespan_reporting::files;
use serde::Serialize;
use std::fmt;
use std::ops;

/// File identfiier.
/// References a source file in the source database.
pub type FileId = usize;

/// Source database.
/// Stores the source file contents for reference.
pub type SourceDatabase = files::SimpleFiles<String, String>;

#[derive(Debug, Copy, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SourceLocation {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct SourceRange {
    pub file: FileId,
    pub start: SourceLocation,
    pub end: SourceLocation,
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename = "comment")]
pub struct Comment {
    pub loc: SourceRange,
    pub text: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EndiannessValue {
    LittleEndian,
    BigEndian,
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename = "endianness_declaration")]
pub struct Endianness {
    pub loc: SourceRange,
    pub value: EndiannessValue,
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind")]
pub enum Expr {
    #[serde(rename = "identifier")]
    Identifier { loc: SourceRange, name: String },
    #[serde(rename = "integer")]
    Integer { loc: SourceRange, value: usize },
    #[serde(rename = "unary_expr")]
    Unary { loc: SourceRange, op: String, operand: Box<Expr> },
    #[serde(rename = "binary_expr")]
    Binary { loc: SourceRange, op: String, operands: Box<(Expr, Expr)> },
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename = "tag")]
pub struct Tag {
    pub id: String,
    pub loc: SourceRange,
    pub value: usize,
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename = "constraint")]
pub struct Constraint {
    pub id: String,
    pub loc: SourceRange,
    pub value: Expr,
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind")]
pub enum Field {
    #[serde(rename = "checksum_field")]
    Checksum { loc: SourceRange, field_id: String },
    #[serde(rename = "padding_field")]
    Padding { loc: SourceRange, width: usize },
    #[serde(rename = "size_field")]
    Size { loc: SourceRange, field_id: String, width: usize },
    #[serde(rename = "count_field")]
    Count { loc: SourceRange, field_id: String, width: usize },
    #[serde(rename = "body_field")]
    Body { loc: SourceRange },
    #[serde(rename = "payload_field")]
    Payload { loc: SourceRange, size_modifier: Option<String> },
    #[serde(rename = "fixed_field")]
    Fixed {
        loc: SourceRange,
        width: Option<usize>,
        value: Option<usize>,
        enum_id: Option<String>,
        tag_id: Option<String>,
    },
    #[serde(rename = "reserved_field")]
    Reserved { loc: SourceRange, width: usize },
    #[serde(rename = "array_field")]
    Array {
        loc: SourceRange,
        id: String,
        width: Option<usize>,
        type_id: Option<String>,
        size_modifier: Option<String>,
        size: Option<usize>,
    },
    #[serde(rename = "scalar_field")]
    Scalar { loc: SourceRange, id: String, width: usize },
    #[serde(rename = "typedef_field")]
    Typedef { loc: SourceRange, id: String, type_id: String },
    #[serde(rename = "group_field")]
    Group { loc: SourceRange, group_id: String, constraints: Vec<Constraint> },
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename = "test_case")]
pub struct TestCase {
    pub loc: SourceRange,
    pub input: String,
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind")]
pub enum Decl {
    #[serde(rename = "checksum_declaration")]
    Checksum { id: String, loc: SourceRange, function: String, width: usize },
    #[serde(rename = "custom_field_declaration")]
    CustomField { id: String, loc: SourceRange, width: Option<usize>, function: String },
    #[serde(rename = "enum_declaration")]
    Enum { id: String, loc: SourceRange, tags: Vec<Tag>, width: usize },
    #[serde(rename = "packet_declaration")]
    Packet {
        id: String,
        loc: SourceRange,
        constraints: Vec<Constraint>,
        fields: Vec<Field>,
        parent_id: Option<String>,
    },
    #[serde(rename = "struct_declaration")]
    Struct {
        id: String,
        loc: SourceRange,
        constraints: Vec<Constraint>,
        fields: Vec<Field>,
        parent_id: Option<String>,
    },
    #[serde(rename = "group_declaration")]
    Group { id: String, loc: SourceRange, fields: Vec<Field> },
    #[serde(rename = "test_declaration")]
    Test { loc: SourceRange, type_id: String, test_cases: Vec<TestCase> },
}

#[derive(Debug, Serialize)]
pub struct Grammar {
    pub version: String,
    pub file: FileId,
    pub comments: Vec<Comment>,
    pub endianness: Option<Endianness>,
    pub declarations: Vec<Decl>,
}

/// Implemented for all AST elements.
pub trait Located<'d> {
    fn loc(&'d self) -> &'d SourceRange;
}

/// Implemented for named AST elements.
pub trait Named<'d> {
    fn id(&'d self) -> Option<&'d String>;
}

impl SourceLocation {
    pub fn new(offset: usize, line_starts: &[usize]) -> SourceLocation {
        for (line, start) in line_starts.iter().enumerate() {
            if *start <= offset {
                return SourceLocation { offset, line, column: offset - start };
            }
        }
        unreachable!()
    }
}

impl SourceRange {
    pub fn primary(&self) -> diagnostic::Label<FileId> {
        diagnostic::Label::primary(self.file, self.start.offset..self.end.offset)
    }
    pub fn secondary(&self) -> diagnostic::Label<FileId> {
        diagnostic::Label::secondary(self.file, self.start.offset..self.end.offset)
    }
}

impl fmt::Display for SourceRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start.line == self.end.line {
            write!(f, "{}:{}-{}", self.start.line, self.start.column, self.end.column)
        } else {
            write!(
                f,
                "{}:{}-{}:{}",
                self.start.line, self.start.column, self.end.line, self.end.column
            )
        }
    }
}

impl ops::Add<SourceRange> for SourceRange {
    type Output = SourceRange;

    fn add(self, rhs: SourceRange) -> SourceRange {
        assert!(self.file == rhs.file);
        SourceRange {
            file: self.file,
            start: self.start.min(rhs.start),
            end: self.end.max(rhs.end),
        }
    }
}

impl Grammar {
    pub fn new(file: FileId) -> Grammar {
        Grammar {
            version: "1,0".to_owned(),
            comments: vec![],
            endianness: None,
            declarations: vec![],
            file,
        }
    }
}

impl<'d> Located<'d> for Field {
    fn loc(&'d self) -> &'d SourceRange {
        match self {
            Field::Checksum { loc, .. }
            | Field::Padding { loc, .. }
            | Field::Size { loc, .. }
            | Field::Count { loc, .. }
            | Field::Body { loc, .. }
            | Field::Payload { loc, .. }
            | Field::Fixed { loc, .. }
            | Field::Reserved { loc, .. }
            | Field::Array { loc, .. }
            | Field::Scalar { loc, .. }
            | Field::Typedef { loc, .. }
            | Field::Group { loc, .. } => loc,
        }
    }
}

impl<'d> Located<'d> for Decl {
    fn loc(&'d self) -> &'d SourceRange {
        match self {
            Decl::Checksum { loc, .. }
            | Decl::CustomField { loc, .. }
            | Decl::Enum { loc, .. }
            | Decl::Packet { loc, .. }
            | Decl::Struct { loc, .. }
            | Decl::Group { loc, .. }
            | Decl::Test { loc, .. } => loc,
        }
    }
}

impl<'d> Named<'d> for Field {
    fn id(&'d self) -> Option<&'d String> {
        match self {
            Field::Checksum { .. }
            | Field::Padding { .. }
            | Field::Size { .. }
            | Field::Count { .. }
            | Field::Body { .. }
            | Field::Payload { .. }
            | Field::Fixed { .. }
            | Field::Reserved { .. }
            | Field::Group { .. } => None,
            Field::Array { id, .. } | Field::Scalar { id, .. } | Field::Typedef { id, .. } => {
                Some(id)
            }
        }
    }
}

impl<'d> Named<'d> for Decl {
    fn id(&'d self) -> Option<&'d String> {
        match self {
            Decl::Test { .. } => None,
            Decl::Checksum { id, .. }
            | Decl::CustomField { id, .. }
            | Decl::Enum { id, .. }
            | Decl::Packet { id, .. }
            | Decl::Struct { id, .. }
            | Decl::Group { id, .. } => Some(id),
        }
    }
}
