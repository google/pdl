#!/usr/bin/env bash

set -euxo pipefail

mkdir -p out/
OUT_DIR="$(pwd)/out"

# move to `pdl-compiler` directory
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." &> /dev/null

sed -e 's/little_endian_packets/big_endian_packets/' \
    -e '/Start: little_endian_only/,/End: little_endian_only/d' \
    < tests/canonical/le_test_file.pdl > "$OUT_DIR"/be_test_file.pdl

cargo run --all-features --bin pdlc -- \
    --output-format python \
    --custom-field tests.custom_types \
    --exclude-declaration Packet_Array_Field_VariableElementSize_ConstantSize \
    --exclude-declaration Packet_Array_Field_VariableElementSize_VariableSize \
    --exclude-declaration Packet_Array_Field_VariableElementSize_VariableCount \
    --exclude-declaration Packet_Array_Field_VariableElementSize_UnknownSize \
    tests/canonical/le_test_file.pdl > "$OUT_DIR"/le_backend.py

cargo run --all-features --bin pdlc -- \
    --output-format python \
    --custom-field tests.custom_types \
    --exclude-declaration Packet_Array_Field_VariableElementSize_ConstantSize \
    --exclude-declaration Packet_Array_Field_VariableElementSize_VariableSize \
    --exclude-declaration Packet_Array_Field_VariableElementSize_VariableCount \
    --exclude-declaration Packet_Array_Field_VariableElementSize_UnknownSize \
    "$OUT_DIR"/be_test_file.pdl > "$OUT_DIR"/be_backend.py

export PYTHONPATH="$OUT_DIR:.:${PYTHONPATH:-}"
python3 tests/python_generator_test.py
