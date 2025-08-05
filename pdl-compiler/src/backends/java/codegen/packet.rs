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

use crate::backends::{
    common::alignment::Alignment,
    java::{
        codegen::expr::ExprId,
        inheritance::{ClassHeirarchy, Constraint, InheritanceNode},
        Context, Field, WidthField,
    },
};

use super::{
    expr::{gen_expr, gen_mask, ExprTree},
    import, Chunk, Integral, PacketDef,
};

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
                        throw new IllegalArgumentException("Provided bytes decodes to a different subpacket of " + $(quoted(&parent.name)));
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

                $(setter_defs(&def.members, &quote!(Builder), &HashMap::new(), &def.width_fields))
            }
        }
    }
}

pub fn gen_abstract_packet(name: &String, def: &PacketDef, ctx: &Context) -> Tokens<Java> {
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
                    $(build_child_fitting_constraints(&def.members, children))
                    return builder;
                }
            } else {
                public static $name fromBytes(byte[] bytes) {
                    return $name.fromBytes($(&*import::BB).wrap(bytes).order($endianness));
                }

                protected static $name fromBytes($(&*import::BB) buf) {
                    $(decoder(name, def, assignment::declare_locally, ctx))
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
    constraints: &HashMap<String, Constraint>,
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
                                    if field.symbol.is_reserved() {
                                        t.num(0)
                                    } else {
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
                                        )
                                    },
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
            Chunk::UnsizedBytes(member @ Field::ArrayElem { .. }) => {
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
    name: &String,
    def: &PacketDef,
    assign: fn(Tokens<Java>, &str, Tokens<Java>) -> Tokens<Java>,
    ctx: &Context,
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
                    let field_type = Integral::try_fitting(field.symbol.width().unwrap())
                        .unwrap_or(Integral::Long);

                    if field.is_partial {
                        partials.push(t.lshift(
                            t.mask(
                                t.cast(t.symbol(chunk_name, chunk_type), field_type),
                                field.width,
                            ),
                            t.num(field.symbol_offset),
                        ));

                        if field.symbol_offset + field.width == field.symbol.width().unwrap() {
                            // We have all partials of the value and can write the decoder for it
                            if !field.symbol.is_reserved() {
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
                            }
                            t.clear();
                        }
                    } else {
                        if !field.symbol.is_reserved() {
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
                                            field_type,
                                        ),
                                    ))
                                    .unwrap(),
                            ));
                        }
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

                tokens.extend(declare_array_count(elem, *count, &def.width_fields, &ctx.heirarchy));
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
                tokens.extend(declare_array_count(val, *count, &def.width_fields, &ctx.heirarchy));
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
                    ))
                ))
            }
            Chunk::UnsizedBytes(member @ Field::Payload { .. }) => {
                if def.width_fields.contains_key("payload") {
                    tokens.extend(assign(
                        member.ty(),
                        member.name(),
                        quote!(buf.slice(buf.position(), payloadSize).order($(ctx.endianness))),
                    ));
                } else if let Some(width) =
                    ctx.heirarchy.field_width_without_dyn_field(name, member.name())
                {
                    let t = ExprTree::new();
                    let root =
                        t.sub(t.symbol(quote!(buf.capacity()), Integral::Int), t.num(width / 8));
                    tokens.extend(assign(
                            member.ty(),
                            member.name(),
                            quote!(buf.slice(buf.position(), $(gen_expr(&t, root))).order($(ctx.endianness)))
                        ));
                } else {
                    // If we don't know the payload's width, assume it is the last field in the packet
                    // (this should really be enforced by the parser) and consume all remaining bytes
                    // in the buffer.
                    tokens.extend(assign(
                        member.ty(),
                        member.name(),
                        quote!(buf.slice().order($(ctx.endianness))),
                    ));
                }
                tokens.extend(quote!(buf.position(buf.position() + payload.capacity());));
            }
            Chunk::UnsizedBytes(member @ Field::StructRef { name, ty, .. }) => {
                let var_name = &name.to_lower_camel_case();
                // If the struct has dynamic width, assume it is the last field in the packet (this should
                // really be enforced by the parser) and decode it. Its decoder will consume all remaining
                // bytes in the buffer.
                tokens.extend(quote!(
                    $ty $var_name = $ty.fromBytes(buf.slice().order($(ctx.endianness)));
                    $(assign(member.ty(), member.name(), quote!($var_name)))
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
            quote!(
                $(&*import::BB) payload = $value;
                byte[] payloadBytes = new byte[payload.remaining()];
                payload.get(payloadBytes);
                builder.setPayload(payloadBytes);
            )
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

fn field_width_def(name: &str, heirarchy: &ClassHeirarchy, members: &Vec<Field>) -> Tokens<Java> {
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
            return $(gen_expr(&t, t.sum(exprs)));
        }
    }
}

fn hashcode_equals_overrides(name: &str, members: &Vec<Field>, call_super: bool) -> Tokens<Java> {
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
    members: &Vec<Field>,
    children: &Vec<&InheritanceNode>,
) -> Tokens<Java> {
    let mut children_iter = children.iter();
    let default = children_iter.next().expect("Parent packet must have at least 1 child");
    let children: Vec<&InheritanceNode> = children_iter
        .map(|child| *child)
        .filter(|child| !child.constraints.is_empty() || child.field_width().is_some())
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
                    $(if !child.constraints.is_empty() && child.field_width().is_some() => &&)
                    $(if let Some(width) = child.field_width() => payload.capacity() == $(width / 8))
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
    heirarchy: &ClassHeirarchy,
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
                if let Some(elem_width) =
                    val.width().or_else(|| val.class().and_then(|class| heirarchy.width(class)))
                {
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
