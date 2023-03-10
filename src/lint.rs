use codespan_reporting::diagnostic::Diagnostic;
use std::collections::HashMap;

use crate::{ast::*, parser};

pub mod ast {
    use serde::Serialize;

    // Field and declaration size information.
    #[derive(Default, Debug, Clone)]
    #[allow(unused)]
    pub enum Size {
        // Constant size in bits.
        Static(usize),
        // Size indicated at packet parsing by
        // a size or count field.
        Dynamic,
        // The size cannot be determined statically or at runtime.
        // The packet assumes the largest possible size.
        #[default]
        Unknown,
    }

    #[derive(Debug, Serialize)]
    pub struct Annotation();

    impl crate::ast::Annotation for Annotation {
        type FieldAnnotation = Size;
        type DeclAnnotation = Size;
    }

    #[allow(unused)]
    pub type Field = crate::ast::Field<Annotation>;
    #[allow(unused)]
    pub type Decl = crate::ast::Decl<Annotation>;
    #[allow(unused)]
    pub type File = crate::ast::File<Annotation>;
}

/// Aggregate linter diagnostics.
#[derive(Debug)]
pub struct LintDiagnostics {
    pub diagnostics: Vec<Diagnostic<FileId>>,
}

/// Gather information about the full AST.
#[derive(Debug)]
pub struct Scope<'d> {
    // Collection of Group, Packet, Enum, Struct, Checksum, and CustomField declarations.
    pub typedef: HashMap<String, &'d parser::ast::Decl>,

    // Collection of Packet, Struct, and Group scope declarations.
    pub scopes: HashMap<&'d parser::ast::Decl, PacketScope<'d>>,

    // Children for the Decl with the given id.
    pub children: HashMap<String, Vec<&'d parser::ast::Decl>>,
}

/// Gather information about a Packet, Struct, or Group declaration.
#[derive(Debug)]
pub struct PacketScope<'d> {
    // Checksum starts, indexed by the checksum field id.
    checksums: HashMap<String, &'d parser::ast::Field>,

    // Size or count fields, indexed by the field id.
    pub sizes: HashMap<String, &'d parser::ast::Field>,

    // Payload or body field.
    pub payload: Option<&'d parser::ast::Field>,

    // Typedef, scalar, array fields.
    pub named: HashMap<String, &'d parser::ast::Field>,

    // Group fields.
    groups: HashMap<String, &'d parser::ast::Field>,

    // Flattened field declarations.
    // Contains field declarations from the original Packet, Struct, or Group,
    // where Group fields have been substituted by their body.
    pub fields: Vec<&'d parser::ast::Field>,

    // Constraint declarations gathered from Group inlining.
    pub constraints: HashMap<String, &'d Constraint>,

    // Local and inherited field declarations. Only named fields are preserved.
    // Saved here for reference for parent constraint resolving.
    pub all_fields: HashMap<String, &'d parser::ast::Field>,

    // Local and inherited constraint declarations.
    // Saved here for constraint conflict checks.
    pub all_constraints: HashMap<String, &'d Constraint>,
}

impl std::cmp::Eq for &parser::ast::Decl {}
impl<'d> std::cmp::PartialEq for &'d parser::ast::Decl {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(*self, *other)
    }
}

impl<'d> std::hash::Hash for &'d parser::ast::Decl {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(*self, state);
    }
}

impl LintDiagnostics {
    fn new() -> LintDiagnostics {
        LintDiagnostics { diagnostics: vec![] }
    }

    fn push(&mut self, diagnostic: Diagnostic<FileId>) {
        self.diagnostics.push(diagnostic)
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

impl<'d> PacketScope<'d> {
    /// Insert a field declaration into a packet scope.
    fn insert(&mut self, field: &'d parser::ast::Field, result: &mut LintDiagnostics) {
        match &field.desc {
            FieldDesc::Checksum { field_id, .. } => {
                self.checksums.insert(field_id.clone(), field).map(|prev| {
                    result.push(
                        Diagnostic::error()
                            .with_message(format!(
                                "redeclaration of checksum start for `{}`",
                                field_id
                            ))
                            .with_labels(vec![
                                field.loc.primary(),
                                prev.loc
                                    .secondary()
                                    .with_message("checksum start is first declared here"),
                            ]),
                    )
                })
            }

            FieldDesc::Padding { .. }
            | FieldDesc::Reserved { .. }
            | FieldDesc::FixedScalar { .. }
            | FieldDesc::FixedEnum { .. }
            | FieldDesc::ElementSize { .. } => None,

            FieldDesc::Size { field_id, .. } | FieldDesc::Count { field_id, .. } => {
                self.sizes.insert(field_id.clone(), field).map(|prev| {
                    result.push(
                        Diagnostic::error()
                            .with_message(format!(
                                "redeclaration of size or count for `{}`",
                                field_id
                            ))
                            .with_labels(vec![
                                field.loc.primary(),
                                prev.loc.secondary().with_message("size is first declared here"),
                            ]),
                    )
                })
            }

            FieldDesc::Body { .. } | FieldDesc::Payload { .. } => {
                if let Some(prev) = self.payload.as_ref() {
                    result.push(
                        Diagnostic::error()
                            .with_message("redeclaration of payload or body field")
                            .with_labels(vec![
                                field.loc.primary(),
                                prev.loc.secondary().with_message("payload is first declared here"),
                            ]),
                    )
                }
                self.payload = Some(field);
                None
            }

            FieldDesc::Array { id, .. }
            | FieldDesc::Scalar { id, .. }
            | FieldDesc::Typedef { id, .. } => self
                .named
                .insert(id.clone(), field)
                .map(|prev| result.err_redeclared(id, "field", &field.loc, &prev.loc)),

            FieldDesc::Group { group_id, .. } => {
                self.groups.insert(group_id.clone(), field).map(|prev| {
                    result.push(
                        Diagnostic::error()
                            .with_message(format!("duplicate group `{}` insertion", group_id))
                            .with_labels(vec![
                                field.loc.primary(),
                                prev.loc
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
        parent: &PacketScope<'d>,
        constraints: impl Iterator<Item = &'d Constraint>,
    ) {
        // Check constraints.
        assert!(self.all_constraints.is_empty());
        self.all_constraints = parent.all_constraints.clone();
        for constraint in constraints {
            let id = constraint.id.clone();
            self.all_constraints.insert(id, constraint);
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
        packet_scope: &PacketScope<'d>,
        group: &'d parser::ast::Field,
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
                    &group.loc,
                    &prev.loc,
                )
            }
        }
        for (id, field) in packet_scope.sizes.iter() {
            if let Some(prev) = self.sizes.insert(id.clone(), field) {
                err_redeclared_by_group(
                    result,
                    format!("inserted group redeclares size or count for `{}`", id),
                    &group.loc,
                    &prev.loc,
                )
            }
        }
        match (&self.payload, &packet_scope.payload) {
            (Some(prev), Some(next)) => err_redeclared_by_group(
                result,
                "inserted group redeclares payload or body field",
                &next.loc,
                &prev.loc,
            ),
            (None, Some(payload)) => self.payload = Some(payload),
            _ => (),
        }
        for (id, field) in packet_scope.named.iter() {
            if let Some(prev) = self.named.insert(id.clone(), field) {
                err_redeclared_by_group(
                    result,
                    format!("inserted group redeclares field `{}`", id),
                    &group.loc,
                    &prev.loc,
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
            let id = constraint.id.clone();
            self.constraints.insert(id, constraint);
        }
    }

    /// Lookup a field by name. This will also find the special
    /// `_payload_` and `_body_` fields.
    pub fn get_packet_field(&self, id: &str) -> Option<&parser::ast::Field> {
        self.named.get(id).copied().or(match id {
            "_payload_" | "_body_" => self.payload,
            _ => None,
        })
    }

    /// Find the size field corresponding to the payload or body
    /// field of this packet.
    pub fn get_payload_size_field(&self) -> Option<&parser::ast::Field> {
        self.sizes.get("_payload_").or_else(|| self.sizes.get("_body_")).copied()
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
                                f.loc.primary(),
                                prev.loc
                                    .secondary()
                                    .with_message(format!("`{}` is first declared here", id)),
                            ]),
                    )
                }
            }
        }
    }
}

impl<'d> Scope<'d> {
    pub fn new(file: &parser::ast::File) -> Result<Scope<'_>, LintDiagnostics> {
        let mut diagnostics = LintDiagnostics::new();
        let mut scope =
            Scope { typedef: HashMap::new(), scopes: HashMap::new(), children: HashMap::new() };

        // Gather top-level declarations.
        // Validate the top-level scopes (Group, Packet, Typedef).
        //
        // TODO: switch to try_insert when stable
        for decl in &file.declarations {
            if let Some(id) = decl.id() {
                if let Some(prev) = scope.typedef.insert(id.to_string(), decl) {
                    diagnostics.err_redeclared(id, decl.kind(), &decl.loc, &prev.loc)
                }
            }
            if let Some(lscope) = decl_scope(decl, &mut diagnostics) {
                scope.scopes.insert(decl, lscope);
            }

            if let DeclDesc::Packet { parent_id: Some(parent_id), .. }
            | DeclDesc::Struct { parent_id: Some(parent_id), .. } = &decl.desc
            {
                scope.children.entry(parent_id.to_string()).or_default().push(decl);
            }
        }

        scope.finalize(&mut diagnostics);

        if !diagnostics.diagnostics.is_empty() {
            return Err(diagnostics);
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
    fn finalize(&mut self, result: &mut LintDiagnostics) -> Vec<&'d parser::ast::Decl> {
        // Auxiliary function implementing BFS on Packet tree.
        enum Mark {
            Temporary,
            Permanent,
        }
        struct Context<'d> {
            list: Vec<&'d parser::ast::Decl>,
            visited: HashMap<&'d parser::ast::Decl, Mark>,
            scopes: HashMap<&'d parser::ast::Decl, PacketScope<'d>>,
        }

        fn bfs<'s, 'd>(
            decl: &'d parser::ast::Decl,
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
                            .with_labels(vec![decl.loc.primary()]),
                    );
                    return None;
                }
                _ => (),
            }

            let (parent_id, fields) = match &decl.desc {
                DeclDesc::Packet { parent_id, fields, .. }
                | DeclDesc::Struct { parent_id, fields, .. } => (parent_id.as_ref(), fields),
                DeclDesc::Group { fields, .. } => (None, fields),
                _ => return None,
            };

            context.visited.insert(decl, Mark::Temporary);
            let mut lscope = decl_scope(decl, result).unwrap();

            // Iterate over Struct and Group fields.
            for f in fields {
                match &f.desc {
                    FieldDesc::Group { group_id, constraints, .. } => {
                        match scope.typedef.get(group_id) {
                            None => result.push(
                                Diagnostic::error()
                                    .with_message(format!(
                                        "undeclared group identifier `{}`",
                                        group_id
                                    ))
                                    .with_labels(vec![f.loc.primary()]),
                            ),
                            Some(group_decl @ Decl { desc: DeclDesc::Group { .. }, .. }) => {
                                // Recurse to flatten the inserted group.
                                if let Some(rscope) = bfs(group_decl, context, scope, result) {
                                    // Inline the group fields and constraints into
                                    // the current scope.
                                    lscope.inline(rscope, f, constraints.iter(), result)
                                }
                            }
                            Some(_) => result.push(
                                Diagnostic::error()
                                    .with_message(format!(
                                        "invalid group field identifier `{}`",
                                        group_id
                                    ))
                                    .with_labels(vec![f.loc.primary()])
                                    .with_notes(vec!["hint: expected group identifier".to_owned()]),
                            ),
                        }
                    }
                    FieldDesc::Typedef { type_id, .. } => {
                        lscope.fields.push(f);
                        match scope.typedef.get(type_id) {
                            None => result.push(
                                Diagnostic::error()
                                    .with_message(format!(
                                        "undeclared typedef identifier `{}`",
                                        type_id
                                    ))
                                    .with_labels(vec![f.loc.primary()]),
                            ),
                            Some(struct_decl @ Decl { desc: DeclDesc::Struct { .. }, .. }) => {
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
            match (&decl.desc, parent) {
                (DeclDesc::Packet { parent_id: Some(_), .. }, None)
                | (DeclDesc::Struct { parent_id: Some(_), .. }, None) => result.push(
                    Diagnostic::error()
                        .with_message(format!(
                            "undeclared parent identifier `{}`",
                            parent_id.unwrap()
                        ))
                        .with_labels(vec![decl.loc.primary()])
                        .with_notes(vec![format!("hint: expected {} parent", decl.kind())]),
                ),
                (DeclDesc::Packet { .. }, Some(Decl { desc: DeclDesc::Struct { .. }, .. }))
                | (DeclDesc::Struct { .. }, Some(Decl { desc: DeclDesc::Packet { .. }, .. })) => {
                    result.push(
                        Diagnostic::error()
                            .with_message(format!(
                                "invalid parent identifier `{}`",
                                parent_id.unwrap()
                            ))
                            .with_labels(vec![decl.loc.primary()])
                            .with_notes(vec![format!("hint: expected {} parent", decl.kind())]),
                    )
                }
                (_, Some(parent_decl)) => {
                    if let Some(rscope) = bfs(parent_decl, context, scope, result) {
                        // Import the parent fields and constraints into the current scope.
                        lscope.inherit(rscope, decl.constraints())
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

fn decl_scope<'d>(
    decl: &'d parser::ast::Decl,
    result: &mut LintDiagnostics,
) -> Option<PacketScope<'d>> {
    match &decl.desc {
        DeclDesc::Packet { fields, .. }
        | DeclDesc::Struct { fields, .. }
        | DeclDesc::Group { fields, .. } => {
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
