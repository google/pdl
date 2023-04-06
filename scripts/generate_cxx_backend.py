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


def mask(width: int) -> str:
    return hex((1 << width) - 1)


def deref(var: Optional[str], id: str) -> str:
    return f'{var}.{id}' if var else id


def get_cxx_scalar_type(width: int) -> str:
    """Return the cxx scalar type to be used to back a PDL type."""
    for n in [8, 16, 32, 64]:
        if width <= n:
            return f'uint{n}_t'
    # PDL type does not fit on non-extended scalar types.
    assert False


@dataclass
class FieldParser:
    byteorder: str
    offset: int = 0
    shift: int = 0
    extract_arrays: bool = field(default=False)
    chunk: List[Tuple[int, int, ast.Field]] = field(default_factory=lambda: [])
    chunk_nr: int = 0
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
        self.append_(f"if (span.size() < {size}) {{")
        self.append_("    return false;")
        self.append_("}")

    def check_code_(self):
        """Generate a size check for pending field parsing."""
        if len(self.unchecked_code) > 0:
            assert len(self.chunk) == 0
            unchecked_code = self.unchecked_code
            self.unchecked_code = []
            self.check_size_(str(self.offset))
            self.code.extend(unchecked_code)
            self.offset = 0

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

        # Parse the backing integer using the configured endianness,
        # extract field values.
        size = int(self.shift / 8)
        backing_type = get_cxx_scalar_type(self.shift)

        # Special case when no field is actually used from
        # the chunk.
        should_skip_value = all(isinstance(field, ast.ReservedField) for (_, _, field) in self.chunk)
        if should_skip_value:
            self.unchecked_append_(f"span.skip({size}); // skip reserved fields")
            self.offset += size
            self.shift = 0
            self.chunk = []
            return

        if len(self.chunk) > 1:
            value = f"chunk{self.chunk_nr}"
            self.unchecked_append_(f"{backing_type} {value} = span.read_{self.byteorder}<{backing_type}, {size}>();")
            self.chunk_nr += 1
        else:
            value = f"span.read_{self.byteorder}<{backing_type}, {size}>()"

        for shift, width, field in self.chunk:
            v = (value if len(self.chunk) == 1 and shift == 0 else f"({value} >> {shift}) & {mask(width)}")

            if isinstance(field, ast.ScalarField):
                self.unchecked_append_(f"{field.id}_ = {v};")
            elif isinstance(field, ast.FixedField) and field.enum_id:
                self.unchecked_append_(f"if ({field.enum_id}({v}) != {field.enum_id}::{field.tag_id}) {{")
                self.unchecked_append_("    return false;")
                self.unchecked_append_("}")
            elif isinstance(field, ast.FixedField):
                self.unchecked_append_(f"if (({v}) != {hex(field.value)}) {{")
                self.unchecked_append_("    return false;")
                self.unchecked_append_("}")
            elif isinstance(field, ast.TypedefField):
                self.unchecked_append_(f"{field.id}_ = {field.type_id}({v});")
            elif isinstance(field, ast.SizeField):
                self.unchecked_append_(f"{field.field_id}_size = {v};")
            elif isinstance(field, ast.CountField):
                self.unchecked_append_(f"{field.field_id}_count = {v};")
            elif isinstance(field, ast.ReservedField):
                pass
            else:
                raise Exception(f'Unsupported bit field type {field.kind}')

        # Reset state.
        self.offset += size
        self.shift = 0
        self.chunk = []

    def parse_typedef_field_(self, field: ast.TypedefField):
        """Parse a typedef field, to the exclusion of Enum fields."""
        if self.shift != 0:
            raise Exception('Typedef field does not start on an octet boundary')

        self.check_code_()
        self.append_(
            dedent("""\
            if (!{field_type}::Parse(span, &{field_id}_)) {{
                return false;
            }}""".format(field_type=field.type.id, field_id=field.id)))

    def parse_array_field_lite_(self, field: ast.ArrayField):
        """Parse the selected array field.
        This function does not attempt to parse all elements but just to
        identify the span of the array."""
        array_size = core.get_array_field_size(field)
        element_width = core.get_array_element_size(field)
        padded_size = field.padded_size

        if element_width:
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
        self.check_code_()

        # Apply the size modifier.
        if field.size_modifier and size:
            self.append_(f"{size} = {size} - {field.size_modifier};")

        # Compute the array size if the count and element width are known.
        if count is not None and element_width is not None:
            size = f"{count} * {element_width}"

        # Parse from the padded array if padding is present.
        if padded_size:
            self.check_size_(padded_size)
            self.append_("{")
            self.append_(
                f"pdl::packet::slice remaining_span = span.subrange({padded_size}, span.size() - {padded_size});")
            self.append_(f"span = span.subrange(0, {padded_size});")

        # The array size is known in bytes.
        if size is not None:
            self.check_size_(size)
            self.append_(f"{field.id}_ = span.subrange(0, {size});")
            self.append_(f"span.skip({size});")

        # The array count is known. The element width is dynamic.
        # Parse each element iteratively and derive the array span.
        elif count is not None:
            self.append_("{")
            self.append_("pdl::packet::slice temp_span = span;")
            self.append_(f"for (size_t n = 0; n < {count}; n++) {{")
            self.append_(f"    {field.type_id} element;")
            self.append_(f"    if (!{field.type_id}::Parse(temp_span, &element)) {{")
            self.append_("        return false;")
            self.append_("    }")
            self.append_("}")
            self.append_(f"{field.id}_ = span.subrange(0, span.size() - temp_span.size());")
            self.append_(f"span.skip({field.id}_.size());")
            self.append_("}")

        # The array size is not known, assume the array takes the
        # full remaining space. TODO support having fixed sized fields
        # following the array.
        else:
            self.append_(f"{field.id}_ = span;")
            self.append_("span.clear();")

        if padded_size:
            self.append_(f"span = remaining_span;")
            self.append_("}")

    def parse_array_field_full_(self, field: ast.ArrayField):
        """Parse the selected array field.
        This function does not attempt to parse all elements but just to
        identify the span of the array."""
        array_size = core.get_array_field_size(field)
        element_width = core.get_array_element_size(field)
        element_type = field.type_id or get_cxx_scalar_type(field.width)
        padded_size = field.padded_size

        if element_width:
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
        self.check_code_()

        # Apply the size modifier.
        if field.size_modifier and size:
            self.append_(f"{size} = {size} - {field.size_modifier};")

        # Compute the array size if the count and element width are known.
        if count is not None and element_width is not None:
            size = f"{count} * {element_width}"

        # Parse from the padded array if padding is present.
        if padded_size:
            self.check_size_(padded_size)
            self.append_("{")
            self.append_(
                f"pdl::packet::slice remaining_span = span.subrange({padded_size}, span.size() - {padded_size});")
            self.append_(f"span = span.subrange(0, {padded_size});")

        # The array size is known in bytes.
        if size is not None:
            self.check_size_(size)
            self.append_("{")
            self.append_(f"pdl::packet::slice temp_span = span.subrange(0, {size});")
            self.append_(f"span.skip({size});")
            self.append_(f"while (temp_span.size() > 0) {{")
            if field.width:
                element_size = int(field.width / 8)
                self.append_(f"    if (temp_span.size() < {element_size}) {{")
                self.append_(f"        return false;")
                self.append_("    }")
                self.append_(
                    f"    {field.id}_.push_back(temp_span.read_{self.byteorder}<{element_type}, {element_size}>());")
            elif isinstance(field.type, ast.EnumDeclaration):
                backing_type = get_cxx_scalar_type(field.type.width)
                element_size = int(field.type.width / 8)
                self.append_(f"    if (temp_span.size() < {element_size}) {{")
                self.append_(f"        return false;")
                self.append_("    }")
                self.append_(
                    f"    {field.id}_.push_back({element_type}(temp_span.read_{self.byteorder}<{backing_type}, {element_size}>()));"
                )
            else:
                self.append_(f"    {element_type} element;")
                self.append_(f"    if (!{element_type}::Parse(temp_span, &element)) {{")
                self.append_(f"        return false;")
                self.append_("    }")
                self.append_(f"    {field.id}_.emplace_back(std::move(element));")
            self.append_("}")
            self.append_("}")

        # The array count is known. The element width is dynamic.
        # Parse each element iteratively and derive the array span.
        elif count is not None:
            self.append_(f"for (size_t n = 0; n < {count}; n++) {{")
            self.append_(f"    {element_type} element;")
            self.append_(f"    if (!{field.type_id}::Parse(span, &element)) {{")
            self.append_("        return false;")
            self.append_("    }")
            self.append_(f"    {field.id}_.emplace_back(std::move(element));")
            self.append_("}")

        # The array size is not known, assume the array takes the
        # full remaining space. TODO support having fixed sized fields
        # following the array.
        elif field.width:
            element_size = int(field.width / 8)
            self.append_(f"while (span.size() > 0) {{")
            self.append_(f"    if (span.size() < {element_size}) {{")
            self.append_(f"        return false;")
            self.append_("    }")
            self.append_(f"    {field.id}_.push_back(span.read_{self.byteorder}<{element_type}, {element_size}>());")
            self.append_("}")
        elif isinstance(field.type, ast.EnumDeclaration):
            element_size = int(field.type.width / 8)
            backing_type = get_cxx_scalar_type(field.type.width)
            self.append_(f"while (span.size() > 0) {{")
            self.append_(f"    if (span.size() < {element_size}) {{")
            self.append_(f"        return false;")
            self.append_("    }")
            self.append_(
                f"    {field.id}_.push_back({element_type}(span.read_{self.byteorder}<{backing_type}, {element_size}>()));"
            )
            self.append_("}")
        else:
            self.append_(f"while (span.size() > 0) {{")
            self.append_(f"    {element_type} element;")
            self.append_(f"    if (!{element_type}::Parse(span, &element)) {{")
            self.append_(f"        return false;")
            self.append_("    }")
            self.append_(f"    {field.id}_.emplace_back(std::move(element));")
            self.append_("}")

        if padded_size:
            self.append_(f"span = remaining_span;")
            self.append_("}")

    def parse_payload_field_lite_(self, field: Union[ast.BodyField, ast.PayloadField]):
        """Parse body and payload fields."""
        if self.shift != 0:
            raise Exception('Payload field does not start on an octet boundary')

        payload_size = core.get_payload_field_size(field)
        offset_from_end = core.get_field_offset_from_end(field)
        self.check_code_()

        if payload_size and getattr(field, 'size_modifier', None):
            self.append_(f"{field.id}_size -= {field.size_modifier};")

        # The payload or body has a known size.
        # Consume the payload and update the span in case
        # fields are placed after the payload.
        if payload_size:
            self.check_size_(f"{field.id}_size")
            self.append_(f"payload_ = span.subrange(0, {field.id}_size);")
            self.append_(f"span.skip({field.id}_size);")
        # The payload or body is the last field of a packet,
        # consume the remaining span.
        elif offset_from_end == 0:
            self.append_(f"payload_ = span;")
            self.append_(f"span.clear();")
        # The payload or body is followed by fields of static size.
        # Consume the span that is not reserved for the following fields.
        elif offset_from_end:
            if (offset_from_end % 8) != 0:
                raise Exception('Payload field offset from end of packet is not a multiple of 8')
            offset_from_end = int(offset_from_end / 8)
            self.check_size_(f'{offset_from_end}')
            self.append_(f"payload_ = span.subrange(0, span.size() - {offset_from_end});")
            self.append_(f"span.skip(payload_.size());")

    def parse_payload_field_full_(self, field: Union[ast.BodyField, ast.PayloadField]):
        """Parse body and payload fields."""
        if self.shift != 0:
            raise Exception('Payload field does not start on an octet boundary')

        payload_size = core.get_payload_field_size(field)
        offset_from_end = core.get_field_offset_from_end(field)
        self.check_code_()

        if payload_size and getattr(field, 'size_modifier', None):
            self.append_(f"{field.id}_size -= {field.size_modifier};")

        # The payload or body has a known size.
        # Consume the payload and update the span in case
        # fields are placed after the payload.
        if payload_size:
            self.check_size_(f"{field.id}_size")
            self.append_(f"for (size_t n = 0; n < {field.id}_size; n++) {{")
            self.append_(f"    payload_.push_back(span.read_{self.byteorder}<uint8_t>();")
            self.append_("}")
        # The payload or body is the last field of a packet,
        # consume the remaining span.
        elif offset_from_end == 0:
            self.append_("while (span.size() > 0) {")
            self.append_(f"    payload_.push_back(span.read_{self.byteorder}<uint8_t>();")
            self.append_("}")
        # The payload or body is followed by fields of static size.
        # Consume the span that is not reserved for the following fields.
        elif offset_from_end is not None:
            if (offset_from_end % 8) != 0:
                raise Exception('Payload field offset from end of packet is not a multiple of 8')
            offset_from_end = int(offset_from_end / 8)
            self.check_size_(f'{offset_from_end}')
            self.append_(f"while (span.size() > {offset_from_end}) {{")
            self.append_(f"    payload_.push_back(span.read_{self.byteorder}<uint8_t>();")
            self.append_("}")

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
        elif isinstance(field, ast.ArrayField) and self.extract_arrays:
            self.parse_array_field_full_(field)

        elif isinstance(field, ast.ArrayField) and not self.extract_arrays:
            self.parse_array_field_lite_(field)

        # Other typedef fields.
        elif isinstance(field, ast.TypedefField):
            self.parse_typedef_field_(field)

        # Payload and body fields.
        elif isinstance(field, (ast.PayloadField, ast.BodyField)) and self.extract_arrays:
            self.parse_payload_field_full_(field)

        elif isinstance(field, (ast.PayloadField, ast.BodyField)) and not self.extract_arrays:
            self.parse_payload_field_lite_(field)

        else:
            raise Exception(f'Unsupported field type {field.kind}')

    def done(self):
        self.check_code_()


@dataclass
class FieldSerializer:
    byteorder: str
    shift: int = 0
    value: List[Tuple[str, int]] = field(default_factory=lambda: [])
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

    def get_payload_field_size(self, var: Optional[str], payload: ast.PayloadField, decl: ast.Declaration) -> str:
        """Compute the size of the selected payload field, with the information
        of the builder for the selected declaration. The payload field can be
        the payload of any of the parent declarations, or the current declaration."""

        if payload.parent.id == decl.id:
            return deref(var, 'payload_.size()')

        # Get the child packet declaration that will match the current
        # declaration further down.
        child = decl
        while child.parent_id != payload.parent.id:
            child = child.parent

        # The payload is the result of serializing the children fields.
        constant_width = 0
        variable_width = []
        for f in child.fields:
            field_size = core.get_field_size(f)
            if field_size is not None:
                constant_width += field_size
            elif isinstance(f, (ast.PayloadField, ast.BodyField)):
                variable_width.append(self.get_payload_field_size(var, f, decl))
            elif isinstance(f, ast.TypedefField):
                variable_width.append(f"{f.id}_.GetSize()")
            elif isinstance(f, ast.ArrayField):
                variable_width.append(f"Get{to_pascal_case(f.id)}Size()")
            else:
                raise Exception("Unsupported field type")

        constant_width = int(constant_width / 8)
        if constant_width and not variable_width:
            return str(constant_width)

        temp_var = f'{payload.parent.id.lower()}_payload_size'
        self.append_(f"size_t {temp_var} = {constant_width};")
        for dyn in variable_width:
            self.append_(f"{temp_var} += {dyn};")
        return temp_var

    def serialize_array_element_(self, field: ast.ArrayField, var: str):
        """Serialize a single array field element."""
        if field.width:
            backing_type = get_cxx_scalar_type(field.width)
            element_size = int(field.width / 8)
            self.append_(
                f"pdl::packet::Builder::write_{self.byteorder}<{backing_type}, {element_size}>(output, {var});")
        elif isinstance(field.type, ast.EnumDeclaration):
            backing_type = get_cxx_scalar_type(field.type.width)
            element_size = int(field.type.width / 8)
            self.append_(f"pdl::packet::Builder::write_{self.byteorder}<{backing_type}, {element_size}>(" +
                         f"output, static_cast<{backing_type}>({var}));")
        else:
            self.append_(f"{var}.Serialize(output);")

    def serialize_array_field_(self, field: ast.ArrayField, var: str):
        """Serialize the selected array field."""
        if field.padded_size:
            self.append_(f"size_t {field.id}_end = output.size() + {field.padded_size};")

        if field.width == 8:
            self.append_(f"output.insert(output.end(), {var}.begin(), {var}.end());")
        else:
            self.append_(f"for (size_t n = 0; n < {var}.size(); n++) {{")
            self.indent_()
            self.serialize_array_element_(field, f'{var}[n]')
            self.unindent_()
            self.append_("}")

        if field.padded_size:
            self.append_(f"while (output.size() < {field.id}_end) {{")
            self.append_("    output.push_back(0);")
            self.append_("}")

    def serialize_bit_field_(self, field: ast.Field, parent_var: Optional[str], var: Optional[str],
                             decl: ast.Declaration):
        """Serialize the selected field as a bit field.
        The field is added to the current chunk. When a byte boundary
        is reached all saved fields are serialized together."""

        # Add to current chunk.
        width = core.get_field_size(field)
        shift = self.shift

        if isinstance(field, ast.ScalarField):
            self.value.append((f"{var} & {mask(field.width)}", shift))
        elif isinstance(field, ast.FixedField) and field.enum_id:
            self.value.append((f"{field.enum_id}::{field.tag_id}", shift))
        elif isinstance(field, ast.FixedField):
            self.value.append((f"{field.value}", shift))
        elif isinstance(field, ast.TypedefField):
            self.value.append((f"{var}", shift))

        elif isinstance(field, ast.SizeField):
            max_size = (1 << field.width) - 1
            value_field = core.get_packet_field(field.parent, field.field_id)
            size_modifier = ''

            if getattr(value_field, 'size_modifier', None):
                size_modifier = f' + {value_field.size_modifier}'

            if isinstance(value_field, (ast.PayloadField, ast.BodyField)):
                array_size = self.get_payload_field_size(var, field, decl) + size_modifier

            elif isinstance(value_field, ast.ArrayField):
                accessor_name = to_pascal_case(field.field_id)
                array_size = deref(var, f'Get{accessor_name}Size()') + size_modifier

            self.value.append((f"{array_size}", shift))

        elif isinstance(field, ast.CountField):
            max_count = (1 << field.width) - 1
            self.value.append((f"{field.field_id}_.size()", shift))

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
        backing_type = get_cxx_scalar_type(self.shift)
        value = [f"(static_cast<{backing_type}>({v[0]}) << {v[1]})" for v in self.value]

        if len(value) == 0:
            self.append_(f"pdl::packet::Builder::write_{self.byteorder}<{backing_type}, {size}>(output, 0);")
        elif len(value) == 1:
            self.append_(f"pdl::packet::Builder::write_{self.byteorder}<{backing_type}, {size}>(output, {value[0]});")
        else:
            self.append_(
                f"pdl::packet::Builder::write_{self.byteorder}<{backing_type}, {size}>(output, {' | '.join(value)});")

        # Reset state.
        self.shift = 0
        self.value = []

    def serialize_typedef_field_(self, field: ast.TypedefField, var: str):
        """Serialize a typedef field, to the exclusion of Enum fields."""

        if self.shift != 0:
            raise Exception('Typedef field does not start on an octet boundary')
        if (isinstance(field.type, ast.StructDeclaration) and field.type.parent_id is not None):
            raise Exception('Derived struct used in typedef field')

        self.append_(f"{var}.Serialize(output);")

    def serialize_payload_field_(self, field: Union[ast.BodyField, ast.PayloadField], var: str):
        """Serialize body and payload fields."""

        if self.shift != 0:
            raise Exception('Payload field does not start on an octet boundary')

        self.append_(f"output.insert(output.end(), {var}.begin(), {var}.end());")

    def serialize(self, field: ast.Field, decl: ast.Declaration, var: Optional[str] = None):
        field_var = deref(var, f'{field.id}_') if hasattr(field, 'id') else None

        # Field has bit granularity.
        # Append the field to the current chunk,
        # check if a byte boundary was reached.
        if core.is_bit_field(field):
            self.serialize_bit_field_(field, var, field_var, decl)

        # Padding fields.
        elif isinstance(field, ast.PaddingField):
            pass

        # Array fields.
        elif isinstance(field, ast.ArrayField):
            self.serialize_array_field_(field, field_var)

        # Other typedef fields.
        elif isinstance(field, ast.TypedefField):
            self.serialize_typedef_field_(field, field_var)

        # Payload and body fields.
        elif isinstance(field, (ast.PayloadField, ast.BodyField)):
            self.serialize_payload_field_(field, deref(var, 'payload_'))

        else:
            raise Exception(f'Unimplemented field type {field.kind}')


def generate_enum_declaration(decl: ast.EnumDeclaration) -> str:
    """Generate the implementation of an enum type."""

    enum_name = decl.id
    enum_type = get_cxx_scalar_type(decl.width)
    tag_decls = []
    for t in decl.tags:
        tag_decls.append(f"{t.id} = {hex(t.value)},")

    return dedent("""\

        enum class {enum_name} : {enum_type} {{
            {tag_decls}
        }};
        """).format(enum_name=enum_name, enum_type=enum_type, tag_decls=indent(tag_decls, 1))


def generate_enum_to_text(decl: ast.EnumDeclaration) -> str:
    """Generate the helper function that will convert an enum tag to string."""

    enum_name = decl.id
    tag_cases = []
    for t in decl.tags:
        tag_cases.append(f"case {enum_name}::{t.id}: return \"{t.id}\";")

    return dedent("""\

        inline std::string {enum_name}Text({enum_name} tag) {{
            switch (tag) {{
                {tag_cases}
                default:
                    return std::string("Unknown {enum_name}: " +
                           std::to_string(static_cast<uint64_t>(tag)));
            }}
        }}
        """).format(enum_name=enum_name, tag_cases=indent(tag_cases, 2))


def generate_packet_field_members(decl: ast.Declaration, view: bool) -> List[str]:
    """Return the declaration of fields that are backed in the view
    class declaration.

    Backed fields include all named fields that do not have a constrained
    value in the selected declaration and its parents.

    :param decl: target declaration
    :param view: if true the payload and array fields are generated as slices"""

    fields = core.get_unconstrained_parent_fields(decl) + decl.fields
    members = []
    for field in fields:
        if isinstance(field, (ast.PayloadField, ast.BodyField)) and view:
            members.append("pdl::packet::slice payload_;")
        elif isinstance(field, (ast.PayloadField, ast.BodyField)):
            members.append("std::vector<uint8_t> payload_;")
        elif isinstance(field, ast.ArrayField) and view:
            members.append(f"pdl::packet::slice {field.id}_;")
        elif isinstance(field, ast.ArrayField):
            element_type = field.type_id or get_cxx_scalar_type(field.width)
            members.append(f"std::vector<{element_type}> {field.id}_;")
        elif isinstance(field, ast.ScalarField):
            members.append(f"{get_cxx_scalar_type(field.width)} {field.id}_{{0}};")
        elif isinstance(field, ast.TypedefField) and isinstance(field.type, ast.EnumDeclaration):
            members.append(f"{field.type_id} {field.id}_{{{field.type_id}::{field.type.tags[0].id}}};")
        elif isinstance(field, ast.TypedefField):
            members.append(f"{field.type_id} {field.id}_;")

    return members


def generate_packet_field_serializers(packet: ast.Declaration) -> List[str]:
    """Generate the code to serialize the fields of a packet builder or struct."""
    serializer = FieldSerializer(byteorder=packet.file.byteorder_short)
    constraints = core.get_parent_constraints(packet)
    constraints = dict([(c.id, c) for c in constraints])
    for field in core.get_packet_fields(packet):
        field_id = getattr(field, 'id', None)
        constraint = constraints.get(field_id, None)
        fixed_field = None
        if constraint and constraint.tag_id:
            fixed_field = ast.FixedField(enum_id=field.type_id,
                                         tag_id=constraint.tag_id,
                                         loc=field.loc,
                                         kind='fixed_field')
            fixed_field.parent = field.parent
        elif constraint:
            fixed_field = ast.FixedField(width=field.width, value=constraint.value, loc=field.loc, kind='fixed_field')
            fixed_field.parent = field.parent
        serializer.serialize(fixed_field or field, packet)
    return serializer.code


def generate_scalar_array_field_accessor(field: ast.ArrayField) -> str:
    """Parse the selected scalar array field."""
    element_size = int(field.width / 8)
    backing_type = get_cxx_scalar_type(field.width)
    byteorder = field.parent.file.byteorder_short
    return dedent("""\
        pdl::packet::slice span = {field_id}_;
        std::vector<{backing_type}> elements;
        while (span.size() >= {element_size}) {{
            elements.push_back(span.read_{byteorder}<{backing_type}, {element_size}>());
        }}
        return elements;""").format(field_id=field.id,
                                    backing_type=backing_type,
                                    element_size=element_size,
                                    byteorder=byteorder)


def generate_enum_array_field_accessor(field: ast.ArrayField) -> str:
    """Parse the selected enum array field."""
    element_size = int(field.type.width / 8)
    backing_type = get_cxx_scalar_type(field.type.width)
    byteorder = field.parent.file.byteorder_short
    return dedent("""\
        pdl::packet::slice span = {field_id}_;
        std::vector<{enum_type}> elements;
        while (span.size() >= {element_size}) {{
            elements.push_back({enum_type}(span.read_{byteorder}<{backing_type}, {element_size}>()));
        }}
        return elements;""").format(field_id=field.id,
                                    enum_type=field.type_id,
                                    backing_type=backing_type,
                                    element_size=element_size,
                                    byteorder=byteorder)


def generate_typedef_array_field_accessor(field: ast.ArrayField) -> str:
    """Parse the selected typedef array field."""
    return dedent("""\
        pdl::packet::slice span = {field_id}_;
        std::vector<{struct_type}> elements;
        for (;;) {{
            {struct_type} element;
            if (!{struct_type}::Parse(span, &element)) {{
                break;
            }}
            elements.emplace_back(std::move(element));
        }}
        return elements;""").format(field_id=field.id, struct_type=field.type_id)


def generate_array_field_accessor(field: ast.ArrayField):
    """Parse the selected array field."""

    if field.width is not None:
        return generate_scalar_array_field_accessor(field)
    elif isinstance(field.type, ast.EnumDeclaration):
        return generate_enum_array_field_accessor(field)
    else:
        return generate_typedef_array_field_accessor(field)


def generate_array_field_size_getters(decl: ast.Declaration) -> str:
    """Generate size getters for array fields. Produces the serialized
    size of the array in bytes."""

    getters = []
    fields = core.get_unconstrained_parent_fields(decl) + decl.fields
    for field in fields:
        if not isinstance(field, ast.ArrayField):
            continue

        element_width = field.width or core.get_declaration_size(field.type)
        size = None

        if element_width and field.size:
            size = int(element_width * field.size / 8)
        elif element_width:
            size = f"{field.id}_.size() * {int(element_width / 8)}"

        if size:
            getters.append(
                dedent("""\
                size_t Get{accessor_name}Size() const {{
                    return {size};
                }}
                """).format(accessor_name=to_pascal_case(field.id), size=size))
        else:
            getters.append(
                dedent("""\
                size_t Get{accessor_name}Size() const {{
                    size_t array_size = 0;
                    for (size_t n = 0; n < {field_id}_.size(); n++) {{
                        array_size += {field_id}_[n].GetSize();
                    }}
                    return array_size;
                }}
                """).format(accessor_name=to_pascal_case(field.id), field_id=field.id))

    return '\n'.join(getters)


def generate_packet_size_getter(decl: ast.Declaration) -> List[str]:
    """Generate a size getter the current packet. Produces the serialized
    size of the packet in bytes."""

    constant_width = 0
    variable_width = []
    for f in core.get_packet_fields(decl):
        field_size = core.get_field_size(f)
        if field_size is not None:
            constant_width += field_size
        elif isinstance(f, (ast.PayloadField, ast.BodyField)):
            variable_width.append("payload_.size()")
        elif isinstance(f, ast.TypedefField):
            variable_width.append(f"{f.id}_.GetSize()")
        elif isinstance(f, ast.ArrayField):
            variable_width.append(f"Get{to_pascal_case(f.id)}Size()")
        else:
            raise Exception("Unsupported field type")

    constant_width = int(constant_width / 8)
    if not variable_width:
        return [f"return {constant_width};"]
    elif len(variable_width) == 1 and constant_width:
        return [f"return {variable_width[0]} + {constant_width};"]
    elif len(variable_width) == 1:
        return [f"return {variable_width[0]};"]
    elif len(variable_width) > 1 and constant_width:
        return ([f"return {constant_width} + ("] + " +\n    ".join(variable_width).split("\n") + [");"])
    elif len(variable_width) > 1:
        return (["return ("] + " +\n    ".join(variable_width).split("\n") + [");"])
    else:
        assert False


def generate_packet_view_field_accessors(packet: ast.PacketDeclaration) -> List[str]:
    """Return the declaration of accessors for the named packet fields."""

    accessors = []

    # Add accessors for the backed fields.
    fields = core.get_unconstrained_parent_fields(packet) + packet.fields
    for field in fields:
        if isinstance(field, (ast.PayloadField, ast.BodyField)):
            accessors.append(
                dedent("""\
                std::vector<uint8_t> GetPayload() const {
                    ASSERT(valid_);
                    return payload_.bytes();
                }

                """))
        elif isinstance(field, ast.ArrayField):
            element_type = field.type_id or get_cxx_scalar_type(field.width)
            accessor_name = to_pascal_case(field.id)
            accessors.append(
                dedent("""\
                std::vector<{element_type}> Get{accessor_name}() const {{
                    ASSERT(valid_);
                    {accessor}
                }}

                """).format(element_type=element_type,
                            accessor_name=accessor_name,
                            accessor=indent(generate_array_field_accessor(field), 1)))
        elif isinstance(field, ast.ScalarField):
            field_type = get_cxx_scalar_type(field.width)
            accessor_name = to_pascal_case(field.id)
            accessors.append(
                dedent("""\
                {field_type} Get{accessor_name}() const {{
                    ASSERT(valid_);
                    return {member_name}_;
                }}

                """).format(field_type=field_type, accessor_name=accessor_name, member_name=field.id))
        elif isinstance(field, ast.TypedefField):
            field_qualifier = "" if isinstance(field.type, ast.EnumDeclaration) else " const&"
            accessor_name = to_pascal_case(field.id)
            accessors.append(
                dedent("""\
                {field_type}{field_qualifier} Get{accessor_name}() const {{
                    ASSERT(valid_);
                    return {member_name}_;
                }}

                """).format(field_type=field.type_id,
                            field_qualifier=field_qualifier,
                            accessor_name=accessor_name,
                            member_name=field.id))

    # Add accessors for constrained parent fields.
    # The accessors return a constant value in this case.
    for c in core.get_parent_constraints(packet):
        field = core.get_packet_field(packet, c.id)
        if isinstance(field, ast.ScalarField):
            field_type = get_cxx_scalar_type(field.width)
            accessor_name = to_pascal_case(field.id)
            accessors.append(
                dedent("""\
                {field_type} Get{accessor_name}() const {{
                    return {value};
                }}

                """).format(field_type=field_type, accessor_name=accessor_name, value=c.value))
        else:
            accessor_name = to_pascal_case(field.id)
            accessors.append(
                dedent("""\
                {field_type} Get{accessor_name}() const {{
                    return {field_type}::{tag_id};
                }}

                """).format(field_type=field.type_id, accessor_name=accessor_name, tag_id=c.tag_id))

    return "".join(accessors)


def generate_packet_stringifier(packet: ast.PacketDeclaration) -> str:
    """Generate the packet printer. TODO """
    return dedent("""\
        std::string ToString() const {
            return "";
        }
        """)


def generate_packet_view_field_parsers(packet: ast.PacketDeclaration) -> str:
    """Generate the packet parser. The validator will extract
    the fields it can in a pre-parsing phase. """

    code = []

    # Generate code to check the validity of the parent,
    # and import parent fields that do not have a fixed value in the
    # current packet.
    if packet.parent:
        code.append(
            dedent("""\
            // Check validity of parent packet.
            if (!parent.IsValid()) {
                return false;
            }
            """))
        parent_fields = core.get_unconstrained_parent_fields(packet)
        if parent_fields:
            code.append("// Copy parent field values.")
            for f in parent_fields:
                code.append(f"{f.id}_ = parent.{f.id}_;")
            code.append("")
        span = "parent.payload_"
    else:
        span = "parent"

    # Validate parent constraints.
    for c in packet.constraints:
        if c.tag_id:
            enum_type = core.get_packet_field(packet.parent, c.id).type_id
            code.append(
                dedent("""\
                if (parent.{field_id}_ != {enum_type}::{tag_id}) {{
                    return false;
                }}
                """).format(field_id=c.id, enum_type=enum_type, tag_id=c.tag_id))
        else:
            code.append(
                dedent("""\
                if (parent.{field_id}_ != {value}) {{
                    return false;
                }}
                """).format(field_id=c.id, value=c.value))

    # Parse fields linearly.
    if packet.fields:
        code.append("// Parse packet field values.")
        code.append(f"pdl::packet::slice span = {span};")
        for f in packet.fields:
            if isinstance(f, ast.SizeField):
                code.append(f"{get_cxx_scalar_type(f.width)} {f.field_id}_size;")
            elif isinstance(f, (ast.SizeField, ast.CountField)):
                code.append(f"{get_cxx_scalar_type(f.width)} {f.field_id}_count;")
        parser = FieldParser(extract_arrays=False, byteorder=packet.file.byteorder_short)
        for f in packet.fields:
            parser.parse(f)
        parser.done()
        code.extend(parser.code)

    code.append("return true;")
    return '\n'.join(code)


def generate_packet_view_friend_classes(packet: ast.PacketDeclaration) -> str:
    """Generate the list of friend declarations for a packet.
    These are the direct children of the class."""

    return [f"friend class {decl.id}View;" for (_, decl) in core.get_derived_packets(packet, traverse=False)]


def generate_packet_view(packet: ast.PacketDeclaration) -> str:
    """Generate the implementation of the View class for a
    packet declaration."""

    parent_class = f"{packet.parent.id}View" if packet.parent else "pdl::packet::slice"
    field_members = generate_packet_field_members(packet, view=True)
    field_accessors = generate_packet_view_field_accessors(packet)
    field_parsers = generate_packet_view_field_parsers(packet)
    friend_classes = generate_packet_view_friend_classes(packet)
    stringifier = generate_packet_stringifier(packet)

    return dedent("""\

        class {packet_name}View {{
        public:
            static {packet_name}View Create({parent_class} const& parent) {{
                return {packet_name}View(parent);
            }}

            {field_accessors}
            {stringifier}

            bool IsValid() const {{
                return valid_;
            }}

        protected:
            explicit {packet_name}View({parent_class} const& parent) {{
                valid_ = Parse(parent);
            }}

            bool Parse({parent_class} const& parent) {{
                {field_parsers}
            }}

            bool valid_{{false}};
            {field_members}

            {friend_classes}
        }};
        """).format(packet_name=packet.id,
                    parent_class=parent_class,
                    field_accessors=indent(field_accessors, 1),
                    field_members=indent(field_members, 1),
                    field_parsers=indent(field_parsers, 2),
                    friend_classes=indent(friend_classes, 1),
                    stringifier=indent(stringifier, 1))


def generate_packet_constructor(struct: ast.StructDeclaration, constructor_name: str) -> str:
    """Generate the implementation of the constructor for a
    struct declaration."""

    constructor_params = []
    constructor_initializers = []
    fields = core.get_unconstrained_parent_fields(struct) + struct.fields

    for field in fields:
        if isinstance(field, (ast.PayloadField, ast.BodyField)):
            constructor_params.append("std::vector<uint8_t> payload")
            constructor_initializers.append("payload_(std::move(payload))")
        elif isinstance(field, ast.ArrayField):
            element_type = field.type_id or get_cxx_scalar_type(field.width)
            constructor_params.append(f"std::vector<{element_type}> {field.id}")
            constructor_initializers.append(f"{field.id}_(std::move({field.id}))")
        elif isinstance(field, ast.ScalarField):
            backing_type = get_cxx_scalar_type(field.width)
            constructor_params.append(f"{backing_type} {field.id}")
            constructor_initializers.append(f"{field.id}_({field.id})")
        elif (isinstance(field, ast.TypedefField) and isinstance(field.type, ast.EnumDeclaration)):
            constructor_params.append(f"{field.type_id} {field.id}")
            constructor_initializers.append(f"{field.id}_({field.id})")
        elif isinstance(field, ast.TypedefField):
            constructor_params.append(f"{field.type_id} {field.id}")
            constructor_initializers.append(f"{field.id}_(std::move({field.id}))")

    if not constructor_params:
        return ""

    explicit = 'explicit ' if len(constructor_params) == 1 else ''
    constructor_params = ', '.join(constructor_params)
    constructor_initializers = ', '.join(constructor_initializers)

    return dedent("""\
        {explicit}{constructor_name}({constructor_params})
            : {constructor_initializers} {{}}""").format(explicit=explicit,
                                                         constructor_name=constructor_name,
                                                         constructor_params=constructor_params,
                                                         constructor_initializers=constructor_initializers)


def generate_packet_builder(packet: ast.PacketDeclaration) -> str:
    """Generate the implementation of the Builder class for a
    packet declaration."""

    class_name = f'{packet.id}Builder'
    builder_constructor = generate_packet_constructor(packet, constructor_name=class_name)
    field_members = generate_packet_field_members(packet, view=False)
    field_serializers = generate_packet_field_serializers(packet)
    size_getter = generate_packet_size_getter(packet)
    array_field_size_getters = generate_array_field_size_getters(packet)

    return dedent("""\

        class {class_name} : public pdl::packet::Builder {{
        public:
            ~{class_name}() override = default;
            {class_name}() = default;
            {class_name}({class_name} const&) = default;
            {class_name}({class_name}&&) = default;
            {class_name}& operator=({class_name} const&) = default;
            {builder_constructor}

            void Serialize(std::vector<uint8_t>& output) const override {{
                {field_serializers}
            }}

            size_t GetSize() const override {{
                {size_getter}
            }}

            {array_field_size_getters}
            {field_members}
        }};
        """).format(class_name=f'{packet.id}Builder',
                    builder_constructor=builder_constructor,
                    field_members=indent(field_members, 1),
                    field_serializers=indent(field_serializers, 2),
                    size_getter=indent(size_getter, 1),
                    array_field_size_getters=indent(array_field_size_getters, 1))


def generate_struct_field_parsers(struct: ast.StructDeclaration) -> str:
    """Generate the struct parser. The validator will extract
    the fields it can in a pre-parsing phase. """

    code = []
    parsed_fields = []
    post_processing = []

    for field in struct.fields:
        if isinstance(field, (ast.PayloadField, ast.BodyField)):
            code.append("std::vector<uint8_t> payload_;")
            parsed_fields.append("std::move(payload_)")
        elif isinstance(field, ast.ArrayField):
            element_type = field.type_id or get_cxx_scalar_type(field.width)
            code.append(f"std::vector<{element_type}> {field.id}_;")
            parsed_fields.append(f"std::move({field.id}_)")
        elif isinstance(field, ast.ScalarField):
            backing_type = get_cxx_scalar_type(field.width)
            code.append(f"{backing_type} {field.id}_;")
            parsed_fields.append(f"{field.id}_")
        elif (isinstance(field, ast.TypedefField) and isinstance(field.type, ast.EnumDeclaration)):
            code.append(f"{field.type_id} {field.id}_;")
            parsed_fields.append(f"{field.id}_")
        elif isinstance(field, ast.TypedefField):
            code.append(f"{field.type_id} {field.id}_;")
            parsed_fields.append(f"std::move({field.id}_)")
        elif isinstance(field, ast.SizeField):
            code.append(f"{get_cxx_scalar_type(field.width)} {field.field_id}_size;")
        elif isinstance(field, ast.CountField):
            code.append(f"{get_cxx_scalar_type(field.width)} {field.field_id}_count;")

    parser = FieldParser(extract_arrays=True, byteorder=struct.file.byteorder_short)
    for f in struct.fields:
        parser.parse(f)
    parser.done()
    code.extend(parser.code)

    parsed_fields = ', '.join(parsed_fields)
    code.append(f"*output = {struct.id}({parsed_fields});")
    code.append("return true;")
    return '\n'.join(code)


def generate_struct_declaration(struct: ast.StructDeclaration) -> str:
    """Generate the implementation of the class for a
    struct declaration."""

    if struct.parent:
        raise Exception("Struct declaration with parents are not supported")

    struct_constructor = generate_packet_constructor(struct, constructor_name=struct.id)
    field_members = generate_packet_field_members(struct, view=False)
    field_parsers = generate_struct_field_parsers(struct)
    field_serializers = generate_packet_field_serializers(struct)
    size_getter = generate_packet_size_getter(struct)
    array_field_size_getters = generate_array_field_size_getters(struct)
    stringifier = generate_packet_stringifier(struct)

    return dedent("""\

        class {struct_name} : public pdl::packet::Builder {{
        public:
            ~{struct_name}() override = default;
            {struct_name}() = default;
            {struct_name}({struct_name} const&) = default;
            {struct_name}({struct_name}&&) = default;
            {struct_name}& operator=({struct_name} const&) = default;
            {struct_constructor}

            static bool Parse(pdl::packet::slice& span, {struct_name}* output) {{
                {field_parsers}
            }}

            void Serialize(std::vector<uint8_t>& output) const override {{
                {field_serializers}
            }}

            size_t GetSize() const override {{
                {size_getter}
            }}

            {array_field_size_getters}
            {stringifier}
            {field_members}
        }};
        """).format(struct_name=struct.id,
                    struct_constructor=struct_constructor,
                    field_members=indent(field_members, 1),
                    field_parsers=indent(field_parsers, 2),
                    field_serializers=indent(field_serializers, 2),
                    stringifier=indent(stringifier, 1),
                    size_getter=indent(size_getter, 1),
                    array_field_size_getters=indent(array_field_size_getters, 1))


def run(input: argparse.FileType, output: argparse.FileType, namespace: Optional[str], include_header: List[str],
        using_namespace: List[str]):

    file = ast.File.from_json(json.load(input))
    core.desugar(file)

    include_header = '\n'.join([f'#include <{header}>' for header in include_header])
    using_namespace = '\n'.join([f'using namespace {namespace};' for namespace in using_namespace])
    open_namespace = f"namespace {namespace} {{" if namespace else ""
    close_namespace = f"}}  // {namespace}" if namespace else ""

    # Disable unsupported features in the canonical test suite.
    skipped_decls = [
        'Packet_Custom_Field_ConstantSize',
        'Packet_Custom_Field_VariableSize',
        'Packet_Checksum_Field_FromStart',
        'Packet_Checksum_Field_FromEnd',
        'Struct_Custom_Field_ConstantSize',
        'Struct_Custom_Field_VariableSize',
        'Struct_Checksum_Field_FromStart',
        'Struct_Checksum_Field_FromEnd',
        'Struct_Custom_Field_ConstantSize_',
        'Struct_Custom_Field_VariableSize_',
        'Struct_Checksum_Field_FromStart_',
        'Struct_Checksum_Field_FromEnd_',
        'PartialParent5',
        'PartialChild5_A',
        'PartialChild5_B',
        'PartialParent12',
        'PartialChild12_A',
        'PartialChild12_B',
    ]

    output.write(
        dedent("""\
        // File generated from {input_name}, with the command:
        //  {input_command}
        // /!\\ Do not edit by hand

        #pragma once

        #include <cstdint>
        #include <string>
        #include <utility>
        #include <vector>

        #include <packet_runtime.h>

        {include_header}
        {using_namespace}

        #ifndef ASSERT
        #include <cassert>
        #define ASSERT assert
        #endif  // !ASSERT

        {open_namespace}
        """).format(input_name=input.name,
                    input_command=' '.join(sys.argv),
                    include_header=include_header,
                    using_namespace=using_namespace,
                    open_namespace=open_namespace))

    for d in file.declarations:
        if d.id in skipped_decls:
            continue

        if isinstance(d, ast.EnumDeclaration):
            output.write(generate_enum_declaration(d))
            output.write(generate_enum_to_text(d))
        elif isinstance(d, ast.PacketDeclaration):
            output.write(generate_packet_view(d))
            output.write(generate_packet_builder(d))
        elif isinstance(d, ast.StructDeclaration):
            output.write(generate_struct_declaration(d))

    output.write(f"{close_namespace}\n")


def main() -> int:
    """Generate cxx PDL backend."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--input', type=argparse.FileType('r'), default=sys.stdin, help='Input PDL-JSON source')
    parser.add_argument('--output', type=argparse.FileType('w'), default=sys.stdout, help='Output C++ file')
    parser.add_argument('--namespace', type=str, help='Generated module namespace')
    parser.add_argument('--include-header', type=str, default=[], action='append', help='Added include directives')
    parser.add_argument('--using-namespace',
                        type=str,
                        default=[],
                        action='append',
                        help='Added using namespace statements')
    return run(**vars(parser.parse_args()))


if __name__ == '__main__':
    sys.exit(main())
