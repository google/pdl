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
    collections::HashMap,
    fs::{self, OpenOptions},
    path::{Path, PathBuf},
};

use crate::{
    ast::{self, EndiannessValue},
    backends::common::alignment::{ByteAligner, Chunk},
};

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

fn generate_classes<'a>(file: &ast::File, context: &'a GeneratorContext) -> Vec<Class<'a>> {
    let mut classes = HashMap::new();

    for decl in file.declarations.iter() {
        match &decl.desc {
            ast::DeclDesc::Packet { id, constraints, fields, parent_id: None } => {
                classes.insert(
                    id,
                    Class {
                        name: id.to_upper_camel_case(),
                        ctx: context,
                        def: ClassDef::Packet(PacketDef::new(fields)),
                    },
                );
            }
            _ => {
                dbg!(decl);
                todo!()
            }
        }
    }

    classes.into_values().collect()
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

pub enum ClassDef {
    Packet(PacketDef),
    Enum,
}

pub struct PacketDef {
    members: Vec<Variable>,
    chunks: Vec<Chunk<Variable>>,
}

impl PacketDef {
    pub fn new(fields: &Vec<ast::Field>) -> Self {
        let mut members = vec![];
        let mut aligner = ByteAligner::<Variable>::new(64);

        for field in fields {
            match &field.desc {
                ast::FieldDesc::Scalar { id, width } => {
                    let variable = Variable {
                        ty: Type::from_width(*width).limit_to_int(),
                        name: id.to_lower_camel_case(),
                        width: *width,
                    };

                    aligner.add_field(variable.clone(), *width);
                    members.push(variable);
                }
                _ => todo!(),
            }
        }

        PacketDef {
            members,
            chunks: aligner.into_aligned_chunks().expect("Failed to align fields"),
        }
    }

    pub fn get_byte_width(&self) -> usize {
        self.members.iter().map(|member| member.width).sum::<usize>() / 8
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
#[derive(Debug, Clone)]
pub struct Variable {
    name: String,
    ty: Type,
    width: usize,
}

impl From<&ast::Field> for Variable {
    fn from(field: &ast::Field) -> Self {
        match &field.desc {
            ast::FieldDesc::Scalar { id, width } => Variable {
                name: id.to_lower_camel_case(),
                ty: Type::from_width(*width),
                width: *width,
            },
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Type {
    Byte,
    Short,
    Int,
    Long,
}

impl Type {
    pub fn from_width(width: usize) -> Self {
        if width <= 8 {
            Type::Byte
        } else if width <= 16 {
            Type::Short
        } else if width <= 32 {
            Type::Int
        } else if width <= 64 {
            Type::Long
        } else {
            panic!("Width too large!")
        }
    }

    pub fn limit_to_int(self) -> Self {
        cmp::max(self, Type::Int)
    }

    pub fn get_width(&self) -> u8 {
        match self {
            Type::Byte => 8,
            Type::Short => 16,
            Type::Int => 32,
            Type::Long => 64,
        }
    }
}
