#!/usr/bin/env bash

set -euxo pipefail

EXCLUDE_DECLS="--exclude-declaration SizedCustomField \
--exclude-declaration UnsizedCustomField \
--exclude-declaration Checksum \
--exclude-declaration ScalarGroup \
--exclude-declaration EnumGroup \
--exclude-declaration Packet_Body_Field_VariableSize \
--exclude-declaration Packet_Body_Field_UnknownSize \
--exclude-declaration Packet_Body_Field_UnknownSize_Terminal \
--exclude-declaration Packet_ScalarGroup_Field \
--exclude-declaration Packet_EnumGroup_Field \
--exclude-declaration Packet_Checksum_Field_FromStart \
--exclude-declaration Packet_Checksum_Field_FromEnd \
--exclude-declaration Packet_Custom_Field_ConstantSize \
--exclude-declaration Packet_Custom_Field_VariableSize \
--exclude-declaration Packet_Array_Field_SizedElement_VariableSize_Padded \
--exclude-declaration Packet_Array_Field_UnsizedElement_VariableCount_Padded \
--exclude-declaration Packet_Array_Field_VariableElementSize_ConstantSize \
--exclude-declaration Packet_Array_Field_VariableElementSize_VariableSize \
--exclude-declaration Packet_Array_Field_VariableElementSize_VariableCount \
--exclude-declaration Packet_Array_Field_VariableElementSize_UnknownSize \
--exclude-declaration Packet_Optional_Scalar_Field \
--exclude-declaration Packet_Optional_Enum_Field \
--exclude-declaration Packet_Optional_Struct_Field \
--exclude-declaration AliasedChild_A \
--exclude-declaration AliasedChild_B \
--exclude-declaration Struct_ScalarGroup_Field_ \
--exclude-declaration Struct_ScalarGroup_Field \
--exclude-declaration Struct_EnumGroup_Field_ \
--exclude-declaration Struct_EnumGroup_Field \
--exclude-declaration Struct_Checksum_Field_FromStart_ \
--exclude-declaration Struct_Checksum_Field_FromStart \
--exclude-declaration Struct_Checksum_Field_FromEnd_ \
--exclude-declaration Struct_Checksum_Field_FromEnd \
--exclude-declaration Struct_Custom_Field_ConstantSize_ \
--exclude-declaration Struct_Custom_Field_ConstantSize \
--exclude-declaration Struct_Custom_Field_VariableSize_ \
--exclude-declaration Struct_Custom_Field_VariableSize \
--exclude-declaration Struct_Array_Field_SizedElement_VariableSize_Padded_ \
--exclude-declaration Struct_Array_Field_SizedElement_VariableSize_Padded \
--exclude-declaration Struct_Array_Field_UnsizedElement_VariableCount_Padded_ \
--exclude-declaration Struct_Array_Field_UnsizedElement_VariableCount_Padded \
--exclude-declaration Struct_Optional_Scalar_Field_ \
--exclude-declaration Struct_Optional_Scalar_Field \
--exclude-declaration Struct_Optional_Enum_Field_ \
--exclude-declaration Struct_Optional_Enum_Field \
--exclude-declaration Struct_Optional_Struct_Field_ \
--exclude-declaration Struct_Optional_Struct_Field"

mkdir -p out/
OUT_DIR="$(pwd)/out"

# move to `pdl-compiler` directory
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." &>/dev/null

sed -e 's/little_endian_packets/big_endian_packets/' \
  -e '/Start: little_endian_only/,/End: little_endian_only/d' \
  <tests/canonical/le_test_file.pdl >"$OUT_DIR/be_test_file.pdl"

mkdir -p "$OUT_DIR/java/test"

# Little Endian Codegen

cargo run --all-features --bin pdlc -- \
  tests/canonical/le_test_file.pdl \
  --output-format java \
  --output-dir "$OUT_DIR/java" \
  --java-package test.little_endian \
  $EXCLUDE_DECLS

cargo run --all-features --bin pdlc -- \
  tests/canonical/le_test_vectors.json \
  --output-format java \
  --tests \
  --output-dir "$OUT_DIR/java" \
  --java-package "test.little_endian" \
  --pdl-file-under-test "tests/canonical/le_test_file.pdl" \
  $EXCLUDE_DECLS

# Big Endian Codegen

cargo run --all-features --bin pdlc -- \
  "$OUT_DIR/be_test_file.pdl" \
  --output-format java \
  --output-dir "$OUT_DIR/java" \
  --java-package test.big_endian \
  $EXCLUDE_DECLS

cargo run --all-features --bin pdlc -- \
  tests/canonical/be_test_vectors.json \
  --output-format java \
  --tests \
  --output-dir "$OUT_DIR/java" \
  --java-package "test.big_endian" \
  --pdl-file-under-test "$OUT_DIR/be_test_file.pdl" \
  $EXCLUDE_DECLS

# Compile and Execute

cd "$OUT_DIR/java/"

javac test/little_endian/PdlTests.java
java -enableassertions test.little_endian.PdlTests

javac test/big_endian/PdlTests.java
java -enableassertions test.big_endian.PdlTests
