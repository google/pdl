Rust Generated Code Guide
=========================

Usage
-----

The crate `pdl_derive` lets developers embed their grammar in a source module
using the `pdl` proc_macro attribute. Example usage:

.. sourcecode:: rust

        use pdl_derive::pdl;
        use pdl_runtime::*;

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

.. warning::
    PDL authorizes the use of rust keywords as identifier. Keyword identifiers
    are generated as raw identifiers, e.g. `type` is generated as `r#type`.

Enum declarations
^^^^^^^^^^^^^^^^^

Private prevents users from creating arbitrary scalar values in situations where
the value needs to be validated. Users can freely deref the value, but only the
backend may create it.

.. sourcecode:: rust

        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub struct Private<T>(T);

        impl<T> std::ops::Deref for Private<T> { .. }

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

Packet declarations
^^^^^^^^^^^^^^^^^^^

Generated packet representations will all implement the `Packet` trait from
the `pdl_runtime` crate, which declares methods for parsing and serializing
packets:

.. sourcecode:: rust

        pub trait Packet: Sized {
            fn decode(buf: &[u8]) -> Result<(Self, &[u8]), DecodeError>;
            fn encode(&self, buf: &mut impl BufMut) -> Result<(), EncodeError>;
            fn encoded_len(&self) -> usize;
        }

Additional methods are generated depending on characteristics of the PDL
declaration (see the table below).

* derived packet declarations implement `decode_partial` (resp. `encode_partial`).
    These methods will decode (resp. encode) fields from the parent payload.

* packets with child declarations implement `specialize`. This method will
    attempt to decode one of the child packets from the input packet based on the
    constraints that are available.

+---------------------------------------+---------------------------------------------------------------+
| ::                                    | .. sourcecode:: rust                                          |
|                                       |                                                               |
|     packet Parent {                   |     #[device(Debug, Clone, PartialEq, Eq)]                    |
|         a:8,                          |     struct TestPacket {                                       |
|         _paylolad_,                   |         a: u8,                                                |
|     }                                 |         b: TestEnum,                                          |
|                                       |         payload: Vec<u8>,                                     |
|     packet TestPacket : Parent {      |     }                                                         |
|         b: TestEnum,                  |                                                               |
|         _payload_,                    |     impl TestPacket {                                         |
|     }                                 |         pub fn a(&self) -> u8 { .. }                          |
|                                       |         pub fn b(&self) -> TestEnum { .. }                    |
|                                       |         pub fn payload(&self) -> &[u8] { .. }                 |
|                                       |                                                               |
|                                       |         pub fn encode_partial(&self, buf: &mut impl BufMut)   |
|                                       |             -> Result<(), EncodeError> { .. }                 |
|                                       |                                                               |
|                                       |         pub fn decode_partial(parent: &Parent)                |
|                                       |             -> Result<Self, DecoderError { .. }               |
|                                       |     }                                                         |
|                                       |                                                               |
|                                       |     impl pdl_runtime::Packet for TestPacket { .. }            |
|                                       |     impl TryFrom<&TestPacket> for Bytes { .. }                |
|                                       |     impl TryFrom<&TestPacket> for Vec<u8> { .. }              |
+---------------------------------------+---------------------------------------------------------------+
| ::                                    | .. sourcecode:: rust                                          |
|                                       |                                                               |
|     packet TestPacket {               |     #[derive(Debug, Clone, PartialEq, Eq)]                    |
|         a: 8,                         |     struct TestPacket {                                       |
|         _payload_,                    |         a: u8,                                                |
|     }                                 |         payload: Vec<u8>,                                     |
|                                       |     }                                                         |
|     packet Child1 : TestPacket {      |                                                               |
|         ..                            |     #[derive(Debug, Clone, PartialEq, Eq)]                    |
|     }                                 |     struct TestPacketChild {                                  |
|                                       |         Child1(Child1),                                       |
|     packet Child2 : TestPacket {      |         Child2(Child2),                                       |
|         ..                            |         None,                                                 |
|     }                                 |     }                                                         |
|                                       |                                                               |
|                                       |     impl TestPacket {                                         |
|                                       |         pub fn a(&self) -> u8 { .. }                          |
|                                       |         pub fn payload(&self) -> &[u8] { .. }                 |
|                                       |         pub fn specialize(&self)                              |
|                                       |             -> Result<TestPacketChild, DecodeError> { .. }    |
|                                       |     }                                                         |
|                                       |                                                               |
|                                       |     impl pdl_runtime::Packet for TestPacket { .. }            |
|                                       |     impl TryFrom<&TestPacket> for Bytes { .. }                |
|                                       |     impl TryFrom<&TestPacket> for Vec<u8> { .. }              |
+---------------------------------------+---------------------------------------------------------------+

Field declarations
^^^^^^^^^^^^^^^^^^

+---------------------------------------+---------------------------------------------------------------+
| ::                                    | .. sourcecode:: rust                                          |
|                                       |                                                               |
|     a: 8                              |     a: u8                                                     |
|     b: 24                             |     b: u32                                                    |
+---------------------------------------+---------------------------------------------------------------+
| ::                                    | .. sourcecode:: rust                                          |
|                                       |                                                               |
|     a: TestEnum,                      |     a: TestEnum                                               |
|     b: TestStruct                     |     b: TestStruct                                             |
+---------------------------------------+---------------------------------------------------------------+
| ::                                    | .. sourcecode:: rust                                          |
|                                       |                                                               |
|     a: 8[],                           |     a: Vec<u8>                                                |
|     b: 16[128],                       |     b: [u16; 128]                                             |
|     c: TestEnum[],                    |     c: Vec<TestEnum>                                          |
|     d: TestStruct[]                   |     d: Vec<TestStruct>                                        |
+---------------------------------------+---------------------------------------------------------------+
| ::                                    | .. sourcecode:: rust                                          |
|                                       |                                                               |
|     a: 8 if c_a = 1,                  |     a: Option<u8>                                             |
|     b: TestEnum if c_b = 1,           |     b: Option<TestEnum>                                       |
|     c: TestStruct if c_c = 1,         |     c: Option<TestStruct>                                     |
+---------------------------------------+---------------------------------------------------------------+
| ::                                    | .. sourcecode:: rust                                          |
|                                       |                                                               |
|     _payload_,                        |     payload: Vec<u8>                                          |
+---------------------------------------+---------------------------------------------------------------+
