#!/usr/bin/env bash

set -euxo pipefail

mkdir -p out/
OUT_DIR="$(pwd)/out"

# move to `pdl-compiler` directory
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." &> /dev/null

mkdir -p "$OUT_DIR/generated_test/src"
./tests/generated_files_compile.sh tests/generated/*.rs > "$OUT_DIR/generated_test/src/lib.rs"

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

cd "$OUT_DIR/generated_test"
RUSTFLAGS=-Awarnings cargo build
