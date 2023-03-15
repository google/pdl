#!/usr/bin/env bash

# Run this script with a number of Rust files as input. It will combine them to
# a single file which you can compile to check the validity of the inputs.
#
# For a Cargo based workflow, you can run
#
# ./generated_files_compile.sh generated/*.rs > generated_files.rs
#
# followed by cargo test.

for input_path in "$@"; do
    echo "mod $(basename -s .rs "$input_path") {"
    cat "$input_path"
    echo "}"
done

cat <<EOF
#[test]
fn generated_files_compile() {
    // Empty test, we only want to see that things compile.
}
EOF
