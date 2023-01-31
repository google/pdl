use crate::lint;
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

#[derive(Debug, Default, Copy, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SourceLocation {
    /// Byte offset into the file (counted from zero).
    pub offset: usize,
    /// Line number (counted from zero).
    pub line: usize,
    /// Column number (counted from zero)
    pub column: usize,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EndiannessValue {
    LittleEndian,
    BigEndian,
}

#[derive(Debug, Copy, Clone, Serialize)]
#[serde(tag = "kind", rename = "endianness_declaration")]
pub struct Endianness {
    pub loc: SourceRange,
    pub value: EndiannessValue,
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename = "tag")]
pub struct Tag {
    pub id: String,
    pub loc: SourceRange,
    pub value: usize,
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "kind", rename = "constraint")]
pub struct Constraint {
    pub id: String,
    pub loc: SourceRange,
    pub value: Option<usize>,
    pub tag_id: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "kind")]
pub enum Field {
    #[serde(rename = "checksum_field")]
    Checksum { loc: SourceRange, field_id: String },
    #[serde(rename = "padding_field")]
    Padding { loc: SourceRange, size: usize },
    #[serde(rename = "size_field")]
    Size { loc: SourceRange, field_id: String, width: usize },
    #[serde(rename = "count_field")]
    Count { loc: SourceRange, field_id: String, width: usize },
    #[serde(rename = "elementsize_field")]
    ElementSize { loc: SourceRange, field_id: String, width: usize },
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
pub struct File {
    pub version: String,
    pub file: FileId,
    pub comments: Vec<Comment>,
    pub endianness: Endianness,
    pub declarations: Vec<Decl>,
}

impl SourceLocation {
    /// Construct a new source location.
    ///
    /// The `line_starts` indicates the byte offsets where new lines
    /// start in the file. The first element should thus be `0` since
    /// every file has at least one line starting at offset `0`.
    pub fn new(offset: usize, line_starts: &[usize]) -> SourceLocation {
        let mut loc = SourceLocation { offset, line: 0, column: offset };
        for (line, start) in line_starts.iter().enumerate() {
            if *start > offset {
                break;
            }
            loc = SourceLocation { offset, line, column: offset - start };
        }
        loc
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

impl File {
    pub fn new(file: FileId) -> File {
        File {
            version: "1,0".to_owned(),
            comments: vec![],
            // The endianness is mandatory, so this default value will
            // be updated while parsing.
            endianness: Endianness {
                loc: SourceRange::default(),
                value: EndiannessValue::LittleEndian,
            },
            declarations: vec![],
            file,
        }
    }
}

impl Decl {
    pub fn loc(&self) -> &SourceRange {
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

    pub fn id(&self) -> Option<&str> {
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

impl Field {
    pub fn loc(&self) -> &SourceRange {
        match self {
            Field::Checksum { loc, .. }
            | Field::Padding { loc, .. }
            | Field::Size { loc, .. }
            | Field::ElementSize { loc, .. }
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

    pub fn id(&self) -> Option<&str> {
        match self {
            Field::Checksum { .. }
            | Field::Padding { .. }
            | Field::Size { .. }
            | Field::ElementSize { .. }
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

    pub fn is_bitfield(&self, scope: &lint::Scope<'_>) -> bool {
        match self {
            Field::Size { .. }
            | Field::Count { .. }
            | Field::Fixed { .. }
            | Field::Reserved { .. }
            | Field::Scalar { .. } => true,
            Field::Typedef { type_id, .. } => {
                let field = scope.typedef.get(type_id.as_str());
                matches!(field, Some(Decl::Enum { .. }))
            }
            _ => false,
        }
    }

    pub fn width(&self, scope: &lint::Scope<'_>) -> Option<usize> {
        match self {
            Field::Scalar { width, .. }
            | Field::Size { width, .. }
            | Field::Count { width, .. }
            | Field::Reserved { width, .. } => Some(*width),
            Field::Typedef { type_id, .. } => match scope.typedef.get(type_id.as_str()) {
                Some(Decl::Enum { width, .. }) => Some(*width),
                _ => None,
            },
            // TODO(mgeisler): padding, arrays, etc.
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_location_new() {
        let line_starts = &[0, 20, 80, 120, 150];
        assert_eq!(
            SourceLocation::new(0, line_starts),
            SourceLocation { offset: 0, line: 0, column: 0 }
        );
        assert_eq!(
            SourceLocation::new(10, line_starts),
            SourceLocation { offset: 10, line: 0, column: 10 }
        );
        assert_eq!(
            SourceLocation::new(50, line_starts),
            SourceLocation { offset: 50, line: 1, column: 30 }
        );
        assert_eq!(
            SourceLocation::new(100, line_starts),
            SourceLocation { offset: 100, line: 2, column: 20 }
        );
        assert_eq!(
            SourceLocation::new(1000, line_starts),
            SourceLocation { offset: 1000, line: 4, column: 850 }
        );
    }

    #[test]
    fn source_location_new_no_crash_with_empty_line_starts() {
        let loc = SourceLocation::new(100, &[]);
        assert_eq!(loc, SourceLocation { offset: 100, line: 0, column: 100 });
    }
}
