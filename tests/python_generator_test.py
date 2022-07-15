#!/usr/bin/env python3
#
# Copyright (C) 2015 The Android Open Source Project
#
# Tests the generated python backend against standard PDL
# constructs, with matching input vectors.

import json
import unittest

# (le|be)_pdl_test are the names of the modules generated from the canonical
# little endian and big endian test grammars. The purpose of this module
# is to validate the generated parsers against the set of pre-generated
# test vectors in canonical/(le|be)_test_vectors.json.
import le_pdl_test
import be_pdl_test


def match_object(self, left, right):
    """Recursively match a python class object against a reference
       json object."""
    if isinstance(right, int):
        self.assertEqual(left, right)
    elif isinstance(right, list):
        self.assertEqual(len(left), len(right))
        for n in range(len(right)):
            match_object(self, left[n], right[n])
    elif isinstance(right, dict):
        for (k, v) in right.items():
            self.assertTrue(hasattr(left, k))
            match_object(self, getattr(left, k), v)


class PacketParserTest(unittest.TestCase):
    """Validate the generated parser against pre-generated test
       vectors in canonical/(le|be)_test_vectors.json"""

    def testLittleEndian(self):
        with open('tests/canonical/le_test_vectors.json') as f:
            reference = json.load(f)

        for item in reference:
            # 'packet' is the name of the packet being tested,
            # 'tests' lists input vectors that must match the
            # selected packet.
            packet = item['packet']
            tests = item['tests']
            with self.subTest(packet=packet):
                # Retrieve the class object from the generated
                # module, in order to invoke the proper parse
                # method for this test.
                cls = getattr(le_pdl_test, packet)
                for test in tests:
                    result = cls.parse_all(bytes.fromhex(test['packed']))
                    match_object(self, result, test['unpacked'])

    def testBigEndian(self):
        with open('tests/canonical/be_test_vectors.json') as f:
            reference = json.load(f)

        for item in reference:
            # 'packet' is the name of the packet being tested,
            # 'tests' lists input vectors that must match the
            # selected packet.
            packet = item['packet']
            tests = item['tests']
            with self.subTest(packet=packet):
                # Retrieve the class object from the generated
                # module, in order to invoke the proper parse
                # method for this test.
                cls = getattr(be_pdl_test, packet)
                for test in tests:
                    result = cls.parse_all(bytes.fromhex(test['packed']))
                    match_object(self, result, test['unpacked'])


class CustomPacketParserTest(unittest.TestCase):
    """Manual testing for custom fields."""

    def testCustomField(self):
        result = le_pdl_test.Packet_Custom_Field_ConstantSize.parse_all([1])
        self.assertEqual(result.a.value, 1)

        result = le_pdl_test.Packet_Custom_Field_VariableSize.parse_all([1])
        self.assertEqual(result.a.value, 1)

        result = le_pdl_test.Struct_Custom_Field_ConstantSize.parse_all([1])
        self.assertEqual(result.s.a.value, 1)

        result = le_pdl_test.Struct_Custom_Field_VariableSize.parse_all([1])
        self.assertEqual(result.s.a.value, 1)

        result = be_pdl_test.Packet_Custom_Field_ConstantSize.parse_all([1])
        self.assertEqual(result.a.value, 1)

        result = be_pdl_test.Packet_Custom_Field_VariableSize.parse_all([1])
        self.assertEqual(result.a.value, 1)

        result = be_pdl_test.Struct_Custom_Field_ConstantSize.parse_all([1])
        self.assertEqual(result.s.a.value, 1)

        result = be_pdl_test.Struct_Custom_Field_VariableSize.parse_all([1])
        self.assertEqual(result.s.a.value, 1)


if __name__ == '__main__':
    unittest.main(verbosity=2)
