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
        java::{
            codegen::expr::ExprId,
            inheritance::{ClassHeirarchy, Constraint, InheritanceNode},
            Context, Field, WidthField,
        },
    },
};

use super::{expr::ExprTree, import, Chunk, Integral, PacketDef};

pub fn gen_packet(name: &String, def: &PacketDef, ctx: &Context) -> Tokens<Java> {
    let endianness = ctx.endianness;
    let parent = ctx.heirarchy.parent(name);

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
                        throw new IllegalArgumentException(
                            "Provided bytes decode to a different subpacket of " +
                            $(quoted(&parent.name))
                        );
                    }
                }

                protected static $(&parent.name).Builder<?> fromPayload($(&*import::BB) buf) {
                    Builder builder = new Builder();
                    $(decoder(name, def, assignment::build, ctx))
                    return builder;
                }
            } else {
                protected static $name fromBytes($(&*import::BB) buf) {
                    Builder builder = new Builder();
                    $(decoder(name, def, assignment::build, ctx))
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

                    $(encoder(&def.alignment, &def.width_fields, endianness))
                    return super.toBytes(buf);
                }
            } else {
                public byte[] toBytes() {
                    $(&*import::BB) buf = $(&*import::BB)
                        .allocate(fieldWidth())
                        .order($endianness);

                    $(encoder(&def.alignment, &def.width_fields, endianness))
                    return buf.array();
                }
            })

            $(width_def(name, &ctx.heirarchy, true))

            $(field_width_def(name, &ctx.heirarchy, &def.members))

            $(getter_defs(&def.members))

            @Override
            public String toString() {
                $(let members_str = quote!(
                    $(for member in def.members.iter() {
                        + $(match member.width().or_else(|| ctx.heirarchy.width(member.name())) {
                            Some(width) => $(quoted(format!("{}[{}]=", member.name(), width))),
                            None => $(quoted(format!("{}=", member.name())))
                        }) + $(member.stringify(&def.width_fields)) + ", "
                    }) + "}"
                ))

                $(if parent.is_some() {
                    return super.toString($(quoted("{ ")) $members_str, fieldWidth());
                } else {
                    return $(quoted(format!("{name}{{ "))) $members_str;
                })
            }

            $(hashcode_equals_overrides(name, &def.members, parent.is_some()))

            public static final class
            $(if let Some(parent) = parent {
                $(if ctx.heirarchy.get(name).constraints.is_empty() {
                    Builder extends $(&parent.name).UnconstrainedBuilder<Builder>
                } else {
                    Builder extends $(&parent.name).$(name)Builder<Builder>
                })
            } else {
                Builder
            }) {
                $(member_defs(&def.members, false, &HashMap::new()))

                public Builder() { $(if parent.is_some() => super();) }

                protected Builder self() { return this; }

                public $name build() { return new $name(this); }

                $(setter_defs(
                    &def.members, &quote!(Builder),
                    &HashMap::new(), &def.width_fields, &ctx.heirarchy))
            }
        }
    }
}

pub fn gen_abstract_packet(
    name: &String,
    def: &PacketDef,
    fallback_child: Option<&String>,
    ctx: &Context,
) -> Tokens<Java> {
    let endianness = ctx.endianness;
    let parent = ctx.heirarchy.parent(name);
    let children = &ctx.heirarchy.children(name);

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
                    $(decoder(name, def, assignment::declare_locally, ctx))
                    $(build_child_fitting_constraints(name, &def.members, children, fallback_child))
                    return builder;
                }
            } else {
                public static $name fromBytes(byte[] bytes) {
                    return $name.fromBytes($(&*import::BB).wrap(bytes).order($endianness));
                }

                protected static $name fromBytes($(&*import::BB) buf) {
                    $(decoder(name, def, assignment::declare_locally, ctx))
                    $(build_child_fitting_constraints(name, &def.members, children, fallback_child))
                    return builder.build();
                }
            })

            $(if parent.is_none() => public abstract byte[] toBytes();)

            protected byte[] toBytes($(&*import::BB) payload) {
                payload.rewind();
                $(&*import::BB) buf = $(&*import::BB)
                    .allocate(fieldWidth() + payload.limit())
                    .order($endianness);

                $(encoder(&def.alignment, &def.width_fields, endianness))
                return buf.array();
            }

            $(width_def(name, &ctx.heirarchy, false))

            $(field_width_def(name, &ctx.heirarchy, &def.members))

            $(getter_defs(&def.members))

            $(if parent.is_some() => @Override)
            protected String toString(String payload, int payloadSize) {
                $(let members_str = quote!(
                    $(for member in def.members.iter() {
                        + $(match member.width().or_else(|| ctx.heirarchy.width(member.name())) {
                            Some(width) => {
                                $(quoted(format!("{}[{}]=", member.name(), width))) +
                                    $(if member.name() == "payloadSize" {
                                        Integer.toHexString(payloadSize)
                                        $(if let Some(modifier) =
                                            def.width_fields.get("payload").unwrap().modifier() {
                                                + "(+" + $modifier + ")"
                                        })
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
                        })  + ", "
                    }) + "}"
                ))

                $(if parent.is_some() {
                    return super.toString($(quoted("{ ")) $members_str, fieldWidth());
                } else {
                    return $(quoted(format!("{name}{{ "))) $members_str;
                })
            }

            $(if !def.members.is_empty() => $(hashcode_equals_overrides(name, &def.members, false)))

            protected abstract static class
            $(if let Some(parent) = parent {
                $(if ctx.heirarchy.get(name).constraints.is_empty() {
                    Builder<B extends $(&parent.name).UnconstrainedBuilder<B>>
                    extends $(&parent.name).UnconstrainedBuilder<B>
                } else {
                    Builder<B extends $(&parent.name).$(name)Builder<B>>
                    extends $(&parent.name).$(name)Builder<B>
                })
            } else {
                Builder<B extends Builder<B>>
            }) {
                $(member_defs(&def.members, false, &HashMap::new()))

                protected abstract B self();

                protected abstract $name build();
            }

            protected abstract static class UnconstrainedBuilder<B extends Builder<B>> extends Builder<B> {
                $(setter_defs(&def.members, &quote!(B), &HashMap::new(), &def.width_fields, &ctx.heirarchy))
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

                        $(setter_defs(&def.members, &quote!(B),
                            &child.constraints, &def.width_fields, &ctx.heirarchy))
                    }
                })
            })
        }
    }
}

fn member_defs(
    members: &[Field],
    are_final: bool,
    constraints: &HashMap<String, String>,
) -> Tokens<Java> {
    quote! {
        $(for member in members.iter().filter(|member| member.is_member()) {
            $(let is_final = are_final || constraints.contains_key(member.name()))
            protected $(if is_final => final) $(member.ty()) $(member.name())
                $(if let Some(constraint) = constraints.get(member.name()) => = $constraint)
                $(if let Some(fixed_val) = member.fixed_val() => = $fixed_val);
        })
    }
}

fn getter_defs(members: &[Field]) -> Tokens<Java> {
    quote! {
        $(for member in members.iter().filter(|member| member.is_member()) {
            public $(member.ty()) get$(member.name().to_upper_camel_case())() {
                return $(member.name());
            }
        })
    }
}

fn setter_defs(
    members: &[Field],
    builder_type: &Tokens<Java>,
    constraints: &HashMap<String, Constraint>,
    width_fields: &HashMap<String, WidthField>,
    heirarchy: &ClassHeirarchy,
) -> Tokens<Java> {
    quote! {
        $(for member in members.iter().filter(
            |member| member.is_member() && !constraints.contains_key(member.name())
        ) {
            public $builder_type set$(member.name().to_upper_camel_case())(
                $(member.ty()) $(member.name())
            ) {
                $(match member {
                    Field::Integral { width, ty, .. } if *width != 1 => {
                        $(let t = ExprTree::new())
                        if ($(t.compare_width(t.symbol(member.name(), *ty), *width)) > 0) {
                            throw new IllegalArgumentException(
                                "Value " +
                                $(member.stringify(width_fields)) +
                                $(quoted(format!(
                                    " is too wide for field '{}' with width {}",
                                    member.name(),
                                    width
                                )))
                            );
                        }
                    }
                    Field::Payload { width_field_width: Some(width), size_modifier, .. } => {
                        $(assert_array_fits_width_field(
                            quote!($(member.name()).length),
                            member.stringify(width_fields),
                            *width,
                            *size_modifier,
                        ))
                    }
                    Field::ArrayElem { count: Some(count), .. } => {
                        if ($(member.name()).length != $(*count)) {
                            throw new IllegalArgumentException(
                                "Array " +
                                $(member.stringify(width_fields)) +
                                $(quoted(format!(
                                    " is too long for field '{}' with length {}",
                                    member.name(),
                                    count
                                )))
                            );
                        }
                    }
                    Field::ArrayElem { .. } => {
                        $(match width_fields.get(member.name()) {
                            Some(WidthField::Count { field_width, modifier }) => {
                                $(assert_array_fits_width_field(
                                    quote!($(member.name()).length),
                                    member.stringify(width_fields),
                                    *field_width,
                                    *modifier,
                                ))
                            }
                            Some(WidthField::Size { field_width, modifier, .. }) => {
                                $(assert_array_fits_width_field(
                                    member.width_expr(heirarchy),
                                    member.stringify(width_fields),
                                    *field_width,
                                    *modifier,
                                ))
                            }
                            _ =>,
                        })
                    }
                    _ =>, // No special checks for other members.
                })

                this.$(member.name()) = $(member.name());
                return self();
            }
        })
    }
}

fn assert_array_fits_width_field(
    arr_width_expr: Tokens<Java>,
    stringified_arr: Tokens<Java>,
    width_field_width: usize,
    width_field_modifier: Option<usize>,
) -> Tokens<Java> {
    let t = ExprTree::new();
    let compare_width = t.compare_width(
        if let Some(modifier) = width_field_modifier {
            t.add(t.symbol(arr_width_expr, Integral::Int), t.num(modifier))
        } else {
            t.symbol(arr_width_expr, Integral::Int)
        },
        width_field_width,
    );

    quote! {
        if ($compare_width > 0) {
            throw new IllegalArgumentException(
                "Value " + $stringified_arr + $(quoted(format!(
                    " is too wide for its _size_ or _count_ field with width {width_field_width}"
                )))
            );
        }
    }
}

fn builder_assigns(members: &[Field]) -> Tokens<Java> {
    quote! {
        $(for member in members.iter().filter(|member| member.is_member()) {
            $(member.name()) = builder.$(member.name());
        })
    }
}

fn encoder(
    alignment: &Alignment<Field>,
    width_fields: &HashMap<String, WidthField>,
    endianness: EndiannessValue,
) -> Tokens<Java> {
    alignment
        .iter()
        .flat_map(|chunk| match chunk {
            Chunk::Bitpack { fields, width } => {
                let t = ExprTree::new();
                let root = t.cast(
                    t.or_all(
                        fields
                            .iter()
                            .map(|field| {
                                t.lshift(
                                    field.symbol.to_num(&t, field.symbol.name(), width_fields),
                                    t.num(field.offset),
                                )
                            })
                            .collect(),
                    ),
                    Integral::fitting(*width),
                );

                quote!($(endianness.encode_bytes(quote!(buf), *width, t.gen_expr(root)));)
            }
            Chunk::Bytes { symbol: member @ Field::ArrayElem { val, .. }, width } => {
                let t = ExprTree::new();
                let root = val.to_num(&t, quote!($(member.name())[i]), width_fields);
                quote!(
                    for (int i = 0; i < $(member.name()).length; i++) {
                        $(endianness.encode_bytes(quote!(buf), *width, t.gen_expr(root)));
                    }
                )
            }
            Chunk::DynBytes(Field::Payload { .. }) => quote!(buf.put(payload);),
            Chunk::DynBytes(member @ Field::ArrayElem { .. }) => {
                quote!(
                    for (int i = 0; i < $(member.name()).length; i++) {
                        buf.put($(member.name())[i].toBytes());
                    }
                )
            }
            Chunk::DynBytes(member) => quote!(buf.put($(member.name()).toBytes());),
            other => {
                dbg!(other);
                unreachable!();
            }
        })
        .collect::<Tokens<Java>>()
}

fn decoder(
    name: &str,
    def: &PacketDef,
    assign: fn(&Field, &Tokens<Java>) -> Tokens<Java>,
    ctx: &Context,
) -> Tokens<Java> {
    let mut tokens = Tokens::new();

    for (i, chunk) in def.alignment.iter().enumerate() {
        match chunk {
            Chunk::Bitpack { fields, width } => {
                let chunk_name = &quote!(chunk$i);
                let chunk_type = Integral::fitting(*width);
                tokens.extend(
                    quote!($chunk_type $chunk_name = $(ctx.endianness.decode_bytes(quote!(buf), *width));)
                );

                for field in fields.iter().filter(|field| !field.symbol.is_reserved()) {
                    let t = ExprTree::new();

                    tokens.extend(assign(
                        &field.symbol,
                        &field.symbol.from_num(
                            t.gen_expr(t.mask(
                                t.symbol(chunk_name, chunk_type),
                                field.offset,
                                field.width,
                            )),
                            &def.width_fields,
                        ),
                    ));
                }
            }
            Chunk::Bytes { symbol: member @ Field::ArrayElem { val, count }, width } => {
                let name = member.name();
                tokens.extend(quote!(
                    $(declare_array_count(val, *count, &def.width_fields, &ctx.heirarchy).unwrap())
                    $(member.ty()) $(name) = new $(val.ty())[$(name)Count];
                    for (int i = 0; i < $(name)Count; i++) {
                        $(name)[i] = $(val.from_num(
                            ctx.endianness.decode_bytes(quote!(buf), *width),
                            &def.width_fields
                        ));
                    }
                    $(assign(member, &quote!($(name))))
                ))
            }
            Chunk::DynBytes(member @ Field::ArrayElem { val, count }) => {
                let arr_name = member.name();
                if let Some(count_decl) =
                    declare_array_count(val, *count, &def.width_fields, &ctx.heirarchy)
                {
                    tokens.extend(quote!(
                        $(count_decl)
                        $(member.ty()) $arr_name = new $(val.ty())[$(arr_name)Count];
                        for (int i = 0; i < $(arr_name)Count; i++) {
                            $arr_name[i] = $(val.ty()).fromBytes(buf);
                        }
                        $(assign(member, &quote!($arr_name)))
                    ))
                } else {
                    tokens.extend(quote!(
                        $(declare_array_size(name, val, &def.width_fields, &ctx.heirarchy))
                        $(&*import::LIST)<$(val.ty())> $(arr_name)List = new $(&*import::LIST)<$(val.ty())>();
                        while ($(arr_name)Size != 0) {
                            $(val.ty()) elem = $(val.ty()).fromBytes(buf);
                            $(arr_name)List.add(elem);
                            $(arr_name)Size -= elem.width();
                        }
                        $(assign(member, &quote!($(arr_name)List.toArray(new $(val.ty())[0]))))
                    ));
                }
            }
            Chunk::DynBytes(member @ Field::Payload { .. }) => {
                if def.width_fields.contains_key("payload") {
                    tokens.extend(assign(
                        member,
                        &quote!(buf.slice(buf.position(), payloadSize).order($(ctx.endianness))),
                    ));
                } else if let Some(width_without_payload) =
                    ctx.heirarchy.field_width_without_dyn_field(name, member.name())
                {
                    let t = ExprTree::new();
                    let root = t.sub(
                        t.symbol(quote!(buf.limit()), Integral::Int),
                        t.num(width_without_payload / 8),
                    );
                    tokens.extend(assign(
                        member,
                        &quote!(buf.slice(buf.position(), $(t.gen_expr(root))).order($(ctx.endianness)))
                    ));
                } else {
                    // Assume payload is the last field in the packet (this should really be enforced by the parser)
                    // and consume all remaining bytes in the buffer.
                    tokens.extend(assign(member, &quote!(buf.slice().order($(ctx.endianness)))));
                }
                tokens.extend(quote!(buf.position(buf.position() + payload.limit());));
            }
            Chunk::DynBytes(member @ Field::StructRef { name, ty, .. }) => {
                let var_name = &name.to_lower_camel_case();
                // Assume struct is the last field in the packet (this should really be enforced by the parser) and decode it.
                // Its decoder will consume all remaining bytes in the buffer.
                tokens.extend(quote!(
                    $ty $var_name = $ty.fromBytes(buf.slice().order($(ctx.endianness)));
                    $(assign(member, &quote!($var_name)))
                    buf.position(buf.position() + $var_name.width());
                ));
            }
            other => {
                dbg!(other);
                unreachable!();
            }
        }
    }

    tokens
}

mod assignment {
    use super::*;

    pub fn build(field: &Field, value: &Tokens<Java>) -> Tokens<Java> {
        match field {
            _ if field.is_width() => declare_locally(field, value),
            Field::Payload { .. } => quote!(
                $(&*import::BB) payload = $value;
                byte[] payloadBytes = new byte[payload.remaining()];
                payload.get(payloadBytes);
                builder.setPayload(payloadBytes);
            ),
            Field::Integral { ty, fixed_val: Some(fixed_val), .. } => {
                enforce_integral_fixed(*ty, value, *fixed_val)
            }
            Field::EnumRef { ty, fixed_tag: Some(fixed_tag), .. } => {
                enforce_enum_fixed(ty, value, fixed_tag)
            }
            _ => quote!(builder.set$(field.name().to_upper_camel_case())($value);),
        }
    }

    pub fn declare_locally(field: &Field, value: &Tokens<Java>) -> Tokens<Java> {
        match field {
            Field::Payload { .. } => quote!($(&*import::BB) payload = $value;),
            Field::ArrayElem { .. } => quote!(),
            Field::Integral { ty, fixed_val: Some(fixed_val), .. } => {
                enforce_integral_fixed(*ty, value, *fixed_val)
            }
            Field::EnumRef { ty, fixed_tag: Some(fixed_tag), .. } => {
                enforce_enum_fixed(ty, value, fixed_tag)
            }
            _ => quote!($(field.ty()) $(field.name()) = $value;),
        }
    }

    fn enforce_integral_fixed(ty: Integral, val: &Tokens<Java>, fixed_val: usize) -> Tokens<Java> {
        let t = ExprTree::new();
        quote!(
            if ($(t.compare(t.symbol(val, ty), t.num(fixed_val))) != 0) {
                throw new IllegalArgumentException(
                    "Value " + $val + " invalid for field fixed to " + $fixed_val
                );
            }
        )
    }

    fn enforce_enum_fixed(ty: &str, val: &Tokens<Java>, fixed_tag: &str) -> Tokens<Java> {
        quote!(
            if (!$val.equals($ty.$fixed_tag)) {
                throw new IllegalArgumentException(
                    "Value " + $val + " invalid for field fixed to " + $ty.$fixed_tag
                );
            }
        )
    }
}

fn width_def(name: &str, heirarchy: &ClassHeirarchy, is_public: bool) -> Tokens<Java> {
    quote! {
        $(if heirarchy.parent(name).is_some() => @Override)
        $(if is_public { public } else { protected }) int width() {
            $(if let Some(width) = heirarchy.width(name) {
                return $(width / 8);
            } else {
                return $(if heirarchy.parent(name).is_some() => super.width() +) fieldWidth();
            })

        }
    }
}

fn field_width_def(name: &str, heirarchy: &ClassHeirarchy, members: &[Field]) -> Tokens<Java> {
    let inheritence = heirarchy.get(name);

    let t = ExprTree::new();
    let mut exprs: Vec<ExprId> = members
        .iter()
        .filter_map(|member| {
            if member.name() == "payload" && !member.is_member() {
                None
            } else if inheritence.dyn_fields.contains(member.name()) {
                Some(t.symbol(member.width_expr(heirarchy), Integral::Int))
            } else {
                None
            }
        })
        .collect();
    exprs.push(t.num(heirarchy.static_field_width(name) / 8));

    quote! {
        private final int fieldWidth() {
            return $(t.gen_expr(t.sum(exprs)));
        }
    }
}

fn hashcode_equals_overrides(name: &str, members: &[Field], call_super: bool) -> Tokens<Java> {
    let members: Vec<&Field> = members.iter().filter(|m| m.is_member()).collect();
    quote! {
        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            $(if members.is_empty() {
                return $(if call_super => super.equals(other) &&) o instanceof $name;
            } else {
                if (!(o instanceof $name other)) return false;
                return $(if call_super => super.equals(other) &&)
                    $(for member in members.iter() join ( && ) {
                        $(member.equals(quote!(other.$(member.name()))))
                    });
            })
        }

        @Override
        public int hashCode() {
            $(if members.is_empty() {
                return $(if call_super => 31 * super.equals(other) +)
                    this.getClass().getSimpleName().hashCode();
            } else {
                $(let mut members = members.iter())
                $(if call_super {
                    int result = super.hashCode();
                } else {
                    $(let first = members.next().unwrap())
                    int result = $(first.hash_code());
                })

                $(for member in members => result = 31 * result + $(member.hash_code());)
                return result;
            })
        }
    }
}

fn build_child_fitting_constraints(
    name: &String,
    members: &[Field],
    children: &Vec<&InheritanceNode>,
    fallback_child: Option<&String>,
) -> Tokens<Java> {
    let mut children_iter = children.iter();
    let fallback = fallback_child
        .is_some()
        .then(|| children_iter.next().expect("Parent packet must have at least 1 child"));

    let children: Vec<&InheritanceNode> = children_iter
        .copied()
        .filter(|child| !child.constraints.is_empty() || child.field_width().is_some())
        .collect();

    let mut tokens = Tokens::new();

    if children.is_empty() {
        if let Some(fallback) = fallback {
            tokens.extend(quote!(
                Builder<?> builder = $(&fallback.name).fromPayload(payload);
            ));
        } else {
            panic!("Packet with _body_ field and no children!")
        }
    } else {
        tokens.extend(quote!(
            Builder<?> builder;
            $(for child in children {
                if (
                    $(for member in members.iter() {
                        $(if let Some(value) = child.constraints.get(member.name()) {
                            $(member.equals(member.constraint(value)))
                        })
                    })
                    $(if !child.constraints.is_empty() && child.field_width().is_some() => &&)
                    $(if let Some(width) = child.field_width() => payload.limit() == $(width / 8))
                ) {
                    builder = $(&child.name).fromPayload(payload);
                } else$[' ']
            })
        ));

        if let Some(fallback) = fallback {
            tokens.extend(quote!({
                builder = $(&fallback.name).fromPayload(payload);
            }))
        } else {
            tokens.extend(quote!({
                throw new IllegalArgumentException(
                    $(quoted(format!("Provided bytes do not decode into any child of {name}")))
                );
            }));
        }
    }

    tokens.extend(quote! {
        $(for member in members.iter().filter(|member| member.is_member()) {
            builder.$(member.name()) = $(member.name());
        })
    });

    tokens
}

/// Generates a decalaration $(val.name())Count that stores the number of elements in the array
/// described by the function args.
fn declare_array_count(
    val: &Field,
    count: Option<usize>,
    width_fields: &HashMap<String, WidthField>,
    heirarchy: &ClassHeirarchy,
) -> Option<Tokens<Java>> {
    let name = val.name();

    if let Some(count) = count {
        Some(quote!(int $(name)Count = $count;))
    } else {
        match width_fields.get(name) {
            Some(WidthField::Size { elem_width: Some(elem_width), .. }) => {
                let t = ExprTree::new();
                let root =
                    t.div(t.symbol(quote!($(name)Size), Integral::Int), t.num(*elem_width / 8));
                Some(quote!(int $(name)Count = $(t.gen_expr(root));))
            }
            Some(WidthField::Size { elem_width: None, .. }) => {
                // We have dynamic array of dynamically sized elements.
                // No way to calculate the count in this case.
                None
            }
            Some(WidthField::Count { .. }) => {
                // No-op: $(name)Count should already be declared
                Some(quote!())
            }
            None => {
                if let Some(elem_width) =
                    val.width().or_else(|| val.class().and_then(|class| heirarchy.width(class)))
                {
                    let t = ExprTree::new();
                    let root = t.div(
                        t.symbol(quote!(buf.remaining()), Integral::Int),
                        t.num(elem_width / 8),
                    );
                    Some(quote!(int $(name)Count = $(t.gen_expr(root));))
                } else {
                    None
                }
            }
        }
    }
}

fn declare_array_size(
    class_name: &str,
    val: &Field,
    width_fields: &HashMap<String, WidthField>,
    heirarchy: &ClassHeirarchy,
) -> Tokens<Java> {
    if let Some(WidthField::Size { .. }) = width_fields.get(val.name()) {
        // No-op: $(name)Size should already be declared
        quote!()
    } else if let Some(width_without_arr) =
        heirarchy.field_width_without_dyn_field(class_name, val.name())
    {
        let t = ExprTree::new();
        let root =
            t.sub(t.symbol(quote!(buf.limit()), Integral::Int), t.num(width_without_arr / 8));
        quote!(int $(val.name())Size = $(t.gen_expr(root));)
    } else {
        // Assume array is the last field in the packet and consume all remaining bytes.
        quote!(int $(val.name())Size = buf.limit();)
    }
}
