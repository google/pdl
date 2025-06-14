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
    quote, quote_fn, quote_in,
    tokens::FormatInto,
    Tokens,
};
use heck::{self, ToUpperCamelCase};
use std::iter::{self};

mod r#enum;
mod packet;

use super::{
    import, Chunk, Class, ClassDef, EndiannessValue, Integral, JavaFile, PacketDef, RValue, Type,
    Variable,
};

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
    fn maybe_widen(self, from: &Type, to: Integral) -> Tokens<Java>;
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

    /// `from` and `to` must be >= int
    fn maybe_widen(self, from: &Type, to: Integral) -> Tokens<Java> {
        quote! {
            $(match (from, to) {
                (Type::Integral { ty: Integral::Int, .. }, Integral::Long) => Integer.toUnsignedLong($(self)),
                (Type::EnumRef { width, .. }, to) =>
                    $(let from = Integral::fitting(*width).limit_to_int())
                    $(quote!($(self).to$(from.capitalized())())
                        .maybe_widen(&Type::Integral { ty: from, width: *width }, to)),
                _ => $(self),
            })
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

impl Variable {
    fn value<'a>(&'a self, expr: impl FormatInto<Java> + 'a) -> impl FormatInto<Java> + 'a {
        match &self.ty {
            Type::Integral { .. } | Type::StructRef { .. } => {
                quote!($expr)
            }
            Type::EnumRef { name, width } => {
                quote!($name.from$(Integral::fitting(*width).limit_to_int().capitalized())($expr))
            }
            Type::Payload { .. } => {
                quote!($expr.array())
            }
        }
    }

    fn encode_value(&self) -> impl FormatInto<Java> + '_ {
        quote_fn! {
            $(match &self.ty {
                Type::Integral { .. } if self.name == "payloadSize" => payload.capacity(),
                Type::Integral { .. } if self.name.ends_with("Size") => $(self.name.strip_suffix("Size")).length,
                _ => $(&self.name),
            })
        }
    }

    fn setter<'a>(&'a self, expr: impl FormatInto<Java> + 'a) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            set$(self.name.to_upper_camel_case())($expr)
        }
    }

    fn stringify(&self) -> impl FormatInto<Java> + '_ {
        quote_fn! {
            $(match &self.ty {
                Type::Integral { ty, .. } => $(ty.boxed()).toHexString($(&self.name)),
                Type::EnumRef { .. } | Type::StructRef { .. } => $(&self.name).toString(),
                Type::Payload { .. } => $(&*import::ARRAYS).toString($(&self.name)),
            })
        }
    }

    fn hash_code(&self) -> impl FormatInto<Java> + '_ {
        quote_fn! {
            $(match &self.ty {
                Type::Integral { ty, .. } => $(ty.boxed()).hashCode($(&self.name)),
                Type::EnumRef { .. } | Type::StructRef { .. } => $(&self.name).hashCode(),
                Type::Payload { .. } => $(&*import::ARRAYS).hashCode($(&self.name)),
            })
        }
    }

    fn equals<'a>(&'a self, other: impl FormatInto<Java> + 'a) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            $(match &self.ty {
                Type::Integral { .. } => $(&self.name) == $other,
                Type::EnumRef { .. } | Type::StructRef { .. } => $(&self.name).equals($other),
                Type::Payload { .. } => $(&*import::ARRAYS).equals($(&self.name), $other)
            })
        }
    }

    fn gen_width<'a>(&'a self) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            $(match &self.ty {
                Type::Integral { width, .. } | Type::EnumRef { width, .. } => $(*width),
                Type::StructRef { .. } => $(&self.name).width(),
                Type::Payload { .. } => payload.length,
            })
        }
    }
}

impl FormatInto<Java> for &Type {
    fn format_into(self, tokens: &mut Tokens<Java>) {
        quote_in!(*tokens => $(match self {
            Type::Integral { ty, .. } => $(*ty),
            Type::EnumRef { name, .. } | Type::StructRef { name, .. } => $name,
            Type::Payload { .. } => byte[]
        }));
    }
}

impl Type {
    fn boxed(&self) -> impl FormatInto<Java> + '_ {
        match self {
            Type::Integral { ty, .. } => ty.boxed(),
            Type::EnumRef { name, .. } | Type::StructRef { name, .. } => name,
            Type::Payload { .. } => "Arrays",
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

impl RValue {
    fn generate<'a>(&'a self, ty: &'a Type) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            $(match self {
                RValue::Integral(integral) => $(integral.to_string()),
                RValue::EnumTag(tag) => $ty.$tag,
            })
        }
    }
}
