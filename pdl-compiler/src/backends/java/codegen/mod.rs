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
use std::collections::HashMap;

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
pub mod expr;
mod packet;

use crate::backends::java::{
    codegen::expr::{cast_symbol, ExprId, ExprTree},
    inheritance::{ClassHeirarchy, Constraint},
    Context, Field, WidthField,
};

use super::{import, Chunk, Class, EndiannessValue, Integral, JavaFile, PacketDef};

impl JavaFile<&Context> for Class {
    fn generate(self, context: &Context) -> Tokens<Java> {
        match self {
            Class::Packet { name, def } => packet::gen_packet(&name, &def, context),
            Class::AbstractPacket { name, def, fallback_child } => {
                packet::gen_abstract_packet(&name, &def, fallback_child.as_ref(), context)
            }
            Class::Enum { name, tags, width, fallback_tag } => {
                r#enum::gen_enum(&name, &tags, width, fallback_tag)
            }
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
    pub fn width_expr(&self, heirarchy: &ClassHeirarchy) -> Tokens<Java> {
        match self {
            Field::StructRef { name, .. } => quote!($name.width()),
            Field::Payload { .. } => quote!(payload.length),
            Field::ArrayElem { val, .. } => {
                if let Some(width) = val.width().or_else(|| heirarchy.width(val.class().unwrap())) {
                    let t = ExprTree::new();
                    let root = t.mul(
                        t.symbol(quote!($(val.name()).length), Integral::Int),
                        t.num(width / 8),
                    );
                    t.gen_expr(root)
                } else {
                    sum_array_elem_widths(val.name())
                }
            }
            _ => quote!($(self.width().unwrap())),
        }
    }

    pub fn stringify(&self, width_fields: &HashMap<String, WidthField>) -> Tokens<Java> {
        match self {
            Field::Integral { name, ty, .. } if name.ends_with("Size") => {
                let array_name = name.strip_suffix("Size").unwrap();
                if let Some(WidthField::Size { elem_width: Some(elem_width), modifier, .. }) =
                    width_fields.get(array_name)
                {
                    let t = ExprTree::new();
                    quote!(
                        $(ty.stringify(t.gen_expr(
                            t.mul(
                                t.symbol(quote!($array_name.length), Integral::Int),
                                t.num(*elem_width),
                            )
                        )))
                        $(if let Some(modifier) = modifier => + "(+" + $(*modifier) + ")")
                    )
                } else {
                    ty.stringify(sum_array_elem_widths(array_name))
                }
            }
            Field::Integral { name, ty, .. } if name.ends_with("Count") => {
                let modifier =
                    width_fields.get(name.strip_suffix("Count").unwrap()).unwrap().modifier();

                quote!(
                    $(ty.stringify(quote!($(name.strip_suffix("Count").unwrap()).length)))
                    $(if let Some(modifier) = modifier => + "(+" + $modifier + ")")
                )
            }
            Field::Integral { fixed_val: Some(val), .. } => quote!($(*val)),
            Field::Integral { name, width: 1, .. } => quote!(($name ? 1 : 0)),
            Field::Integral { name, ty, .. } => ty.stringify(name),
            Field::Reserved { .. } => quote!("..."),
            Field::EnumRef { ty, fixed_tag: Some(tag), .. } => quote!($ty.$tag),
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
            Field::Integral { width: 1, .. } => quote!(boolean),
            Field::Integral { ty, .. } => quote!($ty),
            Field::EnumRef { ty, .. } => quote!($ty),
            Field::StructRef { ty, .. } => quote!($ty),
            Field::Payload { .. } => quote!(byte[]),
            Field::ArrayElem { val, .. } => quote!($(val.ty())[]),
            other => panic!("cannot ty() {:?}", other),
        }
    }

    pub fn hash_code(&self) -> Tokens<Java> {
        match self {
            Field::Integral { name, width: 1, .. } => quote!(Boolean.hashCode($name)),
            Field::Integral { name, ty, .. } => quote!($(ty.boxed()).hashCode($name)),
            Field::EnumRef { name, .. } | Field::StructRef { name, .. } => quote!($name.hashCode()),
            Field::Payload { .. } => quote!($(&*import::ARRAYS).hashCode(payload)),
            Field::ArrayElem { val, .. } => quote!($(&*import::ARRAYS).hashCode($(val.name()))),
            other => panic!("cannot hash {:?}", other),
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
            other => panic!("cannot eq {:?}", other),
        }
    }

    pub fn constraint(&self, to: &Constraint) -> Tokens<Java> {
        match (self, to) {
            (Field::Integral { width: 1, .. }, Constraint::Integral(0)) => quote!(false),
            (Field::Integral { width: 1, .. }, Constraint::Integral(1)) => quote!(true),
            (Field::Integral { .. }, Constraint::Integral(i)) => quote!($(*i)),
            (Field::EnumRef { ty, .. }, Constraint::EnumTag(tag)) => quote!($ty.$tag),
            _ => panic!("invalid constraint"),
        }
    }

    pub fn fixed_val(&self) -> Option<Tokens<Java>> {
        match self {
            Field::Integral { fixed_val: Some(val), .. } => Some(quote!($(*val))),
            Field::EnumRef { ty, fixed_tag: Some(tag), .. } => Some(quote!($ty.$tag)),
            _ => None,
        }
    }

    pub fn from_num(
        &self,
        expr: impl FormatInto<Java>,
        width_fields: &HashMap<String, WidthField>,
    ) -> Tokens<Java> {
        match self {
            Field::Integral { name, ty, .. } if self.is_width() => {
                let arr_name = name
                    .strip_suffix("Size")
                    .unwrap_or_else(|| name.strip_suffix("Count").unwrap());

                let t = ExprTree::new();
                t.gen_expr(t.sub(
                    t.symbol(quote!($expr), *ty),
                    t.num(width_fields.get(arr_name).unwrap().modifier().unwrap_or(0)),
                ))
            }
            Field::Integral { width: 1, .. } => quote!($expr != 0),
            Field::Integral { .. } => quote!($expr),
            Field::EnumRef { ty, width, .. } => {
                quote!($ty.from$(Integral::fitting(*width).capitalized())($expr))
            }
            _ => panic!("failed to decode field from num: {}", quote!($expr).to_string().unwrap()),
        }
    }

    pub fn to_num(
        &self,
        t: &ExprTree,
        expr: impl FormatInto<Java>,
        width_fields: &HashMap<String, WidthField>,
    ) -> ExprId {
        let width = self.width().expect("cannot encode field with dynamic width into num");
        let ty = self.integral_ty().unwrap();

        if let Field::Integral { fixed_val: Some(val), .. } = self {
            t.num(*val)
        } else if self.is_reserved() {
            t.num(0)
        } else if let Field::EnumRef { ty: class, fixed_tag: Some(tag), .. } = self {
            t.symbol(quote!($class.$tag.to$(ty.capitalized())()), ty)
        } else if let Field::EnumRef { .. } = self {
            t.symbol(quote!($expr.to$(ty.capitalized())()), ty)
        } else if let Some(array_name) =
            self.name().strip_suffix("Size").or_else(|| self.name().strip_suffix("Count"))
        {
            match width_fields.get(array_name) {
                Some(WidthField::Size { elem_width: Some(elem_width), modifier, .. }) => t.cast(
                    t.add(
                        t.mul(
                            t.symbol(
                                quote!(
                                    $(if array_name == "payload" {
                                        payload.limit()
                                    } else {
                                        $array_name.length
                                    })
                                ),
                                Integral::Int,
                            ),
                            t.num(elem_width / 8),
                        ),
                        t.num(modifier.unwrap_or(0)),
                    ),
                    Integral::fitting(width),
                ),
                Some(WidthField::Size { elem_width: None, modifier, .. }) => t.cast(
                    t.add(
                        t.symbol(sum_array_elem_widths(array_name), Integral::Int),
                        t.num(modifier.unwrap_or(0)),
                    ),
                    Integral::fitting(width),
                ),
                Some(WidthField::Count { modifier, .. }) => t.cast(
                    t.add(
                        t.symbol(quote!($array_name.length), Integral::Int),
                        t.num(modifier.unwrap_or(0)),
                    ),
                    Integral::fitting(width),
                ),
                _ => {
                    panic!("Bitfields ending in 'size' or 'count' are not supported. Use _size_ or _count_ instead.")
                }
            }
        } else if let Field::Integral { width: 1, .. } = self {
            t.symbol(quote!(($expr ? 1 : 0)), Integral::Int)
        } else {
            t.symbol(quote!($expr), ty)
        }
    }
}

impl EndiannessValue {
    /// Width in bits. Must be byte-divisible and <= 64
    fn encode_bytes(&self, buf: Tokens<Java>, width: usize, val: Tokens<Java>) -> Tokens<Java> {
        match width {
            8 => quote!($buf.put($val)),
            16 => quote!($buf.putShort($val)),
            24 => quote!(Utils.put24($buf, $val)),
            32 => quote!($buf.putInt($val)),
            40 => quote!(Utils.put40($buf, $val)),
            48 => quote!(Utils.put48($buf, $val)),
            56 => quote!(Utils.put56($buf, $val)),
            64 => quote!($buf.putLong($val)),
            _ => panic!("can't encode value of width {width}"),
        }
    }

    // Width in bits. Must be byte-divisible and <= 64
    fn decode_bytes(&self, buf: Tokens<Java>, width: usize) -> Tokens<Java> {
        match width {
            8 => quote!($buf.get()),
            16 => quote!($buf.getShort()),
            24 => quote!(Utils.get24($buf)),
            32 => quote!($buf.getInt()),
            40 => quote!(Utils.get40($buf)),
            48 => quote!(Utils.get48($buf)),
            56 => quote!(Utils.get56($buf)),
            64 => quote!($buf.getLong()),
            _ => panic!("can't decode value of width {width}"),
        }
    }
}

impl Integral {
    fn stringify(&self, expr: impl FormatInto<Java>) -> Tokens<Java> {
        let stringable_ty = self.limit_to_int();
        quote!("0x" + $(stringable_ty.boxed()).toHexString(
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
}

impl FormatInto<Java> for Integral {
    fn format_into(self, tokens: &mut java::Tokens) {
        (&self).format_into(tokens);
    }
}

impl FormatInto<Java> for &Integral {
    fn format_into(self, tokens: &mut Tokens<Java>) {
        quote_in!(*tokens => $(match self {
            Integral::Byte => byte,
            Integral::Short => short,
            Integral::Int => int,
            Integral::Long => long,
        }));
    }
}

fn sum_array_elem_widths(name: impl FormatInto<Java>) -> Tokens<Java> {
    quote!($(&*import::ARRAYS).stream($name).mapToInt(elem -> elem.width()).sum())
}
