Rust Generated Code Guide
=========================

Usage
-----

The crate `pdl_derive` lets developers embed their grammar in a source module
using the `pdl` proc_macro attribute. Example usage:

.. sourcecode:: rust

        use pdl_derive::pdl

        #[pdl("my-protocol.pdl")]
        mod my_protocol {
        }

The `pdl` proc_macro attribute must be attached to a module declaration.
`pdl` preserves the original name, attributes, and items of the associated
module.

The backend can also be pre-generated from the `pdlc` tool,
and compiled as source. Example invocation:

.. sourcecode:: bash

    cargo run my-protocol.pdl --output-format rust > my-protocol.rs

Language bindings
-----------------

This section contains the generated rust bindings for language constructs that
are stabilized.

Preamble
^^^^^^^^

Private prevents users from creating arbitrary scalar values in situations where
the value needs to be validated. Users can freely deref the value, but only the
backend may create it.

.. sourcecode:: rust

        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub struct Private<T>(T);

        impl<T> std::ops::Deref for Private<T> { .. }

.. warning::
    PDL authorizes the use of rust keywords as identifier. Keyword identifiers
    are generated as raw identifiers, e.g. `type` is generated as `r#type`.

Enum declarations
^^^^^^^^^^^^^^^^^

+---------------------------------------+---------------------------------------------------------------+
| ::                                    | .. sourcecode:: rust                                          |
|                                       |                                                               |
|     enum TestEnum : 8 {               |     #[repr(u64)]                                              |
|         A = 1,                        |     #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]        |
|         B = 2,                        |     enum TestEnum {                                           |
|     }                                 |         A = 1,                                                |
|                                       |         B = 2,                                                |
|                                       |     }                                                         |
|                                       |                                                               |
|                                       |     impl TryFrom<u8> for TestEnum { .. }                      |
|                                       |     impl From<TestEnum> for u8 { .. }                         |
|                                       |     impl From<TestEnum> for u16 { .. }                        |
|                                       |     impl From<TestEnum> for u32 { .. }                        |
|                                       |     impl From<TestEnum> for u64 { .. }                        |
|                                       |     impl From<TestEnum> for i8 { .. }                         |
|                                       |     impl From<TestEnum> for i16 { .. }                        |
|                                       |     impl From<TestEnum> for i32 { .. }                        |
|                                       |     impl From<TestEnum> for i64 { .. }                        |
+---------------------------------------+---------------------------------------------------------------+
| ::                                    | .. sourcecode:: rust                                          |
|                                       |                                                               |
|     enum TestEnum : 8 {               |     #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]        |
|         A = 1,                        |     enum TestEnum {                                           |
|         B = 2..10 {                   |         A,                                                    |
|             C = 3,                    |         C,                                                    |
|         },                            |         B(Private<u8>),                                       |
|     }                                 |     }                                                         |
|                                       |                                                               |
|                                       |     impl TryFrom<u8> for TestEnum { .. }                      |
|                                       |     impl From<TestEnum> for u8 { .. }                         |
|                                       |     impl From<TestEnum> for u16 { .. }                        |
|                                       |     impl From<TestEnum> for u32 { .. }                        |
|                                       |     impl From<TestEnum> for u64 { .. }                        |
|                                       |     impl From<TestEnum> for i8 { .. }                         |
|                                       |     impl From<TestEnum> for i16 { .. }                        |
|                                       |     impl From<TestEnum> for i32 { .. }                        |
|                                       |     impl From<TestEnum> for i64 { .. }                        |
+---------------------------------------+---------------------------------------------------------------+
| ::                                    | .. sourcecode:: rust                                          |
|                                       |                                                               |
|     enum TestEnum : 8 {               |     #[repr(u64)]                                              |
|         A = 1,                        |     #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]        |
|         B = 2,                        |     enum TestEnum {                                           |
|         OTHER = ..,                   |         A,                                                    |
|     }                                 |         B,                                                    |
|                                       |         Other(Private<u8>),                                   |
|                                       |     }                                                         |
|                                       |                                                               |
|                                       |     impl From<u8> for TestEnum { .. }                         |
|                                       |     impl From<TestEnum> for u8 { .. }                         |
|                                       |     impl From<TestEnum> for u16 { .. }                        |
|                                       |     impl From<TestEnum> for u32 { .. }                        |
|                                       |     impl From<TestEnum> for u64 { .. }                        |
|                                       |     impl From<TestEnum> for i8 { .. }                         |
|                                       |     impl From<TestEnum> for i16 { .. }                        |
|                                       |     impl From<TestEnum> for i32 { .. }                        |
|                                       |     impl From<TestEnum> for i64 { .. }                        |
+---------------------------------------+---------------------------------------------------------------+
