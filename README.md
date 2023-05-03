# Packet Description Language (PDL)

PDL is a domain specific language for writing the definition of binary protocol
packets. Parsing and validating packets from raw bytes is tedious and error
prone in any language. PDL generates memory safe and tailored backends for
multiple target languages:

    - Rust
    - C++
    - Python

Historically PDL was developed as part of the Android Bluetooth stack
([bluetooth_packetgen](https://cs.android.com/android/platform/superproject/+/master:packages/modules/Bluetooth/system/gd/packet/))
as a way to generate the parser and serializer for Bluetooth packets, and
reduce the number of memory safety issues that come with manipulating
and validating raw data.

## How to use PDL

1. Write the protocol definition
1. `cargo run my-protocol.pdl --output-format rust > my-protocol.rs`

Language specific instructions are provided for all supported backends:

1. [Rust generated code guide](doc/rust-generated-code-guide.rst)
1. [Python generated code guide](doc/python-generated-code-guide.rst)
1. [C++ generated code guide](doc/cxx-generated-code-guide.rst)

## Supported Features

[Full reference documentation](doc/reference.md)
- Scalar values
- Enumerators
- Arrays
- Nested packets
- Conditional packet derivation
- Custom field definitions

## Similar projects

- [Kaitai](https://kaitai.io)
- [EMBOSS](https://github.com/kimrutherford/EMBOSS)
- [P4](https://p4.org/p4-spec/docs/P4-16-v1.0.0-spec.html)
