// Copyright 2025 Google LLC
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

use genco::{
    self,
    prelude::{java, Java},
    tokens::FormatInto,
    Tokens,
};
use heck::{self, ToLowerCamelCase, ToUpperCamelCase};
use std::{
    cmp,
    collections::HashMap,
    fs::{self, OpenOptions},
    iter,
    path::{Path, PathBuf},
};

use crate::{
    ast::{self, Constraint, EndiannessValue, Tag, TagOther, TagRange, TagValue},
    backends::common::alignment::{AlignedSymbol, ByteAligner, Chunk, UnalignedSymbol},
};

use super::common::alignment::Alignment;

pub mod test;
pub mod import {
    use genco::prelude::java;
    use once_cell::sync::Lazy;

    pub static BO: Lazy<java::Import> = Lazy::new(|| java::import("java.nio", "ByteOrder"));
    pub static BB: Lazy<java::Import> = Lazy::new(|| java::import("java.nio", "ByteBuffer"));
    pub static ARRAYS: Lazy<java::Import> = Lazy::new(|| java::import("java.util", "Arrays"));
}

mod codegen;

pub fn generate(
    sources: &ast::SourceDatabase,
    file: &ast::File,
    custom_fields: &[String],
    output_dir: &Path,
    package: &str,
) -> Result<(), String> {
    let mut dir = PathBuf::from(output_dir);
    dir.extend(package.split("."));
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let classes = generate_classes(&file);

    for (name, class) in classes.into_iter() {
        class.write_to_fs(
            &dir.join(name).with_extension("java"),
            package,
            sources.get(file.file).expect("could not read source").name(),
            file.endianness.value,
        )?;
    }

    Ok(())
}

fn generate_classes(file: &ast::File) -> HashMap<String, Class> {
    let mut classes: HashMap<String, Class> = HashMap::new();

    for decl in file.declarations.iter() {
        match &decl.desc {
            // If this is a parent packet, make a new abstract class and defer parenthood to it.
            ast::DeclDesc::Packet { id, fields, parent_id, constraints }
            | ast::DeclDesc::Struct { id, constraints, fields, parent_id }
                if fields.iter().any(|field| {
                    matches!(&field.desc, ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body)
                }) =>
            {
                let parent_name = id.to_upper_camel_case();

                let (mut parent, child) = Class::new_parent(
                    parent_name.clone(),
                    PacketDef::from_fields(fields, &classes),
                );

                // This parent might also be a child
                if let Some(parent_id) = parent_id {
                    let grandparent = classes
                        .get_mut(&parent_id.to_upper_camel_case())
                        .expect("Packet inherits from unknown parent");

                    grandparent.add_child(
                        &mut parent,
                        constraints.iter().map(Constraint::to_assignment).collect(),
                        None,
                    );
                }

                classes.insert(parent_name, parent);
                classes.insert(child.name().into(), child);
            }
            // If this is a child packet, set its parent to the appropriate abstract class.
            ast::DeclDesc::Packet { id, constraints, fields, parent_id: Some(parent_id) }
            | ast::DeclDesc::Struct { id, constraints, fields, parent_id: Some(parent_id) } => {
                let child_name = id.to_upper_camel_case();
                let def = PacketDef::from_fields(fields, &classes);

                let parent = classes
                    .get_mut(&parent_id.to_upper_camel_case())
                    .expect("Packet inherits from unknown parent");

                let child = parent.new_child(
                    child_name.clone(),
                    def,
                    constraints.iter().map(Constraint::to_assignment).collect(),
                );

                classes.insert(child_name, child);
            }
            // Otherwise, the packet has no inheritence (no parent and no children)
            ast::DeclDesc::Packet { id, fields, parent_id: None, .. }
            | ast::DeclDesc::Struct { id, fields, parent_id: None, .. } => {
                let name = id.to_upper_camel_case();
                classes.insert(
                    name.clone(),
                    Class::Packet {
                        name,
                        def: PacketDef::from_fields(fields, &classes),
                        parent: None,
                    },
                );
            }
            ast::DeclDesc::Enum { id, tags, width } => {
                let name = id.to_upper_camel_case();
                classes
                    .insert(name.clone(), Class::Enum { name, tags: tags.clone(), width: *width });
            }
            _ => {
                dbg!(decl);
                todo!()
            }
        }
    }

    dbg!(&classes);
    classes
}

trait JavaFile<C>: Sized {
    fn generate(self, context: C) -> Tokens<Java>;

    fn write_to_fs(
        self,
        path: &PathBuf,
        package: &str,
        from_pdl: &str,
        context: C,
    ) -> Result<(), String> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .map_err(|err| err.to_string())?;

        let mut w = genco::fmt::IoWriter::new(file);
        let fmt = genco::fmt::Config::from_lang::<Java>().with_newline("\n");
        let config = java::Config::default().with_package(package);

        let mut tokens = Tokens::new();
        java::block_comment(iter::once(format!("GENERATED BY PDL COMPILER FROM {}", from_pdl)))
            .format_into(&mut tokens);
        tokens.extend(self.generate(context));
        tokens.format_file(&mut w.as_formatter(&fmt), &config).map_err(|e| e.to_string())
    }
}

#[derive(Debug, Clone)]
pub enum Class {
    Packet { name: String, def: PacketDef, parent: Option<Parent> },
    AbstractPacket { name: String, def: PacketDef, parent: Option<Parent>, children: Vec<Child> },
    Enum { name: String, tags: Vec<Tag>, width: usize },
}

impl Class {
    fn new_parent(name: String, def: PacketDef) -> (Self, Self) {
        let child_name = format!("Unknown{}", name);
        let child_member =
            CompoundVal::Payload { size_field: def.size_fields.get("payload").cloned() };
        let child_alignment = {
            let mut aligner = ByteAligner::new(64);
            aligner.add_bytes(child_member.clone());
            aligner.align().unwrap()
        };

        // TODO: Only create child packet here if this packet contains a payload. If its a parent with a
        // body field, we don't want a fallback child

        (
            Class::AbstractPacket {
                name: name.clone(),
                def,
                parent: None,
                children: vec![Child {
                    name: child_name.clone(),
                    constraints: HashMap::new(),
                    width: None,
                }],
            },
            Class::Packet {
                name: child_name,
                def: PacketDef {
                    members: vec![child_member.into()],
                    alignment: child_alignment,
                    width: None,
                    size_fields: HashMap::new(),
                },
                parent: Some(Parent { name, does_constrain: false }),
            },
        )
    }

    fn new_child(
        &mut self,
        name: String,
        def: PacketDef,
        constraints: HashMap<String, ConstrainedTo>,
    ) -> Self {
        let mut child = Class::Packet { name, def, parent: None };

        let child_width = child.width();
        self.add_child(&mut child, constraints, child_width);

        child
    }

    fn add_child(
        &mut self,
        child: &mut Class,
        constraints: HashMap<String, ConstrainedTo>,
        child_width: Option<usize>,
    ) {
        let childs_parent = match child {
            Class::Packet { parent, .. } | Class::AbstractPacket { parent, .. } => parent,
            _ => panic!("Can't add child to non-packet"),
        };
        let _ = childs_parent
            .insert(Parent { name: self.name().into(), does_constrain: !constraints.is_empty() });

        let children = (if let Class::AbstractPacket { ref mut children, .. } = self {
            Some(children)
        } else {
            None
        })
        .expect("Attempt to add child to non-parent packet");
        children.push(Child { name: child.name().into(), constraints, width: child_width });
    }

    pub fn name(&self) -> &str {
        match self {
            Class::Packet { name, .. }
            | Class::AbstractPacket { name, .. }
            | Class::Enum { name, .. } => name,
        }
    }

    pub fn width(&self) -> Option<usize> {
        match self {
            Class::Packet { def, .. } | Class::AbstractPacket { def, .. } => def.width,
            Class::Enum { width, .. } => Some(*width),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PacketDef {
    members: Vec<Member>,
    alignment: Alignment<ScalarVal, CompoundVal>,
    width: Option<usize>,
    size_fields: HashMap<String, usize>,
}

impl PacketDef {
    fn from_fields(fields: &Vec<ast::Field>, classes: &HashMap<String, Class>) -> Self {
        let mut members = Vec::new();
        let mut aligner = ByteAligner::new(64);
        let mut field_width = Some(0);
        let mut size_fields: HashMap<String, usize> = HashMap::new();

        for field in fields.iter() {
            match &field.desc {
                ast::FieldDesc::Scalar { id, width } => {
                    let member = ScalarVal::Integral {
                        name: id.to_lower_camel_case(),
                        ty: Integral::fitting(*width),
                        width: *width,
                    };

                    members.push(member.clone().into());
                    aligner.add_bitfield(member);
                    field_width = field_width.map(|total_width| total_width + width);
                }
                ast::FieldDesc::Payload { size_modifier } => {
                    aligner.add_bytes(CompoundVal::Payload {
                        size_field: size_fields.get("payload").cloned(),
                    });
                    field_width = None;
                }
                ast::FieldDesc::Size { field_id, width } => {
                    let member = ScalarVal::Integral {
                        name: format!("{}Size", field_id.to_lower_camel_case()),
                        ty: Integral::Int,
                        width: *width,
                    };
                    aligner.add_bitfield(member);
                    size_fields.insert(field_id.to_lower_camel_case(), *width);
                }
                ast::FieldDesc::Typedef { id, type_id } => {
                    let class = classes.get(&type_id.to_upper_camel_case()).unwrap();
                    match &class {
                        Class::Enum { width, .. } => {
                            let member = ScalarVal::EnumRef {
                                name: id.to_lower_camel_case(),
                                ty: class.name().into(),
                                width: *width,
                            };

                            members.push(member.clone().into());
                            aligner.add_bitfield(member);
                            field_width = field_width.map(|total_width| total_width + width);
                        }
                        _ => {
                            let member = CompoundVal::StructRef {
                                name: id.to_lower_camel_case(),
                                ty: class.name().into(),
                            };
                            members.push(member.clone().into());
                            aligner.add_bytes(member);
                            field_width = if let Some((field_width, class_width)) =
                                field_width.zip(class.width())
                            {
                                Some(field_width + class_width)
                            } else {
                                None
                            };
                        }
                    }
                }
                _ => {
                    dbg!(field);
                    todo!()
                }
            }
        }

        Self {
            members,
            alignment: aligner.align().expect("failed to align members"),
            width: field_width,
            size_fields: size_fields,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Parent {
    name: String,
    does_constrain: bool,
}

#[derive(Debug, Clone)]
pub struct Child {
    name: String,
    constraints: HashMap<String, ConstrainedTo>,
    width: Option<usize>,
}

impl Tag {
    fn name(&self) -> String {
        match self {
            Tag::Value(TagValue { id, .. })
            | Tag::Range(TagRange { id, .. })
            | Tag::Other(TagOther { id, .. }) => id.to_upper_camel_case(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConstrainedTo {
    Integral(usize),
    EnumTag(String),
}

impl Constraint {
    fn to_assignment(&self) -> (String, ConstrainedTo) {
        (
            self.id.to_lower_camel_case(),
            self.value
                .map(|integral| ConstrainedTo::Integral(integral))
                .or(self.tag_id.as_ref().map(|id| ConstrainedTo::EnumTag(id.to_upper_camel_case())))
                .expect("Malformed constraint"),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Member {
    Scalar(ScalarVal),
    Compound(CompoundVal),
}

impl Member {
    fn as_scalar(&self) -> Option<&ScalarVal> {
        if let Self::Scalar(member) = self {
            Some(member)
        } else {
            None
        }
    }

    fn as_compound(&self) -> Option<&CompoundVal> {
        if let Self::Compound(member) = self {
            Some(member)
        } else {
            None
        }
    }
}

impl From<ScalarVal> for Member {
    fn from(value: ScalarVal) -> Self {
        Self::Scalar(value)
    }
}

impl From<CompoundVal> for Member {
    fn from(value: CompoundVal) -> Self {
        Self::Compound(value)
    }
}

impl Member {
    pub fn name(&self) -> &str {
        match self {
            Member::Scalar(member) => member.name(),
            Member::Compound(member) => member.name(),
        }
    }

    pub fn is_sized(&self) -> bool {
        matches!(self, Member::Scalar(_))
    }
}

/// A value that is readily represented by a scalar Java type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScalarVal {
    Integral { name: String, ty: Integral, width: usize },
    EnumRef { name: String, ty: String, width: usize },
}

impl UnalignedSymbol for ScalarVal {
    fn width(&self) -> usize {
        match self {
            ScalarVal::Integral { width, .. } | ScalarVal::EnumRef { width, .. } => *width,
        }
    }
}

impl ScalarVal {
    fn name(&self) -> &str {
        match self {
            ScalarVal::Integral { name, .. } | ScalarVal::EnumRef { name, .. } => name,
        }
    }
}

/// A value that is readily represented by a compound (non-scalar) Java type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompoundVal {
    StructRef { name: String, ty: String },
    Payload { size_field: Option<usize> },
}

impl AlignedSymbol for CompoundVal {}

impl CompoundVal {
    fn name(&self) -> &str {
        match self {
            CompoundVal::StructRef { name, .. } => name,
            CompoundVal::Payload { .. } => "payload",
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Integral {
    Byte,
    Short,
    Int,
    Long,
}

impl Integral {
    pub fn fitting(width: impl Into<usize>) -> Self {
        let width: usize = width.into();
        if width <= 8 {
            Integral::Byte
        } else if width <= 16 {
            Integral::Short
        } else if width <= 32 {
            Integral::Int
        } else if width <= 64 {
            Integral::Long
        } else {
            panic!("Width too large!")
        }
    }

    /// Widen to Int to avoid widening primitive conversion.
    pub fn limit_to_int(self) -> Self {
        cmp::max(self, Integral::Int)
    }

    pub fn width(&self) -> usize {
        match self {
            Integral::Byte => 8,
            Integral::Short => 16,
            Integral::Int => 32,
            Integral::Long => 64,
        }
    }
}

impl From<&ScalarVal> for Integral {
    fn from(member: &ScalarVal) -> Self {
        Integral::fitting(member.width())
    }
}
