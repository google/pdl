#!/usr/bin/env bash

set -euxo pipefail

mkdir -p out/

sed -e 's/little_endian_packets/big_endian_packets/' \
    -e '/Start: little_endian_only/,/End: little_endian_only/d' \
    < tests/canonical/le_test_file.pdl > out/be_test_file.pdl

pdlc tests/canonical/le_test_file.pdl > out/le_test_file.json
pdlc out/be_test_file.pdl > out/be_test_file.json

python3 scripts/generate_python_backend.py \
    --input out/le_test_file.json \
    --output out/le_backend.py \
    --custom-type-location tests.custom_types
python3 scripts/generate_python_backend.py \
    --input out/be_test_file.json \
    --output out/be_backend.py \
    --custom-type-location tests.custom_types

export PYTHONPATH="./out:.:${PYTHONPATH:-}"
python3 tests/python_generator_test.py
