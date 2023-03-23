use std::collections::HashMap;

use crate::analyzer::ast as analyzer_ast;
use crate::ast::*;

/// Gather information about the full AST.
#[derive(Debug)]
pub struct Scope<'d> {
    // Original file.
    file: &'d analyzer_ast::File,

    // Collection of Group, Packet, Enum, Struct, Checksum, and CustomField declarations.
    pub typedef: HashMap<String, &'d analyzer_ast::Decl>,

    // Collection of Packet, Struct, and Group scope declarations.
    pub scopes: HashMap<&'d analyzer_ast::Decl, PacketScope<'d>>,
}

/// Gather information about a Packet, Struct, or Group declaration.
#[derive(Debug)]
pub struct PacketScope<'d> {
    // Original decl.
    decl: &'d analyzer_ast::Decl,

    // Local and inherited field declarations. Only named fields are preserved.
    // Saved here for reference for parent constraint resolving.
    pub all_fields: HashMap<String, &'d analyzer_ast::Field>,

    // Local and inherited constraint declarations.
    // Saved here for constraint conflict checks.
    pub all_constraints: HashMap<String, &'d Constraint>,
}

impl<'d> std::hash::Hash for &'d analyzer_ast::Decl {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(*self, state);
    }
}

impl<'d> PacketScope<'d> {
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

        // Save parent fields.
        self.all_fields = parent.all_fields.clone();
    }

    /// Iterate over the packet's fields.
    pub fn iter_fields(&self) -> impl Iterator<Item = &'d analyzer_ast::Field> {
        self.decl.fields()
    }

    /// Lookup a field by name. This will also find the special
    /// `_payload_` and `_body_` fields.
    pub fn get_packet_field(&self, id: &str) -> Option<&analyzer_ast::Field> {
        self.decl.fields().find(|field| match &field.desc {
            FieldDesc::Payload { .. } => id == "_payload_",
            FieldDesc::Body { .. } => id == "_body_",
            _ => field.id() == Some(id),
        })
    }

    /// Find the payload or body field, if any.
    pub fn get_payload_field(&self) -> Option<&analyzer_ast::Field> {
        self.decl
            .fields()
            .find(|field| matches!(&field.desc, FieldDesc::Payload { .. } | FieldDesc::Body { .. }))
    }

    /// Lookup the size field for an array field.
    pub fn get_array_size_field(&self, id: &str) -> Option<&analyzer_ast::Field> {
        self.decl.fields().find(|field| match &field.desc {
            FieldDesc::Size { field_id, .. } | FieldDesc::Count { field_id, .. } => field_id == id,
            _ => false,
        })
    }

    /// Find the size field corresponding to the payload or body
    /// field of this packet.
    pub fn get_payload_size_field(&self) -> Option<&analyzer_ast::Field> {
        self.decl.fields().find(|field| match &field.desc {
            FieldDesc::Size { field_id, .. } => field_id == "_payload_" || field_id == "_body_",
            _ => false,
        })
    }

    /// Cleanup scope after processing all fields.
    fn finalize(&mut self) {
        // Check field shadowing.
        for f in self.decl.fields() {
            if let Some(id) = f.id() {
                self.all_fields.insert(id.to_string(), f);
            }
        }
    }
}

impl<'d> Scope<'d> {
    pub fn new(file: &analyzer_ast::File) -> Scope<'_> {
        let mut scope = Scope { file, typedef: HashMap::new(), scopes: HashMap::new() };

        // Gather top-level declarations.
        // Validate the top-level scopes (Group, Packet, Typedef).
        //
        // TODO: switch to try_insert when stable
        for decl in &file.declarations {
            if let Some(id) = decl.id() {
                scope.typedef.insert(id.to_string(), decl);
            }
        }

        scope.finalize();
        scope
    }

    // Sort Packet, Struct, and Group declarations by reverse topological
    // order.
    fn finalize(&mut self) -> Vec<&'d analyzer_ast::Decl> {
        // Auxiliary function implementing BFS on Packet tree.
        enum Mark {
            Temporary,
            Permanent,
        }
        struct Context<'d> {
            list: Vec<&'d analyzer_ast::Decl>,
            visited: HashMap<&'d analyzer_ast::Decl, Mark>,
            scopes: HashMap<&'d analyzer_ast::Decl, PacketScope<'d>>,
        }

        fn bfs<'s, 'd>(
            decl: &'d analyzer_ast::Decl,
            context: &'s mut Context<'d>,
            scope: &Scope<'d>,
        ) -> Option<&'s PacketScope<'d>> {
            match context.visited.get(&decl) {
                Some(Mark::Permanent) => return context.scopes.get(&decl),
                Some(Mark::Temporary) => {
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
            let mut lscope =
                PacketScope { decl, all_fields: HashMap::new(), all_constraints: HashMap::new() };

            // Iterate over Struct and Group fields.
            for f in fields {
                match &f.desc {
                    FieldDesc::Group { .. } => unreachable!(),
                    FieldDesc::Typedef { type_id, .. } => match scope.typedef.get(type_id) {
                        Some(struct_decl @ Decl { desc: DeclDesc::Struct { .. }, .. }) => {
                            bfs(struct_decl, context, scope);
                        }
                        None | Some(_) => (),
                    },
                    _ => (),
                }
            }

            // Iterate over parent declaration.
            let parent = parent_id.and_then(|id| scope.typedef.get(id));
            if let Some(parent_decl) = parent {
                if let Some(rscope) = bfs(parent_decl, context, scope) {
                    // Import the parent fields and constraints into the current scope.
                    lscope.inherit(rscope, decl.constraints())
                }
            }

            lscope.finalize();
            context.list.push(decl);
            context.visited.insert(decl, Mark::Permanent);
            context.scopes.insert(decl, lscope);
            context.scopes.get(&decl)
        }

        let mut context =
            Context::<'d> { list: vec![], visited: HashMap::new(), scopes: HashMap::new() };

        for decl in self.typedef.values() {
            bfs(decl, &mut context, self);
        }

        self.scopes = context.scopes;
        context.list
    }

    pub fn iter_children<'a>(
        &'a self,
        id: &'a str,
    ) -> impl Iterator<Item = &'d analyzer_ast::Decl> + 'a {
        self.file.iter_children(self.typedef.get(id).unwrap())
    }

    /// Return the declaration of the typedef type backing the
    /// selected field.
    pub fn get_field_declaration(
        &self,
        field: &analyzer_ast::Field,
    ) -> Option<&'d analyzer_ast::Decl> {
        match &field.desc {
            FieldDesc::FixedEnum { enum_id, .. } => self.typedef.get(enum_id).copied(),
            FieldDesc::Array { type_id: Some(type_id), .. } => self.typedef.get(type_id).copied(),
            FieldDesc::Typedef { type_id, .. } => self.typedef.get(type_id.as_str()).copied(),
            _ => None,
        }
    }

    /// Test if the selected field is a bitfield.
    pub fn is_bitfield(&self, field: &analyzer_ast::Field) -> bool {
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

    /// Determine the size of a field in bits, if possible.
    ///
    /// If the field is dynamically sized (e.g. unsized array or
    /// payload field), `None` is returned. If `skip_payload` is set,
    /// payload and body fields are counted as having size `0` rather
    /// than a variable size.
    pub fn get_field_width(
        &self,
        field: &analyzer_ast::Field,
        skip_payload: bool,
    ) -> Option<usize> {
        match &field.desc {
            FieldDesc::Scalar { width, .. }
            | FieldDesc::Size { width, .. }
            | FieldDesc::Count { width, .. }
            | FieldDesc::ElementSize { width, .. }
            | FieldDesc::Reserved { width, .. }
            | FieldDesc::FixedScalar { width, .. } => Some(*width),
            FieldDesc::Padding { .. } => todo!(),
            FieldDesc::Array { size: Some(size), width, .. } => {
                let element_width = width
                    .or_else(|| self.get_decl_width(self.get_field_declaration(field)?, false))?;
                Some(element_width * size)
            }
            FieldDesc::FixedEnum { .. } | FieldDesc::Typedef { .. } => {
                self.get_decl_width(self.get_field_declaration(field)?, false)
            }
            FieldDesc::Checksum { .. } => Some(0),
            FieldDesc::Payload { .. } | FieldDesc::Body { .. } if skip_payload => Some(0),
            _ => None,
        }
    }

    /// Determine the size of a declaration type in bits, if possible.
    ///
    /// If the type is dynamically sized (e.g. contains an array or
    /// payload), `None` is returned. If `skip_payload` is set,
    /// payload and body fields are counted as having size `0` rather
    /// than a variable size.
    pub fn get_decl_width(&self, decl: &analyzer_ast::Decl, skip_payload: bool) -> Option<usize> {
        match &decl.desc {
            DeclDesc::Enum { width, .. } | DeclDesc::Checksum { width, .. } => Some(*width),
            DeclDesc::CustomField { width, .. } => *width,
            DeclDesc::Packet { fields, parent_id, .. }
            | DeclDesc::Struct { fields, parent_id, .. } => {
                let mut packet_size = match parent_id {
                    None => 0,
                    Some(id) => self.get_decl_width(self.typedef.get(id.as_str())?, true)?,
                };
                for field in fields.iter() {
                    packet_size += self.get_field_width(field, skip_payload)?;
                }
                Some(packet_size)
            }
            DeclDesc::Group { .. } | DeclDesc::Test { .. } => None,
        }
    }
}
