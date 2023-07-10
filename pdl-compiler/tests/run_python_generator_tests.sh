#!/usr/bin/env bash

set -euxo pipefail

mkdir -p out/
OUT_DIR="$(pwd)/out"

# move to `pdl-compiler` directory
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." &> /dev/null

sed -e 's/little_endian_packets/big_endian_packets/' \
    -e '/Start: little_endian_only/,/End: little_endian_only/d' \
    < tests/canonical/le_test_file.pdl > "$OUT_DIR"/be_test_file.pdl

pdlc tests/canonical/le_test_file.pdl > "$OUT_DIR"/le_test_file.json
pdlc "$OUT_DIR"/be_test_file.pdl > "$OUT_DIR"/be_test_file.json

python3 scripts/generate_python_backend.py \
    --input "$OUT_DIR"/le_test_file.json \
    --output "$OUT_DIR"/le_backend.py \
    --custom-type-location tests.custom_types
python3 scripts/generate_python_backend.py \
    --input "$OUT_DIR"/be_test_file.json \
    --output "$OUT_DIR"/be_backend.py \
    --custom-type-location tests.custom_types

export PYTHONPATH="$OUT_DIR:.:${PYTHONPATH:-}"
python3 tests/python_generator_test.py
