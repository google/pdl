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
    cell::RefCell,
    cmp,
    collections::{HashMap, HashSet},
    fs::{self, OpenOptions},
    iter,
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::{
    ast::{self, Constraint, EndiannessValue, Tag, TagOther, TagRange, TagValue},
    backends::common::alignment::{ByteAligner, Chunk},
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
    let parent_packets: HashSet<&String> = file
        .declarations
        .iter()
        .flat_map(|decl| {
            if let ast::DeclDesc::Packet { parent_id: Some(parent_id), .. } = &decl.desc {
                Some(parent_id)
            } else {
                None
            }
        })
        .collect();

    let mut classes: HashMap<String, Class> = HashMap::new();

    for decl in file.declarations.iter() {
        match &decl.desc {
            // If this is a parent packet, make a new abstract class and defer parenthood to it.
            ast::DeclDesc::Packet { id, fields, parent_id, constraints }
                if parent_packets.contains(id) =>
            {
                let parent_name = id.to_upper_camel_case();
                let (members, alignment, width) = generate_members(fields, &classes);

                let (mut parent, child) = Class::new_parent_with_fallback_child(
                    parent_name.clone(),
                    members,
                    alignment,
                    width,
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
                classes.insert(child.name.clone(), child);
            }
            // If this is a child packet, set its parent to the appropriate abstract class.
            ast::DeclDesc::Packet { id, constraints, fields, parent_id: Some(parent_id) } => {
                let child_name = id.to_upper_camel_case();
                let (members, alignment, width) = generate_members(fields, &classes);

                let parent = classes
                    .get_mut(&parent_id.to_upper_camel_case())
                    .expect("Packet inherits from unknown parent");

                let child = parent.new_child(
                    child_name.clone(),
                    members,
                    alignment,
                    width,
                    constraints.iter().map(Constraint::to_assignment).collect(),
                );

                classes.insert(child_name, child);
            }
            // Otherwise, the packet has no inheritence (no parent and no children)
            ast::DeclDesc::Packet { id, fields, parent_id: None, .. } => {
                let (members, alignment, width) = generate_members(fields, &classes);
                let name = id.to_upper_camel_case();
                classes.insert(
                    name.clone(),
                    Class {
                        name,
                        def: ClassDef::Packet(PacketDef {
                            members,
                            alignment,
                            width,
                            parent: None,
                            children: Vec::new(),
                        }),
                    },
                );
            }
            ast::DeclDesc::Enum { id, tags, width } => {
                let name = id.to_upper_camel_case();
                classes.insert(
                    name.clone(),
                    Class { name, def: ClassDef::Enum { tags: tags.clone(), width: *width } },
                );
            }
            _ => {
                dbg!(decl);
                todo!()
            }
        }
    }

    // dbg!(&classes
    //     .iter()
    //     .map(|(name, class)| (name, &class.def))
    //     .collect::<Vec<(&String, &ClassDef)>>());
    classes
}

fn generate_members(
    fields: &Vec<ast::Field>,
    classes: &HashMap<String, Class>,
) -> (Vec<Rc<Variable>>, Alignment<Rc<Variable>>, Option<usize>) {
    let mut members = Vec::new();
    let mut aligner = ByteAligner::new(64);
    let mut total_width = Some(0);

    for field in fields.iter() {
        match &field.desc {
            ast::FieldDesc::Scalar { id, width } => {
                let variable = Rc::new(Variable {
                    name: id.to_lower_camel_case(),
                    ty: Type::Integral {
                        ty: Integral::fitting(*width).limit_to_int(),
                        width: *width,
                    },
                });
                members.push(variable.clone());
                aligner.add_bitfield(variable, *width);
                total_width = total_width.map(|total_width| total_width + width);
            }
            ast::FieldDesc::Payload { size_modifier } => {
                aligner.add_bytes(Rc::new(Variable::new_payload()));
            }
            ast::FieldDesc::Size { field_id, width } => {}
            ast::FieldDesc::Typedef { id, type_id } => {
                let class = classes.get(&type_id.to_upper_camel_case()).unwrap();
                let variable = Rc::new(Variable {
                    name: id.to_lower_camel_case(),
                    ty: Type::Class { name: class.name.clone(), width: class.width() },
                });
                members.push(variable.clone());
                if let ClassDef::Enum { width, .. } = &class.def {
                    aligner.add_bitfield(variable, *width);
                    total_width = total_width.map(|total_width| total_width + width);
                } else {
                    aligner.add_bytes(variable);
                    total_width =
                        if let Some((total_width, class_width)) = total_width.zip(class.width()) {
                            Some(total_width + class_width)
                        } else {
                            None
                        };
                }
            }
            _ => {
                dbg!(field);
                todo!()
            }
        }
    }

    (members, aligner.align().expect("Failed to align members"), total_width)
}

trait JavaFile<C>: Sized {
    fn gen(self, context: C) -> Tokens<Java>;

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
        tokens.extend(self.gen(context));
        tokens.format_file(&mut w.as_formatter(&fmt), &config).map_err(|e| e.to_string())
    }
}

pub struct Class {
    name: String,
    def: ClassDef,
}

impl Class {
    fn new_parent_with_fallback_child(
        name: String,
        members: Vec<Rc<Variable>>,
        alignment: Alignment<Rc<Variable>>,
        width: Option<usize>,
    ) -> (Self, Self) {
        let child_name = Self::fallback_child_name(&name);
        let child_member = Rc::new(Variable::new_payload());
        let child_alignment = {
            let mut aligner = ByteAligner::new(64);
            aligner.add_bytes(child_member.clone());
            aligner.align().unwrap()
        };

        // TODO: Only create child packet here if this packet contains a payload. If its a parent with a
        // body field, we don't want a 'default' child

        (
            Class {
                name: name.clone(),
                def: ClassDef::AbstractPacket(PacketDef {
                    members,
                    alignment,
                    width,
                    parent: None,
                    children: vec![Child {
                        name: child_name.clone(),
                        constraints: HashMap::new(),
                        width: None,
                    }],
                }),
            },
            Class {
                name: child_name,
                def: ClassDef::Packet(PacketDef {
                    members: vec![child_member],
                    alignment: child_alignment,
                    width,
                    parent: Some(Parent { name, does_constrain: false }),
                    children: vec![],
                }),
            },
        )
    }

    fn new_child(
        &mut self,
        name: String,
        members: Vec<Rc<Variable>>,
        alignment: Alignment<Rc<Variable>>,
        width: Option<usize>,
        constraints: HashMap<String, RValue>,
    ) -> Self {
        let mut child = Class {
            name,
            def: ClassDef::Packet(PacketDef {
                members,
                alignment,
                width,
                parent: None,
                children: vec![],
            }),
        };

        let child_width = child.width();
        self.add_child(&mut child, constraints, child_width);

        child
    }

    fn add_child(
        &mut self,
        child: &mut Class,
        constraints: HashMap<String, RValue>,
        child_width: Option<usize>,
    ) {
        let child_def = match &mut child.def {
            ClassDef::Packet(def) => def,
            ClassDef::AbstractPacket(def) => def,
            _ => panic!("Can't add child to non-packet"),
        };
        let _ = child_def
            .parent
            .insert(Parent { name: self.name.clone(), does_constrain: !constraints.is_empty() });

        let children =
            (if let ClassDef::AbstractPacket(PacketDef { ref mut children, .. }) = &mut self.def {
                Some(children)
            } else {
                None
            })
            .expect("Attempt to add child to non-parent packet");
        children.push(Child { name: child.name.clone(), constraints, width: child_width });
    }

    fn fallback_child_name(parent_name: &str) -> String {
        format!("Unknown{}", parent_name)
    }

    pub fn width(&self) -> Option<usize> {
        match self.def {
            ClassDef::Packet(PacketDef { width, .. })
            | ClassDef::AbstractPacket(PacketDef { width, .. }) => width,
            ClassDef::Enum { width, .. } => Some(width),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ClassDef {
    Packet(PacketDef),
    AbstractPacket(PacketDef),
    Enum { tags: Vec<Tag>, width: usize },
}

#[derive(Debug, Clone)]
pub struct PacketDef {
    members: Vec<Rc<Variable>>,
    alignment: Alignment<Rc<Variable>>,
    width: Option<usize>,
    parent: Option<Parent>,
    children: Vec<Child>,
}

#[derive(Debug, Clone)]
pub struct Parent {
    name: String,
    does_constrain: bool,
}

#[derive(Debug, Clone)]
pub struct Child {
    name: String,
    constraints: HashMap<String, RValue>,
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
pub enum RValue {
    Integral(usize),
    EnumTag(String),
}

impl Constraint {
    fn to_assignment(&self) -> (String, RValue) {
        (
            self.id.to_lower_camel_case(),
            self.value
                .map(|integral| RValue::Integral(integral))
                .or(self.tag_id.as_ref().map(|id| RValue::EnumTag(id.to_upper_camel_case())))
                .expect("Malformed constraint"),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    name: String,
    ty: Type,
}

impl Variable {
    fn new_payload() -> Self {
        Variable { name: String::from("payload"), ty: Type::Payload }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Integral { ty: Integral, width: usize },
    Payload,
    Class { name: String, width: Option<usize> },
}

impl Type {
    fn width(&self) -> Option<usize> {
        match self {
            Type::Integral { width, .. } => Some(*width),
            Type::Class { width, .. } => *width,
            Type::Payload => None,
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

    /// The JLS specifies that operands of certain operators including
    ///  - [shifts](https://docs.oracle.com/javase/specs/jls/se8/html/jls-15.html#jls-15.19)
    ///  - [bitwise operators](https://docs.oracle.com/javase/specs/jls/se8/html/jls-15.html#jls-15.22.1)
    ///
    /// are subject to [widening primitive conversion](https://docs.oracle.com/javase/specs/jls/se8/html/jls-5.html#jls-5.1.2). Effectively,
    /// this means that `byte` or `short` operands are casted to `int` before the operation. Furthermore, Java does not have unsigned types,
    /// so:
    ///
    /// > A widening conversion of a signed integer value to an integral type T simply sign-extends the two's-complement representation of the integer value to fill the wider format.
    ///
    /// In other words, bitwise operations on smaller types can change the binary representation of the value before the operation.
    /// To get around this, we only use types `int` and `long` for variables, even when the field would fit in something smaller. This way,
    /// we can forget that 'widening primitive conversion' is a thing and pretend that all is right with the world.
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
