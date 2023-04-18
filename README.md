# Packet Description Language (PDL)

PDL is a domain specific language for writing the definition of binary protocol
packets. Parsing and validating packets from raw bytes is tedious and error
prone in any language. PDL generates memory safe and tailored backends for
mulitple target languages:

    - Rust
    - C++
    - Python

## How to use PDL

1. Write the protocol definition
1. `cargo run my-protocol.pdl --output-format rust > my-protocol.rs`

Language specific instructions are provided in another section.

## Supported Features

[Full reference documentation](#doc/reference.md)
- Scalar values
- Enumerators
- Arrays
- Nested packets
- Conditional packet derivation
- Custom field definitions

## Similar projects

* [Kaitai](https://kaitai.io)
* [EMBOSS](https://github.com/kimrutherford/EMBOSS)
* [P4](https://p4.org/p4-spec/docs/P4-16-v1.0.0-spec.html)
