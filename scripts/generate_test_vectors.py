#!/usr/bin/env python3

import argparse
import collections
import copy
import json
from pathlib import Path
import pprint
import traceback
from typing import Iterable, List, Optional, Union
import sys

from pdl import ast, core

MAX_ARRAY_SIZE = 256
MAX_ARRAY_COUNT = 32
DEFAULT_ARRAY_COUNT = 3
DEFAULT_PAYLOAD_SIZE = 5


class BitSerializer:
    def __init__(self, big_endian: bool):
        self.stream = []
        self.value = 0
        self.shift = 0
        self.byteorder = "big" if big_endian else "little"

    def append(self, value: int, width: int):
        self.value = self.value | (value << self.shift)
        self.shift += width

        if (self.shift % 8) == 0:
            width = int(self.shift / 8)
            self.stream.extend(self.value.to_bytes(width, byteorder=self.byteorder))
            self.shift = 0
            self.value = 0


class Value:
    def __init__(self, value: object, width: Optional[int] = None):
        self.value = value
        if width is not None:
            self.width = width
        elif isinstance(value, int) or callable(value):
            raise Exception("Creating scalar value of unspecified width")
        elif isinstance(value, list):
            self.width = sum([v.width for v in value])
        elif isinstance(value, Packet):
            self.width = value.width
        else:
            raise Exception(f"Malformed value {value}")

    def finalize(self, parent: "Packet"):
        if callable(self.width):
            self.width = self.width(parent)

        if callable(self.value):
            self.value = self.value(parent)
        elif isinstance(self.value, list):
            for v in self.value:
                v.finalize(parent)
        elif isinstance(self.value, Packet):
            self.value.finalize()

    def serialize_(self, serializer: BitSerializer):
        if isinstance(self.value, int):
            serializer.append(self.value, self.width)
        elif isinstance(self.value, list):
            for v in self.value:
                v.serialize_(serializer)
        elif isinstance(self.value, Packet):
            self.value.serialize_(serializer)
        else:
            raise Exception(f"Malformed value {self.value}")

    def show(self, indent: int = 0):
        space = " " * indent
        if isinstance(self.value, int):
            print(f"{space}{self.name}: {hex(self.value)}")
        elif isinstance(self.value, list):
            print(f"{space}{self.name}[{len(self.value)}]:")
            for v in self.value:
                v.show(indent + 2)
        elif isinstance(self.value, Packet):
            print(f"{space}{self.name}:")
            self.value.show(indent + 2)

    def to_json(self) -> object:
        if isinstance(self.value, int):
            return self.value
        elif isinstance(self.value, list):
            return [v.to_json() for v in self.value]
        elif isinstance(self.value, Packet):
            return self.value.to_json()


class Field:
    def __init__(self, value: Value, ref: ast.Field):
        self.value = value
        self.ref = ref

    def finalize(self, parent: "Packet"):
        self.value.finalize(parent)

    def serialize_(self, serializer: BitSerializer):
        self.value.serialize_(serializer)

    def clone(self):
        return Field(copy.copy(self.value), self.ref)


class Packet:
    def __init__(self, fields: List[Field], ref: ast.Declaration):
        self.fields = fields
        self.ref = ref

    def finalize(self, parent: Optional["Packet"] = None):
        for f in self.fields:
            f.finalize(self)

    def serialize_(self, serializer: BitSerializer):
        for f in self.fields:
            f.serialize_(serializer)

    def serialize(self, big_endian: bool) -> bytes:
        serializer = BitSerializer(big_endian)
        self.serialize_(serializer)
        if serializer.shift != 0:
            raise Exception("The packet size is not an integral number of octets")
        return bytes(serializer.stream)

    def show(self, indent: int = 0):
        for f in self.fields:
            f.value.show(indent)

    def to_json(self) -> dict:
        result = dict()
        for f in self.fields:
            if isinstance(f.ref, (ast.PayloadField, ast.BodyField)) and isinstance(
                f.value.value, Packet
            ):
                result.update(f.value.to_json())
            elif isinstance(f.ref, (ast.PayloadField, ast.BodyField)):
                result["payload"] = f.value.to_json()
            elif hasattr(f.ref, "id"):
                result[f.ref.id] = f.value.to_json()
        return result

    @property
    def width(self) -> int:
        self.finalize()
        return sum([f.value.width for f in self.fields])


class BitGenerator:
    def __init__(self):
        self.value = 0
        self.shift = 0

    def generate(self, width: int) -> Value:
        """Generate an integer value of the selected width."""
        value = 0
        remains = width
        while remains > 0:
            w = min(8 - self.shift, remains)
            mask = (1 << w) - 1
            value = (value << w) | ((self.value >> self.shift) & mask)
            remains -= w
            self.shift += w
            if self.shift >= 8:
                self.shift = 0
                self.value = (self.value + 1) % 0xFF
        return Value(value, width)

    def generate_list(self, width: int, count: int) -> List[Value]:
        return [self.generate(width) for n in range(count)]


generator = BitGenerator()


def generate_size_field_values(field: ast.SizeField) -> List[Value]:
    def get_field_size(parent: Packet, field_id: str) -> int:
        for f in parent.fields:
            if (
                (field_id == "_payload_" and isinstance(f.ref, ast.PayloadField))
                or (field_id == "_body_" and isinstance(f.ref, ast.BodyField))
                or (getattr(f.ref, "id", None) == field_id)
            ):
                assert f.value.width % 8 == 0
                size_modifier = int(getattr(f.ref, "size_modifier", None) or 0)
                return int(f.value.width / 8) + size_modifier
        raise Exception(
            "Field {} not found in packet {}".format(field_id, parent.ref.id)
        )

    return [Value(lambda p: get_field_size(p, field.field_id), field.width)]


def generate_count_field_values(field: ast.CountField) -> List[Value]:
    def get_array_count(parent: Packet, field_id: str) -> int:
        for f in parent.fields:
            if getattr(f.ref, "id", None) == field_id:
                assert isinstance(f.value.value, list)
                return len(f.value.value)
        raise Exception(
            "Field {} not found in packet {}".format(field_id, parent.ref.id)
        )

    return [Value(lambda p: get_array_count(p, field.field_id), field.width)]


def generate_checksum_field_values(field: ast.TypedefField) -> List[Value]:
    field_width = core.get_field_size(field)

    def basic_checksum(input: bytes, width: int):
        assert width == 8
        return sum(input) % 256

    def compute_checksum(parent: Packet, field_id: str) -> int:
        serializer = None
        for f in parent.fields:
            if isinstance(f.ref, ast.ChecksumField) and f.ref.field_id == field_id:
                serializer = BitSerializer(
                    f.ref.parent.file.endianness.value == "big_endian"
                )
            elif isinstance(f.ref, ast.TypedefField) and f.ref.id == field_id:
                return basic_checksum(serializer.stream, field_width)
            elif serializer:
                f.value.serialize_(serializer)
        raise Exception("malformed checksum")

    return [Value(lambda p: compute_checksum(p, field.id), field_width)]


def generate_padding_field_values(field: ast.PaddingField) -> List[Value]:
    preceding_field_id = field.padded_field.id

    def get_padding(parent: Packet, field_id: str, width: int) -> List[Value]:
        for f in parent.fields:
            if (
                (field_id == "_payload_" and isinstance(f.ref, ast.PayloadField))
                or (field_id == "_body_" and isinstance(f.ref, ast.BodyField))
                or (getattr(f.ref, "id", None) == field_id)
            ):
                assert f.value.width % 8 == 0
                assert f.value.width <= width
                return width - f.value.width
        raise Exception(
            "Field {} not found in packet {}".format(field_id, parent.ref.id)
        )

    return [Value(0, lambda p: get_padding(p, preceding_field_id, 8 * field.size))]


def generate_payload_field_values(
    field: Union[ast.PayloadField, ast.BodyField]
) -> List[Value]:
    payload_size = core.get_payload_field_size(field)
    size_modifier = int(getattr(field, "size_modifier", None) or 0)

    # If the paylaod has a size field, generate an empty payload and
    # a payload of maximum size. If not generate a payload of the default size.
    max_size = (1 << payload_size.width) - 1 if payload_size else DEFAULT_PAYLOAD_SIZE
    max_size -= size_modifier

    assert max_size > 0
    return [Value([]), Value(generator.generate_list(8, max_size))]


def generate_scalar_array_field_values(field: ast.ArrayField) -> List[Value]:
    if field.width % 8 != 0:
        if element_width % 8 != 0:
            raise Exception("Array element size is not a multiple of 8")

    array_size = core.get_array_field_size(field)
    element_width = int(field.width / 8)

    # TODO
    # The array might also be bounded if it is included in the sized payload
    # of a packet.

    # Apply the size modifiers.
    size_modifier = int(getattr(field, "size_modifier", None) or 0)

    # The element width is known, and the array element count is known
    # statically.
    if isinstance(array_size, int):
        return [Value(generator.generate_list(field.width, array_size))]

    # The element width is known, and the array element count is known
    # by count field.
    elif isinstance(array_size, ast.CountField):
        min_count = 0
        max_count = (1 << array_size.width) - 1
        return [Value([]), Value(generator.generate_list(field.width, max_count))]

    # The element width is known, and the array full size is known
    # by size field.
    elif isinstance(array_size, ast.SizeField):
        min_count = 0
        max_size = (1 << array_size.width) - 1 - size_modifier
        max_count = int(max_size / element_width)
        return [Value([]), Value(generator.generate_list(field.width, max_count))]

    # The element width is known, but the array size is unknown.
    # Generate two arrays: one empty and one including some possible element
    # values.
    else:
        return [
            Value([]),
            Value(generator.generate_list(field.width, DEFAULT_ARRAY_COUNT)),
        ]


def generate_typedef_array_field_values(field: ast.ArrayField) -> List[Value]:
    array_size = core.get_array_field_size(field)
    element_width = core.get_array_element_size(field)
    if element_width:
        if element_width % 8 != 0:
            raise Exception("Array element size is not a multiple of 8")
        element_width = int(element_width / 8)

    # Generate element values to use for the generation.
    type_decl = field.parent.file.typedef_scope[field.type_id]

    def generate_list(count: Optional[int]) -> List[Value]:
        """Generate an array of specified length.
        If the count is None all possible array items are returned."""
        element_values = generate_typedef_values(type_decl)

        # Requested a variable count, send everything in one chunk.
        if count is None:
            return [Value(element_values)]
        # Have more items than the requested count.
        # Slice the possible array values in multiple slices.
        elif len(element_values) > count:
            # Add more elements in case of wrap-over.
            elements_count = len(element_values)
            element_values.extend(generate_typedef_values(type_decl))
            chunk_count = int((len(elements) + count - 1) / count)
            return [
                Value(element_values[n * count : (n + 1) * count])
                for n in range(chunk_count)
            ]
        # Have less items than the requested count.
        # Generate additional items to fill the gap.
        else:
            chunk = element_values
            while len(chunk) < count:
                chunk.extend(generate_typedef_values(type_decl))
            return [Value(chunk[:count])]

    # TODO
    # The array might also be bounded if it is included in the sized payload
    # of a packet.

    # Apply the size modifier.
    size_modifier = int(getattr(field, "size_modifier", None) or 0)

    min_size = 0
    max_size = MAX_ARRAY_SIZE
    min_count = 0
    max_count = MAX_ARRAY_COUNT

    if field.padded_size:
        max_size = field.padded_size

    if isinstance(array_size, ast.SizeField):
        max_size = (1 << array_size.width) - 1 - size_modifier
        min_size = size_modifier
    elif isinstance(array_size, ast.CountField):
        max_count = (1 << array_size.width) - 1
    elif isinstance(array_size, int):
        min_count = array_size
        max_count = array_size

    values = []
    chunk = []
    chunk_size = 0

    while not values:
        element_values = generate_typedef_values(type_decl)
        for element_value in element_values:
            element_size = int(element_value.width / 8)

            if len(chunk) >= max_count or chunk_size + element_size > max_size:
                assert len(chunk) >= min_count
                values.append(Value(chunk))
                chunk = []
                chunk_size = 0

            chunk.append(element_value)
            chunk_size += element_size

    if min_count == 0:
        values.append(Value([]))

    return values

    # The element width is not known, but the array full octet size
    # is known by size field. Generate two arrays: of minimal and maximum
    # size. All unused element values are packed into arrays of varying size.
    if element_width is None and isinstance(array_size, ast.SizeField):
        element_values = generate_typedef_values(type_decl)
        chunk = []
        chunk_size = 0
        values = [Value([])]
        for element_value in element_values:
            assert element_value.width % 8 == 0
            element_size = int(element_value.width / 8)
            if chunk_size + element_size > max_size:
                values.append(Value(chunk))
                chunk = []
            chunk.append(element_value)
            chunk_size += element_size
        if chunk:
            values.append(Value(chunk))
        return values

    # The element width is not known, but the array element count
    # is known statically or by count field. Generate two arrays:
    # of minimal and maximum length. All unused element values are packed into
    # arrays of varying count.
    elif element_width is None and isinstance(array_size, ast.CountField):
        return [Value([])] + generate_list(max_count)

    # The element width is not known, and the array element count is known
    # statically.
    elif element_width is None and isinstance(array_size, int):
        return generate_list(array_size)

    # Neither the count not size is known,
    # generate two arrays: one empty and one including all possible element
    # values.
    elif element_width is None:
        return [Value([])] + generate_list(None)

    # The element width is known, and the array element count is known
    # statically.
    elif isinstance(array_size, int):
        return generate_list(array_size)

    # The element width is known, and the array element count is known
    # by count field.
    elif isinstance(array_size, ast.CountField):
        return [Value([])] + generate_list(max_count)

    # The element width is known, and the array full size is known
    # by size field.
    elif isinstance(array_size, ast.SizeField):
        return [Value([])] + generate_list(max_count)

    # The element width is known, but the array size is unknown.
    # Generate two arrays: one empty and one including all possible element
    # values.
    else:
        return [Value([])] + generate_list(None)


def generate_array_field_values(field: ast.ArrayField) -> List[Value]:
    if field.width is not None:
        return generate_scalar_array_field_values(field)
    else:
        return generate_typedef_array_field_values(field)


def generate_typedef_field_values(
    field: ast.TypedefField, constraints: List[ast.Constraint]
) -> List[Value]:
    type_decl = field.parent.file.typedef_scope[field.type_id]

    # Check for constraint on enum field.
    if isinstance(type_decl, ast.EnumDeclaration):
        for c in constraints:
            if c.id == field.id:
                for tag in type_decl.tags:
                    if tag.id == c.tag_id:
                        return [Value(tag.value, type_decl.width)]
                raise Exception("undefined enum tag")

    # Checksum field needs to known the checksum range.
    if isinstance(type_decl, ast.ChecksumDeclaration):
        return generate_checksum_field_values(field)

    return generate_typedef_values(type_decl)


def generate_field_values(
    field: ast.Field, constraints: List[ast.Constraint], payload: Optional[List[Packet]]
) -> List[Value]:
    if isinstance(field, ast.ChecksumField):
        # Checksum fields are just markers.
        return [Value(0, 0)]

    elif isinstance(field, ast.PaddingField):
        return generate_padding_field_values(field)

    elif isinstance(field, ast.SizeField):
        return generate_size_field_values(field)

    elif isinstance(field, ast.CountField):
        return generate_count_field_values(field)

    elif isinstance(field, (ast.BodyField, ast.PayloadField)) and payload:
        return [Value(p) for p in payload]

    elif isinstance(field, (ast.BodyField, ast.PayloadField)):
        return generate_payload_field_values(field)

    elif isinstance(field, ast.FixedField) and field.enum_id:
        enum_decl = field.parent.file.typedef_scope[field.enum_id]
        for tag in enum_decl.tags:
            if tag.id == field.tag_id:
                return [Value(tag.value, enum_decl.width)]
        raise Exception("undefined enum tag")

    elif isinstance(field, ast.FixedField) and field.width:
        return [Value(field.value, field.width)]

    elif isinstance(field, ast.ReservedField):
        return [Value(0, field.width)]

    elif isinstance(field, ast.ArrayField):
        return generate_array_field_values(field)

    elif isinstance(field, ast.ScalarField):
        for c in constraints:
            if c.id == field.id:
                return [Value(c.value, field.width)]
        mask = (1 << field.width) - 1
        return [
            Value(0, field.width),
            Value(-1 & mask, field.width),
            generator.generate(field.width),
        ]

    elif isinstance(field, ast.TypedefField):
        return generate_typedef_field_values(field, constraints)

    else:
        raise Exception("unsupported field kind")


def generate_fields(
    decl: ast.Declaration,
    constraints: List[ast.Constraint],
    payload: Optional[List[Packet]],
) -> List[List[Field]]:
    return [
        [Field(v, f) for v in generate_field_values(f, constraints, payload)]
        for f in decl.fields
    ]


def generate_fields_recursive(
    scope: dict,
    decl: ast.Declaration,
    constraints: List[ast.Constraint] = [],
    payload: Optional[List[Packet]] = None,
) -> List[List[Field]]:
    fields = generate_fields(decl, constraints, payload)

    if not decl.parent_id:
        return fields

    packets = [Packet(fields, decl) for fields in product(fields)]
    parent_decl = scope[decl.parent_id]
    return generate_fields_recursive(
        scope, parent_decl, constraints + decl.constraints, payload=packets
    )


def generate_struct_values(decl: ast.StructDeclaration) -> List[Packet]:
    fields = generate_fields_recursive(decl.file.typedef_scope, decl)
    return [Packet(fields, decl) for fields in product(fields)]


def generate_packet_values(decl: ast.PacketDeclaration) -> List[Packet]:
    fields = generate_fields_recursive(decl.file.packet_scope, decl)
    return [Packet(fields, decl) for fields in product(fields)]


def generate_typedef_values(decl: ast.Declaration) -> List[Value]:
    if isinstance(decl, ast.EnumDeclaration):
        return [Value(t.value, decl.width) for t in decl.tags]

    elif isinstance(decl, ast.ChecksumDeclaration):
        raise Exception("ChecksumDeclaration handled in typedef field")

    elif isinstance(decl, ast.CustomFieldDeclaration):
        raise Exception("TODO custom field")

    elif isinstance(decl, ast.StructDeclaration):
        return [Value(p) for p in generate_struct_values(decl)]

    else:
        raise Exception("unsupported typedef declaration type")


def product(fields: List[List[Field]]) -> List[List[Field]]:
    """Perform a cartesian product of generated options for packet field values."""

    def aux(vec: List[List[Field]]) -> List[List[Field]]:
        if len(vec) == 0:
            return [[]]
        return [[item.clone()] + items for item in vec[0] for items in aux(vec[1:])]

    count = 1
    max_len = 0
    for f in fields:
        count *= len(f)
        max_len = max(max_len, len(f))

    # Limit products to 32 elements to prevent combinatorial
    # explosion.
    if count <= 32:
        return aux(fields)

    # If too many products, select samples which test all fields value
    # values at the minimum.
    else:
        return [[f[idx % len(f)] for f in fields] for idx in range(0, max_len + 1)]


def serialize_values(file: ast.File, values: List[Value]) -> List[dict]:
    results = []
    for v in values:
        v.finalize()
        packed = v.serialize(file.endianness.value == "big_endian")
        result = {
            "packed": "".join([f"{b:02x}" for b in packed]),
            "unpacked": v.to_json(),
        }
        if v.ref.parent_id:
            result["packet"] = v.ref.id
        results.append(result)
    return results


def run(input: Path, packet: List[str]):
    with open(input) as f:
        file = ast.File.from_json(json.load(f))
    core.desugar(file)

    results = dict()
    for decl in file.packet_scope.values():
        if core.get_derived_packets(decl) or (packet and decl.id not in packet):
            continue

        try:
            values = generate_packet_values(decl)
            ancestor = core.get_packet_ancestor(decl)
            results[ancestor.id] = results.get(ancestor.id, []) + serialize_values(
                file, values
            )
        except Exception as exn:
            print(
                f"Skipping packet {decl.id}; cannot generate values: {exn}",
                file=sys.stderr,
            )

    results = [{"packet": k, "tests": v} for (k, v) in results.items()]
    json.dump(results, sys.stdout, indent=2)


def main() -> int:
    """Generate test vectors for top-level PDL packets."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--input", type=Path, required=True, help="Input PDL-JSON source"
    )
    parser.add_argument(
        "--packet",
        type=lambda x: x.split(","),
        required=False,
        action="extend",
        default=[],
        help="Select PDL packet to test",
    )
    return run(**vars(parser.parse_args()))


if __name__ == "__main__":
    sys.exit(main())
