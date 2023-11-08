#!/usr/bin/env bash

set -euxo pipefail

mkdir -p out/
OUT_DIR="$(pwd)/out"

# move to `pdl-compiler` directory
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." &> /dev/null

sed -e 's/little_endian_packets/big_endian_packets/' \
    -e '/Start: little_endian_only/,/End: little_endian_only/d' \
    < tests/canonical/le_test_file.pdl > "$OUT_DIR/be_test_file.pdl"

mkdir -p "$OUT_DIR/canonical_test/src"

cargo run --bin pdlc -- \
    tests/canonical/le_test_file.pdl \
    --output-format rust \
    --exclude-declaration UnsizedCustomField \
    --exclude-declaration Packet_Custom_Field_VariableSize \
    --exclude-declaration Struct_Custom_Field_VariableSize_ \
    --exclude-declaration Struct_Custom_Field_VariableSize \
    --exclude-declaration Checksum \
    --exclude-declaration Packet_Checksum_Field_FromStart \
    --exclude-declaration Packet_Checksum_Field_FromEnd \
    --exclude-declaration Struct_Checksum_Field_FromStart_ \
    --exclude-declaration Struct_Checksum_Field_FromStart \
    --exclude-declaration Struct_Checksum_Field_FromEnd_ \
    --exclude-declaration Struct_Checksum_Field_FromEnd \
    --exclude-declaration PartialParent5 \
    --exclude-declaration PartialParent12 \
    --exclude-declaration PartialChild5_A \
    --exclude-declaration PartialChild5_B \
    --exclude-declaration PartialChild12_A \
    --exclude-declaration PartialChild12_B \
    --exclude-declaration Packet_Array_Field_UnsizedElement_SizeModifier \
    --exclude-declaration Struct_Array_Field_UnsizedElement_SizeModifier_ \
    --exclude-declaration Struct_Array_Field_UnsizedElement_SizeModifier \
    > "$OUT_DIR/canonical_test/src/le_backend.rs"
cargo run --bin pdlc -- \
    "$OUT_DIR/be_test_file.pdl" \
    --output-format rust \
    --exclude-declaration UnsizedCustomField \
    --exclude-declaration Packet_Custom_Field_VariableSize \
    --exclude-declaration Struct_Custom_Field_VariableSize_ \
    --exclude-declaration Struct_Custom_Field_VariableSize \
    --exclude-declaration Checksum \
    --exclude-declaration Packet_Checksum_Field_FromStart \
    --exclude-declaration Packet_Checksum_Field_FromEnd \
    --exclude-declaration Struct_Checksum_Field_FromStart_ \
    --exclude-declaration Struct_Checksum_Field_FromStart \
    --exclude-declaration Struct_Checksum_Field_FromEnd_ \
    --exclude-declaration Struct_Checksum_Field_FromEnd \
    --exclude-declaration Packet_Array_Field_UnsizedElement_SizeModifier \
    --exclude-declaration Struct_Array_Field_UnsizedElement_SizeModifier_ \
    --exclude-declaration Struct_Array_Field_UnsizedElement_SizeModifier \
    > "$OUT_DIR/canonical_test/src/be_backend.rs"
cargo run --bin generate-canonical-tests -- \
    tests/canonical/le_test_vectors.json "crate::le_backend" \
    > "$OUT_DIR/canonical_test/src/le_backend_tests.rs"
cargo run --bin generate-canonical-tests -- \
    tests/canonical/be_test_vectors.json "crate::be_backend" \
    > "$OUT_DIR/canonical_test/src/be_backend_tests.rs"


cat <<EOT > "$OUT_DIR/canonical_test/src/lib.rs"
mod le_backend;
mod le_backend_tests;
mod be_backend;
mod be_backend_tests;
EOT

cat <<EOT > "$OUT_DIR/canonical_test/Cargo.toml"
[package]
name = "canonical_test"
version = "0.0.0"
publish = false
edition = "2021"

[features]
default = ["serde"]

[dependencies]
bytes = {version = "1.4.0", features = ["serde"]}
thiserror = "1.0.47"
serde_json = "1.0.86"

[dependencies.serde]
version = "1.0.145"
features = ["default", "derive", "serde_derive", "std", "rc"]
optional = true

[dependencies.pdl-runtime]
path = "../../pdl-runtime"

[workspace]
EOT

cd "$OUT_DIR/canonical_test"
RUSTFLAGS=-Awarnings cargo test --tests
