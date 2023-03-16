#!/usr/bin/env python3

import argparse
from dataclasses import dataclass, field
import json
from pathlib import Path
import sys
from textwrap import dedent
from typing import List, Tuple, Union, Optional

from pdl import ast, core
from pdl.utils import indent


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
            payload: Optional[bytes] = field(repr=False, default_factory=bytes, compare=False)

            @classmethod
            def parse_all(cls, span: bytes) -> 'Packet':
                packet, remain = getattr(cls, 'parse')(span)
                if len(remain) > 0:
                    raise Exception('Unexpected parsing remainder')
                return packet

            @property
            def size(self) -> int:
                pass

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
                    elif getattr(typ, '__origin__', None) == list:
                        print(f'{p}{name:{align}}')
                        last = len(val) - 1
                        align = 5
                        for (idx, elt) in enumerate(val):
                            n_p  = pp + ('├── ' if idx != last else '└── ')
                            n_pp = pp + ('│   ' if idx != last else '    ')
                            print_val(n_p, n_pp, f'[{idx}]', align, typ.__args__[0], val[idx])

                    # Custom fields.
                    elif inspect.isclass(typ):
                        print(f'{p}{name:{align}} = {repr(val)}')

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

    def consume_span_(self, keep: int = 0) -> str:
        """Skip consumed span bytes."""
        if self.offset > 0:
            self.check_code_()
            self.append_(f'span = span[{self.offset - keep}:]')
            self.offset = 0

    def parse_array_element_dynamic_(self, field: ast.ArrayField, span: str):
        """Parse a single array field element of variable size."""
        if isinstance(field.type, ast.StructDeclaration):
            self.append_(f"    element, {span} = {field.type_id}.parse({span})")
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

    def parse_byte_array_field_(self, field: ast.ArrayField):
        """Parse the selected u8 array field."""
        array_size = core.get_array_field_size(field)
        padded_size = field.padded_size

        # Shift the span to reset the offset to 0.
        self.consume_span_()

        # Derive the array size.
        if isinstance(array_size, int):
            size = array_size
        elif isinstance(array_size, ast.SizeField):
            size = f'{field.id}_size - {field.size_modifier}' if field.size_modifier else f'{field.id}_size'
        elif isinstance(array_size, ast.CountField):
            size = f'{field.id}_count'
        else:
            size = None

        # Parse from the padded array if padding is present.
        if padded_size and size is not None:
            self.check_size_(padded_size)
            self.append_(f"if {size} > {padded_size}:")
            self.append_("    raise Exception('Array size is larger than the padding size')")
            self.append_(f"fields['{field.id}'] = list(span[:{size}])")
            self.append_(f"span = span[{padded_size}:]")

        elif size is not None:
            self.check_size_(size)
            self.append_(f"fields['{field.id}'] = list(span[:{size}])")
            self.append_(f"span = span[{size}:]")

        else:
            self.append_(f"fields['{field.id}'] = list(span)")
            self.append_(f"span = bytes()")

    def parse_array_field_(self, field: ast.ArrayField):
        """Parse the selected array field."""
        array_size = core.get_array_field_size(field)
        element_width = core.get_array_element_size(field)
        padded_size = field.padded_size

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
            self.append_(f"{size} = {size} - {field.size_modifier}")

        # Parse from the padded array if padding is present.
        if padded_size:
            self.check_size_(padded_size)
            self.append_(f"remaining_span = span[{padded_size}:]")
            self.append_(f"span = span[:{padded_size}]")

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
            if element_width != 1:
                self.append_(f"if {array_size} % {element_width} != 0:")
                self.append_("    raise Exception('Array size is not a multiple of the element size')")
                self.append_(f"{field.id}_count = int({array_size} / {element_width})")
                array_count = f'{field.id}_count'
            else:
                array_count = array_size
            self.append_(f"{field.id} = []")
            self.append_(f"for n in range({array_count}):")
            span = ('span[n:n + 1]' if element_width == 1 else f'span[n * {element_width}:(n + 1) * {element_width}]')
            self.parse_array_element_static_(field, span)
            self.append_(f"fields['{field.id}'] = {field.id}")
            if size is not None:
                self.append_(f"span = span[{size}:]")
            else:
                self.append_(f"span = bytes()")

        # Drop the padding
        if padded_size:
            self.append_(f"span = remaining_span")

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
            v = (value if len(self.chunk) == 1 and shift == 0 else f"({value} >> {shift}) & {mask(width)}")

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

    def parse_payload_field_(self, field: Union[ast.BodyField, ast.PayloadField]):
        """Parse body and payload fields."""

        payload_size = core.get_payload_field_size(field)
        offset_from_end = core.get_field_offset_from_end(field)

        # If the payload is not byte aligned, do parse the bit fields
        # that can be extracted, but do not consume the input bytes as
        # they will also be included in the payload span.
        if self.shift != 0:
            if payload_size:
                raise Exception("Unexpected payload size for non byte aligned payload")

            rounded_size = int((self.shift + 7) / 8)
            padding_bits = 8 * rounded_size - self.shift
            self.parse_bit_field_(core.make_reserved_field(padding_bits))
            self.consume_span_(rounded_size)
        else:
            self.consume_span_()

        # The payload or body has a known size.
        # Consume the payload and update the span in case
        # fields are placed after the payload.
        if payload_size:
            if getattr(field, 'size_modifier', None):
                self.append_(f"{field.id}_size -= {field.size_modifier}")
            self.check_size_(f'{field.id}_size')
            self.append_(f"payload = span[:{field.id}_size]")
            self.append_(f"span = span[{field.id}_size:]")
        # The payload or body is the last field of a packet,
        # consume the remaining span.
        elif offset_from_end == 0:
            self.append_(f"payload = span")
            self.append_(f"span = bytes([])")
        # The payload or body is followed by fields of static size.
        # Consume the span that is not reserved for the following fields.
        elif offset_from_end is not None:
            if (offset_from_end % 8) != 0:
                raise Exception('Payload field offset from end of packet is not a multiple of 8')
            offset_from_end = int(offset_from_end / 8)
            self.check_size_(f'{offset_from_end}')
            self.append_(f"payload = span[:-{offset_from_end}]")
            self.append_(f"span = span[-{offset_from_end}:]")
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
                end = offset_from_end - value_size
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
            pass

        # Array fields.
        elif isinstance(field, ast.ArrayField) and field.width == 8:
            self.parse_byte_array_field_(field)

        elif isinstance(field, ast.ArrayField):
            self.parse_array_field_(field)

        # Other typedef fields.
        elif isinstance(field, ast.TypedefField):
            self.parse_typedef_field_(field)

        # Payload and body fields.
        elif isinstance(field, (ast.PayloadField, ast.BodyField)):
            self.parse_payload_field_(field)

        # Checksum fields.
        elif isinstance(field, ast.ChecksumField):
            self.parse_checksum_field_(field)

        else:
            raise Exception(f'Unimplemented field type {field.kind}')

    def done(self):
        self.consume_span_()


@dataclass
class FieldSerializer:
    byteorder: str
    shift: int = 0
    value: List[str] = field(default_factory=lambda: [])
    code: List[str] = field(default_factory=lambda: [])
    indent: int = 0

    def indent_(self):
        self.indent += 1

    def unindent_(self):
        self.indent -= 1

    def append_(self, line: str):
        """Append field serializing code."""
        lines = line.split('\n')
        self.code.extend(['    ' * self.indent + line for line in lines])

    def extend_(self, value: str, length: int):
        """Append data to the span being constructed."""
        if length == 1:
            self.append_(f"_span.append({value})")
        else:
            self.append_(f"_span.extend(int.to_bytes({value}, length={length}, byteorder='{self.byteorder}'))")

    def serialize_array_element_(self, field: ast.ArrayField):
        """Serialize a single array field element."""
        if field.width is not None:
            length = int(field.width / 8)
            self.extend_('_elt', length)
        elif isinstance(field.type, ast.EnumDeclaration):
            length = int(field.type.width / 8)
            self.extend_('_elt', length)
        else:
            self.append_("_span.extend(_elt.serialize())")

    def serialize_array_field_(self, field: ast.ArrayField):
        """Serialize the selected array field."""
        if field.padded_size:
            self.append_(f"_{field.id}_start = len(_span)")

        if field.width == 8:
            self.append_(f"_span.extend(self.{field.id})")
        else:
            self.append_(f"for _elt in self.{field.id}:")
            self.indent_()
            self.serialize_array_element_(field)
            self.unindent_()

        if field.padded_size:
            self.append_(f"_span.extend([0] * ({field.padded_size} - len(_span) + _{field.id}_start))")

    def serialize_bit_field_(self, field: ast.Field):
        """Serialize the selected field as a bit field.
        The field is added to the current chunk. When a byte boundary
        is reached all saved fields are serialized together."""

        # Add to current chunk.
        width = core.get_field_size(field)
        shift = self.shift

        if isinstance(field, str):
            self.value.append(f"({field} << {shift})")
        elif isinstance(field, ast.ScalarField):
            max_value = (1 << field.width) - 1
            self.append_(f"if self.{field.id} > {max_value}:")
            self.append_(f"    print(f\"Invalid value for field {field.parent.id}::{field.id}:" +
                         f" {{self.{field.id}}} > {max_value}; the value will be truncated\")")
            self.append_(f"    self.{field.id} &= {max_value}")
            self.value.append(f"(self.{field.id} << {shift})")
        elif isinstance(field, ast.FixedField) and field.enum_id:
            self.value.append(f"({field.enum_id}.{field.tag_id} << {shift})")
        elif isinstance(field, ast.FixedField):
            self.value.append(f"({field.value} << {shift})")
        elif isinstance(field, ast.TypedefField):
            self.value.append(f"(self.{field.id} << {shift})")

        elif isinstance(field, ast.SizeField):
            max_size = (1 << field.width) - 1
            value_field = core.get_packet_field(field.parent, field.field_id)
            size_modifier = ''

            if getattr(value_field, 'size_modifier', None):
                size_modifier = f' + {value_field.size_modifier}'

            if isinstance(value_field, (ast.PayloadField, ast.BodyField)):
                self.append_(f"_payload_size = len(payload or self.payload or []){size_modifier}")
                self.append_(f"if _payload_size > {max_size}:")
                self.append_(f"    print(f\"Invalid length for payload field:" +
                             f"  {{_payload_size}} > {max_size}; the packet cannot be generated\")")
                self.append_(f"    raise Exception(\"Invalid payload length\")")
                array_size = "_payload_size"
            elif isinstance(value_field, ast.ArrayField) and value_field.width:
                array_size = f"(len(self.{value_field.id}) * {int(value_field.width / 8)}{size_modifier})"
            elif isinstance(value_field, ast.ArrayField) and isinstance(value_field.type, ast.EnumDeclaration):
                array_size = f"(len(self.{value_field.id}) * {int(value_field.type.width / 8)}{size_modifier})"
            elif isinstance(value_field, ast.ArrayField):
                self.append_(
                    f"_{value_field.id}_size = sum([elt.size for elt in self.{value_field.id}]){size_modifier}")
                array_size = f"_{value_field.id}_size"
            else:
                raise Exception("Unsupported field type")
            self.value.append(f"({array_size} << {shift})")

        elif isinstance(field, ast.CountField):
            max_count = (1 << field.width) - 1
            self.append_(f"if len(self.{field.field_id}) > {max_count}:")
            self.append_(f"    print(f\"Invalid length for field {field.parent.id}::{field.field_id}:" +
                         f"  {{len(self.{field.field_id})}} > {max_count}; the array will be truncated\")")
            self.append_(f"    del self.{field.field_id}[{max_count}:]")
            self.value.append(f"(len(self.{field.field_id}) << {shift})")
        elif isinstance(field, ast.ReservedField):
            pass
        else:
            raise Exception(f'Unsupported bit field type {field.kind}')

        # Check if a byte boundary is reached.
        self.shift += width
        if (self.shift % 8) == 0:
            self.pack_bit_fields_()

    def pack_bit_fields_(self):
        """Pack serialized bit fields."""

        # Should have an integral number of bytes now.
        assert (self.shift % 8) == 0

        # Generate the backing integer, and serialize it
        # using the configured endiannes,
        size = int(self.shift / 8)

        if len(self.value) == 0:
            self.append_(f"_span.extend([0] * {size})")
        elif len(self.value) == 1:
            self.extend_(self.value[0], size)
        else:
            self.append_(f"_value = (")
            self.append_("    " + " |\n    ".join(self.value))
            self.append_(")")
            self.extend_('_value', size)

        # Reset state.
        self.shift = 0
        self.value = []

    def serialize_typedef_field_(self, field: ast.TypedefField):
        """Serialize a typedef field, to the exclusion of Enum fields."""

        if self.shift != 0:
            raise Exception('Typedef field does not start on an octet boundary')
        if (isinstance(field.type, ast.StructDeclaration) and field.type.parent_id is not None):
            raise Exception('Derived struct used in typedef field')

        if isinstance(field.type, ast.ChecksumDeclaration):
            size = int(field.type.width / 8)
            self.append_(f"_checksum = {field.type.function}(_span[_checksum_start:])")
            self.extend_('_checksum', size)
        else:
            self.append_(f"_span.extend(self.{field.id}.serialize())")

    def serialize_payload_field_(self, field: Union[ast.BodyField, ast.PayloadField]):
        """Serialize body and payload fields."""

        if self.shift != 0 and self.byteorder == 'big':
            raise Exception('Payload field does not start on an octet boundary')

        if self.shift == 0:
            self.append_(f"_span.extend(payload or self.payload or [])")
        else:
            # Supported case of packet inheritance;
            # the incomplete fields are serialized into
            # the payload, rather than separately.
            # First extract the padding bits from the payload,
            # then recombine them with the bit fields to be serialized.
            rounded_size = int((self.shift + 7) / 8)
            padding_bits = 8 * rounded_size - self.shift
            self.append_(f"_payload = payload or self.payload or bytes()")
            self.append_(f"if len(_payload) < {rounded_size}:")
            self.append_(f"    raise Exception(f\"Invalid length for payload field:" +
                         f"  {{len(_payload)}} < {rounded_size}\")")
            self.append_(
                f"_padding = int.from_bytes(_payload[:{rounded_size}], byteorder='{self.byteorder}') >> {self.shift}")
            self.value.append(f"(_padding << {self.shift})")
            self.shift += padding_bits
            self.pack_bit_fields_()
            self.append_(f"_span.extend(_payload[{rounded_size}:])")

    def serialize_checksum_field_(self, field: ast.ChecksumField):
        """Generate a checksum check."""

        self.append_("_checksum_start = len(_span)")

    def serialize(self, field: ast.Field):
        # Field has bit granularity.
        # Append the field to the current chunk,
        # check if a byte boundary was reached.
        if core.is_bit_field(field):
            self.serialize_bit_field_(field)

        # Padding fields.
        elif isinstance(field, ast.PaddingField):
            pass

        # Array fields.
        elif isinstance(field, ast.ArrayField):
            self.serialize_array_field_(field)

        # Other typedef fields.
        elif isinstance(field, ast.TypedefField):
            self.serialize_typedef_field_(field)

        # Payload and body fields.
        elif isinstance(field, (ast.PayloadField, ast.BodyField)):
            self.serialize_payload_field_(field)

        # Checksum fields.
        elif isinstance(field, ast.ChecksumField):
            self.serialize_checksum_field_(field)

        else:
            raise Exception(f'Unimplemented field type {field.kind}')


def generate_toplevel_packet_serializer(packet: ast.Declaration) -> List[str]:
    """Generate the serialize() function for a toplevel Packet or Struct
       declaration."""

    serializer = FieldSerializer(byteorder=packet.file.byteorder)
    for f in packet.fields:
        serializer.serialize(f)
    return ['_span = bytearray()'] + serializer.code + ['return bytes(_span)']


def generate_derived_packet_serializer(packet: ast.Declaration) -> List[str]:
    """Generate the serialize() function for a derived Packet or Struct
       declaration."""

    packet_shift = core.get_packet_shift(packet)
    if packet_shift and packet.file.byteorder == 'big':
        raise Exception(f"Big-endian packet {packet.id} has an unsupported body shift")

    serializer = FieldSerializer(byteorder=packet.file.byteorder, shift=packet_shift)
    for f in packet.fields:
        serializer.serialize(f)
    return ['_span = bytearray()'
           ] + serializer.code + [f'return {packet.parent.id}.serialize(self, payload = bytes(_span))']


def generate_packet_parser(packet: ast.Declaration) -> List[str]:
    """Generate the parse() function for a toplevel Packet or Struct
       declaration."""

    packet_shift = core.get_packet_shift(packet)
    if packet_shift and packet.file.byteorder == 'big':
        raise Exception(f"Big-endian packet {packet.id} has an unsupported body shift")

    # Convert the packet constraints to a boolean expression.
    validation = []
    if packet.constraints:
        cond = []
        for c in packet.constraints:
            if c.value is not None:
                cond.append(f"fields['{c.id}'] != {hex(c.value)}")
            else:
                field = core.get_packet_field(packet, c.id)
                cond.append(f"fields['{c.id}'] != {field.type_id}.{c.tag_id}")

        validation = [f"if {' or '.join(cond)}:", "    raise Exception(\"Invalid constraint field values\")"]

    # Parse fields iteratively.
    parser = FieldParser(byteorder=packet.file.byteorder, shift=packet_shift)
    for f in packet.fields:
        parser.parse(f)
    parser.done()

    # Specialize to child packets.
    children = core.get_derived_packets(packet)
    decl = [] if packet.parent_id else ['fields = {\'payload\': None}']
    specialization = []

    if len(children) != 0:
        # Try parsing every child packet successively until one is
        # successfully parsed. Return a parsing error if none is valid.
        # Return parent packet if no child packet matches.
        # TODO: order child packets by decreasing size in case no constraint
        # is given for specialization.
        for _, child in children:
            specialization.append("try:")
            specialization.append(f"    return {child.id}.parse(fields.copy(), payload)")
            specialization.append("except Exception as exn:")
            specialization.append("    pass")

    return decl + validation + parser.code + specialization + [f"return {packet.id}(**fields), span"]


def generate_packet_size_getter(packet: ast.Declaration) -> List[str]:
    constant_width = 0
    variable_width = []
    for f in packet.fields:
        field_size = core.get_field_size(f)
        if field_size is not None:
            constant_width += field_size
        elif isinstance(f, (ast.PayloadField, ast.BodyField)):
            variable_width.append("len(self.payload)")
        elif isinstance(f, ast.TypedefField):
            variable_width.append(f"self.{f.id}.size")
        elif isinstance(f, ast.ArrayField) and isinstance(f.type, (ast.StructDeclaration, ast.CustomFieldDeclaration)):
            variable_width.append(f"sum([elt.size for elt in self.{f.id}])")
        elif isinstance(f, ast.ArrayField) and isinstance(f.type, ast.EnumDeclaration):
            variable_width.append(f"len(self.{f.id}) * {f.type.width}")
        elif isinstance(f, ast.ArrayField):
            variable_width.append(f"len(self.{f.id}) * {int(f.width / 8)}")
        else:
            raise Exception("Unsupported field type")

    constant_width = int(constant_width / 8)
    if len(variable_width) == 0:
        return [f"return {constant_width}"]
    elif len(variable_width) == 1 and constant_width:
        return [f"return {variable_width[0]} + {constant_width}"]
    elif len(variable_width) == 1:
        return [f"return {variable_width[0]}"]
    elif len(variable_width) > 1 and constant_width:
        return ([f"return {constant_width} + ("] + " +\n    ".join(variable_width).split("\n") + [")"])
    elif len(variable_width) > 1:
        return (["return ("] + " +\n    ".join(variable_width).split("\n") + [")"])
    else:
        assert False


def generate_packet_post_init(decl: ast.Declaration) -> List[str]:
    """Generate __post_init__ function to set constraint field values."""

    # Gather all constraints from parent packets.
    constraints = []
    current = decl
    while current.parent_id:
        constraints.extend(current.constraints)
        current = current.parent

    if constraints:
        code = []
        for c in constraints:
            if c.value is not None:
                code.append(f"self.{c.id} = {c.value}")
            else:
                field = core.get_packet_field(decl, c.id)
                code.append(f"self.{c.id} = {field.type_id}.{c.tag_id}")
        return code

    else:
        return ["pass"]


def generate_enum_declaration(decl: ast.EnumDeclaration) -> str:
    """Generate the implementation of an enum type."""

    enum_name = decl.id
    tag_decls = []
    for t in decl.tags:
        tag_decls.append(f"{t.id} = {hex(t.value)}")

    return dedent("""\

        class {enum_name}(enum.IntEnum):
            {tag_decls}
        """).format(enum_name=enum_name, tag_decls=indent(tag_decls, 1))


def generate_packet_declaration(packet: ast.Declaration) -> str:
    """Generate the implementation a toplevel Packet or Struct
       declaration."""

    packet_name = packet.id
    field_decls = []
    for f in packet.fields:
        if isinstance(f, ast.ScalarField):
            field_decls.append(f"{f.id}: int = field(kw_only=True, default=0)")
        elif isinstance(f, ast.TypedefField):
            if isinstance(f.type, ast.EnumDeclaration):
                field_decls.append(
                    f"{f.id}: {f.type_id} = field(kw_only=True, default={f.type_id}.{f.type.tags[0].id})")
            elif isinstance(f.type, ast.ChecksumDeclaration):
                field_decls.append(f"{f.id}: int = field(kw_only=True, default=0)")
            elif isinstance(f.type, (ast.StructDeclaration, ast.CustomFieldDeclaration)):
                field_decls.append(f"{f.id}: {f.type_id} = field(kw_only=True, default_factory={f.type_id})")
            else:
                raise Exception("Unsupported typedef field type")
        elif isinstance(f, ast.ArrayField) and f.width == 8:
            field_decls.append(f"{f.id}: bytearray = field(kw_only=True, default_factory=bytearray)")
        elif isinstance(f, ast.ArrayField) and f.width:
            field_decls.append(f"{f.id}: List[int] = field(kw_only=True, default_factory=list)")
        elif isinstance(f, ast.ArrayField) and f.type_id:
            field_decls.append(f"{f.id}: List[{f.type_id}] = field(kw_only=True, default_factory=list)")

    if packet.parent_id:
        parent_name = packet.parent_id
        parent_fields = 'fields: dict, '
        serializer = generate_derived_packet_serializer(packet)
    else:
        parent_name = 'Packet'
        parent_fields = ''
        serializer = generate_toplevel_packet_serializer(packet)

    parser = generate_packet_parser(packet)
    size = generate_packet_size_getter(packet)
    post_init = generate_packet_post_init(packet)

    return dedent("""\

        @dataclass
        class {packet_name}({parent_name}):
            {field_decls}

            def __post_init__(self):
                {post_init}

            @staticmethod
            def parse({parent_fields}span: bytes) -> Tuple['{packet_name}', bytes]:
                {parser}

            def serialize(self, payload: bytes = None) -> bytes:
                {serializer}

            @property
            def size(self) -> int:
                {size}
        """).format(packet_name=packet_name,
                    parent_name=parent_name,
                    parent_fields=parent_fields,
                    field_decls=indent(field_decls, 1),
                    post_init=indent(post_init, 2),
                    parser=indent(parser, 2),
                    serializer=indent(serializer, 2),
                    size=indent(size, 2))


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
    file = ast.File.from_json(json.load(input))
    core.desugar(file)

    custom_types = []
    custom_type_checks = ""
    for d in file.declarations:
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

    for d in file.declarations:
        if isinstance(d, ast.EnumDeclaration):
            output.write(generate_enum_declaration(d))
        elif isinstance(d, (ast.PacketDeclaration, ast.StructDeclaration)):
            output.write(generate_packet_declaration(d))


def main() -> int:
    """Generate python PDL backend."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--input', type=argparse.FileType('r'), default=sys.stdin, help='Input PDL-JSON source')
    parser.add_argument('--output', type=argparse.FileType('w'), default=sys.stdout, help='Output Python file')
    parser.add_argument('--custom-type-location',
                        type=str,
                        required=False,
                        help='Module of declaration of custom types')
    return run(**vars(parser.parse_args()))


if __name__ == '__main__':
    sys.exit(main())
