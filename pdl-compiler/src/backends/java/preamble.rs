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

use genco::{lang::Java, quote, Tokens};

use crate::{
    ast::EndiannessValue,
    backends::java::{codegen::expr::ExprTree, import, Integral, JavaFile},
};

pub struct Utils;

impl Utils {
    fn byte_encoder(endianness: EndiannessValue, width: usize) -> Tokens<Java> {
        Self::gen_offsets(endianness, width)
            .into_iter()
            .flat_map(|offset| {
                let t = ExprTree::new();
                let root = t.mask(t.symbol(quote!(value), Integral::fitting(width)), offset, 8);
                quote!(buf.put($(t.gen_expr(root)));)
            })
            .collect()
    }

    fn byte_decoder(endianness: EndiannessValue, width: usize) -> Tokens<Java> {
        let t = ExprTree::new();
        t.gen_expr(
            t.or_all(
                Self::gen_offsets(endianness, width)
                    .into_iter()
                    .map(|offset| {
                        t.rshift(
                            t.cast(
                                t.symbol(quote!(buf.get()), Integral::Byte),
                                Integral::fitting(width),
                            ),
                            t.num(offset),
                        )
                    })
                    .collect(),
            ),
        )
    }

    fn gen_offsets(endianness: EndiannessValue, width: usize) -> Vec<usize> {
        match endianness {
            EndiannessValue::LittleEndian => (0..width).step_by(8).collect(),
            EndiannessValue::BigEndian => (0..width).step_by(8).rev().collect(),
        }
    }
}

impl JavaFile<EndiannessValue> for Utils {
    fn generate(self, endianness: EndiannessValue) -> Tokens<Java> {
        quote! {
            class Utils {
                $(for width in [24, 40, 48, 56] {
                    $(let ty = Integral::fitting(width))

                    static void put$width($(&*import::BB) buf, $ty value) {
                        $(Self::byte_encoder(endianness, width))
                    }

                    static $ty get$width($(&*import::BB) buf) {
                        return $(Self::byte_decoder(endianness, width));
                    }
                })
            }
        }
    }
}
