from typing import Optional, List, Dict, Union, Tuple
from .ast import *


def desugar_field_(field: Field, constraints: Dict[str, Constraint]) -> List[Field]:
    """Inline group and constrained fields.
    Constrained fields are transformed into fixed fields.
    Group fields are inlined and recursively desugared."""

    if isinstance(field, ScalarField) and field.id in constraints:
        value = constraints[field.id].value
        return [FixedField(kind='fixed_field', loc=field.loc, width=field.width, value=value)]

    elif isinstance(field, TypedefField) and field.id in constraints:
        tag_id = constraints[field.id].tag_id
        return [FixedField(kind='fixed_field', loc=field.loc, enum_id=field.type_id, tag_id=tag_id)]

    elif isinstance(field, GroupField):
        group = field.parent.grammar.group_scope[field.group_id]
        constraints = dict([(c.id, c) for c in field.constraints])
        fields = []
        for f in group.fields:
            fields.extend(desugar_field_(f, constraints))
        return fields

    else:
        return [field]


def desugar(grammar: Grammar):
    """Inline group fields.
    Constrained fields are transformed into fixed fields.
    Group declarations are removed from the grammar object.
    **The original grammar object is modified inline.**"""

    declarations = []
    for d in grammar.declarations:
        if isinstance(d, GroupDeclaration):
            continue

        if isinstance(d, (PacketDeclaration, StructDeclaration)):
            fields = []
            for f in d.fields:
                fields.extend(desugar_field_(f, {}))
            d.fields = fields

        declarations.append(d)

    grammar.declarations = declarations
    grammar.group_scope = {}


def get_packet_field(packet: Union[PacketDeclaration, StructDeclaration], id: str) -> Optional[Field]:
    """Return the field with selected identifier declared in the provided
    packet or its ancestors."""
    for f in packet.fields:
        if getattr(f, 'id', None) == id:
            return f
    if isinstance(packet, PacketDeclaration) and packet.parent_id:
        parent = packet.grammar.packet_scope[packet.parent_id]
        return get_packet_field(parent, id)
    elif isinstance(packet, StructDeclaration) and packet.parent_id:
        parent = packet.grammar.typedef_scope[packet.parent_id]
        return get_packet_field(parent, id)
    else:
        return None


def get_derived_packets(decl: Union[PacketDeclaration, StructDeclaration]
                       ) -> List[Tuple[List[Constraint], Union[PacketDeclaration, StructDeclaration]]]:
    """Return the list of packets or structs that immediately derive from the
    selected packet or struct, coupled with the field constraints.
    Packet aliases (containing no field declarations other than a payload)
    are traversed."""

    children = []
    for d in decl.grammar.declarations:
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
        return field.width * 8

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


def is_bit_field(field: Field) -> bool:
    """Identify fields that can have bit granularity.
    These include: ScalarField, FixedField, TypedefField with enum type,
    SizeField, and CountField. Returns the size of the field in bits."""

    if isinstance(field, (ScalarField, SizeField, CountField, FixedField, ReservedField)):
        return True

    elif isinstance(field, TypedefField) and isinstance(field.type, EnumDeclaration):
        return True

    else:
        return False
