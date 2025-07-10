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
    quote, quote_fn, quote_in,
    tokens::FormatInto,
    Tokens,
};

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

use crate::backends::{
    common::alignment::UnalignedSymbol,
    java::{codegen::expr::cast_symbol, CompoundVal, ConstrainedTo, Member, ScalarVal},
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

impl Member {
    fn stringify(&self) -> Tokens<Java> {
        match self {
            Member::Scalar(member) => member.stringify(),
            Member::Compound(member) => member.stringify(),
        }
    }

    fn ty(&self) -> Tokens<Java> {
        match self {
            Member::Scalar(member) => member.ty(),
            Member::Compound(member) => member.ty(),
        }
    }

    fn hash_code(&self) -> Tokens<Java> {
        match self {
            Member::Scalar(ScalarVal::Integral { name, ty, .. }) => {
                quote!($(ty.boxed()).hashCode($name))
            }
            Member::Scalar(ScalarVal::EnumRef { name, .. })
            | Member::Compound(CompoundVal::StructRef { name, .. }) => quote!($name.hashCode()),
            Member::Compound(CompoundVal::Payload { size_field }) => {
                quote!($(&*import::ARRAYS).hashCode(payload))
            }
        }
    }

    fn equals<'a>(&'a self, other: impl FormatInto<Java> + 'a) -> Tokens<Java> {
        match self {
            Member::Scalar(ScalarVal::Integral { name, .. }) => quote!($name == $other),
            Member::Scalar(ScalarVal::EnumRef { name, .. })
            | Member::Compound(CompoundVal::StructRef { name, .. }) => {
                quote!($name.equals($other))
            }
            Member::Compound(CompoundVal::Payload { size_field }) => {
                quote!($(&*import::ARRAYS).equals(payload, $other))
            }
        }
    }

    fn constraint(&self, to: &ConstrainedTo) -> Tokens<Java> {
        match (self, to) {
            (Member::Scalar(ScalarVal::Integral { .. }), ConstrainedTo::Integral(i)) => {
                quote!($(*i))
            }
            (Member::Scalar(ScalarVal::EnumRef { ty, .. }), ConstrainedTo::EnumTag(tag)) => {
                quote!($ty.$tag)
            }
            _ => panic!("invalid constraint"),
        }
    }
}

impl ScalarVal {
    fn ty(&self) -> Tokens<Java> {
        match self {
            ScalarVal::Integral { ty, .. } => quote!($ty),
            ScalarVal::EnumRef { ty, .. } => quote!($ty),
        }
    }

    fn stringify(&self) -> Tokens<Java> {
        match self {
            ScalarVal::Integral { name, ty, .. } => ty.stringify(name),
            ScalarVal::EnumRef { name, .. } => quote!($name.toString()),
        }
    }

    fn from_integral<'a>(&'a self, expr: impl FormatInto<Java> + 'a) -> Tokens<Java> {
        match self {
            ScalarVal::Integral { .. } => quote!($expr),
            ScalarVal::EnumRef { ty, width, .. } => {
                quote!($ty.from$(Integral::fitting(*width).capitalized())($expr))
            }
        }
    }

    fn expr_to_encode(&self) -> Tokens<Java> {
        if let ScalarVal::EnumRef { name, width, .. } = self {
            quote!($name.to$(Integral::fitting(*width).capitalized())())
        } else if self.name() == "payloadSize" {
            cast_symbol(quote!(payload.capacity()), Integral::Int, Integral::fitting(self.width()))
        } else if self.name().ends_with("Size") {
            cast_symbol(
                quote!($(self.name().strip_suffix("Size")).length),
                Integral::Int,
                Integral::fitting(self.width()),
            )
        } else {
            quote!($(self.name()))
        }
    }
}

impl CompoundVal {
    fn ty(&self) -> Tokens<Java> {
        match self {
            CompoundVal::StructRef { ty, .. } => quote!($ty),
            CompoundVal::Payload { .. } => quote!(byte[]),
        }
    }

    fn stringify(&self) -> Tokens<Java> {
        match self {
            CompoundVal::StructRef { name, .. } => quote!($name.toString()),
            CompoundVal::Payload { .. } => quote!($(&*import::ARRAYS).toString(payload)),
        }
    }

    fn width_expr(&self) -> Tokens<Java> {
        match self {
            CompoundVal::StructRef { name, .. } => quote!($name.width()),
            CompoundVal::Payload { .. } => quote!(payload.length),
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

    fn decoder<'a>(&'a self, buf: impl FormatInto<Java> + 'a) -> impl FormatInto<Java> + 'a {
        quote_fn! {
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
