from typing import Optional, List, Dict, Union, Tuple
from .ast import *


def desugar_field_(field: Field, previous: Field, constraints: Dict[str, Constraint]) -> List[Field]:
    """Inline group and constrained fields.
    Constrained fields are transformed into fixed fields.
    Group fields are inlined and recursively desugared."""

    if isinstance(field, ScalarField) and field.id in constraints:
        value = constraints[field.id].value
        fixed = FixedField(kind='fixed_field', loc=field.loc, width=field.width, value=value)
        fixed.parent = field.parent
        return [fixed]

    elif isinstance(field, PaddingField):
        previous.padded_size = field.size
        field.padded_field = previous
        return [field]

    elif isinstance(field, TypedefField) and field.id in constraints:
        tag_id = constraints[field.id].tag_id
        fixed = FixedField(kind='fixed_field', loc=field.loc, enum_id=field.type_id, tag_id=tag_id)
        fixed.parent = field.parent
        return [fixed]

    elif isinstance(field, GroupField):
        group = field.parent.file.group_scope[field.group_id]
        constraints = dict([(c.id, c) for c in field.constraints])
        fields = []
        for f in group.fields:
            fields.extend(desugar_field_(f, previous, constraints))
            previous = f
        return fields

    else:
        return [field]


def desugar(file: File):
    """Inline group fields.
    Constrained fields are transformed into fixed fields.
    Group declarations are removed from the file object.
    **The original file object is modified inline.**"""

    declarations = []
    for d in file.declarations:
        if isinstance(d, GroupDeclaration):
            continue

        if isinstance(d, (PacketDeclaration, StructDeclaration)):
            fields = []
            for f in d.fields:
                fields.extend(desugar_field_(f, fields[-1] if len(fields) > 0 else None, {}))
            d.fields = fields

        declarations.append(d)

    file.declarations = declarations
    file.group_scope = {}


def make_reserved_field(width: int) -> ReservedField:
    """Create a reserved field of specified width."""
    return ReservedField(kind='reserved_field', loc=None, width=width)


def get_packet_field(packet: Union[PacketDeclaration, StructDeclaration], id: str) -> Optional[Field]:
    """Return the field with selected identifier declared in the provided
    packet or its ancestors."""
    for f in packet.fields:
        if getattr(f, 'id', None) == id:
            return f
    if isinstance(packet, PacketDeclaration) and packet.parent_id:
        parent = packet.file.packet_scope[packet.parent_id]
        return get_packet_field(parent, id)
    elif isinstance(packet, StructDeclaration) and packet.parent_id:
        parent = packet.file.typedef_scope[packet.parent_id]
        return get_packet_field(parent, id)
    else:
        return None


def get_packet_shift(packet: Union[PacketDeclaration, StructDeclaration]) -> int:
    """Return the bit shift of the payload or body field in the parent packet.

    When using packet derivation on bit fields, the body may be shifted.
    The shift is handled statically in the implementation of child packets,
    and the incomplete field is included in the body.
    ```
    packet Basic {
        type: 1,
        _body_
    }
    ```
    """

    # Traverse empty parents.
    parent = packet.parent
    while parent and len(parent.fields) == 1:
        parent = parent.parent

    if not parent:
        return 0

    shift = 0
    for f in packet.parent.fields:
        if isinstance(f, (BodyField, PayloadField)):
            return 0 if (shift % 8) == 0 else shift
        else:
            # Fields that do not have a constant size are assumed to start
            # on a byte boundary, and measure an integral number of bytes.
            # Start the count over.
            size = get_field_size(f)
            shift = 0 if size is None else shift + size

    # No payload or body in parent packet.
    # Not raising an error, the generation will fail somewhere else.
    return 0


def get_packet_ancestor(
        decl: Union[PacketDeclaration, StructDeclaration]) -> Union[PacketDeclaration, StructDeclaration]:
    """Return the root ancestor of the selected packet or struct."""
    if decl.parent_id is None:
        return decl
    else:
        return get_packet_ancestor(decl.file.packet_scope[decl.parent_id])


def get_derived_packets(
    decl: Union[PacketDeclaration, StructDeclaration]
) -> List[Tuple[List[Constraint], Union[PacketDeclaration, StructDeclaration]]]:
    """Return the list of packets or structs that immediately derive from the
    selected packet or struct, coupled with the field constraints.
    Packet aliases (containing no field declarations other than a payload)
    are traversed."""

    children = []
    for d in decl.file.declarations:
        if type(d) is type(decl) and d.parent_id == decl.id:
            if (len(d.fields) == 1 and isinstance(d.fields[0], (PayloadField, BodyField))):
                children.extend([(d.constraints + sub_constraints, sub_child)
                                 for (sub_constraints, sub_child) in get_derived_packets(d)])
            else:
                children.append((d.constraints, d))
    return children


def get_field_size(field: Field, skip_payload: bool = False) -> Optional[int]:
    """Determine the size of a field in bits, if possible.
    If the field is dynamically sized (e.g. unsized array or payload field),
    None is returned instead. If skip_payload is set, payload and body fields
    are counted as having size 0 rather than a variable size."""

    if isinstance(field, (ScalarField, SizeField, CountField, ReservedField)):
        return field.width

    elif isinstance(field, FixedField):
        return field.width or field.type.width

    elif isinstance(field, PaddingField):
        # Padding field width is added to the padded field size.
        return 0

    elif isinstance(field, ArrayField) and field.padded_size is not None:
        return field.padded_size * 8

    elif isinstance(field, ArrayField) and field.size is not None:
        element_width = field.width or get_declaration_size(field.type)
        return element_width * field.size if element_width is not None else None

    elif isinstance(field, TypedefField):
        return get_declaration_size(field.type)

    elif isinstance(field, ChecksumField):
        return 0

    elif isinstance(field, (PayloadField, BodyField)) and skip_payload:
        return 0

    else:
        return None


def get_declaration_size(decl: Declaration, skip_payload: bool = False) -> Optional[int]:
    """Determine the size of a declaration type in bits, if possible.
    If the type is dynamically sized (e.g. contains an array or payload),
    None is returned instead. If skip_payload is set, payload and body fields
    are counted as having size 0 rather than a variable size."""

    if isinstance(decl, (EnumDeclaration, CustomFieldDeclaration, ChecksumDeclaration)):
        return decl.width

    elif isinstance(decl, (PacketDeclaration, StructDeclaration)):
        parent = decl.parent
        packet_size = get_declaration_size(parent, skip_payload=True) if parent else 0
        if packet_size is None:
            return None
        for f in decl.fields:
            field_size = get_field_size(f, skip_payload=skip_payload)
            if field_size is None:
                return None
            packet_size += field_size
        return packet_size

    else:
        return None


def get_array_field_size(field: ArrayField) -> Union[None, int, Field]:
    """Return the array static size, size field, or count field.
    If the array is unsized None is returned instead."""

    if field.size is not None:
        return field.size
    for f in field.parent.fields:
        if isinstance(f, (SizeField, CountField)) and f.field_id == field.id:
            return f
    return None


def get_payload_field_size(field: Union[PayloadField, BodyField]) -> Optional[Field]:
    """Return the payload or body size field.
    If the payload is unsized None is returned instead."""

    for f in field.parent.fields:
        if isinstance(f, SizeField) and f.field_id == field.id:
            return f
    return None


def get_array_element_size(field: ArrayField) -> Optional[int]:
    """Return the array element size, if possible.
    If the element size is not known at compile time,
    None is returned instead."""

    return field.width or get_declaration_size(field.type)


def get_field_offset_from_start(field: Field) -> Optional[int]:
    """Return the field bit offset from the start of the parent packet, if it
    can be statically computed. If the offset is variable None is returned
    instead."""
    offset = 0
    field_index = field.parent.fields.index(field)
    for f in field.parent.fields[:field_index]:
        size = get_field_size(f)
        if size is None:
            return None

        offset += size
    return offset


def get_field_offset_from_end(field: Field) -> Optional[int]:
    """Return the field bit offset from the end of the parent packet, if it
    can be statically computed. If the offset is variable None is returned
    instead. The selected field size is not counted towards the offset."""
    offset = 0
    field_index = field.parent.fields.index(field)
    for f in field.parent.fields[field_index + 1:]:
        size = get_field_size(f)
        if size is None:
            return None
        offset += size
    return offset


def is_bit_field(field: Field) -> bool:
    """Identify fields that can have bit granularity.
    These include: ScalarField, FixedField, TypedefField with enum type,
    SizeField, and CountField."""

    if isinstance(field, (ScalarField, SizeField, CountField, FixedField, ReservedField)):
        return True

    elif isinstance(field, TypedefField) and isinstance(field.type, EnumDeclaration):
        return True

    else:
        return False
