from dataclasses import dataclass, field
from typing import Optional, List, Dict, Tuple

constructors_ = dict()


def node(kind: str):

    def decorator(cls):
        cls = dataclass(cls)
        constructors_[kind] = cls
        return cls

    return decorator


@dataclass
class SourceLocation:
    offset: int
    line: int
    column: int


@dataclass
class SourceRange:
    file: int
    start: SourceLocation
    end: SourceLocation


@dataclass
class Node:
    kind: str
    loc: SourceLocation


@node('tag')
class Tag(Node):
    id: str
    value: Optional[int] = field(default=None)
    range: Optional[Tuple[int, int]] = field(default=None)
    tags: Optional[List['Tag']] = field(default=None)


@node('constraint')
class Constraint(Node):
    id: str
    value: Optional[int]
    tag_id: Optional[str]


@dataclass
class Field(Node):
    parent: Node = field(init=False)


@node('checksum_field')
class ChecksumField(Field):
    field_id: str


@node('padding_field')
class PaddingField(Field):
    size: int


@node('size_field')
class SizeField(Field):
    field_id: str
    width: int


@node('count_field')
class CountField(Field):
    field_id: str
    width: int


@node('body_field')
class BodyField(Field):
    id: str = field(init=False, default='_body_')


@node('payload_field')
class PayloadField(Field):
    size_modifier: Optional[str]
    id: str = field(init=False, default='_payload_')


@node('fixed_field')
class FixedField(Field):
    width: Optional[int] = None
    value: Optional[int] = None
    enum_id: Optional[str] = None
    tag_id: Optional[str] = None

    @property
    def type(self) -> Optional['Declaration']:
        return self.parent.file.typedef_scope[self.enum_id] if self.enum_id else None


@node('reserved_field')
class ReservedField(Field):
    width: int


@node('array_field')
class ArrayField(Field):
    id: str
    width: Optional[int]
    type_id: Optional[str]
    size_modifier: Optional[str]
    size: Optional[int]
    padded_size: Optional[int] = field(init=False, default=None)

    @property
    def type(self) -> Optional['Declaration']:
        return self.parent.file.typedef_scope[self.type_id] if self.type_id else None


@node('scalar_field')
class ScalarField(Field):
    id: str
    width: int


@node('typedef_field')
class TypedefField(Field):
    id: str
    type_id: str

    @property
    def type(self) -> 'Declaration':
        return self.parent.file.typedef_scope[self.type_id]


@node('group_field')
class GroupField(Field):
    group_id: str
    constraints: List[Constraint]


@dataclass
class Declaration(Node):
    file: 'File' = field(init=False)

    def __post_init__(self):
        if hasattr(self, 'fields'):
            for f in self.fields:
                f.parent = self


@node('endianness_declaration')
class EndiannessDeclaration(Node):
    value: str


@node('checksum_declaration')
class ChecksumDeclaration(Declaration):
    id: str
    function: str
    width: int


@node('custom_field_declaration')
class CustomFieldDeclaration(Declaration):
    id: str
    function: str
    width: Optional[int]


@node('enum_declaration')
class EnumDeclaration(Declaration):
    id: str
    tags: List[Tag]
    width: int


@node('packet_declaration')
class PacketDeclaration(Declaration):
    id: str
    parent_id: Optional[str]
    constraints: List[Constraint]
    fields: List[Field]

    @property
    def parent(self) -> Optional['PacketDeclaration']:
        return self.file.packet_scope[self.parent_id] if self.parent_id else None


@node('struct_declaration')
class StructDeclaration(Declaration):
    id: str
    parent_id: Optional[str]
    constraints: List[Constraint]
    fields: List[Field]

    @property
    def parent(self) -> Optional['StructDeclaration']:
        return self.file.typedef_scope[self.parent_id] if self.parent_id else None


@node('group_declaration')
class GroupDeclaration(Declaration):
    id: str
    fields: List[Field]


@dataclass
class File:
    endianness: EndiannessDeclaration
    declarations: List[Declaration]
    packet_scope: Dict[str, Declaration] = field(init=False)
    typedef_scope: Dict[str, Declaration] = field(init=False)
    group_scope: Dict[str, Declaration] = field(init=False)

    def __post_init__(self):
        self.packet_scope = dict()
        self.typedef_scope = dict()
        self.group_scope = dict()

        # Construct the toplevel declaration scopes.
        for d in self.declarations:
            d.file = self
            if isinstance(d, PacketDeclaration):
                self.packet_scope[d.id] = d
            elif isinstance(d, GroupDeclaration):
                self.group_scope[d.id] = d
            else:
                self.typedef_scope[d.id] = d

    @staticmethod
    def from_json(obj: object) -> 'File':
        """Import a File exported as JSON object by the PDL parser."""
        endianness = convert_(obj['endianness'])
        declarations = convert_(obj['declarations'])
        return File(endianness, declarations)

    @property
    def byteorder(self) -> str:
        return 'little' if self.endianness.value == 'little_endian' else 'big'


def convert_(obj: object) -> object:
    if obj is None:
        return None
    if isinstance(obj, (int, str)):
        return obj
    if isinstance(obj, list):
        return [convert_(elt) for elt in obj]
    if isinstance(obj, object):
        if 'start' in obj.keys() and 'end' in obj.keys():
            return (objs.start, obj.end)
        kind = obj['kind']
        loc = obj['loc']
        loc = SourceRange(loc['file'], SourceLocation(**loc['start']), SourceLocation(**loc['end']))
        constructor = constructors_.get(kind)
        members = {'loc': loc, 'kind': kind}
        for name, value in obj.items():
            if name != 'kind' and name != 'loc':
                members[name] = convert_(value)
        return constructor(**members)
    raise Exception('Unhandled json object type')
