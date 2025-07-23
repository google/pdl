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

use core::panic;
use genco::{
    self,
    prelude::{java, Java},
    quote, quote_in,
    tokens::FormatInto,
    Tokens,
};
use std::{array, collections::HashMap};

mod r#enum;
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
/// To get around this, we must sign-safe cast every smaller operand to int or long.
///
/// This module contains utilities for generating simplified Java expressions that perform this casting.
mod expr;
mod packet;

use crate::backends::java::{
    codegen::expr::{cast_symbol, gen_expr, ExprTree},
    ConstrainedTo, Field, WidthField,
};

use super::{import, Chunk, Class, EndiannessValue, Integral, JavaFile, PacketDef};

impl JavaFile<EndiannessValue> for Class {
    fn generate(self, endianness: EndiannessValue) -> Tokens<Java> {
        match self {
            Class::Packet { name, def, parent } => {
                packet::gen_packet(&name, &def, parent.as_ref(), endianness)
            }
            Class::AbstractPacket { name, def, parent, children } => {
                packet::gen_abstract_packet(&name, &def, parent.as_ref(), &children, endianness)
            }
            Class::Enum { name, tags, width } => r#enum::gen_enum(&name, &tags, width),
        }
    }
}

impl FormatInto<Java> for EndiannessValue {
    fn format_into(self, tokens: &mut Tokens<Java>) {
        quote_in! { *tokens =>
            $(match self {
                EndiannessValue::LittleEndian => $(&*import::BO).LITTLE_ENDIAN,
                EndiannessValue::BigEndian => $(&*import::BO).BIG_ENDIAN,
            })
        }
    }
}

impl Field {
    pub fn width_expr(&self) -> Tokens<Java> {
        match self {
            Field::StructRef { name, .. } => quote!($name.width()),
            Field::Payload { .. } => quote!(payload.length),
            Field::ArrayElem { val, .. } => {
                let t = ExprTree::new();
                let root = t.mul(
                    t.symbol(quote!($(val.name()).length), Integral::Int),
                    t.num(val.width().unwrap()),
                );
                gen_expr(&t, root)
            }
            _ => quote!($(self.width().unwrap())),
        }
    }

    pub fn stringify(&self, width_fields: &HashMap<String, WidthField>) -> Tokens<Java> {
        match self {
            Field::Integral { name, ty, .. } if name.ends_with("Size") => {
                let (array_name, _, elem_width) = self.get_size_field_info(width_fields).unwrap();
                let t = ExprTree::new();
                ty.stringify(gen_expr(
                    &t,
                    t.mul(t.symbol(quote!($array_name.length), Integral::Int), t.num(elem_width)),
                ))
            }
            Field::Integral { name, ty, .. } if name.ends_with("Count") => {
                ty.stringify(quote!($(name.strip_suffix("Count").unwrap()).length))
            }
            Field::Integral { name, ty, .. } => ty.stringify(name),
            Field::EnumRef { name, .. } => quote!($name.toString()),
            Field::StructRef { name, .. } => quote!($name.toString()),
            Field::Payload { .. } => quote!($(&*import::ARRAYS).toString(payload)),
            Field::ArrayElem { val, .. } => {
                quote!($(&*import::ARRAYS).toString($(val.name())))
            }
        }
    }

    pub fn ty(&self) -> Tokens<Java> {
        match self {
            Field::Integral { ty, .. } => quote!($ty),
            Field::EnumRef { ty, .. } => quote!($ty),
            Field::StructRef { ty, .. } => quote!($ty),
            Field::Payload { .. } => quote!(byte[]),
            Field::ArrayElem { val, .. } => quote!($(val.ty())[]),
        }
    }

    pub fn hash_code(&self) -> Tokens<Java> {
        match self {
            Field::Integral { name, ty, .. } => quote!($(ty.boxed()).hashCode($name)),
            Field::EnumRef { name, .. } | Field::StructRef { name, .. } => quote!($name.hashCode()),
            Field::Payload { .. } => quote!($(&*import::ARRAYS).hashCode(payload)),
            Field::ArrayElem { val, .. } => quote!($(&*import::ARRAYS).hashCode($(val.name()))),
        }
    }

    pub fn equals(&self, other: impl FormatInto<Java>) -> Tokens<Java> {
        match self {
            Field::Integral { name, .. } => quote!($name == $other),
            Field::EnumRef { name, .. } | Field::StructRef { name, .. } => {
                quote!($name.equals($other))
            }
            Field::Payload { .. } => quote!($(&*import::ARRAYS).equals(payload, $other)),
            Field::ArrayElem { val, .. } => {
                quote!($(&*import::ARRAYS).equals($(val.name()), $other))
            }
        }
    }

    pub fn constraint(&self, to: &ConstrainedTo) -> Tokens<Java> {
        match (self, to) {
            (Field::Integral { .. }, ConstrainedTo::Integral(i)) => quote!($(*i)),
            (Field::EnumRef { ty, .. }, ConstrainedTo::EnumTag(tag)) => quote!($ty.$tag),
            _ => panic!("invalid constraint"),
        }
    }

    pub fn try_decode_from_num(&self, expr: impl FormatInto<Java>) -> Result<Tokens<Java>, ()> {
        match self {
            Field::Integral { .. } => Ok(quote!($expr)),
            Field::EnumRef { ty, width, .. } => {
                Ok(quote!($ty.from$(Integral::fitting(*width).capitalized())($expr)))
            }
            _ => Err(()),
        }
    }

    pub fn try_encode_to_num(
        &self,
        expr: impl FormatInto<Java>,
        width_fields: &HashMap<String, WidthField>,
    ) -> Result<Tokens<Java>, ()> {
        let width = self.width().ok_or(())?;

        Ok(if let Field::EnumRef { width, .. } = self {
            quote!($expr.to$(Integral::fitting(*width).capitalized())())
        } else if let Some((array_name, _, elem_width)) = self.get_size_field_info(width_fields) {
            let t = ExprTree::new();
            gen_expr(
                &t,
                t.cast(
                    t.mul(
                        t.symbol(
                            quote!($(if array_name == "payload" { payload.capacity() } else { $array_name.length })),
                            Integral::Int,
                        ),
                        t.num(elem_width),
                    ),
                    Integral::fitting(width),
                ),
            )
        } else if let Some((array_name, field_width)) = self.get_count_field_info(width_fields) {
            cast_symbol(quote!($array_name.length), Integral::Int, Integral::fitting(width))
        } else {
            quote!($expr)
        })
    }

    fn get_size_field_info(
        &self,
        width_fields: &HashMap<String, WidthField>,
    ) -> Option<(&str, usize, usize)> {
        if let Some((array_name, WidthField::Size { field_width, elem_width })) =
            self.name().strip_suffix("Size").and_then(|array_name| {
                width_fields.get(array_name).map(|width_field| (array_name, width_field))
            })
        {
            Some((array_name, *field_width, *elem_width))
        } else {
            None
        }
    }

    fn get_count_field_info(
        &self,
        width_fields: &HashMap<String, WidthField>,
    ) -> Option<(&str, usize)> {
        if let Some((array_name, WidthField::Count { field_width })) =
            self.name().strip_suffix("Count").and_then(|array_name| {
                width_fields.get(array_name).map(|width_field| (array_name, width_field))
            })
        {
            Some((array_name, *field_width))
        } else {
            None
        }
    }
}

impl Integral {
    fn encoder(&self) -> impl FormatInto<Java> + '_ {
        match self {
            Integral::Byte => "put",
            Integral::Short => "putShort",
            Integral::Int => "putInt",
            Integral::Long => "putLong",
        }
    }

    fn decode_from(&self, buf: Tokens<Java>) -> Tokens<Java> {
        quote! {
            $(match self {
                Integral::Byte => $buf.get(),
                Integral::Short => $buf.getShort(),
                Integral::Int => $buf.getInt(),
                Integral::Long => $buf.getLong(),
            })
        }
    }
    pub fn compare(&self, expr: impl FormatInto<Java>, to: impl FormatInto<Java>) -> Tokens<Java> {
        let comparable_ty = self.limit_to_int();
        quote!($(comparable_ty.boxed()).compareUnsigned(
            $(cast_symbol(quote!($expr), *self, comparable_ty)),
            $to
        ))
    }

    fn stringify(&self, expr: impl FormatInto<Java>) -> Tokens<Java> {
        let stringable_ty = self.limit_to_int();
        quote!($(stringable_ty.boxed()).toHexString(
            $(cast_symbol(quote!($(expr)), *self, stringable_ty))
        ))
    }

    pub fn boxed(&self) -> &'static str {
        match self {
            Integral::Byte => "Byte",
            Integral::Short => "Short",
            Integral::Int => "Integer",
            Integral::Long => "Long",
        }
    }

    pub fn capitalized(&self) -> &'static str {
        match self {
            Integral::Byte => "Byte",
            Integral::Short => "Short",
            Integral::Int => "Int",
            Integral::Long => "Long",
        }
    }

    pub fn literal(&self, expr: impl FormatInto<Java>) -> Tokens<Java> {
        match self {
            Integral::Byte => quote!((byte) $(expr)),
            Integral::Short => quote!((short) $(expr)),
            Integral::Int => quote!($(expr)),
            Integral::Long => quote!($(expr)L),
        }
    }
}

impl FormatInto<Java> for Integral {
    fn format_into(self, tokens: &mut java::Tokens) {
        (&self).format_into(tokens);
    }
}

impl FormatInto<Java> for &Integral {
    fn format_into(self, tokens: &mut java::Tokens) {
        quote_in!(*tokens => $(match self {
            Integral::Byte => byte,
            Integral::Short => short,
            Integral::Int => int,
            Integral::Long => long,
        }));
    }
}
