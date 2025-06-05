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

use std::{collections::HashMap, rc::Rc};

use genco::{
    self,
    prelude::Java,
    quote, quote_fn, quote_in,
    tokens::{quoted, FormatInto},
    Tokens,
};
use heck::{self, ToUpperCamelCase};

use crate::{
    ast::EndiannessValue,
    backends::{
        common::alignment::Alignment,
        java::{Child, Parent, RValue},
    },
};

use super::{gen_mask, import, Chunk, Class, Expr, Integral, PacketDef, Type, Variable};

impl PacketDef {
    pub fn gen_packet<'a>(
        &'a self,
        name: &'a String,
        endianness: EndiannessValue,
    ) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            public final class $name $(if let Some(parent) = &self.parent => extends $(&parent.name)) {
                $(self.member_defs(true, None))

                private $name() { throw new UnsupportedOperationException(); }

                private $name(Builder builder) {
                    $(if self.parent.is_some() => super(builder);)
                    $(self.builder_assigns())
                }

                // Decoder
                $(if let Some(parent) = &self.parent {
                    // If we inherit, don't build the Builder so super can add it's fields
                    protected static $(&parent.name).Builder<?> fromPayload($(&*import::BB) buf) {
                        Builder builder = new Builder();
                        $(self.decoder(Self::builder_set, endianness))
                        return builder;
                    }
                } else {
                    public static $name fromBytes(byte[] bytes) {
                        $(&*import::BB) buf = $(&*import::BB).wrap(bytes).order($endianness);
                        Builder builder = new Builder();
                        $(self.decoder(Self::builder_set, endianness))
                        return builder.build();
                    }
                })

                // Encoder
                $(if self.parent.is_some() {
                    public byte[] toBytes() {
                        $(&*import::BB) buf = $(&*import::BB)
                            .allocate(fieldWidth())
                            .order($endianness);

                        $(self.encoder())
                        return super.toBytes(buf);
                    }
                } else {
                    public byte[] toBytes() {
                        $(&*import::BB) buf = $(&*import::BB)
                            .allocate(fieldWidth())
                            .order($endianness);

                        $(self.encoder())
                        return buf.array();
                    }
                })

                $(if self.parent.is_some() => @Override)
                public int width() {
                    $(self.width_def())
                }

                $(self.field_width_def())

                $(self.getter_defs())

                @Override
                public String toString() {
                    $(let members_str = quote!(
                        $(for member in self.members.iter() {
                            + $(if let Some(width) = member.ty.width() {
                                $(quoted(format!("{}[{}]=", &member.name, width)))
                            } else {
                                $(quoted(format!("{}=", &member.name)))
                            }) + $(member.stringify()) + "\n"
                        }) + "}"
                    ))

                    $(if self.parent.is_some() {
                        return super.toString($(quoted("{\n")) $members_str, fieldWidth());
                    } else {
                        return $(quoted(format!("{}{{\n", name))) $members_str;
                    })
                }

                $(self.hashcode_equals_overrides(name, self.parent.is_some()))

                public static final class $(match &self.parent {
                    Some(Parent { name: parent_name, does_constrain: true }) =>
                        Builder extends $parent_name.$(name)Builder<Builder>,
                    Some(Parent { name: parent_name, does_constrain: false }) =>
                        Builder extends $parent_name.UnconstrainedBuilder<Builder>,
                    _ => Builder,
                }) {
                    $(self.member_defs(false, None))

                    public Builder() { $(if self.parent.is_some() => super();) }

                    protected Builder self() { return this; }

                    public $name build() { return new $name(this); }

                    $(self.setter_defs("Builder", None))
                }
            }
        }
    }

    pub fn gen_abstract_packet<'a>(
        &'a self,
        name: &'a String,
        endianness: EndiannessValue,
    ) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            public abstract sealed class $name
            $(if let Some(parent) = &self.parent => extends $(&parent.name))
            permits $(for child in self.children.iter() join (, ) => $(&child.name)) {
                $(self.member_defs(true, None))

                protected $name() { throw new UnsupportedOperationException(); }

                protected $name(Builder<?> builder) {
                    $(if self.parent.is_some() => super(builder);)
                    $(self.builder_assigns())
                }

                // Decoder
                $(if let Some(parent) = &self.parent {
                    protected static $(&parent.name).Builder<?> fromPayload($(&*import::BB) buf) {
                        $(self.decoder(Self::declare_locally, endianness))
                        $(self.build_child_fitting_constraints(&self.children))
                        return builder;
                    }
                } else {
                    public static $name fromBytes(byte[] bytes) {
                        $(&*import::BB) buf = $(&*import::BB).wrap(bytes).order($endianness);
                        $(self.decoder(Self::declare_locally, endianness))
                        $(self.build_child_fitting_constraints(&self.children))
                        return builder.build();
                    }

                })

                protected byte[] toBytes($(&*import::BB) payload) {
                    payload.rewind();
                    $(&*import::BB) buf = $(&*import::BB)
                        .allocate(fieldWidth() + payload.capacity())
                        .order($endianness);

                    $(self.encoder())
                    return buf.array();
                }

                $(if self.parent.is_some() => @Override)
                protected int width() {
                    $(self.width_def())
                }

                $(self.field_width_def())

                $(self.getter_defs())

                $(if self.parent.is_some() => @Override)
                protected String toString(String payload, int payloadSize) {
                    $(let members_str = quote!(
                        $(for chunk in self.alignment.iter() {
                            $(match chunk {
                                Chunk::PackedBits { fields, .. } => $(for field in fields.iter() =>
                                    $(if !(field.is_partial && field.chunk_offset == 0) =>
                                        + $(quoted(format!("{}[{}]=", &field.symbol.name, field.symbol.ty.width().unwrap())))
                                        + $(field.symbol.stringify()) + "\n"
                                    )),
                                Chunk::Bytes(member) => + $(match member.ty {
                                    Type::Payload { .. } => $(quoted("payload=")) + payload + "\n",
                                    _ => $(quoted(format!("{}=", member.name))) + $(member.stringify()),
                                })
                            })
                        })
                    ))

                    return getClass().getSimpleName() + "[" + width() + "] {\n"
                    $(if self.parent.is_some() {
                        super.toString($members_str, fieldWidth())
                    } else {
                        $members_str
                    }) + "}";
                }

                $(if !self.members.is_empty() => $(self.hashcode_equals_overrides(name, false)))

                protected abstract static class $(match &self.parent {
                    Some(Parent { name: parent_name, does_constrain: true }) =>
                        Builder<B extends $parent_name.$(name)Builder<B>> extends $parent_name.$(name)Builder<B>,
                    Some(Parent { name: parent_name, does_constrain: false }) =>
                        Builder<B extends $parent_name.UnconstrainedBuilder<B>> extends $parent_name.UnconstrainedBuilder<B>,
                    _ => Builder<B extends Builder<B>>,
                }) {
                    $(self.member_defs(false, None))

                    protected abstract B self();

                    protected abstract $name build();
                }

                protected abstract static class UnconstrainedBuilder<B extends Builder<B>> extends Builder<B> {
                    $(self.setter_defs("B", None))
                }

                $(for child in self.children.iter() {
                    $(if !child.constraints.is_empty() {
                        protected abstract static class $(&child.name)Builder<B extends Builder<B>> extends Builder<B> {
                            protected $(&child.name)Builder() {
                                $(for (member, value) in child.constraints.iter() {
                                    // TODO(jmes): handle case when constraining member of ancestor
                                    // This will likely require putting tag_id in constraint
                                    $member = $(value.generate(&self.members.iter().find(|m| &m.name == member).unwrap().ty));
                                })
                            }

                            $(self.setter_defs("B", Some(&child.constraints)))
                        }
                    })
                })
            }
        }
    }

    fn member_defs<'a>(
        &'a self,
        are_final: bool,
        constraints: Option<&'a HashMap<String, String>>,
    ) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            $(for member in self.members.iter() {
                protected $(if are_final => final) $(&member.ty) $(&member.name)
                    $(if let Some(constraint) = constraints.and_then(|cs| cs.get(&member.name)) =>
                        = $constraint);
            })
        }
    }

    fn getter_defs(&self) -> impl FormatInto<Java> + '_ {
        quote_fn! {
            $(for member in self.members.iter() {
                public $(&member.ty) get$(member.name.to_upper_camel_case())() {
                    return $(&member.name);
                }
            })
        }
    }

    fn setter_defs<'a>(
        &'a self,
        builder_type: &'static str,
        constraints: Option<&'a HashMap<String, RValue>>,
    ) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            $(for member in self.members.iter() {
                $(if constraints.is_none_or(|constraints| !constraints.contains_key(&member.name)) {
                    public $builder_type set$(member.name.to_upper_camel_case())($(&member.ty) $(&member.name)) {
                        $(match &member.ty {
                            Type::Integral { width, .. } => {
                                if ($(member.ty.boxed()).compareUnsigned($(&member.name), $(gen_mask(*width))) > 0) {
                                    throw new IllegalArgumentException(
                                        "Value " + $(member.stringify())
                                        + $(quoted(format!(" is too wide for field '{}' with width {}", member.name, width)))
                                    );
                                }
                            }
                            Type::Payload { size_field: Some(size_field) } => {
                                if (Integer.compareUnsigned($(&member.name).length, $(gen_mask(*size_field))) > 0) {
                                    throw new IllegalArgumentException(
                                        "Payload " + $(member.stringify())
                                        + $(quoted(format!(" is too wide for its _size_ field with width {}", size_field)))
                                    );
                                }
                            }
                            _ =>,
                        })

                        this.$(&member.name) = $(&member.name);
                        return self();
                    }
                })
            })
        }
    }

    fn builder_assigns(&self) -> impl FormatInto<Java> + '_ {
        quote_fn! {
            $(for member in self.members.iter() => $(&member.name) = builder.$(&member.name);)
        }
    }

    fn encoder<'a>(&'a self) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            $(for chunk in self.alignment.iter() {
                $(match chunk {
                    Chunk::PackedBits { fields, width } => {
                        $(let chunk_type = Integral::fitting(*width))
                        $(let mut fields = fields.iter())
                        $(let first = fields.next().expect("Attempt to generate encoder for chunk with no fields"))

                        buf.$(chunk_type.encoder())(
                            ($chunk_type) (
                                $(first.symbol.encode_value()
                                    .maybe_widen(&first.symbol.ty, chunk_type)
                                    .maybe_shift(">>>", first.symbol_offset))

                                $(for field in fields => |
                                    $(field.symbol.encode_value()
                                        .maybe_widen(&field.symbol.ty, chunk_type)
                                        .maybe_shift("<<", field.chunk_offset))))
                        );
                    }
                    Chunk::Bytes(member) => $(match member.ty {
                        Type::Payload { .. } => buf.put(payload),
                        _ => buf.put($(&member.name).toBytes()),
                    });,
                })
            })
        }
    }

    fn decoder<'a>(
        &'a self,
        assign: fn(&Variable, Tokens<Java>) -> Tokens<Java>,
        endianness: EndiannessValue,
    ) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            $(for (i, chunk) in self.alignment.iter().enumerate() {
                $(match chunk {
                    Chunk::PackedBits { fields, width } => {
                        $(let chunk_name = format!("chunk{}", i))
                        $(let chunk_type = Integral::fitting(*width).limit_to_int())

                        $chunk_type $(&chunk_name) = $(Integral::fitting(*width).decoder("buf"));

                        $(for field in fields.iter() {
                            $(let ty = match &field.symbol.ty {
                                Type::Integral { ty, ..} => ty,
                                Type::Class { width: Some(width), .. } => &Integral::fitting(*width).limit_to_int(),
                                _ => unreachable!("Packed chunk with dynamic width")
                            })

                            $(let decoded_field = chunk_name.as_str()
                                .maybe_mask(field.chunk_offset, field.width)
                                .maybe_cast(chunk_type, *ty))

                            $(if field.is_partial {
                                // The value is split between chunks.
                                $(if field.symbol_offset == 0 {
                                    // This chunk has the lower-order bits of the value, so store them in a variable until we can
                                    // get the higher-order bits from the next chunk.
                                    $(&field.symbol.ty) $(&field.symbol.name)$i = $decoded_field;
                                } else {
                                    // This chunk has the higher-order bits of the value, so grab the lower-order bits from the
                                    // variable we declared.
                                    $(assign(
                                        &field.symbol,
                                        quote!($(&field.symbol.name)$(i - 1) | (($decoded_field) << $(field.symbol_offset)))));
                                })
                            } else {
                                // The whole value lies within this chunk, so just set it.
                                $(assign(&field.symbol, decoded_field));
                            })
                        })
                    }
                    Chunk::Bytes(member) => $(match member.ty {
                        Type::Payload { .. } => {
                            $(if self.size_fields.contains_key("payload") {
                                $(&*import::BB) payload = buf.slice(buf.position(), payloadSize).order($endianness);
                            } else {
                                $(if let Some(width) = self.width {
                                    $(&*import::BB) payload =
                                        buf.slice(buf.position(), bytes.length - $(width / 8)).order($endianness);
                                } else {
                                    builder.setPayload(buf.array());
                                })
                            })
                        },
                        _ => {
                            byte[] $(&member.name) = new byte[$(&member.ty).width()]
                            $(&member.name).fromBytes($(&member.name)),
                        }
                    })
                })
            })
        }
    }

    fn builder_set(member: &Variable, value: Tokens<Java>) -> Tokens<Java> {
        quote!(
            $(if member.name.ends_with("Size") {
                // Don't need to set set size fields in builder.
                $(Self::declare_locally(member, value))
            } else {
                builder.$(member.setter(member.ty.from_int(value)))
            })
        )
    }

    fn declare_locally(member: &Variable, value: Tokens<Java>) -> Tokens<Java> {
        quote!($(&member.ty) $(&member.name) = $(member.ty.from_int(value)))
    }

    fn width_def<'a>(&'a self) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            return $(if self.parent.is_some() => super.width() +) fieldWidth();
        }
    }

    // TODO(jmes): FIX THIS SO THAT IT INCLUDES WIDTH OF _SIZE_ FIELDS
    fn field_width_def<'a>(&'a self) -> impl FormatInto<Java> + 'a {
        let (statically_sized, dynamically_sized): (Vec<_>, Vec<_>) =
            self.members.iter().partition(|member| member.ty.width().is_some());

        quote_fn! {
            private final int fieldWidth() {
                $(let static_width =
                    (statically_sized.into_iter().map(|member| member.ty.width().unwrap()).sum::<usize>() +
                        self.size_fields.values().sum::<usize>()) / 8)

                return $(if static_width != 0 => $static_width)
                    $(if static_width != 0 && !dynamically_sized.is_empty() => +)
                    $(for member in dynamically_sized.into_iter() join ( + ) => $(member.gen_width()));
            }
        }
    }

    fn hashcode_equals_overrides<'a>(
        &'a self,
        name: &'a str,
        call_super: bool,
    ) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            @Override
            public boolean equals(Object o) {
                if (this == o) return true;
                if (!(o instanceof $name other)) return false;
                return $(if call_super => super.equals(other) &&)
                $(for member in self.members.iter() join ( && ) {
                    $(member.equals(quote!(other.$(&member.name))))
                });
            }

            @Override
            public int hashCode() {
                $(let mut members = self.members.iter())
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

    fn build_child_fitting_constraints<'a>(
        &'a self,
        children: &'a Vec<Child>,
    ) -> impl FormatInto<Java> + 'a {
        let mut children_iter = children.iter();
        let default = children_iter.next().expect("Parent packet must have at least 1 child");
        let children: Vec<&Child> = children_iter
            .filter(|child| !child.constraints.is_empty() || child.width.is_some())
            .collect();

        quote_fn! {
            $(if children.is_empty() {
                Builder<?> builder = $(&default.name).fromPayload(payload);
            } else {
                Builder<?> builder;
                $(for child in children {
                    if (
                        $(if let Some(width) = child.width => payload.capacity() == $(width / 8))
                        $(if child.width.is_some() && !child.constraints.is_empty() => &&)
                        $(for member in self.members.iter() =>
                            $(if let Some(value) = child.constraints.get(&member.name) =>
                                 $(member.equals(value.generate(&member.ty)))))
                    ) {
                        builder = $(&child.name).fromPayload(payload);
                    } else
                }) {
                    builder = $(&default.name).fromPayload(payload);
                }
            })

            $(for member in self.members.iter() => builder.$(&member.name) = $(&member.name);)
        }
    }
}
