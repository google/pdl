use genco::{lang::Java, quote_fn, tokens::quoted, tokens::FormatInto};
use heck::ToUpperCamelCase;

use crate::{
    ast::{Tag, TagRange, TagValue},
    backends::java::{Class, Integral},
};

impl Class {
    pub fn gen_enum<'a>(&'a self, tags: &'a Vec<Tag>, width: usize) -> impl FormatInto<Java> + 'a {
        let ty = Integral::fitting(width).limit_to_int();

        quote_fn! {
            public abstract sealed class $(&self.name)
            permits $(for tag in tags.iter() join (, ) => $(&self.name).$(tag.name())) {

                public static $(&self.name) from$(ty.capitalized())($ty value) {
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
                        $(failed_to_decode_closed_enum(&self.name, ty))
                    }
                }

                $(for tag in tags.iter() {
                    $(let name = tag.name())
                    $(match tag {
                        Tag::Value(_) => public static final $(&name) $(&name) = new $(&name)();,
                        Tag::Range(tag) => $(tag.static_factory(&self.name, ty)),
                        _ =>,
                    })
                })

                public abstract $ty to$(ty.capitalized())();

                $(for tag in tags.iter() => $(match tag {
                    Tag::Value(tag) => $(tag.def(&self.name, ty)),
                    Tag::Range(tag) => $(tag.def(&self.name, ty)),
                    _ =>,
                }))
            }
        }
    }
}

impl TagValue {
    fn def<'a>(&'a self, super_name: &'a String, ty: Integral) -> impl FormatInto<Java> + 'a {
        let name = self.id.to_upper_camel_case();

        quote_fn! {
            public static final class $(&name) extends $super_name {
                private $(&name)() { }

                @Override
                public $ty to$(ty.capitalized())() { return $(self.value); }

                @Override
                public String toString() { return $(quoted(format!("{}.{}(", super_name, &name))) +
                    $(ty.boxed()).toHexString($(self.value)) + ")"; }
            }
        }
    }

    fn subtag_def<'a>(&'a self, super_name: &'a String) -> impl FormatInto<Java> + 'a {
        let name = self.id.to_upper_camel_case();

        quote_fn! {
            public static final class $(&name) extends $super_name {
                private $(&name)() { super($(self.value)); }
            }
        }
    }

    fn matches_value(&self, ty: Integral) -> impl FormatInto<Java> + '_ {
        quote_fn! {
            $(ty.boxed()).compareUnsigned(value, $(self.value)) == 0
        }
    }
}

impl<'a> TagRange {
    fn static_factory(
        &'a self,
        super_name: &'a String,
        ty: Integral,
    ) -> impl FormatInto<Java> + 'a {
        let name = self.id.to_upper_camel_case();

        quote_fn! {
            public static $(&name) $(&name)($ty value) {
                $(for tag in self.tags.iter() {
                    if ($(tag.matches_value(ty))) {
                        return $(&name).$(tag.id.to_upper_camel_case());
                    } else$[' ']
                }) if ($(self.matches_value(ty))) {
                    return new $(&name)(value);
                } else {
                    $(failed_to_decode_closed_enum(&format!("{}.{}", super_name, &name), ty))
                }
            }
        }
    }

    fn def(&'a self, super_name: &'a String, ty: Integral) -> impl FormatInto<Java> + 'a {
        let name = self.id.to_upper_camel_case();

        quote_fn! {
            $(if self.tags.is_empty() {
                public static final class $(&name) extends $super_name
            } else {
                public static sealed class $(&name) extends $super_name permits
                $(for tag in self.tags.iter() => $(&name).$(tag.id.to_upper_camel_case()))
            }) {
                private final $ty value;

                $(for tag in self.tags.iter() {
                    $(let name = tag.id.to_upper_camel_case())
                    public static final $(&name) $(&name) = new $(&name)();
                })

                private $(&name)($ty value) { this.value = value; }

                @Override
                public $ty to$(ty.capitalized())() { return value; }


                @Override
                public String toString() {
                    return $(quoted(format!("{}.{}(", super_name, &name))) +
                        $(ty.boxed()).toHexString(value) + ")";
                }

                @Override
                public boolean equals(Object o) {
                    if (this == o) return true;
                    if (!(o instanceof $(&name) other)) return false;
                    return $(ty.boxed()).compareUnsigned(value, other.value) == 0;
                }

                @Override
                public int hashCode() { return $(ty.boxed()).hashCode(value); }

                $(for tag in self.tags.iter() => $(tag.subtag_def(&name)))
            }
        }
    }

    fn matches_value(&self, ty: Integral) -> impl FormatInto<Java> + '_ {
        quote_fn! {
            $(ty.boxed()).compareUnsigned(value, $(*self.range.start())) >= 0
            && $(ty.boxed()).compareUnsigned(value, $(*self.range.end())) <= 0
        }
    }
}

fn failed_to_decode_closed_enum(name: &String, ty: Integral) -> impl FormatInto<Java> + '_ {
    quote_fn! {
        throw new IllegalArgumentException(
            $(quoted("Value ")) + $(ty.boxed()).toHexString(value) +
            $(quoted(format!(" is invalid for closed enum {}", name))));
    }
}
