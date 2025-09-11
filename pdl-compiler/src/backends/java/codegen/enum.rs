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

use std::iter;

use genco::{lang::Java, quote, tokens::quoted, Tokens};
use heck::ToUpperCamelCase;

use crate::{
    ast::{Tag, TagOther, TagRange, TagValue},
    backends::java::{
        codegen::expr::{cast_symbol, literal, ExprTree},
        Integral,
    },
};

pub fn gen_enum(
    name: &String,
    tags: &[Tag],
    width: usize,
    fallback_tag: Option<TagOther>,
) -> Tokens<Java> {
    let ty = Integral::fitting(width);

    quote! {
        public abstract sealed class $name
        permits $(for tag in tags.iter() join (, ) => $name.$(tag.name())) {

            public static $name from$(ty.capitalized())($ty value) {
                $(for tag in tags.iter() {
                    $(match tag {
                        Tag::Value(tag) => if ($(tag.matches_value(ty))) {
                            return $(tag.name());
                        } else$[' '],
                        Tag::Range(tag) => if ($(tag.matches_value(ty))) {
                            return new $(tag.name())(value);
                        } else$[' '],
                        _ =>,
                    })
                }) $(if let Some(fallback_tag) = fallback_tag {{
                    return new $(fallback_tag.name())(value);
                }} else {{
                    $(failed_to_decode_closed_enum(name, ty))
                }})

            }

            $(for tag in tags.iter() {
                $(match tag {
                    Tag::Value(_) => public static final $(tag.name()) $(tag.name()) = new $(tag.name())();,
                    Tag::Range(tag) => $(tag.static_factory(name, ty)),
                    Tag::Other(tag) => $(tag.static_factory(name, ty)),
                })
            })

            public abstract $ty to$(ty.capitalized())();

            $(for tag in tags.iter() => $(match tag {
                Tag::Value(tag) => $(tag.def(name, ty)),
                Tag::Range(tag) => $(tag.def(&tag.name(), name, ty)),
                Tag::Other(tag) => $(tag.def(&tag.name(), name, ty)),
            }))
        }
    }
}

impl TagValue {
    fn def(&self, super_name: &String, ty: Integral) -> Tokens<Java> {
        quote! {
            public static final class $(self.name()) extends $super_name {
                private $(self.name())() { }

                @Override
                public $ty to$(ty.capitalized())() {
                    return $(literal(ty, self.value));
                }

                @Override
                public String toString() { return $(quoted(format!("{}.{}(", super_name, self.name()))) +
                    $(ty.limit_to_int().stringify(self.value)) + ")"; }
            }
        }
    }

    fn subtag_def(&self, super_name: &String, ty: Integral) -> Tokens<Java> {
        quote! {
            public static final class $(self.name()) extends $super_name {
                private $(self.name())() {
                    super($(cast_symbol(quote!($(self.value)), Integral::Int, ty)));
                }
            }
        }
    }

    fn matches_value(&self, ty: Integral) -> Tokens<Java> {
        let t = ExprTree::new();
        quote! {
            $(t.compare(t.symbol(quote!(value), ty), t.num(self.value))) == 0
        }
    }
}

trait MultiValueTag {
    fn has_subtags(&self) -> bool;
    fn subtags(&self) -> impl Iterator<Item = &TagValue>;
    fn static_factory(&self, super_name: &str, ty: Integral) -> Tokens<Java>;

    fn def(&self, name: &String, super_name: &String, ty: Integral) -> Tokens<Java> {
        quote! {
            $(if self.has_subtags() {
                public static sealed class $name extends $super_name permits
                $(for tag in self.subtags() join (, ) => $name.$(tag.name()))
            } else {
                public static final class $name extends $super_name
            }) {
                private final $ty value;

                $(for tag in self.subtags() {
                    public static final $(tag.name()) $(tag.name()) = new $(tag.name())();
                })

                private $name($ty value) { this.value = value; }

                @Override
                public $ty to$(ty.capitalized())() { return value; }


                @Override
                public String toString() {
                    return $(quoted(format!("{super_name}.{name}("))) +
                        $(ty.stringify(quote!(value))) + ")";
                }

                @Override
                public boolean equals(Object o) {
                    $(let t = ExprTree::new())
                    if (this == o) return true;
                    if (!(o instanceof $name other)) return false;
                    return $(t.compare(
                        t.symbol(quote!(value), ty),
                        t.symbol(quote!(other.value), ty))
                    ) == 0;
                }

                @Override
                public int hashCode() { return $(ty.boxed()).hashCode(value); }

                $(for tag in self.subtags() => $(tag.subtag_def(name, ty)))
            }
        }
    }
}

impl MultiValueTag for TagRange {
    fn has_subtags(&self) -> bool {
        !self.tags.is_empty()
    }

    fn subtags(&self) -> impl Iterator<Item = &TagValue> {
        self.tags.iter()
    }

    fn static_factory(&self, super_name: &str, ty: Integral) -> Tokens<Java> {
        quote! {
            public static $(self.name()) $(self.name())($ty value) {
                $(for tag in self.tags.iter() {
                    if ($(tag.matches_value(ty))) {
                        return $(self.name()).$(tag.id.to_upper_camel_case());
                    } else$[' ']
                }) if ($(self.matches_value(ty))) {
                    return new $(self.name())(value);
                } else {
                    $(failed_to_decode_closed_enum(&format!("{}.{}", super_name, self.name()), ty))
                }
            }
        }
    }
}

impl MultiValueTag for TagOther {
    fn has_subtags(&self) -> bool {
        false
    }

    fn subtags(&self) -> impl Iterator<Item = &TagValue> {
        iter::empty::<&TagValue>()
    }

    fn static_factory(&self, super_name: &str, ty: Integral) -> Tokens<Java> {
        quote! {
            public static $(self.name()) $(self.name())($ty value) {
                $super_name tag = $super_name.fromByte(value);
                if (!(tag instanceof $(self.name()) self)) {
                    throw new IllegalArgumentException(
                        "Value " + $(ty.stringify(quote!(value))) +
                        " is invalid for the fallback tag because it matches named tag " +
                        tag
                    );
                }
                return self;
            }
        }
    }
}

impl TagRange {
    fn matches_value(&self, ty: Integral) -> Tokens<Java> {
        let t = ExprTree::new();

        quote! {
            $(t.compare(t.symbol(quote!(value), ty), t.num(*self.range.start()))) >= 0
            && $(t.compare(t.symbol(quote!(value),ty), t.num(*self.range.end()))) <= 0
        }
    }
}

fn failed_to_decode_closed_enum(name: &String, ty: Integral) -> Tokens<Java> {
    quote! {
        throw new IllegalArgumentException(
            "Value " + $(ty.stringify(quote!(value))) +
            $(quoted(format!(" is invalid for closed enum {name}"))));
    }
}
