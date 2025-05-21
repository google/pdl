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
};
use heck::{self, ToLowerCamelCase, ToUpperCamelCase};
use std::{
    cmp,
    collections::{HashMap, HashSet},
    fs::{self, OpenOptions},
    path::{Path, PathBuf},
    rc::Rc,
};

use crate::{
    ast::{self, EndiannessValue},
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
    let source = sources.get(file.file).expect("could not read source");

    let mut dir = PathBuf::from(output_dir);
    dir.extend(package.split("."));
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let context = GeneratorContext {
        pdl_src: source.name().clone(),
        out_dir: dir,
        package: String::from(package),
        endianness: file.endianness.value,
    };

    generate_classes(&file, &context).into_iter().try_for_each(|f| f.write_to_fs())
}

// TODO: Break this apart, it does multiple impls worth of work...
fn generate_classes<'a>(file: &ast::File, context: &'a GeneratorContext) -> Vec<Class<'a>> {
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

    let mut parent_classes = HashMap::new();
    let mut classes = HashMap::new();

    for decl in file.declarations.iter() {
        match &decl.desc {
            // If this is a parent packet, make a new abstract class and defer parenthood to it.
            ast::DeclDesc::Packet { id, constraints, fields, .. }
                if parent_packets.contains(id) =>
            {
                let parent_name = id.to_upper_camel_case();
                let child_name = format!("Unknown{}", id.to_upper_camel_case());

                let (members, alignment) = generate_members(fields);

                // TODO: Only create child packet here if this packet contains a payload. If its a parent with a
                // body field, we don't want a 'default' child

                parent_classes.insert(
                    id,
                    Class {
                        name: parent_name.clone(),
                        ctx: context,
                        def: ClassDef::Packet {
                            children: vec![child_name.clone()],
                            def: PacketDef { members, alignment },
                        },
                    },
                );

                classes.insert(
                    id,
                    Class {
                        name: child_name,
                        ctx: context,
                        def: ClassDef::Subpacket { parent: parent_name, def: None },
                    },
                );
            }
            // If this is a child packet, set its parent to the appropriate abstract class.
            ast::DeclDesc::Packet { id, constraints, fields, parent_id: Some(parent_id) } => {
                let (parent_name, children) = parent_classes
                    .get_mut(parent_id)
                    .and_then(|parent| {
                        if let ClassDef::Packet { children, .. } = &mut parent.def {
                            Some((&parent.name, children))
                        } else {
                            None
                        }
                    })
                    .expect("Packet inherits from unknown parent");

                let (members, alignment) = generate_members(fields);

                let name = id.to_upper_camel_case();
                children.push(name.clone());
                classes.insert(
                    id,
                    Class {
                        name,
                        ctx: context,
                        def: ClassDef::Subpacket {
                            parent: parent_name.clone(),
                            def: Some(PacketDef { members, alignment }),
                        },
                    },
                );
            }
            // Otherwise, the packet has no inheritence (no parent and no children)
            ast::DeclDesc::Packet { id, constraints, fields, parent_id: None } => {
                let (members, alignment) = generate_members(fields);
                classes.insert(
                    id,
                    Class {
                        name: id.to_upper_camel_case(),
                        ctx: context,
                        def: ClassDef::Packet {
                            children: Vec::new(),
                            def: PacketDef { members, alignment },
                        },
                    },
                );
            }
            _ => {
                dbg!(decl);
                todo!()
            }
        }
    }

    parent_classes.into_values().chain(classes.into_values()).collect()
}

fn generate_members(fields: &Vec<ast::Field>) -> (Vec<Rc<Variable>>, Alignment<Rc<Variable>>) {
    let mut members = Vec::new();
    let mut aligner = ByteAligner::new(64);

    for field in fields.iter() {
        match &field.desc {
            ast::FieldDesc::Scalar { id, width } => {
                let variable = Rc::new(Variable {
                    name: id.to_lower_camel_case(),
                    ty: Type::Integral(Integral::fitting_width(*width).limit_to_int()),
                    width: *width,
                });
                members.push(variable.clone());
                aligner.add_field(variable, *width);
            }
            ast::FieldDesc::Payload { size_modifier } => {
                aligner.add_payload();
            }
            _ => todo!(),
        }
    }

    (members, aligner.align().expect("Failed to align members"))
}

trait JavaFile: Sized + FormatInto<Java> {
    /// Get the path to this file
    fn get_path(&self) -> PathBuf;

    /// Get the java package that contains this file
    fn get_package(&self) -> &str;

    /// Write this file to the filesystem at the path specified by `self.get_path()`
    fn write_to_fs(self) -> Result<(), String> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(self.get_path())
            .map_err(|err| err.to_string())?;

        let mut w = genco::fmt::IoWriter::new(file);
        let fmt = genco::fmt::Config::from_lang::<Java>().with_newline("\n");
        let config = java::Config::default().with_package(self.get_package());

        let mut tokens = java::Tokens::new();
        self.format_into(&mut tokens);
        tokens.format_file(&mut w.as_formatter(&fmt), &config).map_err(|e| e.to_string())
    }
}

pub struct GeneratorContext {
    package: String,
    out_dir: PathBuf,
    pdl_src: String,
    endianness: EndiannessValue,
}

pub struct Class<'a> {
    name: String,
    ctx: &'a GeneratorContext,
    def: ClassDef,
}

impl<'a> JavaFile for Class<'a> {
    fn get_path(&self) -> PathBuf {
        self.ctx.out_dir.join(&self.name).with_extension("java")
    }

    fn get_package(&self) -> &str {
        &self.ctx.package
    }
}

#[derive(Debug, Clone)]
pub enum Inherit {
    From(String),
    Into(Vec<String>),
}

pub enum ClassDef {
    Packet { children: Vec<String>, def: PacketDef },
    Subpacket { parent: String, def: Option<PacketDef> },
}

#[derive(Debug, Clone)]
pub struct PacketDef {
    members: Vec<Rc<Variable>>,
    alignment: Alignment<Rc<Variable>>,
}

impl PacketDef {
    pub fn static_byte_width(&self) -> usize {
        self.members.iter().map(|member| member.width).sum::<usize>() / 8
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    name: String,
    ty: Type,
    width: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Integral(Integral),
    Class(String),
}

impl Type {
    fn width(&self) -> usize {
        match self {
            Type::Integral(i) => i.width(),
            Type::Class(c) => todo!(),
        }
    }
}

impl From<Integral> for Type {
    fn from(i: Integral) -> Self {
        Type::Integral(i)
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
    pub fn fitting_width(width: impl Into<usize>) -> Self {
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
