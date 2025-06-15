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
mod packet;

use crate::backends::java::{ConstrainedTo, Member, SizedMember, UnsizedMember};

use super::{import, Chunk, Class, ClassDef, EndiannessValue, Integral, JavaFile, PacketDef};

impl JavaFile<EndiannessValue> for Class {
    fn generate(self, endianness: EndiannessValue) -> Tokens<Java> {
        quote! {
            $(match &self.def {
                ClassDef::Packet(def) => $(def.gen_packet(&self.name, endianness)),
                ClassDef::AbstractPacket(def) => $(def.gen_abstract_packet(&self.name, endianness)),
                ClassDef::Enum { tags, width } => $(self.gen_enum(tags, *width)),
            })
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

trait Expr {
    fn maybe_cast(self, from: Integral, to: Integral) -> Tokens<Java>;
    fn maybe_widen(self, member: &SizedMember, to: Integral) -> Tokens<Java>;
    fn maybe_shift(self, op: &'static str, by: usize) -> Tokens<Java>;
    fn maybe_mask(self, offset: usize, width: usize) -> Tokens<Java>;
}

impl<J: FormatInto<Java>> Expr for J {
    /// `from` and `to` must be >= int
    fn maybe_cast(self, from: Integral, to: Integral) -> Tokens<Java> {
        quote! {
            $(match (from, to) {
                (Integral::Int, Integral::Long) => Integer.toUnsignedLong($(self)),
                (Integral::Long, Integral::Int) => ((int) ($(self))),
                _ => $(self),
            })
        }
    }

    /// `to` must be >= int
    fn maybe_widen(self, member: &SizedMember, to: Integral) -> Tokens<Java> {
        match (member, to) {
            (SizedMember::Integral { ty: Integral::Int, .. }, Integral::Long) => {
                quote!(Integer.toUnsignedLong($(self)))
            }
            (SizedMember::EnumRef { width, .. }, to) => {
                let from = Integral::fitting(*width).limit_to_int();
                quote!($(self).to$(from.capitalized())()).maybe_widen(
                    &SizedMember::Integral { name: String::default(), ty: from, width: *width },
                    to,
                )
            }
            _ => quote!($(self)),
        }
    }

    fn maybe_mask(self, offset: usize, width: usize) -> Tokens<Java> {
        quote! {
            $(if width == 0 {
                $(self.maybe_shift(">>>", offset))
            } else {
                ($(self.maybe_shift(">>>", offset)) & $(gen_mask(width)))
            } )
        }
    }

    fn maybe_shift(self, op: &'static str, by: usize) -> Tokens<Java> {
        quote! {
            $(if by == 0 {
                $(self)
            } else {
                ($(self) $op $by)
            })
        }
    }
}

impl Member {
    fn stringify(&self) -> Tokens<Java> {
        match self {
            Member::Sized(member) => member.stringify(),
            Member::Unsized(member) => member.stringify(),
        }
    }

    fn ty(&self) -> Tokens<Java> {
        match self {
            Member::Sized(member) => member.ty(),
            Member::Unsized(member) => member.ty(),
        }
    }

    fn hash_code(&self) -> Tokens<Java> {
        match self {
            Member::Sized(SizedMember::Integral { name, ty, .. }) => {
                quote!($(ty.boxed()).hashCode($name))
            }
            Member::Sized(SizedMember::EnumRef { name, .. })
            | Member::Unsized(UnsizedMember::StructRef { name, .. }) => quote!($name.hashCode()),
            Member::Unsized(UnsizedMember::Payload { size_field }) => {
                quote!($(&*import::ARRAYS).hashCode(payload))
            }
        }
    }

    fn equals<'a>(&'a self, other: impl FormatInto<Java> + 'a) -> Tokens<Java> {
        match self {
            Member::Sized(SizedMember::Integral { name, .. }) => quote!($name == $other),
            Member::Sized(SizedMember::EnumRef { name, .. })
            | Member::Unsized(UnsizedMember::StructRef { name, .. }) => {
                quote!($name.equals($other))
            }
            Member::Unsized(UnsizedMember::Payload { size_field }) => {
                quote!($(&*import::ARRAYS).equals(payload, $other))
            }
        }
    }

    fn constraint(&self, to: &ConstrainedTo) -> Tokens<Java> {
        match (self, to) {
            (Member::Sized(SizedMember::Integral { .. }), ConstrainedTo::Integral(i)) => {
                quote!($(*i))
            }
            (Member::Sized(SizedMember::EnumRef { ty, .. }), ConstrainedTo::EnumTag(tag)) => {
                quote!($ty.$tag)
            }
            _ => panic!("invalid constraint"),
        }
    }
}

impl SizedMember {
    fn ty(&self) -> Tokens<Java> {
        match self {
            SizedMember::Integral { ty, .. } => quote!($ty),
            SizedMember::EnumRef { ty, .. } => quote!($ty),
        }
    }

    fn stringify(&self) -> Tokens<Java> {
        match self {
            SizedMember::Integral { name, ty, .. } => quote!($(ty.boxed()).toHexString($name)),
            SizedMember::EnumRef { name, .. } => quote!($name.toString()),
        }
    }

    fn from_integral<'a>(&'a self, expr: impl FormatInto<Java> + 'a) -> Tokens<Java> {
        match self {
            SizedMember::Integral { .. } => quote!($expr),
            SizedMember::EnumRef { ty, width, .. } => {
                quote!($ty.from$(Integral::fitting(*width).limit_to_int().capitalized())($expr))
            }
        }
    }

    fn expr_to_encode(&self) -> Tokens<Java> {
        if self.name() == "payloadSize" {
            quote!(payload.capacity())
        } else if self.name().ends_with("Size") {
            quote!($(self.name().strip_suffix("Size")).length)
        } else {
            quote!($(self.name()))
        }
    }
}

impl UnsizedMember {
    fn ty(&self) -> Tokens<Java> {
        match self {
            UnsizedMember::StructRef { ty, .. } => quote!($ty),
            UnsizedMember::Payload { .. } => quote!(byte[]),
        }
    }

    fn stringify(&self) -> Tokens<Java> {
        match self {
            UnsizedMember::StructRef { name, .. } => quote!($name.toString()),
            UnsizedMember::Payload { .. } => quote!($(&*import::ARRAYS).toString(payload)),
        }
    }

    fn width_expr(&self) -> Tokens<Java> {
        match self {
            UnsizedMember::StructRef { name, .. } => quote!($name.width()),
            UnsizedMember::Payload { .. } => quote!(payload.length),
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
                Integral::Byte => Byte.toUnsignedInt($buf.get()),
                Integral::Short => Short.toUnsignedInt($buf.getShort()),
                Integral::Int => $buf.getInt(),
                Integral::Long => $buf.getLong(),
            })
        }
    }

    fn boxed(&self) -> &'static str {
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

pub fn gen_mask(width: usize) -> impl FormatInto<Java> {
    quote_fn! {
        $(format!("0x{:x}", (1_u128 << width) - 1))$(if Integral::fitting(width) > Integral::Int => L)
    }
}
