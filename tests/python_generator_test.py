#!/usr/bin/env python3
#
# Copyright (C) 2015 The Android Open Source Project
#
# Tests the generated python backend against standard PDL
# constructs, with matching input vectors.

import dataclasses
import enum
import json
import typing
import typing_extensions
import unittest
from importlib import resources

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


def create_object(typ, value):
    """Build an object of the selected type using the input value."""
    if dataclasses.is_dataclass(typ):
        field_types = dict([(f.name, f.type) for f in dataclasses.fields(typ)])
        values = dict()
        for (f, v) in value.items():
            field_type = field_types[f]
            values[f] = create_object(field_type, v)
        return typ(**values)
    elif typing_extensions.get_origin(typ) is list:
        typ = typing_extensions.get_args(typ)[0]
        return [create_object(typ, v) for v in value]
    elif typing_extensions.get_origin(typ) is typing.Union:
        # typing.Optional[int] expands to typing.Union[int, None]
        typ = typing_extensions.get_args(typ)[0]
        return create_object(typ, value) if value else None
    elif typ is bytes:
        return bytes(value)
    elif typ is bytearray:
        return bytearray(value)
    elif issubclass(typ, enum.Enum):
        return typ(value)
    elif typ is int:
        return value
    else:
        raise Exception(f"unsupported type annotation {typ}")


class PacketParserTest(unittest.TestCase):
    """Validate the generated parser against pre-generated test
       vectors in canonical/(le|be)_test_vectors.json"""

    def testLittleEndian(self):
        with resources.files('tests.canonical').joinpath('le_test_vectors.json').open('r') as f:
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
        with resources.files('tests.canonical').joinpath('be_test_vectors.json').open('r') as f:
            reference = json.load(f)

        for item in reference:
            # 'packet' is the name of the packet being tested,
            # 'tests' lists input vectors that must match the
            # selected packet.
            packet = item['packet']
            tests = item['tests']
            with self.subTest(packet=packet):
                # Retrieve the class object from the generated
                # module, in order to invoke the proper constructor
                # method for this test.
                cls = getattr(be_pdl_test, packet)
                for test in tests:
                    result = cls.parse_all(bytes.fromhex(test['packed']))
                    match_object(self, result, test['unpacked'])


class PacketSerializerTest(unittest.TestCase):
    """Validate the generated serializer against pre-generated test
       vectors in canonical/(le|be)_test_vectors.json"""

    def testLittleEndian(self):
        with resources.files('tests.canonical').joinpath('le_test_vectors.json').open('r') as f:
            reference = json.load(f)

        for item in reference:
            # 'packet' is the name of the packet being tested,
            # 'tests' lists input vectors that must match the
            # selected packet.
            packet = item['packet']
            tests = item['tests']
            with self.subTest(packet=packet):
                # Retrieve the class object from the generated
                # module, in order to invoke the proper constructor
                # method for this test.
                for test in tests:
                    cls = getattr(le_pdl_test, test.get('packet', packet))
                    obj = create_object(cls, test['unpacked'])
                    result = obj.serialize()
                    self.assertEqual(result, bytes.fromhex(test['packed']))

    def testBigEndian(self):
        with resources.files('tests.canonical').joinpath('be_test_vectors.json').open('r') as f:
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
                for test in tests:
                    cls = getattr(be_pdl_test, test.get('packet', packet))
                    obj = create_object(cls, test['unpacked'])
                    result = obj.serialize()
                    self.assertEqual(result, bytes.fromhex(test['packed']))


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
    unittest.main(verbosity=3)
