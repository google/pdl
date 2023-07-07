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

python3 scripts/generate_cxx_backend.py \
    --input "$OUT_DIR"/le_test_file.json \
    --output "$OUT_DIR"/le_backend.h \
    --namespace le_backend
python3 scripts/generate_cxx_backend.py \
    --input "$OUT_DIR"/be_test_file.json \
    --output "$OUT_DIR"/be_backend.h \
    --namespace be_backend

python3 scripts/generate_cxx_backend_tests.py \
    --input "$OUT_DIR"/le_test_file.json \
    --output "$OUT_DIR"/le_backend_tests.cc \
    --test-vectors tests/canonical/le_test_vectors.json \
    --namespace le_backend \
    --parser-test-suite le_backend_parser_test \
    --serializer-test-suite le_backend_serializer_test \
    --include-header le_backend.h
python3 scripts/generate_cxx_backend_tests.py \
    --input "$OUT_DIR"/be_test_file.json \
    --output "$OUT_DIR"/be_backend_tests.cc \
    --test-vectors tests/canonical/be_test_vectors.json \
    --namespace be_backend \
    --parser-test-suite be_backend_parser_test \
    --serializer-test-suite be_backend_serializer_test \
    --include-header be_backend.h

g++ -Iscripts -I"$OUT_DIR" \
    "$OUT_DIR"/le_backend_tests.cc \
    "$OUT_DIR"/be_backend_tests.cc \
    -lgtest -lgtest_main -o "$OUT_DIR"/cxx_backend_tests

"$OUT_DIR"/cxx_backend_tests \
    --gtest_output="xml:$OUT_DIR/cxx_backend_tests_detail.xml"
