#!/usr/bin/env bash

set -euxo pipefail

mkdir -p out/
OUT_DIR="$(pwd)/out"

# move to `pdl-compiler` directory
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." &> /dev/null

mkdir -p "$OUT_DIR/generated_test/src"
./tests/generated_files_compile.sh tests/generated/rust/*.rs > "$OUT_DIR/generated_test/src/lib.rs"

cat <<EOT > "$OUT_DIR/generated_test/Cargo.toml"
[package]
name = "generated_test"
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

cp ../Cargo.lock "$OUT_DIR/generated_test/Cargo.lock"
cd "$OUT_DIR/generated_test"
RUSTFLAGS=-Awarnings cargo build

cd - &> /dev/null
mkdir -p "$OUT_DIR/generated_nostd_test/src"
echo "#![no_std]" > "$OUT_DIR/generated_nostd_test/src/lib.rs"
./tests/generated_files_compile.sh tests/generated/rust_nostd/*.rs >> "$OUT_DIR/generated_nostd_test/src/lib.rs"

cat <<EOT > "$OUT_DIR/generated_nostd_test/Cargo.toml"
[package]
name = "generated_nostd_test"
version = "0.0.0"
publish = false
edition = "2021"

[features]
default = ["serde"]

[dependencies]
bytes = {version = "1.4.0", default-features = false, features = ["serde"]}
serde_json = "1.0.86"

[dependencies.serde]
version = "1.0.145"
default-features = false
features = ["alloc", "derive", "serde_derive"]
optional = true

[dependencies.pdl-runtime]
path = "../../pdl-runtime"
features = ["serde"]

[workspace]
EOT

cp ../Cargo.lock "$OUT_DIR/generated_nostd_test/Cargo.lock"
cd "$OUT_DIR/generated_nostd_test"
RUSTFLAGS=-Awarnings cargo build
