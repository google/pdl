#!/usr/bin/env python3

import argparse
from dataclasses import dataclass, field
import json
from pathlib import Path
import sys
from textwrap import dedent
from typing import List, Tuple, Union, Optional

from pdl import ast, core
from pdl.utils import indent, to_pascal_case


def get_cxx_scalar_type(width: int) -> str:
    """Return the cxx scalar type to be used to back a PDL type."""
    for n in [8, 16, 32, 64]:
        if width <= n:
            return f'uint{n}_t'
    # PDL type does not fit on non-extended scalar types.
    assert False


def generate_packet_parser_test(parser_test_suite: str, packet: ast.PacketDeclaration, tests: List[object]) -> str:
    """Generate the implementation of unit tests for the selected packet."""

    def parse_packet(packet: ast.PacketDeclaration) -> str:
        parent = parse_packet(packet.parent) if packet.parent else "input"
        return f"{packet.id}View::Create({parent})"

    def input_bytes(input: str) -> List[str]:
        input = bytes.fromhex(input)
        input_bytes = []
        for i in range(0, len(input), 16):
            input_bytes.append(' '.join(f'0x{b:x},' for b in input[i:i + 16]))
        return input_bytes

    def get_field(decl: ast.Declaration, var: str, id: str) -> str:
        if isinstance(decl, ast.StructDeclaration):
            return f"{var}.{id}_"
        else:
            return f"{var}.Get{to_pascal_case(id)}()"

    def check_members(decl: ast.Declaration, var: str, expected: object) -> List[str]:
        checks = []
        for (id, value) in expected.items():
            field = core.get_packet_field(decl, id)
            sanitized_var = var.replace('[', '_').replace(']', '')
            field_var = f'{sanitized_var}_{id}'

            if isinstance(field, ast.ScalarField):
                checks.append(f"ASSERT_EQ({get_field(decl, var, id)}, {value});")

            elif (isinstance(field, ast.TypedefField) and
                  isinstance(field.type, (ast.EnumDeclaration, ast.CustomFieldDeclaration, ast.ChecksumDeclaration))):
                checks.append(f"ASSERT_EQ({get_field(decl, var, id)}, {field.type_id}({value}));")

            elif isinstance(field, ast.TypedefField):
                checks.append(f"{field.type_id} const& {field_var} = {get_field(decl, var, id)};")
                checks.extend(check_members(field.type, field_var, value))

            elif isinstance(field, (ast.PayloadField, ast.BodyField)):
                checks.append(f"std::vector<uint8_t> expected_{field_var} {{")
                for i in range(0, len(value), 16):
                    checks.append('    ' + ' '.join([f"0x{v:x}," for v in value[i:i + 16]]))
                checks.append("};")
                checks.append(f"ASSERT_EQ({get_field(decl, var, id)}, expected_{field_var});")

            elif isinstance(field, ast.ArrayField) and field.width:
                checks.append(f"std::vector<{get_cxx_scalar_type(field.width)}> expected_{field_var} {{")
                step = int(16 * 8 / field.width)
                for i in range(0, len(value), step):
                    checks.append('    ' + ' '.join([f"0x{v:x}," for v in value[i:i + step]]))
                checks.append("};")
                checks.append(f"ASSERT_EQ({get_field(decl, var, id)}, expected_{field_var});")

            elif (isinstance(field, ast.ArrayField) and isinstance(field.type, ast.EnumDeclaration)):
                checks.append(f"std::vector<{field.type_id}> expected_{field_var} {{")
                for v in value:
                    checks.append(f"    {field.type_id}({v}),")
                checks.append("};")
                checks.append(f"ASSERT_EQ({get_field(decl, var, id)}, expected_{field_var});")

            elif isinstance(field, ast.ArrayField):
                checks.append(f"std::vector<{field.type_id}> {field_var} = {get_field(decl, var, id)};")
                checks.append(f"ASSERT_EQ({field_var}.size(), {len(value)});")
                for (n, value) in enumerate(value):
                    checks.extend(check_members(field.type, f"{field_var}[{n}]", value))

            else:
                pass

        return checks

    generated_tests = []
    for (test_nr, test) in enumerate(tests):
        child_packet_id = test.get('packet', packet.id)
        child_packet = packet.file.packet_scope[child_packet_id]

        generated_tests.append(
            dedent("""\

            TEST_F({parser_test_suite}, {packet_id}_Case{test_nr}) {{
                pdl::packet::slice input(std::shared_ptr<std::vector<uint8_t>>(new std::vector<uint8_t> {{
                    {input_bytes}
                }}));
                {child_packet_id}View packet = {parse_packet};
                ASSERT_TRUE(packet.IsValid());
                {checks}
            }}
            """).format(parser_test_suite=parser_test_suite,
                        packet_id=packet.id,
                        child_packet_id=child_packet_id,
                        test_nr=test_nr,
                        input_bytes=indent(input_bytes(test['packed']), 2),
                        parse_packet=parse_packet(child_packet),
                        checks=indent(check_members(packet, 'packet', test['unpacked']), 1)))

    return ''.join(generated_tests)


def generate_packet_serializer_test(serializer_test_suite: str, packet: ast.PacketDeclaration,
                                    tests: List[object]) -> str:
    """Generate the implementation of unit tests for the selected packet."""

    def build_packet(decl: ast.Declaration, var: str, initializer: object) -> (str, List[str]):
        fields = core.get_unconstrained_parent_fields(decl) + decl.fields
        declarations = []
        parameters = []
        for field in fields:
            sanitized_var = var.replace('[', '_').replace(']', '')
            field_id = getattr(field, 'id', None)
            field_var = f'{sanitized_var}_{field_id}'
            value = initializer['payload'] if isinstance(field, (ast.PayloadField,
                                                                 ast.BodyField)) else initializer.get(field_id, None)

            if isinstance(field, ast.ScalarField):
                parameters.append(f"{value}")

            elif isinstance(field, ast.TypedefField) and isinstance(field.type, ast.EnumDeclaration):
                parameters.append(f"{field.type_id}({value})")

            elif isinstance(field, ast.TypedefField):
                (element, intermediate_declarations) = build_packet(field.type, field_var, value)
                declarations.extend(intermediate_declarations)
                parameters.append(element)

            elif isinstance(field, (ast.PayloadField, ast.BodyField)):
                declarations.append(f"std::vector<uint8_t> {field_var} {{")
                for i in range(0, len(value), 16):
                    declarations.append('    ' + ' '.join([f"0x{v:x}," for v in value[i:i + 16]]))
                declarations.append("};")
                parameters.append(f"std::move({field_var})")

            elif isinstance(field, ast.ArrayField) and field.width:
                declarations.append(f"std::vector<{get_cxx_scalar_type(field.width)}> {field_var} {{")
                step = int(16 * 8 / field.width)
                for i in range(0, len(value), step):
                    declarations.append('    ' + ' '.join([f"0x{v:x}," for v in value[i:i + step]]))
                declarations.append("};")
                parameters.append(f"std::move({field_var})")

            elif isinstance(field, ast.ArrayField) and isinstance(field.type, ast.EnumDeclaration):
                declarations.append(f"std::vector<{field.type_id}> {field_var} {{")
                for v in value:
                    declarations.append(f"    {field.type_id}({v}),")
                declarations.append("};")
                parameters.append(f"std::move({field_var})")

            elif isinstance(field, ast.ArrayField):
                elements = []
                for (n, value) in enumerate(value):
                    (element, intermediate_declarations) = build_packet(field.type, f'{field_var}_{n}', value)
                    elements.append(element)
                    declarations.extend(intermediate_declarations)
                declarations.append(f"std::vector<{field.type_id}> {field_var} {{")
                for element in elements:
                    declarations.append(f"    {element},")
                declarations.append("};")
                parameters.append(f"std::move({field_var})")

            else:
                pass

        constructor_name = f'{decl.id}Builder' if isinstance(decl, ast.PacketDeclaration) else decl.id
        return (f"{constructor_name}({', '.join(parameters)})", declarations)

    def output_bytes(output: str) -> List[str]:
        output = bytes.fromhex(output)
        output_bytes = []
        for i in range(0, len(output), 16):
            output_bytes.append(' '.join(f'0x{b:x},' for b in output[i:i + 16]))
        return output_bytes

    generated_tests = []
    for (test_nr, test) in enumerate(tests):
        child_packet_id = test.get('packet', packet.id)
        child_packet = packet.file.packet_scope[child_packet_id]

        (built_packet, intermediate_declarations) = build_packet(child_packet, 'packet', test['unpacked'])
        generated_tests.append(
            dedent("""\

            TEST_F({serializer_test_suite}, {packet_id}_Case{test_nr}) {{
                std::vector<uint8_t> expected_output {{
                    {output_bytes}
                }};
                {intermediate_declarations}
                {child_packet_id}Builder packet = {built_packet};
                ASSERT_EQ(packet.pdl::packet::Builder::Serialize(), expected_output);
            }}
            """).format(serializer_test_suite=serializer_test_suite,
                        packet_id=packet.id,
                        child_packet_id=child_packet_id,
                        test_nr=test_nr,
                        output_bytes=indent(output_bytes(test['packed']), 2),
                        built_packet=built_packet,
                        intermediate_declarations=indent(intermediate_declarations, 1)))

    return ''.join(generated_tests)


def run(input: argparse.FileType, output: argparse.FileType, test_vectors: argparse.FileType, include_header: List[str],
        using_namespace: List[str], namespace: str, parser_test_suite: str, serializer_test_suite: str):

    file = ast.File.from_json(json.load(input))
    tests = json.load(test_vectors)
    core.desugar(file)

    include_header = '\n'.join([f'#include <{header}>' for header in include_header])
    using_namespace = '\n'.join([f'using namespace {namespace};' for namespace in using_namespace])

    skipped_tests = [
        'Packet_Checksum_Field_FromStart',
        'Packet_Checksum_Field_FromEnd',
        'Struct_Checksum_Field_FromStart',
        'Struct_Checksum_Field_FromEnd',
        'PartialParent5',
        'PartialParent12',
    ]

    output.write(
        dedent("""\
        // File generated from {input_name} and {test_vectors_name}, with the command:
        //  {input_command}
        // /!\\ Do not edit by hand

        #include <cstdint>
        #include <string>
        #include <gtest/gtest.h>
        #include <packet_runtime.h>

        {include_header}
        {using_namespace}

        namespace {namespace} {{

        class {parser_test_suite} : public testing::Test {{}};
        class {serializer_test_suite} : public testing::Test {{}};
        """).format(parser_test_suite=parser_test_suite,
                    serializer_test_suite=serializer_test_suite,
                    input_name=input.name,
                    input_command=' '.join(sys.argv),
                    test_vectors_name=test_vectors.name,
                    include_header=include_header,
                    using_namespace=using_namespace,
                    namespace=namespace))

    for decl in file.declarations:
        if decl.id in skipped_tests:
            continue

        if isinstance(decl, ast.PacketDeclaration):
            matching_tests = [test['tests'] for test in tests if test['packet'] == decl.id]
            matching_tests = [test for test_list in matching_tests for test in test_list]
            if matching_tests:
                output.write(generate_packet_parser_test(parser_test_suite, decl, matching_tests))
                output.write(generate_packet_serializer_test(serializer_test_suite, decl, matching_tests))

    output.write(f"}}  // namespace {namespace}\n")


def main() -> int:
    """Generate cxx PDL backend."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--input', type=argparse.FileType('r'), default=sys.stdin, help='Input PDL-JSON source')
    parser.add_argument('--output', type=argparse.FileType('w'), default=sys.stdout, help='Output C++ file')
    parser.add_argument('--test-vectors', type=argparse.FileType('r'), required=True, help='Input PDL test file')
    parser.add_argument('--namespace', type=str, default='pdl', help='Namespace of the generated file')
    parser.add_argument('--parser-test-suite', type=str, default='ParserTest', help='Name of the parser test suite')
    parser.add_argument('--serializer-test-suite',
                        type=str,
                        default='SerializerTest',
                        help='Name of the serializer test suite')
    parser.add_argument('--include-header', type=str, default=[], action='append', help='Added include directives')
    parser.add_argument('--using-namespace',
                        type=str,
                        default=[],
                        action='append',
                        help='Added using namespace statements')
    return run(**vars(parser.parse_args()))


if __name__ == '__main__':
    sys.exit(main())
