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

use std::collections::HashMap;

use crate::analyzer::ast as analyzer_ast;
use crate::ast::*;

/// Gather information about the full AST.
#[derive(Debug)]
pub struct Scope<'d> {
    // Original file.
    pub file: &'d analyzer_ast::File,

    // Collection of Group, Packet, Enum, Struct, Checksum, and CustomField declarations.
    pub typedef: HashMap<String, &'d analyzer_ast::Decl>,
}

impl<'d> std::hash::Hash for &'d analyzer_ast::Decl {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(*self, state);
    }
}

impl<'d> Scope<'d> {
    pub fn new(file: &analyzer_ast::File) -> Scope<'_> {
        let mut scope = Scope { file, typedef: HashMap::new() };

        // Gather top-level declarations.
        // Validate the top-level scopes (Group, Packet, Typedef).
        //
        // TODO: switch to try_insert when stable
        for decl in &file.declarations {
            if let Some(id) = decl.id() {
                scope.typedef.insert(id.to_string(), decl);
            }
        }

        scope
    }

    /// Return the parent declaration of the selected declaration,
    /// if it has one.
    pub fn get_parent(&self, decl: &analyzer_ast::Decl) -> Option<&'d analyzer_ast::Decl> {
        decl.parent_id().and_then(|parent_id| self.typedef.get(parent_id).cloned())
    }

    pub fn iter_children<'a>(
        &'a self,
        id: &'a str,
    ) -> impl Iterator<Item = &'d analyzer_ast::Decl> + 'a {
        self.file.iter_children(self.typedef.get(id).unwrap())
    }

    /// Iterate over the packet's fields.
    pub fn iter_fields(&'d self, id: &str) -> impl Iterator<Item = &'d analyzer_ast::Field> {
        self.typedef.get(id).map(|d| analyzer_ast::Decl::fields(d)).unwrap_or([].iter())
    }

    /// Iterate over the packet's fields and inherited fields.
    pub fn iter_all_fields<'s>(
        &'s self,
        id: &str,
    ) -> impl Iterator<Item = &'d analyzer_ast::Field> + 's {
        std::iter::successors(self.typedef.get(id).cloned(), |decl| self.get_parent(decl))
            .flat_map(Decl::fields)
    }

    pub fn iter_all_parent_fields<'s>(
        &'s self,
        id: &str,
    ) -> impl Iterator<Item = &'d analyzer_ast::Field> + 's {
        let parent = self.typedef.get(id).and_then(|decl| self.get_parent(decl));
        std::iter::successors(parent, |decl| self.get_parent(decl)).flat_map(Decl::fields)
    }

    /// Iterate over the packet's constraints and inherited constraints.
    pub fn iter_all_constraints<'s>(
        &'s self,
        id: &str,
    ) -> impl Iterator<Item = &'d Constraint> + 's {
        std::iter::successors(self.typedef.get(id).cloned(), |decl| self.get_parent(decl))
            .flat_map(Decl::constraints)
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

    pub fn get_packet_field(&self, decl_id: &str, field_id: &str) -> Option<&analyzer_ast::Field> {
        self.iter_all_fields(decl_id).find(|field| match &field.desc {
            FieldDesc::Payload { .. } => field_id == "_payload_",
            FieldDesc::Body { .. } => field_id == "_body_",
            _ => field.id() == Some(field_id),
        })
    }

    pub fn get_payload_field(&self, id: &str) -> Option<&analyzer_ast::Field> {
        self.iter_fields(id)
            .find(|field| matches!(&field.desc, FieldDesc::Payload { .. } | FieldDesc::Body { .. }))
    }

    /// Lookup the size field for an array field.
    pub fn get_array_size_field(&self, packet_id: &str, id: &str) -> Option<&analyzer_ast::Field> {
        self.iter_fields(packet_id).find(|f| match &f.desc {
            FieldDesc::Size { field_id, .. } | FieldDesc::Count { field_id, .. } => field_id == id,
            _ => false,
        })
    }

    /// Find the size field corresponding to the payload or body
    /// field of this packet.
    pub fn get_payload_size_field(&self, packet_id: &str) -> Option<&analyzer_ast::Field> {
        self.iter_fields(packet_id).find(|f| match &f.desc {
            FieldDesc::Size { field_id, .. } => field_id == "_payload_" || field_id == "_body_",
            _ => false,
        })
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
}
