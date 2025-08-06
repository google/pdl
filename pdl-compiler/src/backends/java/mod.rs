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
    ast::{self, EndiannessValue, Tag, TagOther, TagRange, TagValue},
    backends::{
        common::alignment::{ByteAligner, Chunk},
        java::inheritance::{ClassHeirarchy, Constraint},
    },
};

use super::common::alignment::Alignment;

pub mod test;
pub mod import {
    use genco::prelude::java;
    use once_cell::sync::Lazy;

    pub static BO: Lazy<java::Import> = Lazy::new(|| java::import("java.nio", "ByteOrder"));
    pub static BB: Lazy<java::Import> = Lazy::new(|| java::import("java.nio", "ByteBuffer"));
    pub static ARRAYS: Lazy<java::Import> = Lazy::new(|| java::import("java.util", "Arrays"));
    pub static LIST: Lazy<java::Import> = Lazy::new(|| java::import("java.util", "ArrayList"));
}

mod codegen;
mod inheritance;

pub fn generate(
    sources: &ast::SourceDatabase,
    file: &ast::File,
    _: &[String],
    output_dir: &Path,
    package: &str,
) -> Result<(), String> {
    let mut dir = PathBuf::from(output_dir);
    dir.extend(package.split("."));
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let (classes, heirarchy) = generate_classes(&file);
    let context = Context { endianness: file.endianness.value, heirarchy };

    for (name, class) in classes.into_iter() {
        class.write_to_fs(
            &dir.join(name).with_extension("java"),
            package,
            sources.get(file.file).expect("could not read source").name(),
            &context,
        )?;
    }

    Ok(())
}

fn generate_classes(file: &ast::File) -> (HashMap<String, Class>, ClassHeirarchy) {
    let mut classes: HashMap<String, Class> = HashMap::new();
    let mut heirarchy = ClassHeirarchy::new();

    for decl in file.declarations.iter() {
        match &decl.desc {
            // If this is a parent packet, make a new abstract class and defer parenthood to it.
            ast::DeclDesc::Packet { id, fields, parent_id, constraints }
            | ast::DeclDesc::Struct { id, constraints, fields, parent_id }
                if fields.iter().any(|field| {
                    matches!(&field.desc, ast::FieldDesc::Payload { .. } | ast::FieldDesc::Body)
                }) =>
            {
                let parent_name = Class::name_from_id(id);
                let parent_def = PacketDef::from_fields(fields, &classes, &heirarchy);

                if let Some(parent_id) = parent_id {
                    let grandparent = classes
                        .get_mut(&Class::name_from_id(parent_id))
                        .expect("Packet inherits from unknown parent");

                    heirarchy.add_child(
                        String::from(grandparent.name()),
                        parent_name.clone(),
                        constraints.iter().map(ast::Constraint::to_assignment).collect(),
                        &parent_def.members,
                    );
                } else {
                    heirarchy.add_class(parent_name.clone(), &parent_def.members);
                }

                heirarchy.add_child(
                    String::from(parent_name.clone()),
                    ClassHeirarchy::default_child_name(&parent_name),
                    HashMap::new(),
                    &vec![Field::Payload { is_member: true }],
                );

                let (parent, child) =
                    Class::new_parent_with_default_child(parent_name.clone(), parent_def);

                classes.insert(parent_name, parent);
                classes.insert(child.name().into(), child);
            }
            // If this is a child packet, set its parent to the appropriate abstract class.
            ast::DeclDesc::Packet { id, constraints, fields, parent_id: Some(parent_id) }
            | ast::DeclDesc::Struct { id, constraints, fields, parent_id: Some(parent_id) } => {
                let child_name = Class::name_from_id(id);
                let def = PacketDef::from_fields(fields, &classes, &heirarchy);

                let parent = classes
                    .get_mut(&Class::name_from_id(parent_id))
                    .expect("Packet inherits from unknown parent");

                heirarchy.add_child(
                    String::from(parent.name()),
                    child_name.clone(),
                    constraints.iter().map(ast::Constraint::to_assignment).collect(),
                    &def.members,
                );
                classes.insert(child_name.clone(), Class::Packet { name: child_name, def });
            }
            // Otherwise, the packet has no inheritence (no parent and no children)
            ast::DeclDesc::Packet { id, fields, parent_id: None, .. }
            | ast::DeclDesc::Struct { id, fields, parent_id: None, .. } => {
                let name = Class::name_from_id(id);
                let def = PacketDef::from_fields(fields, &classes, &heirarchy);

                heirarchy.add_class(name.clone(), &def.members);
                classes.insert(name.clone(), Class::Packet { name, def });
            }
            ast::DeclDesc::Enum { id, tags, width } => {
                let name = Class::name_from_id(id);
                classes.insert(
                    name.clone(),
                    Class::Enum {
                        name,
                        tags: tags.clone(),
                        width: *width,
                        fallback_tag: tags.iter().find_map(|tag| {
                            if let Tag::Other(fallback) = tag {
                                Some(fallback.clone())
                            } else {
                                None
                            }
                        }),
                    },
                );
            }
            _ => {
                dbg!(decl);
                todo!()
            }
        }
    }

    // dbg!(&classes);
    // dbg!(&heirarchy);
    // dbg!(classes
    //     .values()
    //     .filter_map(|class| heirarchy
    //         .width(class.name())
    //         .map(|width| (String::from(class.name()), width)))
    //     .collect::<Vec<(String, usize)>>());
    (classes, heirarchy)
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

pub struct Context {
    endianness: EndiannessValue,
    heirarchy: ClassHeirarchy,
}

#[derive(Debug, Clone)]
pub enum Class {
    Packet { name: String, def: PacketDef },
    AbstractPacket { name: String, def: PacketDef },
    Enum { name: String, tags: Vec<Tag>, width: usize, fallback_tag: Option<TagOther> },
}

impl Class {
    fn name_from_id(id: &str) -> String {
        if id.ends_with("_") {
            format!("{}_", id.to_upper_camel_case())
        } else {
            id.to_upper_camel_case()
        }
    }

    fn new_parent_with_default_child(name: String, def: PacketDef) -> (Self, Self) {
        let child_alignment = {
            let mut aligner = ByteAligner::new(&[8, 16, 32, 64]);
            aligner.add_bytes(Field::Payload { is_member: true });
            aligner.align().unwrap()
        };

        // TODO: Only create child packet here if this packet contains a payload. If its a parent with a
        // body field, we don't want a fallback child

        (
            Class::AbstractPacket { name: name.clone(), def },
            Class::Packet {
                name: ClassHeirarchy::default_child_name(&name),
                def: PacketDef {
                    members: vec![Field::Payload { is_member: true }],
                    alignment: child_alignment,
                    width_fields: HashMap::new(),
                },
            },
        )
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
            Class::Enum { width, .. } => Some(*width),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PacketDef {
    members: Vec<Field>,
    alignment: Alignment<Field>,
    width_fields: HashMap<String, WidthField>,
}

impl PacketDef {
    fn from_fields(
        fields: &Vec<ast::Field>,
        classes: &HashMap<String, Class>,
        heirarchy: &ClassHeirarchy,
    ) -> Self {
        let mut members: Vec<Field> = Vec::new();
        let mut aligner = ByteAligner::new(&[8, 16, 32, 64]);
        let mut width_fields: HashMap<String, WidthField> = HashMap::new();
        let mut staged_size_fields: HashMap<String, usize> = HashMap::new();

        for field in fields.iter() {
            match &field.desc {
                ast::FieldDesc::Scalar { id, width } => {
                    let member = Field::Integral {
                        name: id.to_lower_camel_case(),
                        ty: Integral::fitting(*width),
                        width: *width,
                        is_member: true,
                    };

                    members.push(member.clone().into());
                    aligner.add_bitfield(member, *width);
                }
                ast::FieldDesc::Reserved { width } => {
                    let member = Field::Reserved { width: *width };
                    members.push(member.clone());
                    aligner.add_bitfield(member, *width);
                }
                ast::FieldDesc::Payload { .. } => {
                    let member = Field::Payload { is_member: false };
                    members.push(member.clone());
                    aligner.add_bytes(member);
                    if let Some(width) = staged_size_fields.remove("payload") {
                        width_fields.insert(
                            String::from("payload"),
                            WidthField::Size { field_width: width, elem_width: Some(8) },
                        );
                    }
                }
                ast::FieldDesc::Size { field_id, width } => {
                    let member = Field::Integral {
                        name: format!("{}Size", field_id.to_lower_camel_case()),
                        ty: Integral::Int,
                        width: *width,
                        is_member: false,
                    };
                    members.push(member.clone());
                    aligner.add_bitfield(member, *width);
                    staged_size_fields.insert(field_id.to_lower_camel_case(), *width);
                }
                ast::FieldDesc::Count { field_id, width } => {
                    let member = Field::Integral {
                        name: format!("{}Count", field_id.to_lower_camel_case()),
                        ty: Integral::Int,
                        width: *width,
                        is_member: false,
                    };
                    members.push(member.clone());
                    aligner.add_bitfield(member, *width);
                    width_fields.insert(
                        field_id.to_lower_camel_case(),
                        WidthField::Count { field_width: *width },
                    );
                }
                ast::FieldDesc::Typedef { id, type_id } => {
                    let class = classes.get(&Class::name_from_id(type_id)).unwrap();
                    match &class {
                        Class::Enum { width, .. } => {
                            let member = Field::EnumRef {
                                name: id.to_lower_camel_case(),
                                ty: class.name().into(),
                                width: *width,
                            };

                            members.push(member.clone().into());
                            aligner.add_bitfield(member, *width);
                        }
                        _ => {
                            let member = Field::StructRef {
                                name: id.to_lower_camel_case(),
                                ty: class.name().into(),
                            };
                            members.push(member.clone().into());
                            aligner.add_bytes(member);
                        }
                    }
                }
                ast::FieldDesc::Array { id, width, type_id, size: count, .. } => {
                    let (member, elem_width) = match (width, type_id) {
                        (Some(width), None) => {
                            let val = Field::ArrayElem {
                                val: Box::new(
                                    Field::Integral {
                                        name: id.to_lower_camel_case(),
                                        ty: Integral::fitting(*width),
                                        width: *width,
                                        is_member: true,
                                    }
                                    .into(),
                                ),
                                count: *count,
                            };
                            aligner.add_sized_bytes(val.clone(), *width);

                            (val, Some(*width))
                        }
                        (None, Some(type_id)) => {
                            let class = classes.get(&Class::name_from_id(type_id)).unwrap();
                            (
                                if let Class::Enum { width, .. } = class {
                                    let val = Field::ArrayElem {
                                        val: Box::new(
                                            Field::EnumRef {
                                                name: id.to_lower_camel_case(),
                                                ty: class.name().into(),
                                                width: *width,
                                            }
                                            .into(),
                                        ),
                                        count: *count,
                                    };
                                    aligner.add_sized_bytes(val.clone(), *width);
                                    val
                                } else {
                                    let val = Field::ArrayElem {
                                        val: Box::new(
                                            Field::StructRef {
                                                name: id.to_lower_camel_case(),
                                                ty: class.name().into(),
                                            }
                                            .into(),
                                        ),
                                        count: *count,
                                    };
                                    aligner.add_bytes(val.clone());
                                    val
                                },
                                class.width().or_else(|| heirarchy.width(class.name())),
                            )
                        }
                        _ => panic!("invalid array field"),
                    };

                    if let Some(size_field_width) = staged_size_fields.remove(member.name()) {
                        width_fields.insert(
                            String::from(member.name()),
                            WidthField::Size {
                                field_width: size_field_width,
                                elem_width: elem_width,
                            },
                        );
                    }
                    members.push(member.into());
                }
                _ => {
                    dbg!(field);
                    todo!()
                }
            }
        }

        Self { members, alignment: aligner.align().expect("failed to align members"), width_fields }
    }
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

impl TagValue {
    fn name(&self) -> String {
        self.id.to_upper_camel_case()
    }
}

impl TagRange {
    fn name(&self) -> String {
        self.id.to_upper_camel_case()
    }
}

impl TagOther {
    fn name(&self) -> String {
        self.id.to_upper_camel_case()
    }
}

impl ast::Constraint {
    fn to_assignment(&self) -> (String, Constraint) {
        (
            self.id.to_lower_camel_case(),
            self.value
                .map(|integral| Constraint::Integral(integral))
                .or(self.tag_id.as_ref().map(|id| Constraint::EnumTag(id.to_upper_camel_case())))
                .expect("Malformed constraint"),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WidthField {
    Size { field_width: usize, elem_width: Option<usize> },
    Count { field_width: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field {
    Integral { name: String, ty: Integral, width: usize, is_member: bool },
    Reserved { width: usize },
    EnumRef { name: String, ty: String, width: usize },
    StructRef { name: String, ty: String },
    Payload { is_member: bool },
    ArrayElem { val: Box<Field>, count: Option<usize> },
}

impl Field {
    pub fn name(&self) -> &str {
        match self {
            Field::Integral { name, .. } | Field::EnumRef { name, .. } => name,
            Field::Reserved { .. } => "reserved",
            Field::StructRef { name, .. } => name,
            Field::Payload { .. } => "payload",
            Field::ArrayElem { val, .. } => val.name(),
        }
    }

    pub fn width(&self) -> Option<usize> {
        match self {
            Field::Integral { width, .. }
            | Field::EnumRef { width, .. }
            | Field::Reserved { width } => Some(*width),
            _ => None,
        }
    }

    pub fn class(&self) -> Option<&String> {
        match self {
            Field::EnumRef { ty, .. } | Field::StructRef { ty, .. } => Some(ty),
            _ => None,
        }
    }

    pub fn is_member(&self) -> bool {
        match self {
            Field::Integral { is_member, .. } | Field::Payload { is_member } => *is_member,
            Field::Reserved { .. } => false,
            _ => true,
        }
    }

    pub fn is_reserved(&self) -> bool {
        matches!(self, Self::Reserved { .. })
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
        Self::try_fitting(width).expect("width too large!")
    }

    pub fn try_fitting(width: impl Into<usize>) -> Option<Self> {
        let width: usize = width.into();
        if width <= 8 {
            Some(Integral::Byte)
        } else if width <= 16 {
            Some(Integral::Short)
        } else if width <= 32 {
            Some(Integral::Int)
        } else if width <= 64 {
            Some(Integral::Long)
        } else {
            None
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

impl TryFrom<&Field> for Integral {
    type Error = ();

    fn try_from(value: &Field) -> Result<Self, Self::Error> {
        match value.width() {
            Some(width) => Ok(Integral::fitting(width)),
            None => Err(()),
        }
    }
}
