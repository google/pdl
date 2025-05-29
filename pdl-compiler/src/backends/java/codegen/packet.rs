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

use genco::{
    self,
    prelude::Java,
    quote, quote_fn, quote_in,
    tokens::{quoted, FormatInto},
    Tokens,
};
use heck::{self, ToUpperCamelCase};

use crate::backends::java::{Child, Parent};

use super::{
    gen_mask, import, Chunk, Class, Expr, GeneratorContext, Integral, PacketDef, Type, Variable,
};

impl Class<'_> {
    pub fn gen_payload_packet<'a>(
        &'a self,
        name: &'a String,
        parent_name: &'a String,
    ) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            public final class $name extends $parent_name {
                private final byte[] payload;

                private $name() { throw new UnsupportedOperationException(); }

                private $name(Builder b) {
                    super(b);
                    this.payload = b.payload;
                }

                protected static Builder withPayload(byte[] bytes) {
                    return new Builder().setPayload(bytes);
                }

                public int width() { return $parent_name.WIDTH + payload.length; }

                byte[] getPayload() { return payload; }

                @Override
                public String toString() { return super.toString($(&*import::ARRAYS).toString(payload)); }

                @Override
                public boolean equals(Object o) {
                    if (this == o) return true;
                    if (!(o instanceof $name other)) return false;
                    return super.equals(other) && payload.equals(other.payload);
                }

                @Override
                public int hashCode() { return $(&*import::ARRAYS).hashCode(payload); }

                public static class Builder extends $parent_name.UnconstrainedBuilder<Builder> {
                    private byte[] payload;

                    public Builder() { }

                    @Override
                    protected Builder self() { return this; }

                    @Override
                    public $name build() { return new $name(this); }

                    Builder setPayload(byte[] payload) {
                        this.payload = payload;
                        return self();
                    }
                }
            }
        }
    }
}

impl PacketDef {
    pub fn gen_packet<'a>(
        &'a self,
        name: &'a String,
        ctx: &'a GeneratorContext,
    ) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            public final class $name $(if let Some(parent) = &self.parent => extends $(&parent.name)) {
                protected static int WIDTH = $(self.static_byte_width());
                $(ctx.gen_byte_order())
                $(self.member_defs(true, None))

                private $name() { throw new UnsupportedOperationException(); }

                private $name(Builder b) {
                    $(if self.parent.is_some() => super(b);)
                    $(self.builder_assigns())
                }

                $(if let Some(parent) = &self.parent {
                    protected static $(&parent.name).Builder<?> fromPayload(byte[] bytes)
                } else {
                    public static $name fromBytes(byte[] bytes)
                }) {
                    $(&*import::BB) buf = $(&*import::BB).wrap(bytes).order(BYTE_ORDER);
                    Builder b = new Builder();
                    $(self.decoder(|member, value| quote!(b.$(member.setter(member.ty.from_int(value))))))

                    $(if self.parent.is_some() { return b; } else { return b.build(); })
                }

                public byte[] toBytes() {
                    $(&*import::BB) buf = $(&*import::BB).allocate(WIDTH).order(BYTE_ORDER);
                    $(self.encoder())
                    return buf.array();
                }

                $(self.getter_defs())

                @Override
                public String toString() {
                    $(if self.parent.is_some() {
                        String payload = $(quoted("{\n"))
                    } else {
                        return $(quoted(format!("{}{{\n", name)))
                    })

                    $(for member in self.members.iter() {
                        + $(if let Some(width) = member.ty.width() {
                            $(quoted(format!(" {}[{}]=", &member.name, width)))
                        } else {
                            $(quoted(format!(" {}=", &member.name)))
                        }) + $(member.stringify()) + "\n"
                    })
                    + "}";

                    $(if self.parent.is_some() => return super.toString(payload);)
                }

                $(self.hashcode_equals_overrides(name, self.parent.is_some()))

                public static final class $(match &self.parent {
                    Some(Parent { name: parent_name, does_constrain: true }) =>
                        Builder extends $parent_name.$(name)Builder<Builder>,
                    Some(Parent { name: parent_name, does_constrain: false }) =>
                        Builder extends $parent_name.$(name)UnconstrainedBuilder<Builder>,
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
        ctx: &'a GeneratorContext,
    ) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            public abstract sealed class $name
            $(if let Some(parent) = &self.parent => extends $(&parent.name))
            permits $(for child in self.children.iter() join (, ) => $(&child.name)) {

                protected static int WIDTH = $(self.static_byte_width());
                $(ctx.gen_byte_order())
                $(self.member_defs(true, None))

                protected $name() { throw new UnsupportedOperationException(); }

                protected $name(Builder<?> b) {
                    $(if self.parent.is_some() => super(b);)
                    $(self.builder_assigns())
                }

                $(if let Some(parent) = &self.parent {
                    protected static $(&parent.name).Builder<?> fromPayload(byte[] bytes)
                } else {
                    public static $name fromBytes(byte[] bytes)
                }) {
                    $(&*import::BB) buf = $(&*import::BB).wrap(bytes).order(BYTE_ORDER);
                    $(self.decoder(|member, value|
                        quote!($(&member.ty) $(&member.name) = $(member.ty.from_int(value)))))

                    $(self.build_child_fitting_constraints(&self.children))

                    $(if self.parent.is_some() { return b; } else { return b.build(); })
                }


                protected byte[] toBytes($(&*import::BB) payload) {
                    $(&*import::BB) buf = $(&*import::BB).allocate(WIDTH).order(BYTE_ORDER);
                    $(self.encoder())
                    return buf.array();
                }

                $(self.getter_defs())

                $(if self.parent.is_some() => @Override)
                protected String toString(String subPayload) {
                    $(if self.parent.is_some() {
                        String payload =
                    } else {
                        return getClass().getSimpleName() +
                    })

                    $(quoted("{\n"))
                    $(for chunk in self.alignment.iter() {
                        $(match chunk {
                            Chunk::BitPacked { fields, .. } => $(for field in fields.iter() =>
                                $(if !(field.is_partial && field.chunk_offset == 0) =>
                                    + $(quoted(format!("{}[{}]=", &field.symbol.name, field.symbol.ty.width().unwrap())))
                                    + $(field.symbol.stringify()) + "\n"
                                )),
                            Chunk::Symbol(member) => + $(quoted(format!("{}=", member.name))) + $(member.stringify()),
                            Chunk::Payload => + $(quoted("payload=")) + subPayload + "\n",
                        })
                    })
                    + "}";

                    $(if self.parent.is_some() => return super.toString(payload);)
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
                                    $member = $value;
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
        constraints: Option<&'a HashMap<String, String>>,
    ) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            $(for member in self.members.iter() {
                $(if constraints.is_none_or(|constraints| !constraints.contains_key(&member.name)) {
                    public $builder_type set$(member.name.to_upper_camel_case())($(&member.ty) $(&member.name)) {
                        $(if let Type::Integral { width, .. } = member.ty =>
                            if ($(member.ty.boxed()).compareUnsigned($(&member.name), $(gen_mask(width))) > 0) {
                                throw new IllegalArgumentException(
                                    "Value " + $(member.stringify())
                                    + $(quoted(format!(" too wide for field '{}' with width {}", member.name, width)))
                                );
                            }
                        )

                        this.$(&member.name) = $(&member.name);
                        return self();
                    }
                })
            })
        }
    }

    fn builder_assigns(&self) -> impl FormatInto<Java> + '_ {
        quote_fn! {
            $(for member in self.members.iter() => $(&member.name) = b.$(&member.name);)
        }
    }

    fn encoder<'a>(&'a self) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            $(for chunk in self.alignment.iter() {
                $(match chunk {
                    Chunk::BitPacked { fields, width } => {
                        $(let chunk_type = Integral::fitting_width(*width))
                        $(let mut fields = fields.iter())
                        $(let first = fields.next().expect("Attempt to generate encoder for chunk with no fields"))

                        buf.$(chunk_type.encoder())(
                            ($chunk_type) (
                                $((&first.symbol.name)
                                .maybe_widen(&first.symbol.ty, chunk_type)
                                .maybe_shift(">>>", first.symbol_offset))

                                $(for field in fields => |
                                    $((&field.symbol.name)
                                        .maybe_widen(&field.symbol.ty, chunk_type)
                                        .maybe_shift("<<", field.chunk_offset))))
                        );
                    }
                    Chunk::Symbol(member) => buf.put($(&member.name).toBytes()),
                    Chunk::Payload => buf.put(payload);,
                })
            })
        }
    }

    fn decoder<'a>(
        &'a self,
        assign: fn(&Variable, Tokens<Java>) -> Tokens<Java>,
    ) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            $(for (i, chunk) in self.alignment.iter().enumerate() {
                $(match chunk {
                    Chunk::BitPacked { fields, width } => {
                        $(let chunk_name = format!("chunk{}", i))
                        $(let chunk_type = Integral::fitting_width(*width).limit_to_int())

                        $chunk_type $(&chunk_name) = $(Integral::fitting_width(*width).decoder("buf"));

                        $(for field in fields.iter() {
                            $(let ty = match field.symbol.ty {
                                Type::Integral { ty, ..} => ty,
                                Type::EnumClass { fits, .. } => fits,
                                _ => unreachable!("Packed chunk with unexpected symbol")
                            })
                            $(let decoded_field = chunk_name.as_str()
                                .maybe_mask(field.chunk_offset, field.width)
                                .maybe_cast(chunk_type, ty))

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
                    Chunk::Symbol(member) => {
                        byte[] $(&member.name) = new byte[$(&member.ty).width()]
                        $(&member.name).fromBytes($(&member.name)),
                    }
                    Chunk::Payload => {
                        byte[] payload = new byte[bytes.length - WIDTH];
                        buf.get(payload);
                    }
                })
            })
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
        quote_fn! {
            $(let mut children_iter = children.iter())
            $(let last = children_iter.next().unwrap())

            Builder<?> b;

            $(for child in children_iter {
                if (payload.length == $(&child.name).WIDTH
                    $(for member in self.members.iter() =>
                        $(if let Some(constraint) = child.constraints.get(&member.name) =>
                            && $(member.equals(constraint))))

                ) {
                    b = $(&child.name).fromPayload(payload);
                } else
            }) {
                b = $(&last.name).withPayload(payload);
            }

            $(for member in self.members.iter() => b.$(&member.name) = $(&member.name);)
        }
    }
}
