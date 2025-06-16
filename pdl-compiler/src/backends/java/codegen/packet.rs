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

use std::{cmp::max, collections::HashMap};

use genco::{self, prelude::Java, quote, tokens::quoted, Tokens};
use heck::{self, ToLowerCamelCase, ToUpperCamelCase};

use crate::{
    ast::EndiannessValue,
    backends::{
        common::alignment::{Alignment, UnalignedSymbol},
        java::{Child, CompoundVal, ConstrainedTo, Member, Parent, ScalarVal},
    },
};

use super::{gen_mask, import, Chunk, Integral, IntegralOps, PacketDef};

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

            // Decoder
            $(if let Some(parent) = parent {
                // If we inherit, don't build the Builder so super can add it's fields
                protected static $(&parent.name).Builder<?> fromPayload($(&*import::BB) buf) {
                    Builder builder = new Builder();
                    $(decoder(def, assignment::build, endianness))
                    return builder;
                }
            } else {
                public static $name fromBytes(byte[] bytes) {
                    return $name.fromBytes($(&*import::BB).wrap(bytes).order($endianness));
                }

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

                    $(encoder(&def.alignment))
                    return super.toBytes(buf);
                }
            } else {
                public byte[] toBytes() {
                    $(&*import::BB) buf = $(&*import::BB)
                        .allocate(fieldWidth())
                        .order($endianness);

                    $(encoder(&def.alignment))
                    return buf.array();
                }
            })

            $(if parent.is_some() => @Override)
            public int width() {
                $(width_def(parent.is_some()))
            }

            $(field_width_def(&def.members, &def.size_fields))

            $(getter_defs(&def.members))

            @Override
            public String toString() {
                $(let members_str = quote!(
                    $(for member in def.members.iter() {
                        + $(match member {
                            Member::Scalar(member) => $(quoted(format!("{}[{}]=", member.name(), member.width()))),
                            Member::Compound(member) => $(quoted(format!("{}=", member.name())))
                        }) + $(member.stringify()) + "\n"
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

                $(setter_defs(&def.members, &quote!(Builder), &HashMap::new()))
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

                $(encoder(&def.alignment))
                return buf.array();
            }

            $(if parent.is_some() => @Override)
            protected int width() {
                $(width_def(parent.is_some()))
            }

            $(field_width_def(&def.members, &def.size_fields))

            $(getter_defs(&def.members))

            $(if parent.is_some() => @Override)
            protected String toString(String payload, int payloadSize) {
                $(let members_str = quote!(
                    $(for chunk in def.alignment.iter() join ( + ) {
                        $(match chunk {
                            Chunk::PackedBits { fields, .. } =>
                                $(for field in fields.iter().filter(|f| !(f.is_partial && f.chunk_offset == 0))
                                join ( + ) {
                                    $(quoted(format!("{}[{}]=", field.symbol.name(), field.symbol.width())))
                                    + $(field.symbol.stringify()) + "\n"
                                }),
                            Chunk::Bytes(member) =>
                                $(if let CompoundVal::Payload { .. } = member {
                                    "payload=" + payload + "\n"
                                } else {
                                    $(quoted(format!("{}=", member.name()))) + $(member.stringify())
                                }),
                        })
                    })
                ))

                return getClass().getSimpleName() + "[" + width() + "] {\n" +
                $(if parent.is_some() {
                    super.toString($members_str, fieldWidth())
                } else {
                    $members_str
                }) + "}";
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
                $(setter_defs(&def.members, &quote!(B), &HashMap::new()))
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

                        $(setter_defs(&def.members, &quote!(B), &child.constraints))
                    }
                })
            })
        }
    }
}

fn member_defs(
    members: &Vec<Member>,
    are_final: bool,
    constraints: &HashMap<String, String>,
) -> Tokens<Java> {
    quote! {
        $(for member in members.iter() {
            protected $(if are_final => final) $(member.ty()) $(member.name())
                $(if let Some(constraint) = constraints.get(member.name()) => = $constraint);
        })
    }
}

fn getter_defs(members: &Vec<Member>) -> Tokens<Java> {
    quote! {
        $(for member in members.iter() {
            public $(member.ty()) get$(member.name().to_upper_camel_case())() {
                return $(member.name());
            }
        })
    }
}

fn setter_defs(
    members: &Vec<Member>,
    builder_type: &Tokens<Java>,
    constraints: &HashMap<String, ConstrainedTo>,
) -> Tokens<Java> {
    quote! {
        $(for member in members.iter() {
            $(if !constraints.contains_key(member.name()) {
                public $builder_type set$(member.name().to_upper_camel_case())($(member.ty()) $(member.name())) {
                    $(match member {
                        Member::Scalar(ScalarVal::Integral { width, ty, .. }) => {
                            if ($(ty.compare(member.name(), gen_mask(*width))) > 0) {
                                throw new IllegalArgumentException(
                                    "Value " +
                                    $(member.stringify()) +
                                    $(quoted(format!(
                                        " is too wide for field '{}' with width {}", member.name(), width)))
                                );
                            }
                        }
                        Member::Compound(CompoundVal::Payload { size_field: Some(size_field) }) => {
                            if ($(Integral::Int.compare(quote!($(member.name()).length), gen_mask(*size_field))) > 0) {
                                throw new IllegalArgumentException(
                                    "Payload " +
                                    $(member.stringify()) +
                                    $(quoted(format!(
                                        " is too wide for its _size_ field with width {}", size_field)))
                                );
                            }
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

fn builder_assigns(members: &Vec<Member>) -> Tokens<Java> {
    quote! {
        $(for member in members.iter() => $(member.name()) = builder.$(member.name());)
    }
}

fn encoder(alignment: &Alignment<ScalarVal, CompoundVal>) -> Tokens<Java> {
    quote! {
        $(for chunk in alignment.iter() {
            $(match chunk {
                Chunk::PackedBits { fields, width } => {
                    $(let chunk_type = Integral::fitting(*width))
                    $(let promo_safe_type = chunk_type.limit_to_int())

                    $(let mut fields_iter = fields.iter())
                    $(let first = fields_iter.next().unwrap())

                    $(if fields.len() == 1 {
                        buf.$(chunk_type.encoder())($(first.symbol.expr_to_encode()));
                    } else {
                        buf.$(chunk_type.encoder())(
                            $(quote!(
                                $(first.symbol.expr_to_encode()
                                    .maybe_widen(&first.symbol, promo_safe_type)
                                    .maybe_shift(">>>", first.symbol_offset)
                                    .maybe_narrow(&first.symbol, promo_safe_type)
                                )
                                $(for field in fields_iter {
                                    | $(field.symbol.expr_to_encode()
                                        .maybe_widen(&field.symbol, promo_safe_type)
                                        .maybe_shift("<<", field.chunk_offset)
                                        .maybe_narrow(&field.symbol, promo_safe_type)
                                    )
                                })
                            ).maybe_narrow(promo_safe_type, chunk_type))
                        );
                    })

                }
                Chunk::Bytes(member) => $(match member {
                    CompoundVal::Payload { .. } => buf.put(payload),
                    _ => buf.put($(member.name()).toBytes()),
                });,
            })
        })
    }
}

fn decoder(
    def: &PacketDef,
    assign: fn(Tokens<Java>, &str, Tokens<Java>) -> Tokens<Java>,
    endianness: EndiannessValue,
) -> Tokens<Java> {
    quote! {
        $(for (i, chunk) in def.alignment.iter().enumerate() {
            $(match chunk {
                Chunk::PackedBits { fields, width } => {
                    $(let chunk_name = format!("chunk{}", i))
                    $(let chunk_type = Integral::fitting(*width))
                    $(let promo_safe_type = chunk_type.limit_to_int())

                    $promo_safe_type $(&chunk_name) = $(
                        chunk_type.decoder("buf").maybe_widen(chunk_type, promo_safe_type)
                    );

                    $(for field in fields.iter() {
                        $(let decoded_field = chunk_name.as_str().maybe_mask(field.chunk_offset, field.width))
                        $(let field_type = Integral::fitting(field.symbol.width()))

                        $(if field.is_partial {
                            // The value is split between chunks.
                            $(if field.symbol_offset == 0 {
                                // This chunk has the lower-order bits of the value, so store them in a variable until we can
                                // get the higher-order bits from the next chunk.
                                $promo_safe_type $(field.symbol.name())$i = $decoded_field;
                            } else {
                                // This chunk has the higher-order bits of the value, so grab the lower-order bits from the
                                // variable we declared.
                                $(assign(
                                    field.symbol.ty(),
                                    field.symbol.name(),
                                    field.symbol.from_integral(
                                        quote!(
                                            $(field.symbol.name())$(i - 1) | ($decoded_field << $(field.symbol_offset))
                                        ).maybe_narrow(promo_safe_type, field_type)
                                    )
                                ));
                            })
                        } else {
                            // The whole value lies within this chunk, so just set it.
                            $(assign(
                                field.symbol.ty (),
                                field.symbol.name(),
                                field.symbol.from_integral(decoded_field.maybe_narrow(promo_safe_type, field_type))
                            ));
                        })
                    })
                }
                Chunk::Bytes(member) => $(match member {
                    CompoundVal::Payload { .. } => {
                        $(if def.size_fields.contains_key("payload") {
                            // The above condition checks if *this* class contains a size field for the payload.
                            // Unpacking `size_field` from the `UnsizedMember::Payload` would instead check if
                            // *any* class contains a size field for the payload.
                            $(assign(
                                member.ty(),
                                member.name(),
                                quote!(buf.slice(buf.position(), payloadSize).order($endianness))
                            ));
                        } else {
                            $(if let Some(width) = def.width {
                                $(assign(
                                    member.ty(),
                                    member.name(),
                                    quote!(buf.slice(buf.position(), buf.capacity() - $(width / 8)).order($endianness))
                                ));
                            } else {
                                // If we don't know the payload's width, assume it is the last field in the packet
                                // (this should really be enforced by the parser) and consume all remaining bytes
                                // in the buffer.
                                $(assign(
                                    member.ty(),
                                    member.name(),
                                    quote!(buf.slice())
                                ));
                            })
                        })
                    },
                    CompoundVal::StructRef { name, ty, .. } => {
                        $(let var_name = &name.to_lower_camel_case())
                        // If the struct has dynamic width, assume it is the last field in the packet (this should
                        // really be enforced by the parser) and decode it. Its decoder will consume all remaining
                        // bytes in the buffer.
                        $ty $var_name = $ty.fromBytes(buf.slice());
                        $(assign(member.ty(), member.name(), quote!($var_name)));
                        buf.position(buf.position() + $var_name.width());
                    }
                })
            })
        })
    }
}

mod assignment {
    use super::*;

    pub fn build(ty: Tokens<Java>, name: &str, value: Tokens<Java>) -> Tokens<Java> {
        if name.ends_with("Size") {
            // Don't need to set set size fields in builder.
            declare_locally(ty, name, value)
        } else if name == "payload" {
            quote!(builder.set$(name.to_upper_camel_case())($value.array() ))
        } else {
            quote!(builder.set$(name.to_upper_camel_case())($value))
        }
    }

    pub fn declare_locally(ty: Tokens<Java>, name: &str, value: Tokens<Java>) -> Tokens<Java> {
        if name == "payload" {
            quote!($(&*import::BB) payload = $value)
        } else {
            quote!($ty $name = $value)
        }
    }
}

fn width_def(call_super: bool) -> Tokens<Java> {
    quote! {
        return $(if call_super => super.width() +) fieldWidth();
    }
}

fn field_width_def(members: &Vec<Member>, size_fields: &HashMap<String, usize>) -> Tokens<Java> {
    let static_width =
        (members.iter().filter_map(Member::as_scalar).map(|member| member.width()).sum::<usize>()
            + size_fields.values().sum::<usize>())
            / 8;

    let unsized_members: Vec<&CompoundVal> =
        members.iter().filter_map(Member::as_compound).collect();

    quote! {
        private final int fieldWidth() {
            return $static_width
                $(for member in unsized_members.into_iter() => + $(member.width_expr()));
        }
    }
}

fn hashcode_equals_overrides(name: &str, members: &Vec<Member>, call_super: bool) -> Tokens<Java> {
    quote! {
        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (!(o instanceof $name other)) return false;
            return $(if call_super => super.equals(other) &&)
            $(for member in members.iter() join ( && ) {
                $(member.equals(quote!(other.$(member.name()))))
            });
        }

        @Override
        public int hashCode() {
            $(let mut members = members.iter())
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

fn build_child_fitting_constraints(members: &Vec<Member>, children: &Vec<Child>) -> Tokens<Java> {
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

        $(for member in members.iter() => builder.$(member.name()) = $(member.name());)
    }
}
