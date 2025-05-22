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

use crate::backends::java::ChildPacket;

use super::{
    gen_mask, import, Chunk, Class, Expr, GeneratorContext, Integral, PacketDef, Type, Variable,
};

impl Class<'_> {
    pub fn gen_payload_packet<'a>(
        &'a self,
        name: &'a String,
        parent: &'a String,
    ) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            public final class $name extends $parent {
                private final byte[] payload;

                private $name() { throw new UnsupportedOperationException(); }

                private $name(Builder b) {
                    super(b);
                    this.payload = b.payload;
                }

                protected static Builder withPayload(byte[] bytes) {
                    return new Builder().setPayload(bytes);
                }

                public int width() { return $parent.WIDTH + payload.length; }

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

                public static class Builder extends $parent.UnconstrainedBuilder<Builder> {
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
            public final class $name {
                protected static int WIDTH = $(self.static_byte_width());
                $(ctx.gen_byte_order())
                $(self.member_defs(true, None))

                private $name() { throw new UnsupportedOperationException(); }

                private $name(Builder b) { $(self.builder_assigns()) }

                public static $name fromBytes(byte[] bytes) {
                    $(&*import::BB) buf = $(&*import::BB).wrap(bytes).order(BYTE_ORDER);
                    Builder b = new Builder();
                    $(self.decoder(|member, value| quote!(b.$(member.setter(value)))))
                    return b.build();
                }

                public byte[] toBytes() {
                    $(&*import::BB) buf = $(&*import::BB).allocate(WIDTH).order(BYTE_ORDER);
                    $(self.encoder())
                    return buf.array();
                }

                $(self.getter_defs())

                @Override
                public String toString() {
                    return $(quoted(format!("{}{{\n", name)))
                    $(for member in self.members.iter() {
                            + $(quoted(format!(" {}[{}]=", &member.name, member.width)))
                            + $(member.stringify()) + "\n"
                    })
                    + "}";
                }

                $(self.hashcode_equals_overrides(name, false))

                public static class Builder {
                    $(self.member_defs(false, None))

                    public Builder() { }

                    private Builder self() { return this; }

                    public $name build() { return new $name(this); }

                    $(self.setter_defs("Builder", None))
                }
            }
        }
    }

    pub fn gen_parent_packet<'a>(
        &'a self,
        name: &'a String,
        children: &'a Vec<ChildPacket>,
        ctx: &'a GeneratorContext,
    ) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            public abstract sealed class $name permits $(for n in children join (, ) => $(&n.name)) {
                $(ctx.gen_byte_order())
                protected static int WIDTH = $(self.static_byte_width());
                $(self.member_defs(true, None))

                protected $name() { throw new UnsupportedOperationException(); }

                protected $name(Builder<?> b) { $(self.builder_assigns()) }

                public static $name fromBytes(byte[] bytes) {
                    $(&*import::BB) buf = $(&*import::BB).wrap(bytes).order(BYTE_ORDER);
                    $(self.decoder(|member, value| quote!($(&member.ty) $(&member.name) = $value)))

                    $(self.build_child_fitting_constraints(&children))
                }


                protected byte[] toBytes($(&*import::BB) payload) {
                    $(&*import::BB) buf = $(&*import::BB).allocate(WIDTH).order(BYTE_ORDER);
                    $(self.encoder())
                    return buf.array();
                }

                $(self.getter_defs())

                protected String toString(String payload) {
                    return getClass().getSimpleName() + $(quoted("{\n"))
                        $(ref tokens =>
                            let payload_offset = self.alignment.payload_offset().unwrap();
                            let mut offset = 0;

                            for member in self.members.iter() {
                                if offset == payload_offset {
                                    quote_in!(*tokens => + $(quoted("payload=")) + payload + "\n")
                                }
                                quote_in! {
                                    *tokens =>
                                        + $(quoted(format!("{}[{}]=", &member.name, member.width)))
                                        + $(member.stringify()) + "\n"
                                }
                                offset += member.width;
                            }
                        )
                    + "}";
                }

                $(self.hashcode_equals_overrides(name, false))

                protected abstract static class Builder<B extends Builder<B>> {
                    $(self.member_defs(false, None))

                    protected abstract B self();

                    protected abstract $name build();
                }

                protected abstract static class UnconstrainedBuilder<B extends Builder<B>> extends Builder<B> {
                    $(self.setter_defs("B", None))
                }

                $(for child in children.iter() {
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

    pub fn gen_child_packet<'a>(
        &'a self,
        name: &'a String,
        parent: &'a String,
        is_constrained: bool,
        ctx: &'a GeneratorContext,
    ) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            public final class $name extends $parent {
                public static int WIDTH = $parent.WIDTH + $(self.static_byte_width());
                $(ctx.gen_byte_order())
                $(self.member_defs(true, None))

                private $name() { throw new UnsupportedOperationException(); }

                private $name(Builder b) {
                    super(b);
                    $(self.builder_assigns())
                }

                protected static $parent.Builder<?> fromPayload(byte[] bytes) {
                    $(&*import::BB) buf = $(&*import::BB).wrap(bytes).order(BYTE_ORDER);
                    Builder b = new Builder();
                    $(self.decoder(|member, value| quote!(b.$(member.setter(value)))))
                    return b;
                }

                public byte[] toBytes() {
                    $(&*import::BB) buf = $(&*import::BB).allocate(WIDTH).order(BYTE_ORDER);
                    $(self.encoder())
                    return super.toBytes(buf);
                }

                $(self.getter_defs())

                @Override
                public String toString() {
                    String payload = $(quoted("{\n"))
                    $(for member in self.members.iter() {
                            + $(quoted(format!(" {}[{}]=", &member.name, member.width)))
                            + $(member.stringify()) + "\n"
                    })
                    + "}";
                    return super.toString(payload);
                }

                $(self.hashcode_equals_overrides(name, true))

                public static class Builder extends $(if is_constrained {
                    $parent.$(name)Builder<Builder>
                } else {
                    $parent.UnconstrainedBuilder<Builder>
                }) {
                    $(self.member_defs(false, None))

                    public Builder() { super(); }

                    @Override
                    protected Builder self() { return this; }

                    @Override
                    public $name build() { return new $name(this); }

                    $(self.setter_defs("Builder", None))
                }
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
                $(if constraints.is_none_or(|cs| !cs.contains_key(&member.name)) {
                    public $builder_type set$(member.name.to_upper_camel_case())($(&member.ty) $(&member.name)) {
                        if ($(member.ty.boxed()).compareUnsigned($(&member.name), $(gen_mask(member.ty.width()))) > 0) {
                            throw new IllegalArgumentException(
                                "Value " + $(member.stringify())
                                + $(quoted(format!(" too wide for field '{}' with width {}", member.name, member.ty.width())))
                            );
                        }
                        this.$(&member.name) = $(&member.name);
                        return self();
                    }
                })
            })
        }
    }

    fn builder_assigns(&self) -> impl FormatInto<Java> + '_ {
        quote_fn! {
            $(for member in self.members.iter() join (; ) => $(&member.name) = b.$(&member.name));
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
                                $(first.symbol.name.as_str()
                                    .maybe_widen(&first.symbol.ty, chunk_type)
                                    .maybe_shift(">>>", first.symbol_offset))

                                $(for field in fields => |
                                    $(field.symbol.name.as_str()
                                        .maybe_widen(&field.symbol.ty, chunk_type)
                                        .maybe_shift("<<", field.chunk_offset))))
                        );
                    }
                    Chunk::Payload => buf.put(payload);,
                })
            })
        }
    }

    fn decoder<'a>(
        &'a self,
        set_member: fn(&Variable, Tokens<Java>) -> Tokens<Java>,
    ) -> impl FormatInto<Java> + 'a {
        quote_fn! {
            $(for (i, chunk) in self.alignment.iter().enumerate() {
                $(match chunk {
                    Chunk::BitPacked { fields, width } => {
                        $(let chunk_name = format!("chunk{}", i))
                        $(let chunk_type = Integral::fitting_width(*width).limit_to_int())

                        $chunk_type $(&chunk_name) = $(Integral::fitting_width(*width).decoder("buf"));

                        $(for field in fields.iter() {
                            $(match &field.symbol.ty {
                               Type::Integral(ty) => {
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
                                           $(set_member(
                                               &field.symbol,
                                               quote!($(&field.symbol.name)$(i - 1) | (($decoded_field) << $(field.symbol_offset)))));
                                       })
                                   } else {
                                       // The whole value lies within this chunk, so just set it.
                                       $(set_member(&field.symbol, decoded_field));
                                   })

                               }
                               Type::Class(class) =>,
                            })
                        })
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
        children: &'a Vec<ChildPacket>,
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
            return b.build();
        }
    }
}
