#!/usr/bin/env bash

set -euxo pipefail

mkdir -p out/

sed -e 's/little_endian_packets/big_endian_packets/' \
    -e '/Start: little_endian_only/,/End: little_endian_only/d' \
    < tests/canonical/le_test_file.pdl > out/be_test_file.pdl

pdlc tests/canonical/le_test_file.pdl > out/le_test_file.json
pdlc out/be_test_file.pdl > out/be_test_file.json

python3 scripts/generate_cxx_backend.py \
    --input out/le_test_file.json \
    --output out/le_backend.h \
    --namespace le_backend
python3 scripts/generate_cxx_backend.py \
    --input out/be_test_file.json \
    --output out/be_backend.h \
    --namespace be_backend

python3 scripts/generate_cxx_backend_tests.py \
    --input out/le_test_file.json \
    --output out/le_backend_tests.cc \
    --test-vectors tests/canonical/le_test_vectors.json \
    --namespace le_backend \
    --parser-test-suite le_backend_parser_test \
    --serializer-test-suite le_backend_serializer_test \
    --include-header le_backend.h
python3 scripts/generate_cxx_backend_tests.py \
    --input out/be_test_file.json \
    --output out/be_backend_tests.cc \
    --test-vectors tests/canonical/be_test_vectors.json \
    --namespace be_backend \
    --parser-test-suite be_backend_parser_test \
    --serializer-test-suite be_backend_serializer_test \
    --include-header be_backend.h

g++ -Iscripts -Iout \
    out/le_backend_tests.cc \
    out/be_backend_tests.cc \
    -lgtest -lgtest_main -o out/cxx_backend_tests

./out/cxx_backend_tests \
    --gtest_output="xml:out/cxx_backend_tests_detail.xml"
