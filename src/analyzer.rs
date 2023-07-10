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

use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::files;
use codespan_reporting::term;
use codespan_reporting::term::termcolor;
use std::collections::HashMap;

use crate::ast::*;
use crate::parser::ast as parser_ast;
use crate::utils;

pub mod ast {
    use serde::Serialize;

    /// Field and declaration size information.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[allow(unused)]
    pub enum Size {
        /// Constant size in bits.
        Static(usize),
        /// Size indicated at packet parsing by a size or count field.
        /// The parameter is the static part of the size.
        Dynamic,
        /// The size cannot be determined statically or at runtime.
        /// The packet assumes the largest possible size.
        Unknown,
    }

    // TODO: use derive(Default) when UWB is using Rust 1.62.0.
    #[allow(clippy::derivable_impls)]
    impl Default for Size {
        fn default() -> Size {
            Size::Unknown
        }
    }

    #[derive(Debug, Serialize, Default, Clone, PartialEq)]
    pub struct Annotation;

    #[derive(Default, Debug, Clone, PartialEq, Eq)]
    pub struct FieldAnnotation {
        // Size of field.
        pub size: Size,
        // Size of field with padding bytes.
        // This information exists only for array fields.
        pub padded_size: Option<usize>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Eq)]
    pub struct DeclAnnotation {
        // Size computed excluding the payload.
        pub size: Size,
        // Payload size, or Static(0) if the declaration does not
        // have a payload.
        pub payload_size: Size,
    }

    impl std::ops::Add for Size {
        type Output = Size;
        fn add(self, rhs: Size) -> Self::Output {
            match (self, rhs) {
                (Size::Unknown, _) | (_, Size::Unknown) => Size::Unknown,
                (Size::Dynamic, _) | (_, Size::Dynamic) => Size::Dynamic,
                (Size::Static(lhs), Size::Static(rhs)) => Size::Static(lhs + rhs),
            }
        }
    }

    impl std::ops::Mul for Size {
        type Output = Size;
        fn mul(self, rhs: Size) -> Self::Output {
            match (self, rhs) {
                (Size::Unknown, _) | (_, Size::Unknown) => Size::Unknown,
                (Size::Dynamic, _) | (_, Size::Dynamic) => Size::Dynamic,
                (Size::Static(lhs), Size::Static(rhs)) => Size::Static(lhs * rhs),
            }
        }
    }

    impl std::ops::Mul<usize> for Size {
        type Output = Size;
        fn mul(self, rhs: usize) -> Self::Output {
            match self {
                Size::Unknown => Size::Unknown,
                Size::Dynamic => Size::Dynamic,
                Size::Static(lhs) => Size::Static(lhs * rhs),
            }
        }
    }

    impl Size {
        // Returns the width if the size is static.
        pub fn static_(&self) -> Option<usize> {
            match self {
                Size::Static(size) => Some(*size),
                Size::Dynamic | Size::Unknown => None,
            }
        }
    }

    impl DeclAnnotation {
        pub fn total_size(&self) -> Size {
            self.size + self.payload_size
        }
    }

    impl FieldAnnotation {
        pub fn new(size: Size) -> Self {
            FieldAnnotation { size, padded_size: None }
        }

        // Returns the field width or padded width if static.
        pub fn static_(&self) -> Option<usize> {
            match self.padded_size {
                Some(padding) => Some(8 * padding),
                None => self.size.static_(),
            }
        }
    }

    impl crate::ast::Annotation for Annotation {
        type FieldAnnotation = FieldAnnotation;
        type DeclAnnotation = DeclAnnotation;
    }

    #[allow(unused)]
    pub type Field = crate::ast::Field<Annotation>;
    #[allow(unused)]
    pub type Decl = crate::ast::Decl<Annotation>;
    #[allow(unused)]
    pub type File = crate::ast::File<Annotation>;
}

/// List of unique errors reported as analyzer diagnostics.
#[repr(u16)]
pub enum ErrorCode {
    DuplicateDeclIdentifier = 1,
    RecursiveDecl = 2,
    UndeclaredGroupIdentifier = 3,
    InvalidGroupIdentifier = 4,
    UndeclaredTypeIdentifier = 5,
    InvalidTypeIdentifier = 6,
    UndeclaredParentIdentifier = 7,
    InvalidParentIdentifier = 8,
    UndeclaredTestIdentifier = 9,
    InvalidTestIdentifier = 10,
    DuplicateFieldIdentifier = 11,
    DuplicateTagIdentifier = 12,
    DuplicateTagValue = 13,
    InvalidTagValue = 14,
    UndeclaredConstraintIdentifier = 15,
    InvalidConstraintIdentifier = 16,
    E17 = 17,
    ConstraintValueOutOfRange = 18,
    E19 = 19,
    E20 = 20,
    E21 = 21,
    DuplicateConstraintIdentifier = 22,
    DuplicateSizeField = 23,
    UndeclaredSizeIdentifier = 24,
    InvalidSizeIdentifier = 25,
    DuplicateCountField = 26,
    UndeclaredCountIdentifier = 27,
    InvalidCountIdentifier = 28,
    DuplicateElementSizeField = 29,
    UndeclaredElementSizeIdentifier = 30,
    InvalidElementSizeIdentifier = 31,
    FixedValueOutOfRange = 32,
    E33 = 33,
    E34 = 34,
    E35 = 35,
    DuplicatePayloadField = 36,
    MissingPayloadField = 37,
    RedundantArraySize = 38,
    InvalidPaddingField = 39,
    InvalidTagRange = 40,
    DuplicateTagRange = 41,
    E42 = 42,
    E43 = 43,
    DuplicateDefaultTag = 44,
}

impl From<ErrorCode> for String {
    fn from(code: ErrorCode) -> Self {
        format!("E{}", code as u16)
    }
}

/// Aggregate analyzer diagnostics.
#[derive(Debug, Default)]
pub struct Diagnostics {
    pub diagnostics: Vec<Diagnostic<FileId>>,
}

/// Gather information about the full AST.
#[derive(Debug)]
pub struct Scope<'d, A: Annotation = ast::Annotation> {
    /// Reference to the source file.
    pub file: &'d crate::ast::File<A>,
    /// Collection of Group, Packet, Enum, Struct, Checksum, and CustomField
    /// declarations.
    pub typedef: HashMap<String, &'d crate::ast::Decl<A>>,
}

impl Diagnostics {
    fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    fn push(&mut self, diagnostic: Diagnostic<FileId>) {
        self.diagnostics.push(diagnostic)
    }

    fn err_or<T>(self, value: T) -> Result<T, Diagnostics> {
        if self.is_empty() {
            Ok(value)
        } else {
            Err(self)
        }
    }

    pub fn emit(
        &self,
        sources: &SourceDatabase,
        writer: &mut dyn termcolor::WriteColor,
    ) -> Result<(), files::Error> {
        let config = term::Config::default();
        for d in self.diagnostics.iter() {
            term::emit(writer, &config, sources, d)?;
        }
        Ok(())
    }
}

impl<'d, A: Annotation + Default> Scope<'d, A> {
    pub fn new(file: &'d crate::ast::File<A>) -> Result<Scope<'d, A>, Diagnostics> {
        // Gather top-level declarations.
        let mut scope: Scope<A> = Scope { file, typedef: Default::default() };
        let mut diagnostics: Diagnostics = Default::default();
        for decl in &file.declarations {
            if let Some(id) = decl.id() {
                if let Some(prev) = scope.typedef.insert(id.to_string(), decl) {
                    diagnostics.push(
                        Diagnostic::error()
                            .with_code(ErrorCode::DuplicateDeclIdentifier)
                            .with_message(format!(
                                "redeclaration of {} identifier `{}`",
                                decl.kind(),
                                id
                            ))
                            .with_labels(vec![
                                decl.loc.primary(),
                                prev.loc
                                    .secondary()
                                    .with_message(format!("`{}` is first declared here", id)),
                            ]),
                    )
                }
            }
        }

        // Return failure if any diagnostic is raised.
        if diagnostics.is_empty() {
            Ok(scope)
        } else {
            Err(diagnostics)
        }
    }

    /// Iterate over the child declarations of the selected declaration.
    pub fn iter_children<'s>(
        &'s self,
        decl: &'d crate::ast::Decl<A>,
    ) -> impl Iterator<Item = &'d crate::ast::Decl<A>> + 's {
        self.file.iter_children(decl)
    }

    /// Return the parent declaration of the selected declaration,
    /// if it has one.
    pub fn get_parent(&self, decl: &crate::ast::Decl<A>) -> Option<&'d crate::ast::Decl<A>> {
        decl.parent_id().and_then(|parent_id| self.typedef.get(parent_id).cloned())
    }

    /// Iterate over the parent declarations of the selected declaration.
    pub fn iter_parents<'s>(
        &'s self,
        decl: &'d crate::ast::Decl<A>,
    ) -> impl Iterator<Item = &'d Decl<A>> + 's {
        std::iter::successors(self.get_parent(decl), |decl| self.get_parent(decl))
    }

    /// Iterate over the parent declarations of the selected declaration,
    /// including the current declaration.
    pub fn iter_parents_and_self<'s>(
        &'s self,
        decl: &'d crate::ast::Decl<A>,
    ) -> impl Iterator<Item = &'d Decl<A>> + 's {
        std::iter::successors(Some(decl), |decl| self.get_parent(decl))
    }

    /// Iterate over the declaration and its parent's fields.
    pub fn iter_fields<'s>(
        &'s self,
        decl: &'d crate::ast::Decl<A>,
    ) -> impl Iterator<Item = &'d Field<A>> + 's {
        std::iter::successors(Some(decl), |decl| self.get_parent(decl)).flat_map(Decl::fields)
    }

    /// Iterate over the declaration parent's fields.
    pub fn iter_parent_fields<'s>(
        &'s self,
        decl: &'d crate::ast::Decl<A>,
    ) -> impl Iterator<Item = &'d crate::ast::Field<A>> + 's {
        std::iter::successors(self.get_parent(decl), |decl| self.get_parent(decl))
            .flat_map(Decl::fields)
    }

    /// Iterate over the declaration and its parent's constraints.
    pub fn iter_constraints<'s>(
        &'s self,
        decl: &'d crate::ast::Decl<A>,
    ) -> impl Iterator<Item = &'d Constraint> + 's {
        std::iter::successors(Some(decl), |decl| self.get_parent(decl)).flat_map(Decl::constraints)
    }

    /// Return the type declaration for the selected field, if applicable.
    pub fn get_type_declaration(
        &self,
        field: &crate::ast::Field<A>,
    ) -> Option<&'d crate::ast::Decl<A>> {
        match &field.desc {
            FieldDesc::Checksum { .. }
            | FieldDesc::Padding { .. }
            | FieldDesc::Size { .. }
            | FieldDesc::Count { .. }
            | FieldDesc::ElementSize { .. }
            | FieldDesc::Body
            | FieldDesc::Payload { .. }
            | FieldDesc::FixedScalar { .. }
            | FieldDesc::Reserved { .. }
            | FieldDesc::Group { .. }
            | FieldDesc::Scalar { .. }
            | FieldDesc::Array { type_id: None, .. } => None,
            FieldDesc::FixedEnum { enum_id: type_id, .. }
            | FieldDesc::Array { type_id: Some(type_id), .. }
            | FieldDesc::Typedef { type_id, .. } => self.typedef.get(type_id).cloned(),
        }
    }

    /// Test if the selected field is a bit-field.
    pub fn is_bitfield(&self, field: &crate::ast::Field<A>) -> bool {
        match &field.desc {
            FieldDesc::Size { .. }
            | FieldDesc::Count { .. }
            | FieldDesc::ElementSize { .. }
            | FieldDesc::FixedScalar { .. }
            | FieldDesc::FixedEnum { .. }
            | FieldDesc::Reserved { .. }
            | FieldDesc::Scalar { .. } => true,
            FieldDesc::Typedef { type_id, .. } => {
                let field = self.typedef.get(type_id.as_str());
                matches!(field, Some(Decl { desc: DeclDesc::Enum { .. }, .. }))
            }
            _ => false,
        }
    }
}

/// Return the bit-width of a scalar value.
fn bit_width(value: usize) -> usize {
    usize::BITS as usize - value.leading_zeros() as usize
}

/// Return the maximum value for a scalar value.
fn scalar_max(width: usize) -> usize {
    if width >= usize::BITS as usize {
        usize::MAX
    } else {
        (1 << width) - 1
    }
}

/// Check declaration identifiers.
/// Raises error diagnostics for the following cases:
///      - undeclared parent identifier
///      - invalid parent identifier
///      - undeclared group identifier
///      - invalid group identifier
///      - undeclared typedef identifier
///      - invalid typedef identifier
///      - undeclared test identifier
///      - invalid test identifier
///      - recursive declaration
fn check_decl_identifiers(
    file: &parser_ast::File,
    scope: &Scope<parser_ast::Annotation>,
) -> Result<(), Diagnostics> {
    enum Mark {
        Temporary,
        Permanent,
    }
    #[derive(Default)]
    struct Context<'d> {
        visited: HashMap<&'d str, Mark>,
    }

    fn bfs<'d>(
        decl: &'d parser_ast::Decl,
        context: &mut Context<'d>,
        scope: &Scope<'d, parser_ast::Annotation>,
        diagnostics: &mut Diagnostics,
    ) {
        let decl_id = decl.id().unwrap();
        match context.visited.get(decl_id) {
            Some(Mark::Permanent) => return,
            Some(Mark::Temporary) => {
                diagnostics.push(
                    Diagnostic::error()
                        .with_code(ErrorCode::RecursiveDecl)
                        .with_message(format!(
                            "recursive declaration of {} `{}`",
                            decl.kind(),
                            decl_id
                        ))
                        .with_labels(vec![decl.loc.primary()]),
                );
                return;
            }
            _ => (),
        }

        // Start visiting current declaration.
        context.visited.insert(decl_id, Mark::Temporary);

        // Iterate over Struct and Group fields.
        for field in decl.fields() {
            match &field.desc {
                // Validate that the group field has a valid identifier.
                // If the type is a group recurse the group definition.
                FieldDesc::Group { group_id, .. } => match scope.typedef.get(group_id) {
                    None => diagnostics.push(
                        Diagnostic::error()
                            .with_code(ErrorCode::UndeclaredGroupIdentifier)
                            .with_message(format!("undeclared group identifier `{}`", group_id))
                            .with_labels(vec![field.loc.primary()])
                            .with_notes(vec!["hint: expected group identifier".to_owned()]),
                    ),
                    Some(group_decl @ Decl { desc: DeclDesc::Group { .. }, .. }) => {
                        bfs(group_decl, context, scope, diagnostics)
                    }
                    Some(_) => diagnostics.push(
                        Diagnostic::error()
                            .with_code(ErrorCode::InvalidGroupIdentifier)
                            .with_message(format!("invalid group identifier `{}`", group_id))
                            .with_labels(vec![field.loc.primary()])
                            .with_notes(vec!["hint: expected group identifier".to_owned()]),
                    ),
                },
                // Validate that the typedef field has a valid identifier.
                // If the type is a struct recurse the struct definition.
                // Append the field to the packet re-definition.
                FieldDesc::Typedef { type_id, .. }
                | FieldDesc::Array { type_id: Some(type_id), .. } => {
                    match scope.typedef.get(type_id) {
                        None => diagnostics.push(
                            Diagnostic::error().with_code(ErrorCode::UndeclaredTypeIdentifier)
                                .with_message(format!(
                                    "undeclared {} identifier `{}`",
                                    field.kind(),
                                    type_id
                                ))
                                .with_labels(vec![field.loc.primary()])
                                .with_notes(vec!["hint: expected enum, struct, custom_field, or checksum identifier".to_owned()]),
                        ),
                        Some(Decl { desc: DeclDesc::Packet { .. }, .. }) => diagnostics.push(
                            Diagnostic::error().with_code(ErrorCode::InvalidTypeIdentifier)
                                .with_message(format!(
                                    "invalid {} identifier `{}`",
                                    field.kind(),
                                    type_id
                                ))
                                .with_labels(vec![field.loc.primary()])
                                .with_notes(vec!["hint: expected enum, struct, custom_field, or checksum identifier".to_owned()]),
                        ),
                        Some(typedef_decl) =>
                            // Not recursing on array type since it is allowed to
                            // have recursive structures, e.g. nested TLV types.
                            if matches!(&field.desc, FieldDesc::Typedef { .. }) ||
                               matches!(&field.desc, FieldDesc::Array { size: Some(_), .. }) {
                                bfs(typedef_decl, context, scope, diagnostics)
                            }
                    }
                }
                // Ignore other fields.
                _ => (),
            }
        }

        // Iterate over parent declaration.
        if let Some(parent_id) = decl.parent_id() {
            let parent_decl = scope.typedef.get(parent_id);
            match (&decl.desc, parent_decl) {
                (DeclDesc::Packet { .. }, None) | (DeclDesc::Struct { .. }, None) => diagnostics
                    .push(
                        Diagnostic::error()
                            .with_code(ErrorCode::UndeclaredParentIdentifier)
                            .with_message(format!("undeclared parent identifier `{}`", parent_id))
                            .with_labels(vec![decl.loc.primary()])
                            .with_notes(vec![format!("hint: expected {} identifier", decl.kind())]),
                    ),
                (
                    DeclDesc::Packet { .. },
                    Some(parent_decl @ Decl { desc: DeclDesc::Packet { .. }, .. }),
                )
                | (
                    DeclDesc::Struct { .. },
                    Some(parent_decl @ Decl { desc: DeclDesc::Struct { .. }, .. }),
                ) => bfs(parent_decl, context, scope, diagnostics),
                (_, Some(_)) => diagnostics.push(
                    Diagnostic::error()
                        .with_code(ErrorCode::InvalidParentIdentifier)
                        .with_message(format!("invalid parent identifier `{}`", parent_id))
                        .with_labels(vec![decl.loc.primary()])
                        .with_notes(vec![format!("hint: expected {} identifier", decl.kind())]),
                ),
                _ => unreachable!(),
            }
        }

        // Done visiting current declaration.
        context.visited.insert(decl_id, Mark::Permanent);
    }

    // Start bfs.
    let mut diagnostics = Default::default();
    let mut context = Default::default();
    for decl in &file.declarations {
        match &decl.desc {
            DeclDesc::Checksum { .. } | DeclDesc::CustomField { .. } | DeclDesc::Enum { .. } => (),
            DeclDesc::Packet { .. } | DeclDesc::Struct { .. } | DeclDesc::Group { .. } => {
                bfs(decl, &mut context, scope, &mut diagnostics)
            }
            DeclDesc::Test { type_id, .. } => match scope.typedef.get(type_id) {
                None => diagnostics.push(
                    Diagnostic::error()
                        .with_code(ErrorCode::UndeclaredTestIdentifier)
                        .with_message(format!("undeclared test identifier `{}`", type_id))
                        .with_labels(vec![decl.loc.primary()])
                        .with_notes(vec!["hint: expected packet identifier".to_owned()]),
                ),
                Some(Decl { desc: DeclDesc::Packet { .. }, .. }) => (),
                Some(_) => diagnostics.push(
                    Diagnostic::error()
                        .with_code(ErrorCode::InvalidTestIdentifier)
                        .with_message(format!("invalid test identifier `{}`", type_id))
                        .with_labels(vec![decl.loc.primary()])
                        .with_notes(vec!["hint: expected packet identifier".to_owned()]),
                ),
            },
        }
    }

    diagnostics.err_or(())
}

/// Check field identifiers.
/// Raises error diagnostics for the following cases:
///      - duplicate field identifier
fn check_field_identifiers(file: &parser_ast::File) -> Result<(), Diagnostics> {
    let mut diagnostics: Diagnostics = Default::default();
    for decl in &file.declarations {
        let mut local_scope = HashMap::new();
        for field in decl.fields() {
            if let Some(id) = field.id() {
                if let Some(prev) = local_scope.insert(id.to_string(), field) {
                    diagnostics.push(
                        Diagnostic::error()
                            .with_code(ErrorCode::DuplicateFieldIdentifier)
                            .with_message(format!(
                                "redeclaration of {} field identifier `{}`",
                                field.kind(),
                                id
                            ))
                            .with_labels(vec![
                                field.loc.primary(),
                                prev.loc
                                    .secondary()
                                    .with_message(format!("`{}` is first declared here", id)),
                            ]),
                    )
                }
            }
        }
    }

    diagnostics.err_or(())
}

/// Check enum declarations.
/// Raises error diagnostics for the following cases:
///      - duplicate tag identifier
///      - duplicate tag value
fn check_enum_declarations(file: &parser_ast::File) -> Result<(), Diagnostics> {
    // Return the inclusive range with bounds correctly ordered.
    // The analyzer will raise an error if the bounds are incorrectly ordered, but this
    // will enable additional checks.
    fn ordered_range(range: &std::ops::RangeInclusive<usize>) -> std::ops::RangeInclusive<usize> {
        *std::cmp::min(range.start(), range.end())..=*std::cmp::max(range.start(), range.end())
    }

    fn check_tag_value<'a>(
        tag: &'a TagValue,
        range: std::ops::RangeInclusive<usize>,
        reserved_ranges: impl Iterator<Item = &'a TagRange>,
        tags_by_id: &mut HashMap<&'a str, SourceRange>,
        tags_by_value: &mut HashMap<usize, SourceRange>,
        diagnostics: &mut Diagnostics,
    ) {
        if let Some(prev) = tags_by_id.insert(&tag.id, tag.loc) {
            diagnostics.push(
                Diagnostic::error()
                    .with_code(ErrorCode::DuplicateTagIdentifier)
                    .with_message(format!("duplicate tag identifier `{}`", tag.id))
                    .with_labels(vec![
                        tag.loc.primary(),
                        prev.secondary()
                            .with_message(format!("`{}` is first declared here", tag.id)),
                    ]),
            )
        }
        if let Some(prev) = tags_by_value.insert(tag.value, tag.loc) {
            diagnostics.push(
                Diagnostic::error()
                    .with_code(ErrorCode::DuplicateTagValue)
                    .with_message(format!("duplicate tag value `{}`", tag.value))
                    .with_labels(vec![
                        tag.loc.primary(),
                        prev.secondary()
                            .with_message(format!("`{}` is first declared here", tag.value)),
                    ]),
            )
        }
        if !range.contains(&tag.value) {
            diagnostics.push(
                Diagnostic::error()
                    .with_code(ErrorCode::InvalidTagValue)
                    .with_message(format!(
                        "tag value `{}` is outside the range of valid values `{}..{}`",
                        tag.value,
                        range.start(),
                        range.end()
                    ))
                    .with_labels(vec![tag.loc.primary()]),
            )
        }
        for reserved_range in reserved_ranges {
            if ordered_range(&reserved_range.range).contains(&tag.value) {
                diagnostics.push(
                    Diagnostic::error()
                        .with_code(ErrorCode::E43)
                        .with_message(format!(
                            "tag value `{}` is declared inside the reserved range `{} = {}..{}`",
                            tag.value,
                            reserved_range.id,
                            reserved_range.range.start(),
                            reserved_range.range.end()
                        ))
                        .with_labels(vec![tag.loc.primary()]),
                )
            }
        }
    }

    fn check_tag_range<'a>(
        tag: &'a TagRange,
        range: std::ops::RangeInclusive<usize>,
        tags_by_id: &mut HashMap<&'a str, SourceRange>,
        tags_by_value: &mut HashMap<usize, SourceRange>,
        diagnostics: &mut Diagnostics,
    ) {
        if let Some(prev) = tags_by_id.insert(&tag.id, tag.loc) {
            diagnostics.push(
                Diagnostic::error()
                    .with_code(ErrorCode::DuplicateTagIdentifier)
                    .with_message(format!("duplicate tag identifier `{}`", tag.id))
                    .with_labels(vec![
                        tag.loc.primary(),
                        prev.secondary()
                            .with_message(format!("`{}` is first declared here", tag.id)),
                    ]),
            )
        }
        if !range.contains(tag.range.start()) || !range.contains(tag.range.end()) {
            diagnostics.push(
                Diagnostic::error()
                    .with_code(ErrorCode::InvalidTagRange)
                    .with_message(format!(
                        "tag range `{}..{}` has bounds outside the range of valid values `{}..{}`",
                        tag.range.start(),
                        tag.range.end(),
                        range.start(),
                        range.end(),
                    ))
                    .with_labels(vec![tag.loc.primary()]),
            )
        }
        if tag.range.start() >= tag.range.end() {
            diagnostics.push(
                Diagnostic::error()
                    .with_code(ErrorCode::InvalidTagRange)
                    .with_message(format!(
                        "tag start value `{}` is greater than or equal to the end value `{}`",
                        tag.range.start(),
                        tag.range.end()
                    ))
                    .with_labels(vec![tag.loc.primary()]),
            )
        }

        let range = ordered_range(&tag.range);
        for tag in tag.tags.iter() {
            check_tag_value(tag, range.clone(), [].iter(), tags_by_id, tags_by_value, diagnostics)
        }
    }

    fn check_tag_other<'a>(
        tag: &'a TagOther,
        tags_by_id: &mut HashMap<&'a str, SourceRange>,
        tag_other: &mut Option<SourceRange>,
        diagnostics: &mut Diagnostics,
    ) {
        if let Some(prev) = tags_by_id.insert(&tag.id, tag.loc) {
            diagnostics.push(
                Diagnostic::error()
                    .with_code(ErrorCode::DuplicateTagIdentifier)
                    .with_message(format!("duplicate tag identifier `{}`", tag.id))
                    .with_labels(vec![
                        tag.loc.primary(),
                        prev.secondary()
                            .with_message(format!("`{}` is first declared here", tag.id)),
                    ]),
            )
        }
        if let Some(prev) = tag_other {
            diagnostics.push(
                Diagnostic::error()
                    .with_code(ErrorCode::DuplicateDefaultTag)
                    .with_message("duplicate default tag".to_owned())
                    .with_labels(vec![
                        tag.loc.primary(),
                        prev.secondary()
                            .with_message("the default tag is first declared here".to_owned()),
                    ]),
            )
        }
        *tag_other = Some(tag.loc)
    }

    let mut diagnostics: Diagnostics = Default::default();
    for decl in &file.declarations {
        if let DeclDesc::Enum { tags, width, .. } = &decl.desc {
            let mut tags_by_id = HashMap::new();
            let mut tags_by_value = HashMap::new();
            let mut tags_by_range = tags
                .iter()
                .filter_map(|tag| match tag {
                    Tag::Range(tag) => Some(tag),
                    _ => None,
                })
                .collect::<Vec<_>>();
            let mut tag_other = None;

            for tag in tags {
                match tag {
                    Tag::Value(value) => check_tag_value(
                        value,
                        0..=scalar_max(*width),
                        tags_by_range.iter().copied(),
                        &mut tags_by_id,
                        &mut tags_by_value,
                        &mut diagnostics,
                    ),
                    Tag::Range(range) => check_tag_range(
                        range,
                        0..=scalar_max(*width),
                        &mut tags_by_id,
                        &mut tags_by_value,
                        &mut diagnostics,
                    ),
                    Tag::Other(other) => {
                        check_tag_other(other, &mut tags_by_id, &mut tag_other, &mut diagnostics)
                    }
                }
            }

            // Order tag ranges by increasing bounds in order to check for intersecting ranges.
            tags_by_range.sort_by(|lhs, rhs| {
                ordered_range(&lhs.range).into_inner().cmp(&ordered_range(&rhs.range).into_inner())
            });

            // Iterate to check for overlap between tag ranges.
            // Not all potential errors are reported, but the check will report
            // at least one error if the values are incorrect.
            for tag in tags_by_range.windows(2) {
                let left_tag = tag[0];
                let right_tag = tag[1];
                let left = ordered_range(&left_tag.range);
                let right = ordered_range(&right_tag.range);
                if !(left.end() < right.start() || right.end() < left.start()) {
                    diagnostics.push(
                        Diagnostic::error()
                            .with_code(ErrorCode::DuplicateTagRange)
                            .with_message(format!(
                                "overlapping tag range `{}..{}`",
                                right.start(),
                                right.end()
                            ))
                            .with_labels(vec![
                                right_tag.loc.primary(),
                                left_tag.loc.secondary().with_message(format!(
                                    "`{}..{}` is first declared here",
                                    left.start(),
                                    left.end()
                                )),
                            ]),
                    )
                }
            }
        }
    }

    diagnostics.err_or(())
}

/// Check constraints.
/// Raises error diagnostics for the following cases:
///      - undeclared constraint identifier
///      - invalid constraint identifier
///      - invalid constraint scalar value (bad type)
///      - invalid constraint scalar value (overflow)
///      - invalid constraint enum value (bad type)
///      - invalid constraint enum value (undeclared tag)
///      - duplicate constraint
fn check_constraints(
    file: &parser_ast::File,
    scope: &Scope<parser_ast::Annotation>,
) -> Result<(), Diagnostics> {
    fn check_constraint(
        constraint: &Constraint,
        decl: &parser_ast::Decl,
        scope: &Scope<parser_ast::Annotation>,
        diagnostics: &mut Diagnostics,
    ) {
        match scope.iter_fields(decl).find(|field| field.id() == Some(&constraint.id)) {
            None => diagnostics.push(
                Diagnostic::error()
                    .with_code(ErrorCode::UndeclaredConstraintIdentifier)
                    .with_message(format!("undeclared constraint identifier `{}`", constraint.id))
                    .with_labels(vec![constraint.loc.primary()])
                    .with_notes(vec!["hint: expected scalar or typedef identifier".to_owned()]),
            ),
            Some(field @ Field { desc: FieldDesc::Array { .. }, .. }) => diagnostics.push(
                Diagnostic::error()
                    .with_code(ErrorCode::InvalidConstraintIdentifier)
                    .with_message(format!("invalid constraint identifier `{}`", constraint.id))
                    .with_labels(vec![
                        constraint.loc.primary(),
                        field.loc.secondary().with_message(format!(
                            "`{}` is declared here as array field",
                            constraint.id
                        )),
                    ])
                    .with_notes(vec!["hint: expected scalar or typedef identifier".to_owned()]),
            ),
            Some(field @ Field { desc: FieldDesc::Scalar { width, .. }, .. }) => {
                match constraint.value {
                    None => diagnostics.push(
                        Diagnostic::error()
                            .with_code(ErrorCode::E17)
                            .with_message(format!(
                                "invalid constraint value `{}`",
                                constraint.tag_id.as_ref().unwrap()
                            ))
                            .with_labels(vec![
                                constraint.loc.primary(),
                                field.loc.secondary().with_message(format!(
                                    "`{}` is declared here as scalar field",
                                    constraint.id
                                )),
                            ])
                            .with_notes(vec!["hint: expected scalar value".to_owned()]),
                    ),
                    Some(value) if bit_width(value) > *width => diagnostics.push(
                        Diagnostic::error()
                            .with_code(ErrorCode::ConstraintValueOutOfRange)
                            .with_message(format!(
                                "constraint value `{}` is larger than maximum value",
                                value
                            ))
                            .with_labels(vec![constraint.loc.primary(), field.loc.secondary()]),
                    ),
                    _ => (),
                }
            }
            Some(field @ Field { desc: FieldDesc::Typedef { type_id, .. }, .. }) => {
                match scope.typedef.get(type_id) {
                    None => (),
                    Some(Decl { desc: DeclDesc::Enum { tags, .. }, .. }) => {
                        match &constraint.tag_id {
                            None => diagnostics.push(
                                Diagnostic::error()
                                    .with_code(ErrorCode::E19)
                                    .with_message(format!(
                                        "invalid constraint value `{}`",
                                        constraint.value.unwrap()
                                    ))
                                    .with_labels(vec![
                                        constraint.loc.primary(),
                                        field.loc.secondary().with_message(format!(
                                            "`{}` is declared here as typedef field",
                                            constraint.id
                                        )),
                                    ])
                                    .with_notes(vec!["hint: expected enum value".to_owned()]),
                            ),
                            Some(tag_id) => match tags.iter().find(|tag| tag.id() == tag_id) {
                                None => diagnostics.push(
                                    Diagnostic::error()
                                        .with_code(ErrorCode::E20)
                                        .with_message(format!("undeclared enum tag `{}`", tag_id))
                                        .with_labels(vec![
                                            constraint.loc.primary(),
                                            field.loc.secondary().with_message(format!(
                                                "`{}` is declared here",
                                                constraint.id
                                            )),
                                        ]),
                                ),
                                Some(Tag::Range { .. }) => diagnostics.push(
                                    Diagnostic::error()
                                        .with_code(ErrorCode::E42)
                                        .with_message(format!(
                                            "enum tag `{}` defines a range",
                                            tag_id
                                        ))
                                        .with_labels(vec![
                                            constraint.loc.primary(),
                                            field.loc.secondary().with_message(format!(
                                                "`{}` is declared here",
                                                constraint.id
                                            )),
                                        ])
                                        .with_notes(vec![
                                            "hint: expected enum tag with value".to_owned()
                                        ]),
                                ),
                                Some(_) => (),
                            },
                        }
                    }
                    Some(decl) => diagnostics.push(
                        Diagnostic::error()
                            .with_code(ErrorCode::E21)
                            .with_message(format!(
                                "invalid constraint identifier `{}`",
                                constraint.value.unwrap()
                            ))
                            .with_labels(vec![
                                constraint.loc.primary(),
                                field.loc.secondary().with_message(format!(
                                    "`{}` is declared here as {} typedef field",
                                    constraint.id,
                                    decl.kind()
                                )),
                            ])
                            .with_notes(vec!["hint: expected enum value".to_owned()]),
                    ),
                }
            }
            Some(_) => unreachable!(),
        }
    }

    fn check_constraints<'d>(
        constraints: &'d [Constraint],
        parent_decl: &parser_ast::Decl,
        scope: &Scope<parser_ast::Annotation>,
        mut constraints_by_id: HashMap<String, &'d Constraint>,
        diagnostics: &mut Diagnostics,
    ) {
        for constraint in constraints {
            check_constraint(constraint, parent_decl, scope, diagnostics);
            if let Some(prev) = constraints_by_id.insert(constraint.id.to_string(), constraint) {
                // Constraint appears twice in current set.
                diagnostics.push(
                    Diagnostic::error()
                        .with_code(ErrorCode::DuplicateConstraintIdentifier)
                        .with_message(format!(
                            "duplicate constraint identifier `{}`",
                            constraint.id
                        ))
                        .with_labels(vec![
                            constraint.loc.primary(),
                            prev.loc
                                .secondary()
                                .with_message(format!("`{}` is first constrained here", prev.id)),
                        ]),
                )
            }
        }
    }

    let mut diagnostics: Diagnostics = Default::default();
    for decl in &file.declarations {
        // Check constraints for packet inheritance.
        match &decl.desc {
            DeclDesc::Packet { constraints, parent_id: Some(parent_id), .. }
            | DeclDesc::Struct { constraints, parent_id: Some(parent_id), .. } => {
                let parent_decl = scope.typedef.get(parent_id).unwrap();
                check_constraints(
                    constraints,
                    parent_decl,
                    scope,
                    // Include constraints declared in parent declarations
                    // for duplicate check.
                    scope.iter_parents(decl).fold(HashMap::new(), |acc, decl| {
                        decl.constraints().fold(acc, |mut acc, constraint| {
                            let _ = acc.insert(constraint.id.to_string(), constraint);
                            acc
                        })
                    }),
                    &mut diagnostics,
                )
            }
            _ => (),
        }

        // Check constraints for group inlining.
        for field in decl.fields() {
            if let FieldDesc::Group { group_id, constraints } = &field.desc {
                let group_decl = scope.typedef.get(group_id).unwrap();
                check_constraints(constraints, group_decl, scope, HashMap::new(), &mut diagnostics)
            }
        }
    }

    diagnostics.err_or(())
}

/// Check size fields.
/// Raises error diagnostics for the following cases:
///      - undeclared size identifier
///      - invalid size identifier
///      - duplicate size field
///      - undeclared count identifier
///      - invalid count identifier
///      - duplicate count field
///      - undeclared elementsize identifier
///      - invalid elementsize identifier
///      - duplicate elementsize field
fn check_size_fields(file: &parser_ast::File) -> Result<(), Diagnostics> {
    let mut diagnostics: Diagnostics = Default::default();
    for decl in &file.declarations {
        let mut size_for_id = HashMap::new();
        let mut element_size_for_id = HashMap::new();
        for field in decl.fields() {
            // Check for duplicate size, count, or element size fields.
            if let Some((reverse_map, field_id, err)) = match &field.desc {
                FieldDesc::Size { field_id, .. } => {
                    Some((&mut size_for_id, field_id, ErrorCode::DuplicateSizeField))
                }
                FieldDesc::Count { field_id, .. } => {
                    Some((&mut size_for_id, field_id, ErrorCode::DuplicateCountField))
                }
                FieldDesc::ElementSize { field_id, .. } => {
                    Some((&mut element_size_for_id, field_id, ErrorCode::DuplicateElementSizeField))
                }
                _ => None,
            } {
                if let Some(prev) = reverse_map.insert(field_id, field) {
                    diagnostics.push(
                        Diagnostic::error()
                            .with_code(err)
                            .with_message(format!("duplicate {} field", field.kind()))
                            .with_labels(vec![
                                field.loc.primary(),
                                prev.loc.secondary().with_message(format!(
                                    "{} is first declared here",
                                    prev.kind()
                                )),
                            ]),
                    )
                }
            }

            // Check for invalid size, count, or element size field identifiers.
            match &field.desc {
                FieldDesc::Size { field_id, .. } => {
                    match decl.fields().find(|field| match &field.desc {
                        FieldDesc::Payload { .. } => field_id == "_payload_",
                        FieldDesc::Body { .. } => field_id == "_body_",
                        _ => field.id() == Some(field_id),
                    }) {
                        None => diagnostics.push(
                            Diagnostic::error()
                                .with_code(ErrorCode::UndeclaredSizeIdentifier)
                                .with_message(format!(
                                    "undeclared {} identifier `{}`",
                                    field.kind(),
                                    field_id
                                ))
                                .with_labels(vec![field.loc.primary()])
                                .with_notes(vec![
                                    "hint: expected payload, body, or array identifier".to_owned(),
                                ]),
                        ),
                        Some(Field { desc: FieldDesc::Body { .. }, .. })
                        | Some(Field { desc: FieldDesc::Payload { .. }, .. })
                        | Some(Field { desc: FieldDesc::Array { .. }, .. }) => (),
                        Some(Field { loc, .. }) => diagnostics.push(
                            Diagnostic::error()
                                .with_code(ErrorCode::InvalidSizeIdentifier)
                                .with_message(format!(
                                    "invalid {} identifier `{}`",
                                    field.kind(),
                                    field_id
                                ))
                                .with_labels(vec![field.loc.primary(), loc.secondary()])
                                .with_notes(vec![
                                    "hint: expected payload, body, or array identifier".to_owned(),
                                ]),
                        ),
                    }
                }

                FieldDesc::Count { field_id, .. } | FieldDesc::ElementSize { field_id, .. } => {
                    let (undeclared_err, invalid_err) =
                        if matches!(&field.desc, FieldDesc::Count { .. }) {
                            (
                                ErrorCode::UndeclaredCountIdentifier,
                                ErrorCode::InvalidCountIdentifier,
                            )
                        } else {
                            (
                                ErrorCode::UndeclaredElementSizeIdentifier,
                                ErrorCode::InvalidElementSizeIdentifier,
                            )
                        };
                    match decl.fields().find(|field| field.id() == Some(field_id)) {
                        None => diagnostics.push(
                            Diagnostic::error()
                                .with_code(undeclared_err)
                                .with_message(format!(
                                    "undeclared {} identifier `{}`",
                                    field.kind(),
                                    field_id
                                ))
                                .with_labels(vec![field.loc.primary()])
                                .with_notes(vec!["hint: expected array identifier".to_owned()]),
                        ),
                        Some(Field { desc: FieldDesc::Array { .. }, .. }) => (),
                        Some(Field { loc, .. }) => diagnostics.push(
                            Diagnostic::error()
                                .with_code(invalid_err)
                                .with_message(format!(
                                    "invalid {} identifier `{}`",
                                    field.kind(),
                                    field_id
                                ))
                                .with_labels(vec![field.loc.primary(), loc.secondary()])
                                .with_notes(vec!["hint: expected array identifier".to_owned()]),
                        ),
                    }
                }
                _ => (),
            }
        }
    }

    diagnostics.err_or(())
}

/// Check fixed fields.
/// Raises error diagnostics for the following cases:
///      - invalid scalar value
///      - undeclared enum identifier
///      - invalid enum identifier
///      - undeclared tag identifier
fn check_fixed_fields(
    file: &parser_ast::File,
    scope: &Scope<parser_ast::Annotation>,
) -> Result<(), Diagnostics> {
    let mut diagnostics: Diagnostics = Default::default();
    for decl in &file.declarations {
        for field in decl.fields() {
            match &field.desc {
                FieldDesc::FixedScalar { value, width } if bit_width(*value) > *width => {
                    diagnostics.push(
                        Diagnostic::error()
                            .with_code(ErrorCode::FixedValueOutOfRange)
                            .with_message(format!(
                                "fixed value `{}` is larger than maximum value",
                                value
                            ))
                            .with_labels(vec![field.loc.primary()]),
                    )
                }
                FieldDesc::FixedEnum { tag_id, enum_id } => match scope.typedef.get(enum_id) {
                    None => diagnostics.push(
                        Diagnostic::error()
                            .with_code(ErrorCode::E33)
                            .with_message(format!("undeclared type identifier `{}`", enum_id))
                            .with_labels(vec![field.loc.primary()])
                            .with_notes(vec!["hint: expected enum identifier".to_owned()]),
                    ),
                    Some(enum_decl @ Decl { desc: DeclDesc::Enum { tags, .. }, .. }) => {
                        if !tags.iter().any(|tag| tag.id() == tag_id) {
                            diagnostics.push(
                                Diagnostic::error()
                                    .with_code(ErrorCode::E34)
                                    .with_message(format!("undeclared tag identifier `{}`", tag_id))
                                    .with_labels(vec![
                                        field.loc.primary(),
                                        enum_decl.loc.secondary(),
                                    ]),
                            )
                        }
                    }
                    Some(decl) => diagnostics.push(
                        Diagnostic::error()
                            .with_code(ErrorCode::E35)
                            .with_message(format!("invalid type identifier `{}`", enum_id))
                            .with_labels(vec![
                                field.loc.primary(),
                                decl.loc
                                    .secondary()
                                    .with_message(format!("`{}` is declared here", enum_id)),
                            ])
                            .with_notes(vec!["hint: expected enum identifier".to_owned()]),
                    ),
                },
                _ => (),
            }
        }
    }

    diagnostics.err_or(())
}

/// Check payload fields.
/// Raises error diagnostics for the following cases:
///      - duplicate payload field
///      - duplicate payload field size
///      - duplicate body field
///      - duplicate body field size
///      - missing payload field
fn check_payload_fields(file: &parser_ast::File) -> Result<(), Diagnostics> {
    // Check whether the declaration requires a payload field.
    // The payload is required if any child packets declares fields.
    fn requires_payload(file: &parser_ast::File, decl: &parser_ast::Decl) -> bool {
        file.iter_children(decl).any(|child| child.fields().next().is_some())
    }

    let mut diagnostics: Diagnostics = Default::default();
    for decl in &file.declarations {
        let mut payload: Option<&parser_ast::Field> = None;
        for field in decl.fields() {
            match &field.desc {
                FieldDesc::Payload { .. } | FieldDesc::Body { .. } => {
                    if let Some(prev) = payload {
                        diagnostics.push(
                            Diagnostic::error()
                                .with_code(ErrorCode::DuplicatePayloadField)
                                .with_message(format!("duplicate {} field", field.kind()))
                                .with_labels(vec![
                                    field.loc.primary(),
                                    prev.loc.secondary().with_message(format!(
                                        "{} is first declared here",
                                        prev.kind()
                                    )),
                                ]),
                        )
                    } else {
                        payload = Some(field);
                    }
                }
                _ => (),
            }
        }

        if payload.is_none() && requires_payload(file, decl) {
            diagnostics.push(
                Diagnostic::error()
                    .with_code(ErrorCode::MissingPayloadField)
                    .with_message("missing payload field".to_owned())
                    .with_labels(vec![decl.loc.primary()])
                    .with_notes(vec![format!(
                        "hint: one child packet is extending `{}`",
                        decl.id().unwrap()
                    )]),
            )
        }
    }

    diagnostics.err_or(())
}

/// Check array fields.
/// Raises error diagnostics for the following cases:
///      - redundant array field size
fn check_array_fields(file: &parser_ast::File) -> Result<(), Diagnostics> {
    let mut diagnostics: Diagnostics = Default::default();
    for decl in &file.declarations {
        for field in decl.fields() {
            if let FieldDesc::Array { id, size: Some(size), .. } = &field.desc {
                if let Some(size_field) = decl.fields().find(|field| match &field.desc {
                    FieldDesc::Size { field_id, .. } | FieldDesc::Count { field_id, .. } => {
                        field_id == id
                    }
                    _ => false,
                }) {
                    diagnostics.push(
                        Diagnostic::error()
                            .with_code(ErrorCode::RedundantArraySize)
                            .with_message(format!("redundant array {} field", size_field.kind()))
                            .with_labels(vec![
                                size_field.loc.primary(),
                                field
                                    .loc
                                    .secondary()
                                    .with_message(format!("`{}` has constant size {}", id, size)),
                            ]),
                    )
                }
            }
        }
    }

    diagnostics.err_or(())
}

/// Check padding fields.
/// Raises error diagnostics for the following cases:
///      - padding field not following an array field
fn check_padding_fields(file: &parser_ast::File) -> Result<(), Diagnostics> {
    let mut diagnostics: Diagnostics = Default::default();
    for decl in &file.declarations {
        let mut previous_is_array = false;
        for field in decl.fields() {
            match &field.desc {
                FieldDesc::Padding { .. } if !previous_is_array => diagnostics.push(
                    Diagnostic::error()
                        .with_code(ErrorCode::InvalidPaddingField)
                        .with_message("padding field does not follow an array field".to_owned())
                        .with_labels(vec![field.loc.primary()]),
                ),
                FieldDesc::Array { .. } => previous_is_array = true,
                _ => previous_is_array = false,
            }
        }
    }

    diagnostics.err_or(())
}

/// Check checksum fields.
/// Raises error diagnostics for the following cases:
///      - checksum field precedes checksum start
///      - undeclared checksum field
///      - invalid checksum field
fn check_checksum_fields(
    _file: &parser_ast::File,
    _scope: &Scope<parser_ast::Annotation>,
) -> Result<(), Diagnostics> {
    // TODO
    Ok(())
}

/// Check correct definition of packet sizes.
/// Annotate fields and declarations with the size in bits.
fn compute_field_sizes(file: &parser_ast::File) -> ast::File {
    fn annotate_decl(
        decl: &parser_ast::Decl,
        scope: &HashMap<String, ast::DeclAnnotation>,
    ) -> ast::Decl {
        // Annotate the declaration fields.
        // Add the padding information to the fields in the same pass.
        let mut decl = decl.annotate(Default::default(), |fields| {
            let mut fields: Vec<_> =
                fields.iter().map(|field| annotate_field(decl, field, scope)).collect();
            let mut padding = None;
            for field in fields.iter_mut().rev() {
                field.annot.padded_size = padding;
                padding = match &field.desc {
                    FieldDesc::Padding { size } => Some(*size),
                    _ => None,
                };
            }
            fields
        });

        // Compute the declaration annotation.
        decl.annot = match &decl.desc {
            DeclDesc::Packet { fields, .. }
            | DeclDesc::Struct { fields, .. }
            | DeclDesc::Group { fields, .. } => {
                let mut size = decl
                    .parent_id()
                    .and_then(|parent_id| scope.get(parent_id))
                    .map(|annot| annot.size)
                    .unwrap_or(ast::Size::Static(0));
                let mut payload_size = ast::Size::Static(0);
                for field in fields {
                    match &field.desc {
                        FieldDesc::Payload { .. } | FieldDesc::Body { .. } => {
                            payload_size = field.annot.size
                        }
                        _ => {
                            size = size
                                + match field.annot.padded_size {
                                    Some(padding) => ast::Size::Static(8 * padding),
                                    None => field.annot.size,
                                }
                        }
                    }
                }
                ast::DeclAnnotation { size, payload_size }
            }
            DeclDesc::Enum { width, .. }
            | DeclDesc::Checksum { width, .. }
            | DeclDesc::CustomField { width: Some(width), .. } => ast::DeclAnnotation {
                size: ast::Size::Static(*width),
                payload_size: ast::Size::Static(0),
            },
            DeclDesc::CustomField { width: None, .. } => {
                ast::DeclAnnotation { size: ast::Size::Dynamic, payload_size: ast::Size::Static(0) }
            }
            DeclDesc::Test { .. } => ast::DeclAnnotation {
                size: ast::Size::Static(0),
                payload_size: ast::Size::Static(0),
            },
        };
        decl
    }

    fn annotate_field(
        decl: &parser_ast::Decl,
        field: &parser_ast::Field,
        scope: &HashMap<String, ast::DeclAnnotation>,
    ) -> ast::Field {
        field.annotate(match &field.desc {
            FieldDesc::Checksum { .. } | FieldDesc::Padding { .. } => {
                ast::FieldAnnotation::new(ast::Size::Static(0))
            }
            FieldDesc::Size { width, .. }
            | FieldDesc::Count { width, .. }
            | FieldDesc::ElementSize { width, .. }
            | FieldDesc::FixedScalar { width, .. }
            | FieldDesc::Reserved { width }
            | FieldDesc::Scalar { width, .. } => {
                ast::FieldAnnotation::new(ast::Size::Static(*width))
            }
            FieldDesc::Body | FieldDesc::Payload { .. } => {
                let has_payload_size = decl.fields().any(|field| match &field.desc {
                    FieldDesc::Size { field_id, .. } => {
                        field_id == "_body_" || field_id == "_payload_"
                    }
                    _ => false,
                });
                ast::FieldAnnotation::new(if has_payload_size {
                    ast::Size::Dynamic
                } else {
                    ast::Size::Unknown
                })
            }
            FieldDesc::Typedef { type_id, .. }
            | FieldDesc::FixedEnum { enum_id: type_id, .. }
            | FieldDesc::Group { group_id: type_id, .. } => {
                let type_annot = scope.get(type_id).unwrap();
                ast::FieldAnnotation::new(type_annot.size + type_annot.payload_size)
            }
            FieldDesc::Array { width: Some(width), size: Some(size), .. } => {
                ast::FieldAnnotation::new(ast::Size::Static(*size * *width))
            }
            FieldDesc::Array { width: None, size: Some(size), type_id: Some(type_id), .. } => {
                let type_annot = scope.get(type_id).unwrap();
                ast::FieldAnnotation::new((type_annot.size + type_annot.payload_size) * *size)
            }
            FieldDesc::Array { id, size: None, .. } => {
                // The element does not matter when the size of the array is
                // not static. The array size depends on there being a count
                // or size field or not.
                let has_array_size = decl.fields().any(|field| match &field.desc {
                    FieldDesc::Size { field_id, .. } | FieldDesc::Count { field_id, .. } => {
                        field_id == id
                    }
                    _ => false,
                });
                ast::FieldAnnotation::new(if has_array_size {
                    ast::Size::Dynamic
                } else {
                    ast::Size::Unknown
                })
            }
            FieldDesc::Array { .. } => unreachable!(),
        })
    }

    // Construct a scope mapping typedef identifiers to decl annotations.
    let mut scope = HashMap::new();

    // Annotate declarations.
    let mut declarations = Vec::new();
    for decl in file.declarations.iter() {
        let decl = annotate_decl(decl, &scope);
        if let Some(id) = decl.id() {
            scope.insert(id.to_string(), decl.annot.clone());
        }
        declarations.push(decl);
    }

    File {
        version: file.version.clone(),
        file: file.file,
        comments: file.comments.clone(),
        endianness: file.endianness,
        declarations,
    }
}

/// Inline group fields and remove group declarations.
fn inline_groups(file: &mut ast::File) -> Result<(), Diagnostics> {
    fn inline_fields<'a>(
        fields: impl Iterator<Item = &'a ast::Field>,
        groups: &HashMap<String, ast::Decl>,
        constraints: &HashMap<String, Constraint>,
    ) -> Vec<ast::Field> {
        fields
            .flat_map(|field| match &field.desc {
                FieldDesc::Group { group_id, constraints: group_constraints } => {
                    let mut constraints = constraints.clone();
                    constraints.extend(
                        group_constraints
                            .iter()
                            .map(|constraint| (constraint.id.clone(), constraint.clone())),
                    );
                    inline_fields(groups.get(group_id).unwrap().fields(), groups, &constraints)
                }
                FieldDesc::Scalar { id, width } if constraints.contains_key(id) => {
                    vec![ast::Field {
                        desc: FieldDesc::FixedScalar {
                            width: *width,
                            value: constraints.get(id).unwrap().value.unwrap(),
                        },
                        loc: field.loc,
                        annot: field.annot.clone(),
                    }]
                }
                FieldDesc::Typedef { id, type_id, .. } if constraints.contains_key(id) => {
                    vec![ast::Field {
                        desc: FieldDesc::FixedEnum {
                            enum_id: type_id.clone(),
                            tag_id: constraints
                                .get(id)
                                .and_then(|constraint| constraint.tag_id.clone())
                                .unwrap(),
                        },
                        loc: field.loc,
                        annot: field.annot.clone(),
                    }]
                }
                _ => vec![field.clone()],
            })
            .collect()
    }

    let groups = utils::drain_filter(&mut file.declarations, |decl| {
        matches!(&decl.desc, DeclDesc::Group { .. })
    })
    .into_iter()
    .map(|decl| (decl.id().unwrap().to_owned(), decl))
    .collect::<HashMap<String, _>>();

    for decl in file.declarations.iter_mut() {
        match &mut decl.desc {
            DeclDesc::Packet { fields, .. } | DeclDesc::Struct { fields, .. } => {
                *fields = inline_fields(fields.iter(), &groups, &HashMap::new())
            }
            _ => (),
        }
    }

    Ok(())
}

/// Analyzer entry point, produces a new AST with annotations resulting
/// from the analysis.
pub fn analyze(file: &parser_ast::File) -> Result<ast::File, Diagnostics> {
    let scope = Scope::new(file)?;
    check_decl_identifiers(file, &scope)?;
    check_field_identifiers(file)?;
    check_enum_declarations(file)?;
    check_constraints(file, &scope)?;
    check_size_fields(file)?;
    check_fixed_fields(file, &scope)?;
    check_payload_fields(file)?;
    check_array_fields(file)?;
    check_padding_fields(file)?;
    check_checksum_fields(file, &scope)?;
    let mut file = compute_field_sizes(file);
    inline_groups(&mut file)?;
    Ok(file)
}

#[cfg(test)]
mod test {
    use crate::analyzer;
    use crate::ast::*;
    use crate::parser::parse_inline;
    use codespan_reporting::term::termcolor;

    use googletest::prelude::{assert_that, eq};

    macro_rules! raises {
        ($code:ident, $text:literal) => {{
            let mut db = SourceDatabase::new();
            let file = parse_inline(&mut db, "stdin".to_owned(), $text.to_owned())
                .expect("parsing failure");
            let result = analyzer::analyze(&file);
            assert!(matches!(result, Err(_)));
            let diagnostics = result.err().unwrap();
            let mut buffer = termcolor::Buffer::no_color();
            let _ = diagnostics.emit(&db, &mut buffer);
            println!("{}", std::str::from_utf8(buffer.as_slice()).unwrap());
            assert_eq!(diagnostics.diagnostics.len(), 1);
            assert_eq!(diagnostics.diagnostics[0].code, Some(analyzer::ErrorCode::$code.into()));
        }};
    }

    macro_rules! valid {
        ($text:literal) => {{
            let mut db = SourceDatabase::new();
            let file = parse_inline(&mut db, "stdin".to_owned(), $text.to_owned())
                .expect("parsing failure");
            assert!(analyzer::analyze(&file).is_ok());
        }};
    }

    #[test]
    fn test_e1() {
        raises!(
            DuplicateDeclIdentifier,
            r#"
            little_endian_packets
            struct A { }
            packet A { }
            "#
        );

        raises!(
            DuplicateDeclIdentifier,
            r#"
            little_endian_packets
            struct A { }
            enum A : 8 { X = 0, Y = 1 }
            "#
        );
    }

    #[test]
    fn test_e2() {
        raises!(
            RecursiveDecl,
            r#"
            little_endian_packets
            packet A : A { }
            "#
        );

        raises!(
            RecursiveDecl,
            r#"
            little_endian_packets
            packet A : B { }
            packet B : A { }
            "#
        );

        raises!(
            RecursiveDecl,
            r#"
            little_endian_packets
            struct B { x : B }
            "#
        );

        raises!(
            RecursiveDecl,
            r#"
            little_endian_packets
            struct B { x : B[8] }
            "#
        );

        raises!(
            RecursiveDecl,
            r#"
            little_endian_packets
            group C { C { x = 1 } }
            "#
        );
    }

    #[test]
    fn test_e3() {
        raises!(
            UndeclaredGroupIdentifier,
            r#"
        little_endian_packets
        packet A { C { x = 1 } }
        "#
        );
    }

    #[test]
    fn test_e4() {
        raises!(
            InvalidGroupIdentifier,
            r#"
        little_endian_packets
        struct C { x : 8 }
        packet A { C { x = 1 } }
        "#
        );
    }

    #[test]
    fn test_e5() {
        raises!(
            UndeclaredTypeIdentifier,
            r#"
        little_endian_packets
        packet A { x : B }
        "#
        );

        raises!(
            UndeclaredTypeIdentifier,
            r#"
        little_endian_packets
        packet A { x : B[] }
        "#
        );
    }

    #[test]
    fn test_e6() {
        raises!(
            InvalidTypeIdentifier,
            r#"
        little_endian_packets
        packet A { x : 8 }
        packet B { x : A }
        "#
        );

        raises!(
            InvalidTypeIdentifier,
            r#"
        little_endian_packets
        packet A { x : 8 }
        packet B { x : A[] }
        "#
        );
    }

    #[test]
    fn test_e7() {
        raises!(
            UndeclaredParentIdentifier,
            r#"
        little_endian_packets
        packet A : B { }
        "#
        );

        raises!(
            UndeclaredParentIdentifier,
            r#"
        little_endian_packets
        struct A : B { }
        "#
        );
    }

    #[test]
    fn test_e8() {
        raises!(
            InvalidParentIdentifier,
            r#"
        little_endian_packets
        struct A { }
        packet B : A { }
        "#
        );

        raises!(
            InvalidParentIdentifier,
            r#"
        little_endian_packets
        packet A { }
        struct B : A { }
        "#
        );

        raises!(
            InvalidParentIdentifier,
            r#"
        little_endian_packets
        group A { x : 1 }
        struct B : A { }
        "#
        );
    }

    #[ignore]
    #[test]
    fn test_e9() {
        raises!(
            UndeclaredTestIdentifier,
            r#"
        little_endian_packets
        test A { "aaa" }
        "#
        );
    }

    #[ignore]
    #[test]
    fn test_e10() {
        raises!(
            InvalidTestIdentifier,
            r#"
        little_endian_packets
        struct A { }
        test A { "aaa" }
        "#
        );

        raises!(
            InvalidTestIdentifier,
            r#"
        little_endian_packets
        group A { x : 8 }
        test A { "aaa" }
        "#
        );
    }

    #[test]
    fn test_e11() {
        raises!(
            DuplicateFieldIdentifier,
            r#"
        little_endian_packets
        enum A : 8 { X = 0 }
        struct B {
            x : 8,
            x : A
        }
        "#
        );

        raises!(
            DuplicateFieldIdentifier,
            r#"
        little_endian_packets
        enum A : 8 { X = 0 }
        packet B {
            x : 8,
            x : A[]
        }
        "#
        );
    }

    #[test]
    fn test_e12() {
        raises!(
            DuplicateTagIdentifier,
            r#"
        little_endian_packets
        enum A : 8 {
            X = 0,
            X = 1,
        }
        "#
        );

        raises!(
            DuplicateTagIdentifier,
            r#"
        little_endian_packets
        enum A : 8 {
            X = 0,
            A = 1..10 {
                X = 1,
            }
        }
        "#
        );

        raises!(
            DuplicateTagIdentifier,
            r#"
        little_endian_packets
        enum A : 8 {
            X = 0,
            X = 1..10,
        }
        "#
        );

        raises!(
            DuplicateTagIdentifier,
            r#"
        little_endian_packets
        enum A : 8 {
            X = 0,
            X = ..,
        }
        "#
        );
    }

    #[test]
    fn test_e13() {
        raises!(
            DuplicateTagValue,
            r#"
        little_endian_packets
        enum A : 8 {
            X = 0,
            Y = 0,
        }
        "#
        );

        raises!(
            DuplicateTagValue,
            r#"
        little_endian_packets
        enum A : 8 {
            A = 1..10 {
                X = 1,
                Y = 1,
            }
        }
        "#
        );
    }

    #[test]
    fn test_e14() {
        raises!(
            InvalidTagValue,
            r#"
        little_endian_packets
        enum A : 8 {
            X = 256,
        }
        "#
        );

        raises!(
            InvalidTagValue,
            r#"
        little_endian_packets
        enum A : 8 {
            A = 0,
            X = 10..20 {
                B = 1,
            },
        }
        "#
        );
    }

    #[test]
    fn test_e15() {
        raises!(
            UndeclaredConstraintIdentifier,
            r#"
        little_endian_packets
        packet A { }
        packet B : A (x = 1) { }
        "#
        );

        raises!(
            UndeclaredConstraintIdentifier,
            r#"
        little_endian_packets
        group A { x : 8 }
        packet B {
            A { y = 1 }
        }
        "#
        );
    }

    #[test]
    fn test_e16() {
        raises!(
            InvalidConstraintIdentifier,
            r#"
        little_endian_packets
        packet A { x : 8[] }
        packet B : A (x = 1) { }
        "#
        );

        raises!(
            InvalidConstraintIdentifier,
            r#"
        little_endian_packets
        group A { x : 8[] }
        packet B {
            A { x = 1 }
        }
        "#
        );
    }

    #[test]
    fn test_e17() {
        raises!(
            E17,
            r#"
        little_endian_packets
        packet A { x : 8 }
        packet B : A (x = X) { }
        "#
        );

        raises!(
            E17,
            r#"
        little_endian_packets
        group A { x : 8 }
        packet B {
            A { x = X }
        }
        "#
        );
    }

    #[test]
    fn test_e18() {
        raises!(
            ConstraintValueOutOfRange,
            r#"
        little_endian_packets
        packet A { x : 8 }
        packet B : A (x = 256) { }
        "#
        );

        raises!(
            ConstraintValueOutOfRange,
            r#"
        little_endian_packets
        group A { x : 8 }
        packet B {
            A { x = 256 }
        }
        "#
        );
    }

    #[test]
    fn test_e19() {
        raises!(
            E19,
            r#"
        little_endian_packets
        enum C : 8 { X = 0 }
        packet A { x : C }
        packet B : A (x = 0) { }
        "#
        );

        raises!(
            E19,
            r#"
        little_endian_packets
        enum C : 8 { X = 0 }
        group A { x : C }
        packet B {
            A { x = 0 }
        }
        "#
        );
    }

    #[test]
    fn test_e20() {
        raises!(
            E20,
            r#"
        little_endian_packets
        enum C : 8 { X = 0 }
        packet A { x : C }
        packet B : A (x = Y) { }
        "#
        );

        raises!(
            E20,
            r#"
        little_endian_packets
        enum C : 8 { X = 0 }
        group A { x : C }
        packet B {
            A { x = Y }
        }
        "#
        );
    }

    #[test]
    fn test_e21() {
        raises!(
            E21,
            r#"
        little_endian_packets
        struct C { }
        packet A { x : C }
        packet B : A (x = 0) { }
        "#
        );

        raises!(
            E21,
            r#"
        little_endian_packets
        struct C { }
        group A { x : C }
        packet B {
            A { x = 0 }
        }
        "#
        );
    }

    #[test]
    fn test_e22() {
        raises!(
            DuplicateConstraintIdentifier,
            r#"
        little_endian_packets
        packet A { x: 8 }
        packet B : A (x = 0, x = 1) { }
        "#
        );

        raises!(
            DuplicateConstraintIdentifier,
            r#"
        little_endian_packets
        packet A { x: 8 }
        packet B : A (x = 0) { }
        packet C : B (x = 1) { }
        "#
        );

        raises!(
            DuplicateConstraintIdentifier,
            r#"
        little_endian_packets
        group A { x : 8 }
        packet B {
            A { x = 0, x = 1 }
        }
        "#
        );
    }

    #[test]
    fn test_e23() {
        raises!(
            DuplicateSizeField,
            r#"
        little_endian_packets
        struct A {
            _size_ (_payload_) : 8,
            _size_ (_payload_) : 8,
            _payload_,
        }
        "#
        );

        raises!(
            DuplicateSizeField,
            r#"
        little_endian_packets
        struct A {
            _count_ (x) : 8,
            _size_ (x) : 8,
            x: 8[],
        }
        "#
        );
    }

    #[test]
    fn test_e24() {
        raises!(
            UndeclaredSizeIdentifier,
            r#"
        little_endian_packets
        struct A {
            _size_ (x) : 8,
        }
        "#
        );

        raises!(
            UndeclaredSizeIdentifier,
            r#"
        little_endian_packets
        struct A {
            _size_ (_payload_) : 8,
        }
        "#
        );
    }

    #[test]
    fn test_e25() {
        raises!(
            InvalidSizeIdentifier,
            r#"
        little_endian_packets
        enum B : 8 { X = 0 }
        struct A {
            _size_ (x) : 8,
            x : B,
        }
        "#
        );
    }

    #[test]
    fn test_e26() {
        raises!(
            DuplicateCountField,
            r#"
        little_endian_packets
        struct A {
            _size_ (x) : 8,
            _count_ (x) : 8,
            x: 8[],
        }
        "#
        );
    }

    #[test]
    fn test_e27() {
        raises!(
            UndeclaredCountIdentifier,
            r#"
        little_endian_packets
        struct A {
            _count_ (x) : 8,
        }
        "#
        );
    }

    #[test]
    fn test_e28() {
        raises!(
            InvalidCountIdentifier,
            r#"
        little_endian_packets
        enum B : 8 { X = 0 }
        struct A {
            _count_ (x) : 8,
            x : B,
        }
        "#
        );
    }

    #[test]
    fn test_e29() {
        raises!(
            DuplicateElementSizeField,
            r#"
        little_endian_packets
        struct A {
            _elementsize_ (x) : 8,
            _elementsize_ (x) : 8,
            x: 8[],
        }
        "#
        );
    }

    #[test]
    fn test_e30() {
        raises!(
            UndeclaredElementSizeIdentifier,
            r#"
        little_endian_packets
        struct A {
            _elementsize_ (x) : 8,
        }
        "#
        );
    }

    #[test]
    fn test_e31() {
        raises!(
            InvalidElementSizeIdentifier,
            r#"
        little_endian_packets
        enum B : 8 { X = 0 }
        struct A {
            _elementsize_ (x) : 8,
            x : B,
        }
        "#
        );
    }

    #[test]
    fn test_e32() {
        raises!(
            FixedValueOutOfRange,
            r#"
        little_endian_packets
        struct A {
            _fixed_ = 256 : 8,
        }
        "#
        );
    }

    #[test]
    fn test_e33() {
        raises!(
            E33,
            r#"
        little_endian_packets
        struct A {
            _fixed_ = X : B,
        }
        "#
        );
    }

    #[test]
    fn test_e34() {
        raises!(
            E34,
            r#"
        little_endian_packets
        enum B : 8 { X = 0 }
        struct A {
            _fixed_ = Y : B,
        }
        "#
        );
    }

    #[test]
    fn test_e35() {
        raises!(
            E35,
            r#"
        little_endian_packets
        struct B { }
        struct A {
            _fixed_ = X : B,
        }
        "#
        );
    }

    #[test]
    fn test_e36() {
        raises!(
            DuplicatePayloadField,
            r#"
        little_endian_packets
        packet A {
            _payload_,
            _body_,
        }
        "#
        );

        raises!(
            DuplicatePayloadField,
            r#"
        little_endian_packets
        packet A {
            _body_,
            _payload_,
        }
        "#
        );
    }

    #[test]
    fn test_e37() {
        raises!(
            MissingPayloadField,
            r#"
        little_endian_packets
        packet A { x : 8 }
        packet B : A { y : 8 }
        "#
        );

        raises!(
            MissingPayloadField,
            r#"
        little_endian_packets
        packet A { x : 8 }
        packet B : A (x = 0) { }
        packet C : B { y : 8 }
        "#
        );
    }

    #[test]
    fn test_e38() {
        raises!(
            RedundantArraySize,
            r#"
        little_endian_packets
        packet A {
            _size_ (x) : 8,
            x : 8[8]
        }
        "#
        );

        raises!(
            RedundantArraySize,
            r#"
        little_endian_packets
        packet A {
            _count_ (x) : 8,
            x : 8[8]
        }
        "#
        );
    }

    #[test]
    fn test_e39() {
        raises!(
            InvalidPaddingField,
            r#"
        little_endian_packets
        packet A {
            _padding_ [16],
            x : 8[]
        }
        "#
        );

        raises!(
            InvalidPaddingField,
            r#"
        little_endian_packets
        enum A : 8 { X = 0 }
        packet B {
            x : A,
            _padding_ [16]
        }
        "#
        );

        valid!(
            r#"
        little_endian_packets
        packet A {
            x : 8[],
            _padding_ [16]
        }
        "#
        );
    }

    #[test]
    fn test_e40() {
        raises!(
            InvalidTagRange,
            r#"
        little_endian_packets
        enum A : 8 {
            X = 4..2,
        }
        "#
        );

        raises!(
            InvalidTagRange,
            r#"
        little_endian_packets
        enum A : 8 {
            X = 2..2,
        }
        "#
        );

        raises!(
            InvalidTagRange,
            r#"
        little_endian_packets
        enum A : 8 {
            X = 258..259,
        }
        "#
        );
    }

    #[test]
    fn test_e41() {
        raises!(
            DuplicateTagRange,
            r#"
        little_endian_packets
        enum A : 8 {
            X = 0..15,
            Y = 8..31,
        }
        "#
        );

        raises!(
            DuplicateTagRange,
            r#"
        little_endian_packets
        enum A : 8 {
            X = 8..31,
            Y = 0..15,
        }
        "#
        );

        raises!(
            DuplicateTagRange,
            r#"
        little_endian_packets
        enum A : 8 {
            X = 1..9,
            Y = 9..11,
        }
        "#
        );
    }

    #[test]
    fn test_e42() {
        raises!(
            E42,
            r#"
        little_endian_packets
        enum C : 8 { X = 0..15 }
        packet A { x : C }
        packet B : A (x = X) { }
        "#
        );

        raises!(
            E42,
            r#"
        little_endian_packets
        enum C : 8 { X = 0..15 }
        group A { x : C }
        packet B {
            A { x = X }
        }
        "#
        );
    }

    #[test]
    fn test_e43() {
        raises!(
            E43,
            r#"
        little_endian_packets
        enum A : 8 {
            A = 0,
            B = 1,
            X = 1..15,
        }
        "#
        );
    }

    #[test]
    fn test_e44() {
        raises!(
            DuplicateDefaultTag,
            r#"
        little_endian_packets
        enum A : 8 {
            A = 0,
            X = ..,
            B = 1,
            Y = ..,
        }
        "#
        );
    }

    #[test]
    fn test_enum_declaration() {
        valid!(
            r#"
        little_endian_packets
        enum A : 7 {
            X = 0,
            Y = 1,
            Z = 127,
        }
        "#
        );

        valid!(
            r#"
        little_endian_packets
        enum A : 7 {
            A = 50..100 {
                X = 50,
                Y = 100,
            },
            Z = 101,
        }
        "#
        );

        valid!(
            r#"
        little_endian_packets
        enum A : 7 {
            A = 50..100,
            X = 101,
        }
        "#
        );

        valid!(
            r#"
        little_endian_packets
        enum A : 7 {
            A = 50..100,
            X = 101,
            UNKNOWN = ..,
        }
        "#
        );
    }

    use analyzer::ast::Size;
    use Size::*;

    #[derive(Debug, PartialEq, Eq)]
    struct Annotations {
        size: Size,
        payload_size: Size,
        fields: Vec<Size>,
    }

    fn annotations(text: &str) -> Vec<Annotations> {
        let mut db = SourceDatabase::new();
        let file =
            parse_inline(&mut db, "stdin".to_owned(), text.to_owned()).expect("parsing failure");
        let file = analyzer::analyze(&file).expect("analyzer failure");
        file.declarations
            .iter()
            .map(|decl| Annotations {
                size: decl.annot.size,
                payload_size: decl.annot.payload_size,
                fields: decl.fields().map(|field| field.annot.size).collect(),
            })
            .collect()
    }

    #[test]
    fn test_bitfield_annotations() {
        assert_that!(
            annotations(
                r#"
        little_endian_packets
        enum E : 6 { X=0, Y=1 }
        packet A {
            a : 14,
            b : E,
            _reserved_ : 3,
            _fixed_ = 3 : 4,
            _fixed_ = X : E,
            _size_(_payload_) : 7,
            _payload_,
        }
        "#
            ),
            eq(vec![
                Annotations { size: Static(6), payload_size: Static(0), fields: vec![] },
                Annotations {
                    size: Static(40),
                    payload_size: Dynamic,
                    fields: vec![
                        Static(14),
                        Static(6),
                        Static(3),
                        Static(4),
                        Static(6),
                        Static(7),
                        Dynamic
                    ]
                },
            ])
        )
    }

    #[test]
    fn test_typedef_annotations() {
        // Struct with constant size.
        assert_that!(
            annotations(
                r#"
        little_endian_packets
        struct S {
            a: 8[4],
        }
        packet A {
            a: 16,
            s: S,
        }
        "#
            ),
            eq(vec![
                Annotations { size: Static(32), payload_size: Static(0), fields: vec![Static(32)] },
                Annotations {
                    size: Static(48),
                    payload_size: Static(0),
                    fields: vec![Static(16), Static(32)]
                },
            ])
        );

        // Struct with dynamic size.
        assert_that!(
            annotations(
                r#"
        little_endian_packets
        struct S {
            _size_ (a) : 8,
            a: 8[],
        }
        packet A {
            a: 16,
            s: S,
        }
        "#
            ),
            eq(vec![
                Annotations {
                    size: Dynamic,
                    payload_size: Static(0),
                    fields: vec![Static(8), Dynamic]
                },
                Annotations {
                    size: Dynamic,
                    payload_size: Static(0),
                    fields: vec![Static(16), Dynamic]
                },
            ])
        );

        // Struct with unknown size.
        assert_that!(
            annotations(
                r#"
        little_endian_packets
        struct S {
            a: 8[],
        }
        packet A {
            a: 16,
            s: S,
        }
        "#
            ),
            eq(vec![
                Annotations { size: Unknown, payload_size: Static(0), fields: vec![Unknown] },
                Annotations {
                    size: Unknown,
                    payload_size: Static(0),
                    fields: vec![Static(16), Unknown]
                },
            ])
        );
    }

    #[test]
    fn test_array_annotations() {
        // Array with constant size element and constant count.
        assert_that!(
            annotations(
                r#"
        little_endian_packets
        enum E : 8 { X=0, Y=1 }
        packet A {
            a: E[8],
        }
        "#
            ),
            eq(vec![
                Annotations { size: Static(8), payload_size: Static(0), fields: vec![] },
                Annotations { size: Static(64), payload_size: Static(0), fields: vec![Static(64)] },
            ])
        );

        // Array with dynamic size element and constant count.
        assert_that!(
            annotations(
                r#"
        little_endian_packets
        struct S { _size_(a): 8, a: 8[] }
        packet A {
            a: S[8],
        }
        "#
            ),
            eq(vec![
                Annotations {
                    size: Dynamic,
                    payload_size: Static(0),
                    fields: vec![Static(8), Dynamic]
                },
                Annotations { size: Dynamic, payload_size: Static(0), fields: vec![Dynamic] },
            ])
        );

        // Array with constant size element and dynamic size.
        assert_that!(
            annotations(
                r#"
        little_endian_packets
        struct S { a: 7, _reserved_: 1 }
        packet A {
            _size_ (a) : 8,
            a: S[],
        }
        "#
            ),
            eq(vec![
                Annotations {
                    size: Static(8),
                    payload_size: Static(0),
                    fields: vec![Static(7), Static(1)]
                },
                Annotations {
                    size: Dynamic,
                    payload_size: Static(0),
                    fields: vec![Static(8), Dynamic]
                },
            ])
        );

        // Array with dynamic size element and dynamic size.
        assert_that!(
            annotations(
                r#"
        little_endian_packets
        struct S { _size_(a): 8, a: 8[] }
        packet A {
            _size_ (a) : 8,
            a: S[],
        }
        "#
            ),
            eq(vec![
                Annotations {
                    size: Dynamic,
                    payload_size: Static(0),
                    fields: vec![Static(8), Dynamic]
                },
                Annotations {
                    size: Dynamic,
                    payload_size: Static(0),
                    fields: vec![Static(8), Dynamic]
                },
            ])
        );

        // Array with constant size element and dynamic count.
        assert_that!(
            annotations(
                r#"
        little_endian_packets
        struct S { a: 7, _reserved_: 1 }
        packet A {
            _count_ (a) : 8,
            a: S[],
        }
        "#
            ),
            eq(vec![
                Annotations {
                    size: Static(8),
                    payload_size: Static(0),
                    fields: vec![Static(7), Static(1)]
                },
                Annotations {
                    size: Dynamic,
                    payload_size: Static(0),
                    fields: vec![Static(8), Dynamic]
                },
            ])
        );

        // Array with dynamic size element and dynamic count.
        assert_that!(
            annotations(
                r#"
        little_endian_packets
        struct S { _size_(a): 8, a: 8[] }
        packet A {
            _count_ (a) : 8,
            a: S[],
        }
        "#
            ),
            eq(vec![
                Annotations {
                    size: Dynamic,
                    payload_size: Static(0),
                    fields: vec![Static(8), Dynamic]
                },
                Annotations {
                    size: Dynamic,
                    payload_size: Static(0),
                    fields: vec![Static(8), Dynamic]
                },
            ])
        );

        // Array with constant size element and unknown size.
        assert_that!(
            annotations(
                r#"
        little_endian_packets
        struct S { a: 7, _fixed_ = 1 : 1 }
        packet A {
            a: S[],
        }
        "#
            ),
            eq(vec![
                Annotations {
                    size: Static(8),
                    payload_size: Static(0),
                    fields: vec![Static(7), Static(1)]
                },
                Annotations { size: Unknown, payload_size: Static(0), fields: vec![Unknown] },
            ])
        );

        // Array with dynamic size element and unknown size.
        assert_that!(
            annotations(
                r#"
        little_endian_packets
        struct S { _size_(a): 8, a: 8[] }
        packet A {
            a: S[],
        }
        "#
            ),
            eq(vec![
                Annotations {
                    size: Dynamic,
                    payload_size: Static(0),
                    fields: vec![Static(8), Dynamic]
                },
                Annotations { size: Unknown, payload_size: Static(0), fields: vec![Unknown] },
            ])
        );

        // Array with padded size.
        assert_that!(
            annotations(
                r#"
        little_endian_packets
        struct S {
            _count_(a): 40,
            a: 16[],
        }
        packet A {
            a: S[],
            _padding_ [128],
        }
        "#
            ),
            eq(vec![
                Annotations {
                    size: Dynamic,
                    payload_size: Static(0),
                    fields: vec![Static(40), Dynamic]
                },
                Annotations {
                    size: Static(1024),
                    payload_size: Static(0),
                    fields: vec![Unknown, Static(0)]
                },
            ])
        );
    }

    #[test]
    fn test_payload_annotations() {
        // Payload with dynamic size.
        assert_that!(
            annotations(
                r#"
        little_endian_packets
        packet A {
            _size_(_payload_) : 8,
            _payload_
        }
        "#
            ),
            eq(vec![Annotations {
                size: Static(8),
                payload_size: Dynamic,
                fields: vec![Static(8), Dynamic]
            },])
        );

        // Payload with unknown size.
        assert_that!(
            annotations(
                r#"
        little_endian_packets
        packet A {
            a : 8,
            _payload_
        }
        "#
            ),
            eq(vec![Annotations {
                size: Static(8),
                payload_size: Unknown,
                fields: vec![Static(8), Unknown]
            },])
        );
    }

    #[test]
    fn test_body_annotations() {
        // Body with dynamic size.
        assert_that!(
            annotations(
                r#"
        little_endian_packets
        packet A {
            _size_(_body_) : 8,
            _body_
        }
        "#
            ),
            eq(vec![Annotations {
                size: Static(8),
                payload_size: Dynamic,
                fields: vec![Static(8), Dynamic]
            },])
        );

        // Body with unknown size.
        assert_that!(
            annotations(
                r#"
        little_endian_packets
        packet A {
            a : 8,
            _body_
        }
        "#
            ),
            eq(vec![Annotations {
                size: Static(8),
                payload_size: Unknown,
                fields: vec![Static(8), Unknown]
            },])
        );
    }

    #[test]
    fn test_decl_annotations() {
        // Test parent with constant size.
        assert_that!(
            annotations(
                r#"
        little_endian_packets
        packet A {
            a: 2,
            _reserved_: 6,
            _payload_
        }
        packet B : A {
            b: 8,
        }
        "#
            ),
            eq(vec![
                Annotations {
                    size: Static(8),
                    payload_size: Unknown,
                    fields: vec![Static(2), Static(6), Unknown]
                },
                Annotations { size: Static(16), payload_size: Static(0), fields: vec![Static(8)] },
            ])
        );

        // Test parent with dynamic size.
        assert_that!(
            annotations(
                r#"
        little_endian_packets
        packet A {
            _size_(a) : 8,
            a: 8[],
            _size_(_payload_) : 8,
            _payload_
        }
        packet B : A {
            b: 8,
        }
        "#
            ),
            eq(vec![
                Annotations {
                    size: Dynamic,
                    payload_size: Dynamic,
                    fields: vec![Static(8), Dynamic, Static(8), Dynamic]
                },
                Annotations { size: Dynamic, payload_size: Static(0), fields: vec![Static(8)] },
            ])
        );

        // Test parent with unknown size.
        assert_that!(
            annotations(
                r#"
        little_endian_packets
        packet A {
            _size_(_payload_) : 8,
            a: 8[],
            _payload_
        }
        packet B : A {
            b: 8,
        }
        "#
            ),
            eq(vec![
                Annotations {
                    size: Unknown,
                    payload_size: Dynamic,
                    fields: vec![Static(8), Unknown, Dynamic]
                },
                Annotations { size: Unknown, payload_size: Static(0), fields: vec![Static(8)] },
            ])
        );
    }

    fn desugar(text: &str) -> analyzer::ast::File {
        let mut db = SourceDatabase::new();
        let file =
            parse_inline(&mut db, "stdin".to_owned(), text.to_owned()).expect("parsing failure");
        analyzer::analyze(&file).expect("analyzer failure")
    }

    #[test]
    fn test_inline_groups() {
        assert_eq!(
            desugar(
                r#"
        little_endian_packets
        enum E : 8 { X=0, Y=1 }
        group G {
            a: 8,
            b: E,
        }
        packet A {
            G { }
        }
        "#
            ),
            desugar(
                r#"
        little_endian_packets
        enum E : 8 { X=0, Y=1 }
        packet A {
            a: 8,
            b: E,
        }
        "#
            )
        );

        assert_eq!(
            desugar(
                r#"
        little_endian_packets
        enum E : 8 { X=0, Y=1 }
        group G {
            a: 8,
            b: E,
        }
        packet A {
            G { a=1, b=X }
        }
        "#
            ),
            desugar(
                r#"
        little_endian_packets
        enum E : 8 { X=0, Y=1 }
        packet A {
            _fixed_ = 1: 8,
            _fixed_ = X: E,
        }
        "#
            )
        );

        assert_eq!(
            desugar(
                r#"
        little_endian_packets
        enum E : 8 { X=0, Y=1 }
        group G1 {
            a: 8,
        }
        group G2 {
            G1 { a=1 },
            b: E,
        }
        packet A {
            G2 { b=X }
        }
        "#
            ),
            desugar(
                r#"
        little_endian_packets
        enum E : 8 { X=0, Y=1 }
        packet A {
            _fixed_ = 1: 8,
            _fixed_ = X: E,
        }
        "#
            )
        );
    }
}
