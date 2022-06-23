#!/usr/bin/env python3

import argparse
from dataclasses import dataclass, field
import json
from pathlib import Path
import sys
from textwrap import dedent
from typing import List, Tuple, Union, Optional

from pdl import ast, core


def indent(lines: List[str], depth: int) -> str:
    """Indent a code block to the selected depth.
    The first line is intentionally not indented so that
    the caller may use it as:

    '''
    def generated():
        {codeblock}
    '''
    """
    sep = '\n' + (' ' * (depth * 4))
    return sep.join(lines)


def mask(width: int) -> str:
    return hex((1 << width) - 1)


def generate_prelude() -> str:
    return dedent("""\
        from dataclasses import dataclass, field, fields
        from typing import Optional, List, Tuple
        import enum
        import inspect
        import math

        @dataclass
        class Packet:
            payload: Optional[bytes] = field(repr=False)

            @classmethod
            def parse_all(cls, span: bytes) -> 'Packet':
                packet, remain = getattr(cls, 'parse')(span)
                if len(remain) > 0:
                    raise Exception('Unexpected parsing remainder')
                return packet

            def show(self, prefix: str = ''):
                print(f'{self.__class__.__name__}')

                def print_val(p: str, pp: str, name: str, align: int, typ, val):
                    if name == 'payload':
                        pass

                    # Scalar fields.
                    elif typ is int:
                        print(f'{p}{name:{align}} = {val} (0x{val:x})')

                    # Byte fields.
                    elif typ is bytes:
                        print(f'{p}{name:{align}} = [', end='')
                        line = ''
                        n_pp = ''
                        for (idx, b) in enumerate(val):
                            if idx > 0 and idx % 8 == 0:
                                print(f'{n_pp}{line}')
                                line = ''
                                n_pp = pp + (' ' * (align + 4))
                            line += f' {b:02x}'
                        print(f'{n_pp}{line} ]')

                    # Enum fields.
                    elif inspect.isclass(typ) and issubclass(typ, enum.IntEnum):
                        print(f'{p}{name:{align}} = {typ.__name__}::{val.name} (0x{val:x})')

                    # Struct fields.
                    elif inspect.isclass(typ) and issubclass(typ, globals().get('Packet')):
                        print(f'{p}{name:{align}} = ', end='')
                        val.show(prefix=pp)

                    # Array fields.
                    elif typ.__origin__ == list:
                        print(f'{p}{name:{align}}')
                        last = len(val) - 1
                        align = 5
                        for (idx, elt) in enumerate(val):
                            n_p  = pp + ('├── ' if idx != last else '└── ')
                            n_pp = pp + ('│   ' if idx != last else '    ')
                            print_val(n_p, n_pp, f'[{idx}]', align, typ.__args__[0], val[idx])

                    else:
                        print(f'{p}{name:{align}} = ##{typ}##')

                last = len(fields(self)) - 1
                align = max(len(f.name) for f in fields(self) if f.name != 'payload')

                for (idx, f) in enumerate(fields(self)):
                    p  = prefix + ('├── ' if idx != last else '└── ')
                    pp = prefix + ('│   ' if idx != last else '    ')
                    val = getattr(self, f.name)

                    print_val(p, pp, f.name, align, f.type, val)
        """)


@dataclass
class FieldParser:
    byteorder: str
    offset: int = 0
    shift: int = 0
    chunk: List[Tuple[int, int, ast.Field]] = field(default_factory=lambda: [])
    unchecked_code: List[str] = field(default_factory=lambda: [])
    code: List[str] = field(default_factory=lambda: [])

    def unchecked_append_(self, line: str):
        """Append unchecked field parsing code.
        The function check_size_ must be called to generate a size guard
        after parsing is completed."""
        self.unchecked_code.append(line)

    def append_(self, line: str):
        """Append field parsing code.
        There must be no unchecked code left before this function is called."""
        assert len(self.unchecked_code) == 0
        self.code.append(line)

    def check_size_(self, size: str):
        """Generate a check of the current span size."""
        self.append_(f"if len(span) < {size}:")
        self.append_(f"    raise Exception('Invalid packet size')")

    def check_code_(self):
        """Generate a size check for pending field parsing."""
        if len(self.unchecked_code) > 0:
            assert len(self.chunk) == 0
            unchecked_code = self.unchecked_code
            self.unchecked_code = []
            self.check_size_(str(self.offset))
            self.code.extend(unchecked_code)

    def consume_span_(self) -> str:
        """Skip consumed span bytes."""
        if self.offset > 0:
            self.check_code_()
            self.append_(f'span = span[{self.offset}:]')
            self.offset = 0

    def parse_array_element_dynamic_(self, field: ast.ArrayField, span: str):
        """Parse a single array field element of variable size."""
        if isinstance(field.type, ast.StructDeclaration):
            self.append_(f"    element, span = {field.type_id}.parse({span})")
            self.append_(f"    {field.id}.append(element)")
        else:
            raise Exception(f'Unexpected array element type {field.type_id} {field.width}')

    def parse_array_element_static_(self, field: ast.ArrayField, span: str):
        """Parse a single array field element of constant size."""
        if field.width is not None:
            element = f"int.from_bytes({span}, byteorder='{self.byteorder}')"
            self.append_(f"    {field.id}.append({element})")
        elif isinstance(field.type, ast.EnumDeclaration):
            element = f"int.from_bytes({span}, byteorder='{self.byteorder}')"
            element = f"{field.type_id}({element})"
            self.append_(f"    {field.id}.append({element})")
        else:
            element = f"{field.type_id}.parse_all({span})"
            self.append_(f"    {field.id}.append({element})")

    def parse_array_field_(self, field: ast.ArrayField):
        """Parse the selected array field."""
        array_size = core.get_array_field_size(field)
        element_width = core.get_array_element_size(field)
        if element_width:
            if element_width % 8 != 0:
                raise Exception('Array element size is not a multiple of 8')
            element_width = int(element_width / 8)

        if isinstance(array_size, int):
            size = None
            count = array_size
        elif isinstance(array_size, ast.SizeField):
            size = f'{field.id}_size'
            count = None
        elif isinstance(array_size, ast.CountField):
            size = None
            count = f'{field.id}_count'
        else:
            size = None
            count = None

        # Shift the span to reset the offset to 0.
        self.consume_span_()

        # Apply the size modifier.
        if field.size_modifier and size:
            self.append_(f"{size} = {size} {field.size_modifier}")
        if field.size_modifier and count:
            self.append_(f"{count} = {count} {field.size_modifier}")

        # The element width is not known, but the array full octet size
        # is known by size field. Parse elements item by item as a vector.
        if element_width is None and size is not None:
            self.check_size_(size)
            self.append_(f"array_span = span[:{size}]")
            self.append_(f"{field.id} = []")
            self.append_("while len(array_span) > 0:")
            self.parse_array_element_dynamic_(field, 'array_span')
            self.append_(f"fields['{field.id}'] = {field.id}")
            self.append_(f"span = span[{size}:]")

        # The element width is not known, but the array element count
        # is known statically or by count field.
        # Parse elements item by item as a vector.
        elif element_width is None and count is not None:
            self.append_(f"{field.id} = []")
            self.append_(f"for n in range({count}):")
            self.parse_array_element_dynamic_(field, 'span')
            self.append_(f"fields['{field.id}'] = {field.id}")

        # Neither the count not size is known,
        # parse elements until the end of the span.
        elif element_width is None:
            self.append_(f"{field.id} = []")
            self.append_("while len(span) > 0:")
            self.parse_array_element_dynamic_(field, 'span')
            self.append_(f"fields['{field.id}'] = {field.id}")

        # The element width is known, and the array element count is known
        # statically, or by count field.
        elif count is not None:
            array_size = (f'{count}' if element_width == 1 else f'{count} * {element_width}')
            self.check_size_(array_size)
            self.append_(f"{field.id} = []")
            self.append_(f"for n in range({count}):")
            span = ('span[n:n + 1]' if element_width == 1 else f'span[n * {element_width}:(n + 1) * {element_width}]')
            self.parse_array_element_static_(field, span)
            self.append_(f"fields['{field.id}'] = {field.id}")
            self.append_(f"span = span[{array_size}:]")

        # The element width is known, and the array full size is known
        # by size field, or unknown (in which case it is the remaining span
        # length).
        else:
            if size is not None:
                self.check_size_(size)
            array_size = size or 'len(span)'
            array_count = size
            if element_width != 1:
                self.append_(f"if {array_size} % {element_width} != 0:")
                self.append_("    raise Exception('Array size is not a multiple of the element size')")
                self.append_(f"{field.id}_count = int({array_size} / {element_width})")
                array_count = f'{field.id}_count'
            self.append_(f"{field.id} = []")
            self.append_(f"for n in range({array_count}):")
            span = ('span[n:n + 1]' if element_width == 1 else f'span[n * {element_width}:(n + 1) * {element_width}]')
            self.parse_array_element_static_(field, span)
            self.append_(f"fields['{field.id}'] = {field.id}")
            if size is not None:
                self.append_(f"span = span[{size}:]")

    def parse_bit_field_(self, field: ast.Field):
        """Parse the selected field as a bit field.
        The field is added to the current chunk. When a byte boundary
        is reached all saved fields are extracted together."""

        # Add to current chunk.
        width = core.get_field_size(field)
        self.chunk.append((self.shift, width, field))
        self.shift += width

        # Wait for more fields if not on a byte boundary.
        if (self.shift % 8) != 0:
            return

        # Parse the backing integer using the configured endiannes,
        # extract field values.
        size = int(self.shift / 8)
        end_offset = self.offset + size

        if size == 1:
            value = f"span[{self.offset}]"
        else:
            span = f"span[{self.offset}:{end_offset}]"
            self.unchecked_append_(f"value_ = int.from_bytes({span}, byteorder='{self.byteorder}')")
            value = "value_"

        for shift, width, field in self.chunk:
            v = (value if len(self.chunk) == 1 else f"({value} >> {shift}) & {mask(width)}")

            if isinstance(field, ast.ScalarField):
                self.unchecked_append_(f"fields['{field.id}'] = {v}")
            elif isinstance(field, ast.FixedField) and field.enum_id:
                self.unchecked_append_(f"if {v} != {field.enum_id}.{field.tag_id}:")
                self.unchecked_append_(f"    raise Exception('Unexpected fixed field value')")
            elif isinstance(field, ast.FixedField):
                self.unchecked_append_(f"if {v} != {hex(field.value)}:")
                self.unchecked_append_(f"    raise Exception('Unexpected fixed field value')")
            elif isinstance(field, ast.TypedefField):
                self.unchecked_append_(f"fields['{field.id}'] = {field.type_id}({v})")
            elif isinstance(field, ast.SizeField):
                self.unchecked_append_(f"{field.field_id}_size = {v}")
            elif isinstance(field, ast.CountField):
                self.unchecked_append_(f"{field.field_id}_count = {v}")
            elif isinstance(field, ast.ReservedField):
                pass
            else:
                raise Exception(f'Unsupported bit field type {field.kind}')

        # Reset state.
        self.offset = end_offset
        self.shift = 0
        self.chunk = []

    def parse_typedef_field_(self, field: ast.TypedefField):
        """Parse a typedef field, to the exclusion of Enum fields."""

        if self.shift != 0:
            raise Exception('Typedef field does not start on an octet boundary')
        if (isinstance(field.type, ast.StructDeclaration) and field.type.parent_id is not None):
            raise Exception('Derived struct used in typedef field')

        width = core.get_declaration_size(field.type)
        if width is None:
            self.consume_span_()
            self.append_(f"{field.id}, span = {field.type_id}.parse(span)")
            self.append_(f"fields['{field.id}'] = {field.id}")
        else:
            if width % 8 != 0:
                raise Exception('Typedef field type size is not a multiple of 8')
            width = int(width / 8)
            end_offset = self.offset + width
            # Checksum value field is generated alongside checksum start.
            # Deal with this field as padding.
            if not isinstance(field.type, ast.ChecksumDeclaration):
                span = f'span[{self.offset}:{end_offset}]'
                self.unchecked_append_(f"fields['{field.id}'] = {field.type_id}.parse_all({span})")
            self.offset = end_offset

    def parse_padding_field_(self, field: ast.PaddingField):
        """Parse a padding field. The value is ignored."""

        if self.shift != 0:
            raise Exception('Padding field does not start on an octet boundary')
        self.offset += field.width

    def parse_payload_field_(self, field: Union[ast.BodyField, ast.PayloadField]):
        """Parse body and payload fields."""

        size = core.get_payload_field_size(field)
        self.consume_span_()
        if size:
            self.check_size_(f'{field.id}_size')
            self.append_(f"payload = span[:{field.id}_size]")
        else:
            self.append_(f"payload = span")
        self.append_(f"fields['payload'] = payload")

    def parse_checksum_field_(self, field: ast.ChecksumField):
        """Generate a checksum check."""

        # The checksum value field can be read starting from the current
        # offset if the fields in between are of fixed size, or from the end
        # of the span otherwise.
        self.consume_span_()
        value_field = core.get_packet_field(field.parent, field.field_id)
        offset_from_start = 0
        offset_from_end = 0
        start_index = field.parent.fields.index(field)
        value_index = field.parent.fields.index(value_field)
        value_size = int(core.get_field_size(value_field) / 8)

        for f in field.parent.fields[start_index + 1:value_index]:
            size = core.get_field_size(f)
            if size is None:
                offset_from_start = None
                break
            else:
                offset_from_start += size

        trailing_fields = field.parent.fields[value_index:]
        trailing_fields.reverse()
        for f in trailing_fields:
            size = core.get_field_size(f)
            if size is None:
                offset_from_end = None
                break
            else:
                offset_from_end += size

        if offset_from_start is not None:
            if offset_from_start % 8 != 0:
                raise Exception('Checksum value field is not aligned to an octet boundary')
            offset_from_start = int(offset_from_start / 8)
            checksum_span = f'span[:{offset_from_start}]'
            if value_size > 1:
                start = offset_from_start
                end = offset_from_start + value_size
                value = f"int.from_bytes(span[{start}:{end}], byteorder='{self.byteorder}')"
            else:
                value = f'span[{offset_from_start}]'
            self.check_size_(offset_from_start + value_size)

        elif offset_from_end is not None:
            sign = ''
            if offset_from_end % 8 != 0:
                raise Exception('Checksum value field is not aligned to an octet boundary')
            offset_from_end = int(offset_from_end / 8)
            checksum_span = f'span[:-{offset_from_end}]'
            if value_size > 1:
                start = offset_from_end
                end = offset_from_start - value_size
                value = f"int.from_bytes(span[-{start}:-{end}], byteorder='{self.byteorder}')"
            else:
                value = f'span[-{offset_from_end}]'
            self.check_size_(offset_from_end)

        else:
            raise Exception('Checksum value field cannot be read at constant offset')

        self.append_(f"{value_field.id} = {value}")
        self.append_(f"fields['{value_field.id}'] = {value_field.id}")
        self.append_(f"computed_{value_field.id} = {value_field.type.function}({checksum_span})")
        self.append_(f"if computed_{value_field.id} != {value_field.id}:")
        self.append_("    raise Exception(f'Invalid checksum computation:" +
                     f" {{computed_{value_field.id}}} != {{{value_field.id}}}')")

    def parse(self, field: ast.Field):
        # Field has bit granularity.
        # Append the field to the current chunk,
        # check if a byte boundary was reached.
        if core.is_bit_field(field):
            self.parse_bit_field_(field)

        # Padding fields.
        elif isinstance(field, ast.PaddingField):
            self.parse_padding_field_(field)

        # Array fields.
        elif isinstance(field, ast.ArrayField):
            self.parse_array_field_(field)

        # Other typedef fields.
        elif isinstance(field, ast.TypedefField):
            self.parse_typedef_field_(field)

        # Payload and body fields.
        elif (isinstance(field, ast.PayloadField) or isinstance(field, ast.BodyField)):
            self.parse_payload_field_(field)

        # Checksum fields.
        elif isinstance(field, ast.ChecksumField):
            self.parse_checksum_field_(field)

        else:
            raise Exception(f'Unimplemented field type {field.kind}')

    def done(self):
        self.consume_span_()


def generate_toplevel_packet_serializer(packet: ast.Declaration) -> List[str]:
    """Generate the serialize() function for a toplevel Packet or Struct
       declaration."""
    return ["pass"]


def generate_derived_packet_serializer(packet: ast.Declaration) -> List[str]:
    """Generate the serialize() function for a derived Packet or Struct
       declaration."""
    return ["pass"]


def generate_packet_parser(packet: ast.Declaration) -> List[str]:
    """Generate the parse() function for a toplevel Packet or Struct
       declaration."""

    parser = FieldParser(byteorder=packet.grammar.byteorder)
    for f in packet.fields:
        parser.parse(f)
    parser.done()
    children = core.get_derived_packets(packet)
    decl = [] if packet.parent_id else ['fields = {\'payload\': None}']

    if len(children) != 0:
        # Generate dissector on constrained fields, continue parsing the
        # child packets.
        code = decl + parser.code
        op = 'if'
        for constraints, child in children:
            cond = []
            for c in constraints:
                if c.value is not None:
                    cond.append(f"fields['{c.id}'] == {hex(c.value)}")
                else:
                    field = core.get_packet_field(packet, c.id)
                    cond.append(f"fields['{c.id}'] == {field.type_id}.{c.tag_id}")
            cond = ' and '.join(cond)
            code.append(f"{op} {cond}:")
            code.append(f"    return {child.id}.parse(fields, payload)")
            op = 'elif'

        code.append("else:")
        code.append(f"    return {packet.id}(**fields), span")
        return code
    else:
        return decl + parser.code + [f"return {packet.id}(**fields), span"]


def generate_derived_packet_parser(packet: ast.Declaration) -> List[str]:
    """Generate the parse() function for a derived Packet or Struct
       declaration."""
    print(f"Parsing packet {packet.id}", file=sys.stderr)
    parser = FieldParser(byteorder=packet.grammar.byteorder)
    for f in packet.fields:
        parser.parse(f)
    parser.done()
    return parser.code + [f"return {packet.id}(**fields)"]


def generate_enum_declaration(decl: ast.EnumDeclaration) -> str:
    """Generate the implementation of an enum type."""

    enum_name = decl.id
    tag_decls = []
    for t in decl.tags:
        tag_decls.append(f"{t.id} = {hex(t.value)}")

    return dedent("""\

        class {enum_name}(enum.IntEnum):
            {tag_decls}
        """).format(
        enum_name=enum_name, tag_decls=indent(tag_decls, 1))


def generate_packet_declaration(packet: ast.Declaration) -> str:
    """Generate the implementation a toplevel Packet or Struct
       declaration."""

    packet_name = packet.id
    field_decls = []
    for f in packet.fields:
        if isinstance(f, ast.ScalarField):
            field_decls.append(f"{f.id}: int")
        elif isinstance(f, ast.TypedefField):
            field_decls.append(f"{f.id}: {f.type_id}")
        elif isinstance(f, ast.ArrayField) and f.width == 8:
            field_decls.append(f"{f.id}: bytes")
        elif isinstance(f, ast.ArrayField) and f.width:
            field_decls.append(f"{f.id}: List[int]")
        elif isinstance(f, ast.ArrayField) and f.type_id:
            field_decls.append(f"{f.id}: List[{f.type_id}]")

    if packet.parent_id:
        parent_name = packet.parent_id
        parent_fields = 'fields: dict, '
        serializer = generate_derived_packet_serializer(packet)
    else:
        parent_name = 'Packet'
        parent_fields = ''
        serializer = generate_toplevel_packet_serializer(packet)

    parser = generate_packet_parser(packet)

    return dedent("""\

        @dataclass
        class {packet_name}({parent_name}):
            {field_decls}

            @staticmethod
            def parse({parent_fields}span: bytes) -> Tuple['{packet_name}', bytes]:
                {parser}

            def serialize(self) -> bytes:
                {serializer}
        """).format(
        packet_name=packet_name,
        parent_name=parent_name,
        parent_fields=parent_fields,
        field_decls=indent(field_decls, 1),
        parser=indent(parser, 2),
        serializer=indent(serializer, 2))


def generate_custom_field_declaration_check(decl: ast.CustomFieldDeclaration) -> str:
    """Generate the code to validate a user custom field implementation.

    This code is to be executed when the generated module is loaded to ensure
    the user gets an immediate and clear error message when the provided
    custom types do not fit the expected template.
    """
    return dedent("""\

        if (not callable(getattr({custom_field_name}, 'parse', None)) or
            not callable(getattr({custom_field_name}, 'parse_all', None))):
            raise Exception('The custom field type {custom_field_name} does not implement the parse method')
    """).format(custom_field_name=decl.id)


def generate_checksum_declaration_check(decl: ast.ChecksumDeclaration) -> str:
    """Generate the code to validate a user checksum field implementation.

    This code is to be executed when the generated module is loaded to ensure
    the user gets an immediate and clear error message when the provided
    checksum functions do not fit the expected template.
    """
    return dedent("""\

        if not callable({checksum_name}):
            raise Exception('{checksum_name} is not callable')
    """).format(checksum_name=decl.id)


def run(input: argparse.FileType, output: argparse.FileType, custom_type_location: Optional[str]):
    #    with open(input) as pdl_json:
    grammar = ast.Grammar.from_json(json.load(input))

    core.desugar(grammar)

    custom_types = []
    custom_type_checks = ""
    for d in grammar.declarations:
        if isinstance(d, ast.CustomFieldDeclaration):
            custom_types.append(d.id)
            custom_type_checks += generate_custom_field_declaration_check(d)
        elif isinstance(d, ast.ChecksumDeclaration):
            custom_types.append(d.id)
            custom_type_checks += generate_checksum_declaration_check(d)

    output.write(f"# File generated from {input.name}, with the command:\n")
    output.write(f"#  {' '.join(sys.argv)}\n")
    output.write("# /!\\ Do not edit by hand.\n")
    if custom_types and custom_type_location:
        output.write(f"\nfrom {custom_type_location} import {', '.join(custom_types)}\n")
    output.write(generate_prelude())
    output.write(custom_type_checks)

    for d in grammar.declarations:
        if isinstance(d, ast.EnumDeclaration):
            output.write(generate_enum_declaration(d))
        elif isinstance(d, (ast.PacketDeclaration, ast.StructDeclaration)):
            output.write(generate_packet_declaration(d))


def main() -> int:
    """Generate python PDL backend."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--input', type=argparse.FileType('r'), default=sys.stdin, help='Input PDL-JSON source')
    parser.add_argument('--output', type=argparse.FileType('w'), default=sys.stdout, help='Output Python file')
    parser.add_argument(
        '--custom-type-location', type=str, required=False, help='Module of declaration of custom types')
    return run(**vars(parser.parse_args()))


if __name__ == '__main__':
    sys.exit(main())
