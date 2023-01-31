use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::files;
use codespan_reporting::term;
use codespan_reporting::term::termcolor;
use std::collections::HashMap;
use std::ptr;

use crate::ast::*;

/// Aggregate linter diagnostics.
#[derive(Debug)]
pub struct LintDiagnostics {
    pub diagnostics: Vec<Diagnostic<FileId>>,
}

/// Implement lint checks for an AST element.
pub trait Lintable {
    /// Generate lint warnings and errors for the
    /// input element.
    fn lint(&self) -> LintDiagnostics;
}

/// Gather information about the full AST.
#[derive(Debug)]
pub struct Scope<'d> {
    // Collection of Group, Packet, Enum, Struct, Checksum, and CustomField declarations.
    pub typedef: HashMap<String, &'d Decl>,

    // Collection of Packet, Struct, and Group scope declarations.
    pub scopes: HashMap<&'d Decl, PacketScope<'d>>,
}

/// Gather information about a Packet, Struct, or Group declaration.
#[derive(Debug)]
pub struct PacketScope<'d> {
    // Checksum starts, indexed by the checksum field id.
    checksums: HashMap<String, &'d Field>,

    // Size or count fields, indexed by the field id.
    sizes: HashMap<String, &'d Field>,

    // Payload or body field.
    payload: Option<&'d Field>,

    // Typedef, scalar, array fields.
    named: HashMap<String, &'d Field>,

    // Group fields.
    groups: HashMap<String, &'d Field>,

    // Flattened field declarations.
    // Contains field declarations from the original Packet, Struct, or Group,
    // where Group fields have been substituted by their body.
    // Constrained Scalar or Typedef Group fields are substituted by a Fixed
    // field.
    fields: Vec<&'d Field>,

    // Constraint declarations gathered from Group inlining.
    constraints: HashMap<String, &'d Constraint>,

    // Local and inherited field declarations. Only named fields are preserved.
    // Saved here for reference for parent constraint resolving.
    all_fields: HashMap<String, &'d Field>,

    // Local and inherited constraint declarations.
    // Saved here for constraint conflict checks.
    all_constraints: HashMap<String, &'d Constraint>,
}

impl std::cmp::Eq for &Decl {}
impl<'d> std::cmp::PartialEq for &'d Decl {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(*self, *other)
    }
}

impl<'d> std::hash::Hash for &'d Decl {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(*self, state);
    }
}

impl LintDiagnostics {
    fn new() -> LintDiagnostics {
        LintDiagnostics { diagnostics: vec![] }
    }

    pub fn print(
        &self,
        sources: &SourceDatabase,
        color: termcolor::ColorChoice,
    ) -> Result<(), files::Error> {
        let writer = termcolor::StandardStream::stderr(color);
        let config = term::Config::default();
        for d in self.diagnostics.iter() {
            term::emit(&mut writer.lock(), &config, sources, d)?;
        }
        Ok(())
    }

    fn push(&mut self, diagnostic: Diagnostic<FileId>) {
        self.diagnostics.push(diagnostic)
    }

    fn err_undeclared(&mut self, id: &str, loc: &SourceRange) {
        self.diagnostics.push(
            Diagnostic::error()
                .with_message(format!("undeclared identifier `{}`", id))
                .with_labels(vec![loc.primary()]),
        )
    }

    fn err_redeclared(&mut self, id: &str, kind: &str, loc: &SourceRange, prev: &SourceRange) {
        self.diagnostics.push(
            Diagnostic::error()
                .with_message(format!("redeclaration of {} identifier `{}`", kind, id))
                .with_labels(vec![
                    loc.primary(),
                    prev.secondary().with_message(format!("`{}` is first declared here", id)),
                ]),
        )
    }
}

fn bit_width(val: usize) -> usize {
    usize::BITS as usize - val.leading_zeros() as usize
}

impl<'d> PacketScope<'d> {
    /// Insert a field declaration into a packet scope.
    fn insert(&mut self, field: &'d Field, result: &mut LintDiagnostics) {
        match field {
            Field::Checksum { loc, field_id, .. } => {
                self.checksums.insert(field_id.clone(), field).map(|prev| {
                    result.push(
                        Diagnostic::error()
                            .with_message(format!(
                                "redeclaration of checksum start for `{}`",
                                field_id
                            ))
                            .with_labels(vec![
                                loc.primary(),
                                prev.loc()
                                    .secondary()
                                    .with_message("checksum start is first declared here"),
                            ]),
                    )
                })
            }

            Field::Padding { .. }
            | Field::Reserved { .. }
            | Field::Fixed { .. }
            | Field::ElementSize { .. } => None,

            Field::Size { loc, field_id, .. } | Field::Count { loc, field_id, .. } => {
                self.sizes.insert(field_id.clone(), field).map(|prev| {
                    result.push(
                        Diagnostic::error()
                            .with_message(format!(
                                "redeclaration of size or count for `{}`",
                                field_id
                            ))
                            .with_labels(vec![
                                loc.primary(),
                                prev.loc().secondary().with_message("size is first declared here"),
                            ]),
                    )
                })
            }

            Field::Body { loc, .. } | Field::Payload { loc, .. } => {
                if let Some(prev) = self.payload.as_ref() {
                    result.push(
                        Diagnostic::error()
                            .with_message("redeclaration of payload or body field")
                            .with_labels(vec![
                                loc.primary(),
                                prev.loc()
                                    .secondary()
                                    .with_message("payload is first declared here"),
                            ]),
                    )
                }
                self.payload = Some(field);
                None
            }

            Field::Array { loc, id, .. }
            | Field::Scalar { loc, id, .. }
            | Field::Typedef { loc, id, .. } => self
                .named
                .insert(id.clone(), field)
                .map(|prev| result.err_redeclared(id, "field", loc, prev.loc())),

            Field::Group { loc, group_id, .. } => {
                self.groups.insert(group_id.clone(), field).map(|prev| {
                    result.push(
                        Diagnostic::error()
                            .with_message(format!("duplicate group `{}` insertion", group_id))
                            .with_labels(vec![
                                loc.primary(),
                                prev.loc()
                                    .secondary()
                                    .with_message(format!("`{}` is first used here", group_id)),
                            ]),
                    )
                })
            }
        };
    }

    /// Add parent fields and constraints to the scope.
    /// Only named fields are imported.
    fn inherit(
        &mut self,
        scope: &Scope,
        parent: &PacketScope<'d>,
        constraints: impl Iterator<Item = &'d Constraint>,
        result: &mut LintDiagnostics,
    ) {
        // Check constraints.
        assert!(self.all_constraints.is_empty());
        self.all_constraints = parent.all_constraints.clone();
        for constraint in constraints {
            lint_constraint(scope, parent, constraint, result);
            let id = constraint.id.clone();
            if let Some(prev) = self.all_constraints.insert(id, constraint) {
                result.push(
                    Diagnostic::error()
                        .with_message(format!("duplicate constraint on field `{}`", constraint.id))
                        .with_labels(vec![
                            constraint.loc.primary(),
                            prev.loc.secondary().with_message("the constraint is first set here"),
                        ]),
                )
            }
        }

        // Merge group constraints into parent constraints,
        // but generate no duplication warnings, the constraints
        // do no apply to the same field set.
        for (id, constraint) in self.constraints.iter() {
            self.all_constraints.insert(id.clone(), constraint);
        }

        // Save parent fields.
        self.all_fields = parent.all_fields.clone();
    }

    /// Insert group field declarations into a packet scope.
    fn inline(
        &mut self,
        scope: &Scope,
        packet_scope: &PacketScope<'d>,
        group: &'d Field,
        constraints: impl Iterator<Item = &'d Constraint>,
        result: &mut LintDiagnostics,
    ) {
        fn err_redeclared_by_group(
            result: &mut LintDiagnostics,
            message: impl Into<String>,
            loc: &SourceRange,
            prev: &SourceRange,
        ) {
            result.push(Diagnostic::error().with_message(message).with_labels(vec![
                loc.primary(),
                prev.secondary().with_message("first declared here"),
            ]))
        }

        for (id, field) in packet_scope.checksums.iter() {
            if let Some(prev) = self.checksums.insert(id.clone(), field) {
                err_redeclared_by_group(
                    result,
                    format!("inserted group redeclares checksum start for `{}`", id),
                    group.loc(),
                    prev.loc(),
                )
            }
        }
        for (id, field) in packet_scope.sizes.iter() {
            if let Some(prev) = self.sizes.insert(id.clone(), field) {
                err_redeclared_by_group(
                    result,
                    format!("inserted group redeclares size or count for `{}`", id),
                    group.loc(),
                    prev.loc(),
                )
            }
        }
        match (&self.payload, &packet_scope.payload) {
            (Some(prev), Some(next)) => err_redeclared_by_group(
                result,
                "inserted group redeclares payload or body field",
                next.loc(),
                prev.loc(),
            ),
            (None, Some(payload)) => self.payload = Some(payload),
            _ => (),
        }
        for (id, field) in packet_scope.named.iter() {
            if let Some(prev) = self.named.insert(id.clone(), field) {
                err_redeclared_by_group(
                    result,
                    format!("inserted group redeclares field `{}`", id),
                    group.loc(),
                    prev.loc(),
                )
            }
        }

        // Append group fields to the finalizeed fields.
        for field in packet_scope.fields.iter() {
            self.fields.push(field);
        }

        // Append group constraints to the caller packet_scope.
        for (id, constraint) in packet_scope.constraints.iter() {
            self.constraints.insert(id.clone(), constraint);
        }

        // Add constraints to the packet_scope, checking for duplicate constraints.
        for constraint in constraints {
            lint_constraint(scope, packet_scope, constraint, result);
            let id = constraint.id.clone();
            if let Some(prev) = self.constraints.insert(id, constraint) {
                result.push(
                    Diagnostic::error()
                        .with_message(format!("duplicate constraint on field `{}`", constraint.id))
                        .with_labels(vec![
                            constraint.loc.primary(),
                            prev.loc.secondary().with_message("the constraint is first set here"),
                        ]),
                )
            }
        }
    }

    /// Return the field immediately preceding the selected field, or None
    /// if no such field exists.
    fn get_preceding_field(&self, searched_field: &Field) -> Option<&Field> {
        let mut preceding_field: Option<&Field> = None;
        for field in self.fields.iter() {
            if ptr::eq(*field, searched_field) {
                break;
            }
            preceding_field = Some(field);
        }
        preceding_field
    }

    /// Cleanup scope after processing all fields.
    fn finalize(&mut self, result: &mut LintDiagnostics) {
        // Check field shadowing.
        for f in self.fields.iter() {
            if let Some(id) = f.id() {
                if let Some(prev) = self.all_fields.insert(id.to_string(), f) {
                    result.push(
                        Diagnostic::warning()
                            .with_message(format!("declaration of `{}` shadows parent field", id))
                            .with_labels(vec![
                                f.loc().primary(),
                                prev.loc()
                                    .secondary()
                                    .with_message(format!("`{}` is first declared here", id)),
                            ]),
                    )
                }
            }
        }
    }
}

/// Helper for linting value constraints over packet fields.
fn lint_constraint(
    scope: &Scope,
    packet_scope: &PacketScope,
    constraint: &Constraint,
    result: &mut LintDiagnostics,
) {
    // Validate constraint value types.
    match (packet_scope.all_fields.get(&constraint.id), &constraint.value, &constraint.tag_id) {
        (Some(Field::Scalar { loc: field_loc, width, .. }), Some(value), _) => {
            if bit_width(*value) > *width {
                result.push(
                    Diagnostic::error().with_message("invalid integer literal").with_labels(vec![
                        constraint.loc.primary().with_message(format!(
                            "expected maximum value of `{}`",
                            (1 << *width) - 1
                        )),
                        field_loc.secondary().with_message("the value is used here"),
                    ]),
                )
            }
        }

        (Some(Field::Scalar { loc: field_loc, .. }), None, _) => {
            result.push(Diagnostic::error().with_message("invalid literal type").with_labels(vec![
                constraint.loc.primary().with_message("expected integer literal"),
                field_loc.secondary().with_message("the value is used here"),
            ]))
        }

        (Some(Field::Typedef { type_id, loc: field_loc, .. }), _, _) => {
            match (scope.typedef.get(type_id), &constraint.tag_id) {
                (Some(Decl::Enum { tags, .. }), Some(tag_id)) => {
                    if !tags.iter().any(|t| &t.id == tag_id) {
                        result.push(
                            Diagnostic::error()
                                .with_message(format!("undeclared enum tag `{}`", tag_id))
                                .with_labels(vec![
                                    constraint.loc.primary(),
                                    field_loc.secondary().with_message("the value is used here"),
                                ]),
                        )
                    }
                }
                (Some(decl), _) => result.push(
                    Diagnostic::error().with_message("invalid constraint").with_labels(vec![
                        constraint.loc.primary(),
                        field_loc.secondary().with_message(format!(
                            "`{}` has type {}, expected enum field",
                            constraint.id,
                            decl.kind()
                        )),
                    ]),
                ),
                // This error will be reported during field linting
                (None, _) => (),
            }
        }

        (Some(field), _, _) => result.push(
            Diagnostic::error()
                .with_message("invalid constraint field type")
                .with_labels(vec![constraint.loc.primary()])
                .with_notes(vec![format!(
                    "`{}` has type {}, expected enum field or scalar field",
                    constraint.id,
                    field.kind()
                )]),
        ),

        (None, _, _) => result.push(
            Diagnostic::error()
                .with_message(format!("undeclared identifier `{}`", constraint.id))
                .with_labels(vec![constraint.loc.primary()]),
        ),
    }
}

impl<'d> Scope<'d> {
    pub fn new(file: &File) -> Result<Scope<'_>, LintDiagnostics> {
        let mut lint_diagnostics = LintDiagnostics::new();
        let scope = file.scope(&mut lint_diagnostics);

        if !lint_diagnostics.diagnostics.is_empty() {
            return Err(lint_diagnostics);
        }
        for decl in &file.declarations {
            decl.lint(&scope, &mut lint_diagnostics)
        }
        if !lint_diagnostics.diagnostics.is_empty() {
            return Err(lint_diagnostics);
        }

        Ok(scope)
    }

    // Sort Packet, Struct, and Group declarations by reverse topological
    // orde, and inline Group fields.
    // Raises errors and warnings for:
    //      - undeclared included Groups,
    //      - undeclared Typedef fields,
    //      - undeclared Packet or Struct parents,
    //      - recursive Group insertion,
    //      - recursive Packet or Struct inheritance.
    fn finalize(&mut self, result: &mut LintDiagnostics) -> Vec<&'d Decl> {
        // Auxiliary function implementing BFS on Packet tree.
        enum Mark {
            Temporary,
            Permanent,
        }
        struct Context<'d> {
            list: Vec<&'d Decl>,
            visited: HashMap<&'d Decl, Mark>,
            scopes: HashMap<&'d Decl, PacketScope<'d>>,
        }

        fn bfs<'s, 'd>(
            decl: &'d Decl,
            context: &'s mut Context<'d>,
            scope: &Scope<'d>,
            result: &mut LintDiagnostics,
        ) -> Option<&'s PacketScope<'d>> {
            match context.visited.get(&decl) {
                Some(Mark::Permanent) => return context.scopes.get(&decl),
                Some(Mark::Temporary) => {
                    result.push(
                        Diagnostic::error()
                            .with_message(format!(
                                "recursive declaration of {} `{}`",
                                decl.kind(),
                                decl.id().unwrap()
                            ))
                            .with_labels(vec![decl.loc().primary()]),
                    );
                    return None;
                }
                _ => (),
            }

            let (parent_id, fields) = match decl {
                Decl::Packet { parent_id, fields, .. } | Decl::Struct { parent_id, fields, .. } => {
                    (parent_id.as_ref(), fields)
                }
                Decl::Group { fields, .. } => (None, fields),
                _ => return None,
            };

            context.visited.insert(decl, Mark::Temporary);
            let mut lscope = decl.scope(result).unwrap();

            // Iterate over Struct and Group fields.
            for f in fields {
                match f {
                    Field::Group { group_id, constraints, .. } => {
                        match scope.typedef.get(group_id) {
                            None => result.push(
                                Diagnostic::error()
                                    .with_message(format!(
                                        "undeclared group identifier `{}`",
                                        group_id
                                    ))
                                    .with_labels(vec![f.loc().primary()]),
                            ),
                            Some(group_decl @ Decl::Group { .. }) => {
                                // Recurse to flatten the inserted group.
                                if let Some(rscope) = bfs(group_decl, context, scope, result) {
                                    // Inline the group fields and constraints into
                                    // the current scope.
                                    lscope.inline(scope, rscope, f, constraints.iter(), result)
                                }
                            }
                            Some(_) => result.push(
                                Diagnostic::error()
                                    .with_message(format!(
                                        "invalid group field identifier `{}`",
                                        group_id
                                    ))
                                    .with_labels(vec![f.loc().primary()])
                                    .with_notes(vec!["hint: expected group identifier".to_owned()]),
                            ),
                        }
                    }
                    Field::Typedef { type_id, .. } => {
                        lscope.fields.push(f);
                        match scope.typedef.get(type_id) {
                            None => result.push(
                                Diagnostic::error()
                                    .with_message(format!(
                                        "undeclared typedef identifier `{}`",
                                        type_id
                                    ))
                                    .with_labels(vec![f.loc().primary()]),
                            ),
                            Some(struct_decl @ Decl::Struct { .. }) => {
                                bfs(struct_decl, context, scope, result);
                            }
                            Some(_) => (),
                        }
                    }
                    _ => lscope.fields.push(f),
                }
            }

            // Iterate over parent declaration.
            let parent = parent_id.and_then(|id| scope.typedef.get(id));
            match (decl, parent) {
                (Decl::Packet { parent_id: Some(_), .. }, None)
                | (Decl::Struct { parent_id: Some(_), .. }, None) => result.push(
                    Diagnostic::error()
                        .with_message(format!(
                            "undeclared parent identifier `{}`",
                            parent_id.unwrap()
                        ))
                        .with_labels(vec![decl.loc().primary()])
                        .with_notes(vec![format!("hint: expected {} parent", decl.kind())]),
                ),
                (Decl::Packet { .. }, Some(Decl::Struct { .. }))
                | (Decl::Struct { .. }, Some(Decl::Packet { .. })) => result.push(
                    Diagnostic::error()
                        .with_message(format!("invalid parent identifier `{}`", parent_id.unwrap()))
                        .with_labels(vec![decl.loc().primary()])
                        .with_notes(vec![format!("hint: expected {} parent", decl.kind())]),
                ),
                (_, Some(parent_decl)) => {
                    if let Some(rscope) = bfs(parent_decl, context, scope, result) {
                        // Import the parent fields and constraints into the current scope.
                        lscope.inherit(scope, rscope, decl.constraints(), result)
                    }
                }
                _ => (),
            }

            lscope.finalize(result);
            context.list.push(decl);
            context.visited.insert(decl, Mark::Permanent);
            context.scopes.insert(decl, lscope);
            context.scopes.get(&decl)
        }

        let mut context =
            Context::<'d> { list: vec![], visited: HashMap::new(), scopes: HashMap::new() };

        for decl in self.typedef.values() {
            bfs(decl, &mut context, self, result);
        }

        self.scopes = context.scopes;
        context.list
    }
}

impl Field {
    fn kind(&self) -> &str {
        match self {
            Field::Checksum { .. } => "payload",
            Field::Padding { .. } => "padding",
            Field::Size { .. } => "size",
            Field::Count { .. } => "count",
            Field::ElementSize { .. } => "elementsize",
            Field::Body { .. } => "body",
            Field::Payload { .. } => "payload",
            Field::Fixed { .. } => "fixed",
            Field::Reserved { .. } => "reserved",
            Field::Group { .. } => "group",
            Field::Array { .. } => "array",
            Field::Scalar { .. } => "scalar",
            Field::Typedef { .. } => "typedef",
        }
    }
}

// Helper for linting an enum declaration.
fn lint_enum(tags: &[Tag], width: usize, result: &mut LintDiagnostics) {
    let mut local_scope = HashMap::new();
    for tag in tags {
        // Tags must be unique within the scope of the
        // enum declaration.
        if let Some(prev) = local_scope.insert(tag.id.clone(), tag) {
            result.push(
                Diagnostic::error()
                    .with_message(format!("redeclaration of tag identifier `{}`", &tag.id))
                    .with_labels(vec![
                        tag.loc.primary(),
                        prev.loc.secondary().with_message("first declared here"),
                    ]),
            )
        }

        // Tag values must fit the enum declared width.
        if bit_width(tag.value) > width {
            result.push(Diagnostic::error().with_message("invalid literal value").with_labels(
                vec![tag.loc.primary().with_message(format!(
                        "expected maximum value of `{}`",
                        (1 << width) - 1
                    ))],
            ))
        }
    }
}

// Helper for linting checksum fields.
fn lint_checksum(
    scope: &Scope,
    packet_scope: &PacketScope,
    decl: &Field,
    field_id: &str,
    result: &mut LintDiagnostics,
) {
    // Checksum field must be declared before
    // the checksum start. The field must be a typedef with
    // a valid checksum type.
    let checksum_loc = decl.loc();

    match packet_scope.named.get(field_id) {
        Some(Field::Typedef { loc: field_loc, type_id, .. }) => {
            // Check declaration type of checksum field.
            match scope.typedef.get(type_id) {
                Some(Decl::Checksum { .. }) => (),
                Some(decl) => result.push(
                    Diagnostic::error()
                        .with_message(format!("checksum start uses invalid field `{}`", field_id))
                        .with_labels(vec![
                            checksum_loc.primary(),
                            field_loc.secondary().with_message(format!(
                                "`{}` is declared with {} type `{}`, expected checksum_field",
                                field_id,
                                decl.kind(),
                                type_id
                            )),
                        ]),
                ),
                // This error case will be reported when the field itself
                // is checked.
                None => (),
            };
        }
        Some(field) => result.push(
            Diagnostic::error()
                .with_message(format!("checksum start uses invalid field `{}`", field_id))
                .with_labels(vec![
                    checksum_loc.primary(),
                    field.loc().secondary().with_message(format!(
                        "`{}` is declared as {} field, expected typedef",
                        field_id,
                        field.kind()
                    )),
                ]),
        ),
        None => result.err_undeclared(field_id, checksum_loc),
    }
}

// Helper for linting size fields.
fn lint_size(
    _scope: &Scope,
    packet_scope: &PacketScope,
    decl: &Field,
    field_id: &str,
    _width: usize,
    result: &mut LintDiagnostics,
) {
    // Size fields should be declared before
    // the sized field (body, payload, or array).
    // The field must reference a valid body, payload or array
    // field.

    let size_loc = decl.loc();

    if field_id == "_payload_" {
        return match packet_scope.payload.as_ref() {
            Some(Field::Body { .. }) => result.push(
                Diagnostic::error()
                    .with_message("size field uses undeclared payload field, did you mean _body_ ?")
                    .with_labels(vec![size_loc.primary()]),
            ),
            Some(Field::Payload { .. }) => (),
            Some(_) => unreachable!(),
            None => result.push(
                Diagnostic::error()
                    .with_message("size field uses undeclared payload field")
                    .with_labels(vec![size_loc.primary()]),
            ),
        };
    }
    if field_id == "_body_" {
        return match packet_scope.payload.as_ref() {
            Some(Field::Payload { .. }) => result.push(
                Diagnostic::error()
                    .with_message("size field uses undeclared body field, did you mean _payload_ ?")
                    .with_labels(vec![size_loc.primary()]),
            ),
            Some(Field::Body { .. }) => (),
            Some(_) => unreachable!(),
            None => result.push(
                Diagnostic::error()
                    .with_message("size field uses undeclared body field")
                    .with_labels(vec![size_loc.primary()]),
            ),
        };
    }

    match packet_scope.named.get(field_id) {
        Some(Field::Array { size: Some(_), loc: array_loc, .. }) => result.push(
            Diagnostic::warning()
                .with_message(format!("size field uses array `{}` with static size", field_id))
                .with_labels(vec![
                    size_loc.primary(),
                    array_loc.secondary().with_message(format!("`{}` is declared here", field_id)),
                ]),
        ),
        Some(Field::Array { .. }) => (),
        Some(field) => result.push(
            Diagnostic::error()
                .with_message(format!("invalid `{}` field type", field_id))
                .with_labels(vec![
                    field.loc().primary().with_message(format!(
                        "`{}` is declared as {}",
                        field_id,
                        field.kind()
                    )),
                    size_loc
                        .secondary()
                        .with_message(format!("`{}` is used here as array", field_id)),
                ]),
        ),

        None => result.err_undeclared(field_id, size_loc),
    }
}

// Helper for linting count fields.
fn lint_count(
    _scope: &Scope,
    packet_scope: &PacketScope,
    decl: &Field,
    field_id: &str,
    _width: usize,
    result: &mut LintDiagnostics,
) {
    // Count fields should be declared before the sized field.
    // The field must reference a valid array field.
    // Warning if the array already has a known size.

    let count_loc = decl.loc();
    match packet_scope.named.get(field_id) {
        Some(Field::Array { size: Some(_), loc: array_loc, .. }) => result.push(
            Diagnostic::warning()
                .with_message(format!("count field uses array `{}` with static size", field_id))
                .with_labels(vec![
                    count_loc.primary(),
                    array_loc.secondary().with_message(format!("`{}` is declared here", field_id)),
                ]),
        ),

        Some(Field::Array { .. }) => (),
        Some(field) => result.push(
            Diagnostic::error()
                .with_message(format!("invalid `{}` field type", field_id))
                .with_labels(vec![
                    field.loc().primary().with_message(format!(
                        "`{}` is declared as {}",
                        field_id,
                        field.kind()
                    )),
                    count_loc
                        .secondary()
                        .with_message(format!("`{}` is used here as array", field_id)),
                ]),
        ),

        None => result.err_undeclared(field_id, count_loc),
    }
}

// Helper for linting fixed fields.
#[allow(clippy::too_many_arguments)]
fn lint_fixed(
    scope: &Scope,
    _packet_scope: &PacketScope,
    decl: &Field,
    width: &Option<usize>,
    value: &Option<usize>,
    enum_id: &Option<String>,
    tag_id: &Option<String>,
    result: &mut LintDiagnostics,
) {
    // By parsing constraint, we already have that either
    // (width and value) or (enum_id and tag_id) are Some.
    let fixed_loc = decl.loc();

    if width.is_some() {
        // The value of a fixed field should have .
        if bit_width(value.unwrap()) > width.unwrap() {
            result.push(Diagnostic::error().with_message("invalid integer literal").with_labels(
                vec![fixed_loc.primary().with_message(format!(
                    "expected maximum value of `{}`",
                    (1 << width.unwrap()) - 1
                ))],
            ))
        }
    } else {
        // The fixed field should reference a valid enum id and tag id
        // association.
        match scope.typedef.get(enum_id.as_ref().unwrap()) {
            Some(Decl::Enum { tags, .. }) => {
                match tags.iter().find(|t| &t.id == tag_id.as_ref().unwrap()) {
                    Some(_) => (),
                    None => result.push(
                        Diagnostic::error()
                            .with_message(format!(
                                "undeclared enum tag `{}`",
                                tag_id.as_ref().unwrap()
                            ))
                            .with_labels(vec![fixed_loc.primary()]),
                    ),
                }
            }
            Some(decl) => result.push(
                Diagnostic::error()
                    .with_message(format!(
                        "fixed field uses invalid typedef `{}`",
                        decl.id().unwrap()
                    ))
                    .with_labels(vec![fixed_loc.primary().with_message(format!(
                        "{} has kind {}, expected enum",
                        decl.id().unwrap(),
                        decl.kind(),
                    ))]),
            ),
            None => result.push(
                Diagnostic::error()
                    .with_message(format!("undeclared enum type `{}`", enum_id.as_ref().unwrap()))
                    .with_labels(vec![fixed_loc.primary()]),
            ),
        }
    }
}

// Helper for linting array fields.
#[allow(clippy::too_many_arguments)]
fn lint_array(
    scope: &Scope,
    _packet_scope: &PacketScope,
    decl: &Field,
    _width: &Option<usize>,
    type_id: &Option<String>,
    _size_modifier: &Option<String>,
    _size: &Option<usize>,
    result: &mut LintDiagnostics,
) {
    // By parsing constraint, we have that width and type_id are mutually
    // exclusive, as well as size_modifier and size.
    // type_id must reference a valid enum or packet type.
    // TODO(hchataing) unbounded arrays should have a matching size
    // or count field
    let array_loc = decl.loc();

    if type_id.is_some() {
        match scope.typedef.get(type_id.as_ref().unwrap()) {
            Some(Decl::Enum { .. })
            | Some(Decl::Struct { .. })
            | Some(Decl::CustomField { .. }) => (),
            Some(decl) => result.push(
                Diagnostic::error()
                    .with_message(format!(
                        "array field uses invalid {} element type `{}`",
                        decl.kind(),
                        type_id.as_ref().unwrap()
                    ))
                    .with_labels(vec![array_loc.primary()])
                    .with_notes(vec!["hint: expected enum, struct, custom_field".to_owned()]),
            ),
            None => result.push(
                Diagnostic::error()
                    .with_message(format!(
                        "array field uses undeclared element type `{}`",
                        type_id.as_ref().unwrap()
                    ))
                    .with_labels(vec![array_loc.primary()])
                    .with_notes(vec!["hint: expected enum, struct, custom_field".to_owned()]),
            ),
        }
    }
}

// Helper for linting padding fields.
fn lint_padding(
    _scope: &Scope,
    packet_scope: &PacketScope,
    decl: &Field,
    _size: usize,
    result: &mut LintDiagnostics,
) {
    // The padding field must follow an array field.
    let padding_loc = decl.loc();

    match packet_scope.get_preceding_field(decl) {
        None => result.push(
            Diagnostic::error()
                .with_message("padding field cannot be the first field of a packet")
                .with_labels(vec![padding_loc.primary()])
                .with_notes(vec![
                    "hint: padding fields must be placed after an array field".to_owned()
                ]),
        ),
        Some(Field::Array { .. }) => (),
        Some(preceding_field) => result.push(
            Diagnostic::error()
                .with_message(format!(
                    "padding field cannot be placed after {} field",
                    preceding_field.kind()
                ))
                .with_labels(vec![padding_loc.primary(), preceding_field.loc().secondary()])
                .with_notes(vec![
                    "hint: padding fields must be placed after an array field".to_owned()
                ]),
        ),
    }
}

// Helper for linting typedef fields.
fn lint_typedef(
    scope: &Scope,
    _packet_scope: &PacketScope,
    decl: &Field,
    type_id: &str,
    result: &mut LintDiagnostics,
) {
    // The typedef field must reference a valid struct, enum,
    // custom_field, or checksum type.
    // TODO(hchataing) checksum fields should have a matching checksum start
    let typedef_loc = decl.loc();

    match scope.typedef.get(type_id) {
        Some(Decl::Enum { .. })
        | Some(Decl::Struct { .. })
        | Some(Decl::CustomField { .. })
        | Some(Decl::Checksum { .. }) => (),

        Some(decl) => result.push(
            Diagnostic::error()
                .with_message(format!(
                    "typedef field uses invalid {} element type `{}`",
                    decl.kind(),
                    type_id
                ))
                .with_labels(vec![typedef_loc.primary()])
                .with_notes(vec!["hint: expected enum, struct, custom_field, checksum".to_owned()]),
        ),
        None => result.push(
            Diagnostic::error()
                .with_message(format!("typedef field uses undeclared element type `{}`", type_id))
                .with_labels(vec![typedef_loc.primary()])
                .with_notes(vec!["hint: expected enum, struct, custom_field, checksum".to_owned()]),
        ),
    }
}

// Helper for linting a field declaration.
fn lint_field(
    scope: &Scope,
    packet_scope: &PacketScope,
    decl: &Field,
    result: &mut LintDiagnostics,
) {
    match decl {
        Field::Checksum { field_id, .. } => {
            lint_checksum(scope, packet_scope, decl, field_id, result)
        }
        Field::Size { field_id, width, .. } => {
            lint_size(scope, packet_scope, decl, field_id, *width, result)
        }
        Field::Count { field_id, width, .. } => {
            lint_count(scope, packet_scope, decl, field_id, *width, result)
        }
        Field::ElementSize { .. } => { /* TODO(aryarahul) */ }
        Field::Fixed { width, value, enum_id, tag_id, .. } => {
            lint_fixed(scope, packet_scope, decl, width, value, enum_id, tag_id, result)
        }
        Field::Array { width, type_id, size_modifier, size, .. } => {
            lint_array(scope, packet_scope, decl, width, type_id, size_modifier, size, result)
        }
        Field::Typedef { type_id, .. } => lint_typedef(scope, packet_scope, decl, type_id, result),
        Field::Padding { size, .. } => lint_padding(scope, packet_scope, decl, *size, result),
        Field::Reserved { .. }
        | Field::Scalar { .. }
        | Field::Body { .. }
        | Field::Payload { .. } => (),
        Field::Group { .. } => unreachable!(),
    }
}

// Helper for linting a packet declaration.
fn lint_packet(
    scope: &Scope,
    decl: &Decl,
    id: &str,
    loc: &SourceRange,
    constraints: &[Constraint],
    parent_id: &Option<String>,
    result: &mut LintDiagnostics,
) {
    // The parent declaration is checked by Scope::finalize.
    // The local scope is also generated by Scope::finalize.
    // TODO(hchataing) check parent payload size constraint: compute an upper
    // bound of the payload size and check against the encoded maximum size.

    if parent_id.is_none() && !constraints.is_empty() {
        // Constraint list should be empty when there is
        // no inheritance.
        result.push(
            Diagnostic::warning()
                .with_message(format!(
                    "packet `{}` has field constraints, but no parent declaration",
                    id
                ))
                .with_labels(vec![loc.primary()])
                .with_notes(vec!["hint: expected parent declaration".to_owned()]),
        )
    }

    // Retrieve pre-computed packet scope.
    // Scope validation was done before, so it must exist.
    let packet_scope = &scope.scopes.get(&decl).unwrap();

    for field in packet_scope.fields.iter() {
        lint_field(scope, packet_scope, field, result)
    }
}

// Helper for linting a struct declaration.
fn lint_struct(
    scope: &Scope,
    decl: &Decl,
    id: &str,
    loc: &SourceRange,
    constraints: &[Constraint],
    parent_id: &Option<String>,
    result: &mut LintDiagnostics,
) {
    // The parent declaration is checked by Scope::finalize.
    // The local scope is also generated by Scope::finalize.
    // TODO(hchataing) check parent payload size constraint: compute an upper
    // bound of the payload size and check against the encoded maximum size.

    if parent_id.is_none() && !constraints.is_empty() {
        // Constraint list should be empty when there is
        // no inheritance.
        result.push(
            Diagnostic::warning()
                .with_message(format!(
                    "struct `{}` has field constraints, but no parent declaration",
                    id
                ))
                .with_labels(vec![loc.primary()])
                .with_notes(vec!["hint: expected parent declaration".to_owned()]),
        )
    }

    // Retrieve pre-computed packet scope.
    // Scope validation was done before, so it must exist.
    let packet_scope = &scope.scopes.get(&decl).unwrap();

    for field in packet_scope.fields.iter() {
        lint_field(scope, packet_scope, field, result)
    }
}

impl Decl {
    fn constraints(&self) -> impl Iterator<Item = &Constraint> {
        match self {
            Decl::Packet { constraints, .. } | Decl::Struct { constraints, .. } => {
                Some(constraints.iter())
            }
            _ => None,
        }
        .into_iter()
        .flatten()
    }

    fn scope<'d>(&'d self, result: &mut LintDiagnostics) -> Option<PacketScope<'d>> {
        match self {
            Decl::Packet { fields, .. }
            | Decl::Struct { fields, .. }
            | Decl::Group { fields, .. } => {
                let mut scope = PacketScope {
                    checksums: HashMap::new(),
                    sizes: HashMap::new(),
                    payload: None,
                    named: HashMap::new(),
                    groups: HashMap::new(),

                    fields: Vec::new(),
                    constraints: HashMap::new(),
                    all_fields: HashMap::new(),
                    all_constraints: HashMap::new(),
                };
                for field in fields {
                    scope.insert(field, result)
                }
                Some(scope)
            }
            _ => None,
        }
    }

    fn lint<'d>(&'d self, scope: &Scope<'d>, result: &mut LintDiagnostics) {
        match self {
            Decl::Checksum { .. } | Decl::CustomField { .. } => (),
            Decl::Enum { tags, width, .. } => lint_enum(tags, *width, result),
            Decl::Packet { id, loc, constraints, parent_id, .. } => {
                lint_packet(scope, self, id, loc, constraints, parent_id, result)
            }
            Decl::Struct { id, loc, constraints, parent_id, .. } => {
                lint_struct(scope, self, id, loc, constraints, parent_id, result)
            }
            // Groups are finalizeed before linting, to make sure
            // potential errors are raised only once.
            Decl::Group { .. } => (),
            Decl::Test { .. } => (),
        }
    }

    fn kind(&self) -> &str {
        match self {
            Decl::Checksum { .. } => "checksum",
            Decl::CustomField { .. } => "custom field",
            Decl::Enum { .. } => "enum",
            Decl::Packet { .. } => "packet",
            Decl::Struct { .. } => "struct",
            Decl::Group { .. } => "group",
            Decl::Test { .. } => "test",
        }
    }
}

impl File {
    fn scope<'d>(&'d self, result: &mut LintDiagnostics) -> Scope<'d> {
        let mut scope = Scope { typedef: HashMap::new(), scopes: HashMap::new() };

        // Gather top-level declarations.
        // Validate the top-level scopes (Group, Packet, Typedef).
        //
        // TODO: switch to try_insert when stable
        for decl in &self.declarations {
            if let Some(id) = decl.id() {
                if let Some(prev) = scope.typedef.insert(id.to_string(), decl) {
                    result.err_redeclared(id, decl.kind(), decl.loc(), prev.loc())
                }
            }
            if let Some(lscope) = decl.scope(result) {
                scope.scopes.insert(decl, lscope);
            }
        }

        scope.finalize(result);
        scope
    }
}

impl Lintable for File {
    fn lint(&self) -> LintDiagnostics {
        Scope::new(self).err().unwrap_or_else(LintDiagnostics::new)
    }
}

#[cfg(test)]
mod test {
    use crate::ast::*;
    use crate::lint::Lintable;
    use crate::parser::parse_inline;

    macro_rules! lint_success {
        ($name:ident, $text:literal) => {
            #[test]
            fn $name() {
                let mut db = SourceDatabase::new();
                let file = parse_inline(&mut db, "stdin".to_owned(), $text.to_owned())
                    .expect("parsing failure");
                assert!(file.lint().diagnostics.is_empty());
            }
        };
    }

    macro_rules! lint_failure {
        ($name:ident, $text:literal) => {
            #[test]
            fn $name() {
                let mut db = SourceDatabase::new();
                let file = parse_inline(&mut db, "stdin".to_owned(), $text.to_owned())
                    .expect("parsing failure");
                assert!(!file.lint().diagnostics.is_empty());
            }
        };
    }

    lint_failure!(
        test_packet_redeclared,
        r#"
        little_endian_packets
        struct Name { }
        packet Name { }
        "#
    );

    lint_success!(
        test_packet_checksum_start,
        r#"
        little_endian_packets
        checksum Checksum : 8 "Checksum"
        packet P {
          _checksum_start_(crc),
          a: 16,
          crc: Checksum,
        }
        "#
    );

    lint_failure!(
        test_padding_cannot_be_first_field,
        r#"
        little_endian_packets
        struct Test {
            _padding_[10],
        }
        "#
    );

    lint_failure!(
        test_padding_cannot_follow_scalar_field,
        r#"
        little_endian_packets
        struct Test {
            scalar: 8,
            _padding_[10],
        }
        "#
    );

    lint_success!(
        test_padding,
        r#"
        little_endian_packets
        struct Test {
            array: 8[],
            _padding_[10],
        }
        "#
    );
}
