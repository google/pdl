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

use std::collections::HashMap;

use genco::{self, prelude::Java, quote, tokens::quoted, Tokens};
use heck::{self, ToLowerCamelCase, ToUpperCamelCase};

use crate::{
    ast::EndiannessValue,
    backends::{
        common::alignment::Alignment,
        java::{Child, ConstrainedTo, Field, Parent, WidthField},
    },
};

use super::{
    expr::{gen_expr, gen_mask, ExprTree},
    import, Chunk, Integral, PacketDef,
};

pub fn gen_packet(
    name: &String,
    def: &PacketDef,
    parent: Option<&Parent>,
    endianness: EndiannessValue,
) -> Tokens<Java> {
    quote! {
        public final class $name $(if let Some(parent) = parent => extends $(&parent.name)) {
            $(member_defs(&def.members, true, &HashMap::new()))

            private $name() { throw new UnsupportedOperationException(); }

            private $name(Builder builder) {
                $(if parent.is_some() => super(builder);)
                $(builder_assigns(&def.members))
            }

            public static $name fromBytes(byte[] bytes) {
                return $name.fromBytes($(&*import::BB).wrap(bytes).order($endianness));
            }
            $(if let Some(parent) = parent {
                protected static $name fromBytes($(&*import::BB) buf) {
                    if ($(&parent.name).fromBytes(buf) instanceof $name self) {
                        return self;
                    } else {
                        throw new IllegalArgumentException("Provided bytes decodes to a different subpacket of " + $(quoted(&parent.name)));
                    }
                }

                protected static $(&parent.name).Builder<?> fromPayload($(&*import::BB) buf) {
                    Builder builder = new Builder();
                    $(decoder(def, assignment::build, endianness))
                    return builder;
                }
            } else {
                protected static $name fromBytes($(&*import::BB) buf) {
                    Builder builder = new Builder();
                    $(decoder(def, assignment::build, endianness))
                    return builder.build();
                }
            })

            // Encoder
            $(if parent.is_some() {
                @Override
                public byte[] toBytes() {
                    $(&*import::BB) buf = $(&*import::BB)
                        .allocate(fieldWidth())
                        .order($endianness);

                    $(encoder(&def.alignment, &def.width_fields))
                    return super.toBytes(buf);
                }
            } else {
                public byte[] toBytes() {
                    $(&*import::BB) buf = $(&*import::BB)
                        .allocate(fieldWidth())
                        .order($endianness);

                    $(encoder(&def.alignment, &def.width_fields))
                    return buf.array();
                }
            })

            $(if parent.is_some() => @Override)
            public int width() {
                $(width_def(parent.is_some()))
            }

            $(field_width_def(&def.members))

            $(getter_defs(&def.members))

            @Override
            public String toString() {
                $(let members_str = quote!(
                    $(for member in def.members.iter() {
                        + $(match member.width() {
                            Some(width) => $(quoted(format!("{}[{}]=", member.name(), width))),
                            None => $(quoted(format!("{}=", member.name())))
                        }) + $(member.stringify(&def.width_fields)) + "\n"
                    }) + "}"
                ))

                $(if parent.is_some() {
                    return super.toString($(quoted("{\n")) $members_str, fieldWidth());
                } else {
                    return $(quoted(format!("{}{{\n", name))) $members_str;
                })
            }

            $(hashcode_equals_overrides(name, &def.members, parent.is_some()))

            public static final class $(match parent {
                Some(Parent { name: parent_name, does_constrain: true }) =>
                    Builder extends $parent_name.$(name)Builder<Builder>,
                Some(Parent { name: parent_name, does_constrain: false }) =>
                    Builder extends $parent_name.UnconstrainedBuilder<Builder>,
                _ => Builder,
            }) {
                $(member_defs(&def.members, false, &HashMap::new()))

                public Builder() { $(if parent.is_some() => super();) }

                protected Builder self() { return this; }

                public $name build() { return new $name(this); }

                $(setter_defs(&def.members, &quote!(Builder), &HashMap::new(), &def.width_fields))
            }
        }
    }
}

pub fn gen_abstract_packet(
    name: &String,
    def: &PacketDef,
    parent: Option<&Parent>,
    children: &Vec<Child>,
    endianness: EndiannessValue,
) -> Tokens<Java> {
    quote! {
        public abstract sealed class $name
        $(if let Some(parent) = parent => extends $(&parent.name))
        permits $(for child in children.iter() join (, ) => $(&child.name)) {
            $(member_defs(&def.members, true, &HashMap::new()))

            protected $name() { throw new UnsupportedOperationException(); }

            protected $name(Builder<?> builder) {
                $(if parent.is_some() => super(builder);)
                $(builder_assigns(&def.members))
            }

            // Decoder
            $(if let Some(parent) = parent {
                protected static $(&parent.name).Builder<?> fromPayload($(&*import::BB) buf) {
                    $(decoder(def, assignment::declare_locally, endianness))
                    $(build_child_fitting_constraints(&def.members, children))
                    return builder;
                }
            } else {
                public static $name fromBytes(byte[] bytes) {
                    return $name.fromBytes($(&*import::BB).wrap(bytes).order($endianness));
                }

                protected static $name fromBytes($(&*import::BB) buf) {
                    $(decoder(def, assignment::declare_locally, endianness))
                    $(build_child_fitting_constraints(&def.members, children))
                    return builder.build();
                }
            })

            $(if parent.is_none() => public abstract byte[] toBytes();)

            protected byte[] toBytes($(&*import::BB) payload) {
                payload.rewind();
                $(&*import::BB) buf = $(&*import::BB)
                    .allocate(fieldWidth() + payload.capacity())
                    .order($endianness);

                $(encoder(&def.alignment, &def.width_fields))
                return buf.array();
            }

            $(if parent.is_some() => @Override)
            protected int width() {
                $(width_def(parent.is_some()))
            }

            $(field_width_def(&def.members))

            $(getter_defs(&def.members))

            $(if parent.is_some() => @Override)
            protected String toString(String payload, int payloadSize) {
                $(let members_str = quote!(
                    $(for member in def.members.iter() {
                        + $(match member.width() {
                            Some(width) => {
                                $(quoted(format!("{}[{}]=", member.name(), width))) +
                                    $(if member.name() == "payloadSize" {
                                        Integer.toHexString(payloadSize)
                                    } else {
                                        $(member.stringify(&def.width_fields))
                                    } )
                            },
                            None => {
                                $(quoted(format!("{}=", member.name()))) +
                                $(if member.name() == "payload" {
                                    payload
                                } else {
                                    $(member.stringify(&def.width_fields))
                                })
                            }
                        })  + "\n"
                    }) + "}"
                ))

                $(if parent.is_some() {
                    return super.toString($(quoted("{\n")) $members_str, fieldWidth());
                } else {
                    return $(quoted(format!("{}{{\n", name))) $members_str;
                })
            }

            $(if !def.members.is_empty() => $(hashcode_equals_overrides(name, &def.members, false)))

            protected abstract static class $(match &parent {
                Some(Parent { name: parent_name, does_constrain: true }) =>
                    Builder<B extends $parent_name.$(name)Builder<B>>
                        extends $parent_name.$(name)Builder<B>,
                Some(Parent { name: parent_name, does_constrain: false }) =>
                    Builder<B extends $parent_name.UnconstrainedBuilder<B>>
                        extends $parent_name.UnconstrainedBuilder<B>,
                _ => Builder<B extends Builder<B>>,
            }) {
                $(member_defs(&def.members, false, &HashMap::new()))

                protected abstract B self();

                protected abstract $name build();
            }

            protected abstract static class UnconstrainedBuilder<B extends Builder<B>> extends Builder<B> {
                $(setter_defs(&def.members, &quote!(B), &HashMap::new(), &def.width_fields))
            }

            $(for child in children.iter() {
                $(if !child.constraints.is_empty() {
                    protected abstract static class $(&child.name)Builder<B extends Builder<B>> extends Builder<B> {
                        protected $(&child.name)Builder() {
                            $(for (member_name, value) in child.constraints.iter() {
                                // TODO(jmes): handle case when constraining member of ancestor
                                // This will likely require putting tag_id in constraint
                                $member_name =
                                    $(def.members.iter().find(|member| member.name() == member_name).unwrap()
                                        .constraint(value));
                            })
                        }

                        $(setter_defs(&def.members, &quote!(B), &child.constraints, &def.width_fields))
                    }
                })
            })
        }
    }
}

fn member_defs(
    members: &Vec<Field>,
    are_final: bool,
    constraints: &HashMap<String, String>,
) -> Tokens<Java> {
    quote! {
        $(for member in members.iter().filter(|member| member.is_member()) {
            protected $(if are_final => final) $(member.ty()) $(member.name())
                $(if let Some(constraint) = constraints.get(member.name()) => = $constraint);
        })
    }
}

fn getter_defs(members: &Vec<Field>) -> Tokens<Java> {
    quote! {
        $(for member in members.iter().filter(|member| member.is_member()) {
            public $(member.ty()) get$(member.name().to_upper_camel_case())() {
                return $(member.name());
            }
        })
    }
}

fn setter_defs(
    members: &Vec<Field>,
    builder_type: &Tokens<Java>,
    constraints: &HashMap<String, ConstrainedTo>,
    width_fields: &HashMap<String, WidthField>,
) -> Tokens<Java> {
    quote! {
        $(for member in members.iter().filter(|member| member.is_member()) {
            $(if !constraints.contains_key(member.name()) {
                public $builder_type set$(member.name().to_upper_camel_case())(
                    $(member.ty()) $(member.name())
                ) {
                    $(match member {
                        Field::Integral { width, ty, .. } => {
                            if ($(ty.compare(member.name(), gen_mask(*width, *ty))) > 0) {
                                throw new IllegalArgumentException(
                                    "Value " +
                                    $(member.stringify(width_fields)) +
                                    $(quoted(format!(
                                        " is too wide for field '{}' with width {}", member.name(), width)))
                                );
                            }
                        }
                        Field::Payload { .. } => {
                            $(if let Some(WidthField::Size { field_width, .. }) = width_fields.get("payload") {
                                if (
                                    $(Integral::Int.compare(
                                        quote!($(member.name()).length),
                                        gen_mask(*field_width, Integral::fitting(*field_width).limit_to_int()))
                                    ) > 0
                                ) {
                                    throw new IllegalArgumentException(
                                        "Payload " +
                                        $(member.stringify(width_fields)) +
                                        $(quoted(format!(
                                            " is too wide for its _size_ field with width {}", field_width)))
                                    );
                                }
                            })
                        }
                        _ =>, // No special checks for other members.
                    })

                    this.$(member.name()) = $(member.name());
                    return self();
                }
            })
        })
    }
}

fn builder_assigns(members: &Vec<Field>) -> Tokens<Java> {
    quote! {
        $(for member in members.iter().filter(|member| member.is_member()) {
            $(member.name()) = builder.$(member.name());
        })
    }
}

fn encoder(
    alignment: &Alignment<Field>,
    width_fields: &HashMap<String, WidthField>,
) -> Tokens<Java> {
    let mut tokens = Tokens::new();

    for chunk in alignment.iter() {
        match chunk {
            Chunk::Bitpack { fields, width } => {
                let chunk_type = Integral::fitting(*width);
                let t = ExprTree::new();
                let root = t.cast(
                    t.or_all(
                        fields
                            .iter()
                            .map(|field| {
                                t.lshift(
                                    t.rshift(
                                        t.symbol(
                                            field
                                                .symbol
                                                .try_encode_to_num(
                                                    field.symbol.name(),
                                                    width_fields,
                                                )
                                                .unwrap(),
                                            Integral::fitting(field.symbol.width().unwrap()),
                                        ),
                                        t.num(field.symbol_offset),
                                    ),
                                    t.num(field.chunk_offset),
                                )
                            })
                            .collect(),
                    ),
                    chunk_type,
                );

                tokens.extend(quote!(buf.$(chunk_type.encoder())($(gen_expr(&t, root)));));
            }
            Chunk::SizedBytes {
                symbol: member @ Field::ArrayElem { val, count },
                alignment,
                width,
            } => {
                tokens.extend(quote!(
                    for (int i = 0; i < $(member.name()).length; i++) {
                        $(for partial in alignment.iter() {
                            $(let partial_ty = Integral::fitting(partial.width))
                            $(let t = ExprTree::new())
                            $(let root = t.cast(
                                t.rshift(
                                    t.symbol(
                                        val.try_encode_to_num(
                                            quote!($(member.name())[i]), width_fields
                                        ).unwrap(),
                                        Integral::fitting(*width),
                                    ),
                                    t.num(partial.offset),
                                ),
                                partial_ty,
                            ))

                            buf.$(partial_ty.encoder())($(gen_expr(&t, root)));
                        })
                    }
                ));
            }
            Chunk::UnsizedBytes(Field::Payload { .. }) => tokens.extend(quote!(buf.put(payload);)),
            Chunk::UnsizedBytes(member @ Field::ArrayElem { val, count }) => {
                tokens.extend(quote!(
                    for (int i = 0; i < $(member.name()).length; i++) {
                        buf.put($(member.name())[i].toBytes());
                    }
                ));
            }
            Chunk::UnsizedBytes(member) => {
                tokens.extend(quote!(buf.put($(member.name()).toBytes());))
            }
            _ => unreachable!(),
        }
    }

    tokens
}

fn decoder(
    def: &PacketDef,
    assign: fn(Tokens<Java>, &str, Tokens<Java>) -> Tokens<Java>,
    endianness: EndiannessValue,
) -> Tokens<Java> {
    let mut tokens = Tokens::new();

    let t = ExprTree::new();
    let mut partials = Vec::new();
    for (i, chunk) in def.alignment.iter().enumerate() {
        match chunk {
            Chunk::Bitpack { fields, width } => {
                let chunk_name = &quote!(chunk$i);
                let chunk_type = Integral::fitting(*width);
                tokens.extend(
                    quote!($chunk_type $chunk_name = $(chunk_type.decode_from(quote!(buf)));),
                );

                for field in fields.iter() {
                    if field.is_partial {
                        partials.push(t.lshift(
                            t.mask(
                                t.cast(
                                    t.symbol(chunk_name, chunk_type),
                                    Integral::fitting(field.symbol.width().unwrap()),
                                ),
                                field.width,
                            ),
                            t.num(field.symbol_offset),
                        ));

                        if field.symbol_offset + field.width == field.symbol.width().unwrap() {
                            // We have all partials of the value and can write the decoder for it
                            tokens.extend(assign(
                                field.symbol.ty(),
                                field.symbol.name(),
                                field
                                    .symbol
                                    .try_decode_from_num(gen_expr(
                                        &t,
                                        t.or_all(partials.drain(..).collect()),
                                    ))
                                    .unwrap(),
                            ));
                            t.clear();
                        }
                    } else {
                        tokens.extend(assign(
                            field.symbol.ty(),
                            field.symbol.name(),
                            field
                                .symbol
                                .try_decode_from_num(gen_expr(
                                    &t,
                                    t.cast(
                                        t.mask(
                                            t.rshift(
                                                t.symbol(chunk_name, chunk_type),
                                                t.num(field.chunk_offset),
                                            ),
                                            field.width,
                                        ),
                                        Integral::fitting(field.symbol.width().unwrap()),
                                    ),
                                ))
                                .unwrap(),
                        ));
                        t.clear();
                    }
                }
            }
            Chunk::SizedBytes {
                symbol: member @ Field::ArrayElem { val: elem, count },
                alignment,
                width,
            } => {
                let name = member.name();
                let t = ExprTree::new();
                let root = t.or_all(
                    alignment
                        .iter()
                        .map(|partial| {
                            let ty = Integral::fitting(partial.width);
                            t.lshift(
                                t.symbol(ty.decode_from(quote!(buf)), ty),
                                t.num(partial.offset),
                            )
                        })
                        .collect(),
                );

                tokens.extend(declare_array_count(elem, *count, &def.width_fields));
                tokens.extend(quote!(
                    $(member.ty()) $(name) = new $(elem.ty())[$(name)Count];
                    for (int i = 0; i < $(name)Count; i++) {
                        $(name)[i] = $(elem.try_decode_from_num(gen_expr(&t, root)).unwrap());
                    }
                    $(assign(member.ty(), name, quote!($(name))))
                ))
            }
            Chunk::UnsizedBytes(member @ Field::ArrayElem { val, count }) => {
                let name = member.name();
                tokens.extend(declare_array_count(val, *count, &def.width_fields));
                tokens.extend(quote!(
                    $(member.ty()) $name = new $(val.ty())[$(name)Count];
                    for (int i = 0; i < $(name)Count; i++) {
                        $name[i] = $(val.ty()).fromBytes(buf);
                    }
                    // TODO: this will break with declare_locally
                    $(assign(
                        member.ty(),
                        name,
                        quote!($name),
                    ));
                ))
            }
            Chunk::UnsizedBytes(member @ Field::Payload { .. }) => {
                if def.width_fields.contains_key("payload") {
                    // The above condition checks if *this* class contains a size field for the payload.
                    // Unpacking `size_field` from the `UnsizedMember::Payload` would instead check if
                    // *any* class contains a size field for the payload.
                    tokens.extend(assign(
                        member.ty(),
                        member.name(),
                        quote!(buf.slice(buf.position(), payloadSize).order($endianness)),
                    ));
                } else {
                    if let Some(width) = def.width {
                        tokens.extend(assign(
                            member.ty(),
                            member.name(),
                            quote!(buf.slice(buf.position(), buf.capacity() - $(width / 8)).order($endianness))
                        ));
                    } else {
                        // If we don't know the payload's width, assume it is the last field in the packet
                        // (this should really be enforced by the parser) and consume all remaining bytes
                        // in the buffer.
                        tokens.extend(assign(member.ty(), member.name(), quote!(buf.slice())));
                    }
                }
            }
            Chunk::UnsizedBytes(member @ Field::StructRef { name, ty, .. }) => {
                let var_name = &name.to_lower_camel_case();
                // If the struct has dynamic width, assume it is the last field in the packet (this should
                // really be enforced by the parser) and decode it. Its decoder will consume all remaining
                // bytes in the buffer.
                tokens.extend(quote!(
                    $ty $var_name = $ty.fromBytes(buf.slice());
                    $(assign(member.ty(), member.name(), quote!($var_name)));
                    buf.position(buf.position() + $var_name.width());
                ));
            }
            _ => unreachable!(),
        }
    }

    tokens
}

mod assignment {
    use super::*;

    pub fn build(ty: Tokens<Java>, name: &str, value: Tokens<Java>) -> Tokens<Java> {
        if name.ends_with("Size") || name.ends_with("Count") {
            // Don't need to set size/count fields in builder.
            declare_locally(ty, name, value)
        } else if name == "payload" {
            quote!(builder.set$(name.to_upper_camel_case())($value.array());)
        } else {
            quote!(builder.set$(name.to_upper_camel_case())($value);)
        }
    }

    pub fn declare_locally(ty: Tokens<Java>, name: &str, value: Tokens<Java>) -> Tokens<Java> {
        if name == "payload" {
            quote!($(&*import::BB) payload = $value;)
        } else if ty.to_string().unwrap().ends_with("[]") {
            quote!()
        } else {
            quote!($ty $name = $value;)
        }
    }
}

fn width_def(call_super: bool) -> Tokens<Java> {
    quote! {
        return $(if call_super => super.width() +) fieldWidth();
    }
}

fn field_width_def(members: &Vec<Field>) -> Tokens<Java> {
    let static_width = members.iter().filter_map(|field| field.width()).sum::<usize>() / 8;
    let unsized_members: Vec<&Field> = members
        .iter()
        .filter(|field| field.width().is_none())
        .filter(|field| if let Field::Payload { is_member } = field { *is_member } else { true })
        .collect();

    let t = ExprTree::new();
    let root = if unsized_members.is_empty() {
        t.num(static_width)
    } else {
        t.add(
            t.num(static_width),
            t.symbol(
                quote!($(for member in unsized_members.into_iter() join ( + ) => $(member.width_expr()))),
                Integral::Int
            ),
        )
    };
    quote! {
        private final int fieldWidth() {
            return $(gen_expr(&t, root));
        }
    }
}

fn hashcode_equals_overrides(name: &str, members: &Vec<Field>, call_super: bool) -> Tokens<Java> {
    quote! {
        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (!(o instanceof $name other)) return false;
            return $(if call_super => super.equals(other) &&)
            $(for member in members.iter().filter(|m| m.is_member()) join ( && ) {
                $(member.equals(quote!(other.$(member.name()))))
            });
        }

        @Override
        public int hashCode() {
            $(let mut members = members.iter().filter(|m| m.is_member()))
            $(if call_super {
                int result = super.hashCode();
            } else {
                $(let first = members.next().expect("cannot generate hashCode for packet with no members"))
                int result = $(first.hash_code());
            })

            $(for member in members => result = 31 * result + $(member.hash_code());)
            return result;
        }
    }
}

fn build_child_fitting_constraints(members: &Vec<Field>, children: &Vec<Child>) -> Tokens<Java> {
    let mut children_iter = children.iter();
    let default = children_iter.next().expect("Parent packet must have at least 1 child");
    let children: Vec<&Child> = children_iter
        .filter(|child| !child.constraints.is_empty() || child.width.is_some())
        .collect();

    quote! {
        $(if children.is_empty() {
            Builder<?> builder = $(&default.name).fromPayload(payload);
        } else {
            Builder<?> builder;
            $(for child in children {
                if (
                    $(for member in members.iter() {
                        $(if let Some(value) = child.constraints.get(member.name()) {
                            $(member.equals(member.constraint(value)))
                        })
                    })
                    $(if !child.constraints.is_empty() && child.width.is_some() => &&)
                    $(if let Some(width) = child.width => payload.capacity() == $(width / 8))
                ) {
                    builder = $(&child.name).fromPayload(payload);
                } else$[' ']
            }) {
                builder = $(&default.name).fromPayload(payload);
            }
        })

        $(for member in members.iter().filter(|member| member.is_member()) {
            builder.$(member.name()) = $(member.name());
        })
    }
}

/// Generates a decalaration $(val.name())Count that stores the number of elements in the array
/// described by the function args.
fn declare_array_count(
    val: &Box<Field>,
    count: Option<usize>,
    width_fields: &HashMap<String, WidthField>,
) -> Tokens<Java> {
    let name = val.name();

    if let Some(count) = count {
        quote!(int $(name)Count = $count;)
    } else {
        match width_fields.get(name) {
            Some(WidthField::Size { elem_width, .. }) => {
                let t = ExprTree::new();
                let root =
                    t.div(t.symbol(quote!($(name)Size), Integral::Int), t.num(*elem_width / 8));
                quote!(int $(name)Count = $(gen_expr(&t, root));)
            }
            Some(WidthField::Count { .. }) => {
                // No-op: $(name)Count should already be declared
                quote!()
            }
            None => {
                if let Some(elem_width) = val.width() {
                    let t = ExprTree::new();
                    let root = t.div(
                        t.symbol(quote!(buf.remaining()), Integral::Int),
                        t.num(elem_width / 8),
                    );
                    quote!(int $(name)Count = $(gen_expr(&t, root));)
                } else {
                    panic!("Cannot decode array of dynamic elements with no count")
                }
            }
        }
    }
}
