use genco::{lang::Java, quote, tokens::quoted, Tokens};
use heck::ToUpperCamelCase;

use crate::{
    ast::{Tag, TagRange, TagValue},
    backends::java::{Class, Integral},
};

pub fn gen_enum(name: &String, tags: &Vec<Tag>, width: usize) -> Tokens<Java> {
    let ty = Integral::fitting(width).limit_to_int();

    quote! {
        public abstract sealed class $name
        permits $(for tag in tags.iter() join (, ) => $name.$(tag.name())) {

            public static $name from$(ty.capitalized())($ty value) {
                $(for tag in tags.iter() {
                    $(let tag_name = tag.id().to_upper_camel_case())
                    $(match tag {
                        Tag::Value(tag) => if ($(tag.matches_value(ty))) {
                            return $tag_name;
                        } else$[' '],
                        Tag::Range(tag) => if ($(tag.matches_value(ty))) {
                            return new $tag_name(value);
                        } else$[' '],
                        _ =>,
                    })
                }) {
                    $(failed_to_decode_closed_enum(&name, ty))
                }
            }

            $(for tag in tags.iter() {
                $(let tag_name = &tag.name())
                $(match tag {
                    Tag::Value(_) => public static final $tag_name $tag_name = new $tag_name();,
                    Tag::Range(tag) => $(tag.static_factory(name, ty)),
                    _ =>,
                })
            })

            public abstract $ty to$(ty.capitalized())();

            $(for tag in tags.iter() => $(match tag {
                Tag::Value(tag) => $(tag.def(name, ty)),
                Tag::Range(tag) => $(tag.def(name, ty)),
                _ =>,
            }))
        }
    }
}

impl TagValue {
    fn def(&self, super_name: &String, ty: Integral) -> Tokens<Java> {
        let name = &self.id.to_upper_camel_case();

        quote! {
            public static final class $name extends $super_name {
                private $name() { }

                @Override
                public $ty to$(ty.capitalized())() { return $(self.value); }

                @Override
                public String toString() { return $(quoted(format!("{}.{}(", super_name, name))) +
                    $(ty.boxed()).toHexString($(self.value)) + ")"; }
            }
        }
    }

    fn subtag_def(&self, super_name: &String) -> Tokens<Java> {
        let name = &self.id.to_upper_camel_case();

        quote! {
            public static final class $name extends $super_name {
                private $name() { super($(self.value)); }
            }
        }
    }

    fn matches_value(&self, ty: Integral) -> Tokens<Java> {
        quote! {
            $(ty.boxed()).compareUnsigned(value, $(self.value)) == 0
        }
    }
}

impl TagRange {
    fn static_factory(&self, super_name: &String, ty: Integral) -> Tokens<Java> {
        let name = &self.id.to_upper_camel_case();

        quote! {
            public static $name $name($ty value) {
                $(for tag in self.tags.iter() {
                    if ($(tag.matches_value(ty))) {
                        return $name.$(tag.id.to_upper_camel_case());
                    } else$[' ']
                }) if ($(self.matches_value(ty))) {
                    return new $name(value);
                } else {
                    $(failed_to_decode_closed_enum(&format!("{}.{}", super_name, name), ty))
                }
            }
        }
    }

    fn def(&self, super_name: &String, ty: Integral) -> Tokens<Java> {
        let name = &self.id.to_upper_camel_case();

        quote! {
            $(if self.tags.is_empty() {
                public static final class $name extends $super_name
            } else {
                public static sealed class $name extends $super_name permits
                $(for tag in self.tags.iter() => $name.$(tag.id.to_upper_camel_case()))
            }) {
                private final $ty value;

                $(for tag in self.tags.iter() {
                    $(let tag_name = &tag.id.to_upper_camel_case())
                    public static final $tag_name $tag_name = new $tag_name();
                })

                private $name($ty value) { this.value = value; }

                @Override
                public $ty to$(ty.capitalized())() { return value; }


                @Override
                public String toString() {
                    return $(quoted(format!("{}.{}(", super_name, name))) +
                        $(ty.boxed()).toHexString(value) + ")";
                }

                @Override
                public boolean equals(Object o) {
                    if (this == o) return true;
                    if (!(o instanceof $name other)) return false;
                    return $(ty.boxed()).compareUnsigned(value, other.value) == 0;
                }

                @Override
                public int hashCode() { return $(ty.boxed()).hashCode(value); }

                $(for tag in self.tags.iter() => $(tag.subtag_def(&name)))
            }
        }
    }

    fn matches_value(&self, ty: Integral) -> Tokens<Java> {
        quote! {
            $(ty.boxed()).compareUnsigned(value, $(*self.range.start())) >= 0
            && $(ty.boxed()).compareUnsigned(value, $(*self.range.end())) <= 0
        }
    }
}

fn failed_to_decode_closed_enum(name: &String, ty: Integral) -> Tokens<Java> {
    quote! {
        throw new IllegalArgumentException(
            $(quoted("Value ")) + $(ty.boxed()).toHexString(value) +
            $(quoted(format!(" is invalid for closed enum {}", name))));
    }
}
