Rust Generated Code Guide
=========================

Usage
-----

Example invocation:

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
