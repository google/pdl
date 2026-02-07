# File generated from tests/canonical/le_test_file.pdl, with the command:
#  pdlc ...
# /!\ Do not edit by hand.

from tests.custom_types import SizedCustomField, UnsizedCustomField, Checksum
from dataclasses import dataclass, field, fields
from typing import Optional, List, Tuple, Union
import enum
import inspect

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
        return 0

    def show(self, prefix: str = '') -> None:
        print(f'{self.__class__.__name__}')

        def print_val(p: str, pp: str, name: str, align: int, typ: object, val: object) -> None:
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
            elif getattr(typ, '__origin__', None) is list:
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
        align = max((len(f.name) for f in fields(self) if f.name != 'payload'), default=0)

        for (idx, f) in enumerate(fields(self)):
            p  = prefix + ('├── ' if idx != last else '└── ')
            pp = prefix + ('│   ' if idx != last else '    ')
            val = getattr(self, f.name)

            print_val(p, pp, f.name, align, f.type, val)

if (not callable(getattr(SizedCustomField, 'parse', None)) or
    not callable(getattr(SizedCustomField, 'parse_all', None))):
    raise Exception('The custom field type SizedCustomField does not implement the parse method')

if (not callable(getattr(UnsizedCustomField, 'parse', None)) or
    not callable(getattr(UnsizedCustomField, 'parse_all', None))):
    raise Exception('The custom field type UnsizedCustomField does not implement the parse method')

if not callable(Checksum):
    raise Exception('Checksum is not callable')

class Enum7(enum.IntEnum):
    A = 0x1
    B = 0x2

    @staticmethod
    def from_int(v: int) -> Union[int, 'Enum7']:
        try:
            return Enum7(v)
        except ValueError:
            raise ValueError('Invalid enum value')

class Enum16(enum.IntEnum):
    A = 0xaabb
    B = 0xccdd

    @staticmethod
    def from_int(v: int) -> Union[int, 'Enum16']:
        try:
            return Enum16(v)
        except ValueError:
            raise ValueError('Invalid enum value')

@dataclass
class SizedStruct(Packet):
    a: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['SizedStruct', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['a'] = span[0]
        span = span[1:]
        return SizedStruct(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if self.a > 0xff:
            raise ValueError("Invalid scalar value SizedStruct::a: {self.a} > 0xff")
        _span.append((self.a << 0))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 1

@dataclass
class UnsizedStruct(Packet):
    array: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['UnsizedStruct', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_size = (span[0] >> 0) & 0x3
        span = span[1:]
        if len(span) < array_size:
            raise Exception('Invalid packet size')
        array = []
        for n in range(array_size):
            array.append(int.from_bytes(span[n:n + 1], byteorder='little'))
        fields['array'] = array
        span = span[array_size:]
        return UnsizedStruct(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        array_size = len(self.array)
        if array_size > 0x3:
            raise ValueError("Invalid size value UnsizedStruct::array: {array_size} > 0x3")
        _span.append((array_size << 0))
        _span.extend(self.array)
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.array) + 1

@dataclass
class UnknownSizeStruct(Packet):
    array: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['UnknownSizeStruct', bytes]:
        fields = {'payload': None}
        array = []
        for n in range(len(span)):
            array.append(int.from_bytes(span[n:n + 1], byteorder='little'))
        fields['array'] = array
        span = bytes()
        return UnknownSizeStruct(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.array)
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.array)

@dataclass
class ScalarParent(Packet):
    a: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['ScalarParent', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['a'] = span[0]
        _payload__size = span[1]
        span = span[2:]
        if len(span) < _payload__size:
            raise Exception('Invalid packet size')
        payload = span[:_payload__size]
        span = span[_payload__size:]
        fields['payload'] = payload
        try:
            child, remainder = AliasedChild_A.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception:
            pass
        try:
            child, remainder = AliasedChild_B.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception:
            pass
        try:
            child, remainder = ScalarChild_A.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception:
            pass
        try:
            child, remainder = ScalarChild_B.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception:
            pass
        return ScalarParent(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if self.a > 0xff:
            raise ValueError("Invalid scalar value ScalarParent::a: {self.a} > 0xff")
        _span.append((self.a << 0))
        _payload_size = len(payload or self.payload or [])
        if _payload_size > 0xff:
            raise ValueError("Invalid size value ScalarParent::_payload_: {_payload_size} > 0xff")
        _span.append((_payload_size << 0))
        _span.extend(payload or self.payload or [])
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.payload) + 2

@dataclass
class EnumParent(Packet):
    a: Enum16 = field(kw_only=True, default=Enum16.A)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['EnumParent', bytes]:
        fields = {'payload': None}
        if len(span) < 3:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        fields['a'] = Enum16.from_int(value_)
        _payload__size = span[2]
        span = span[3:]
        if len(span) < _payload__size:
            raise Exception('Invalid packet size')
        payload = span[:_payload__size]
        span = span[_payload__size:]
        fields['payload'] = payload
        try:
            child, remainder = EnumChild_A.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception:
            pass
        try:
            child, remainder = EnumChild_B.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception:
            pass
        return EnumParent(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(int.to_bytes((self.a << 0), length=2, byteorder='little'))
        _payload_size = len(payload or self.payload or [])
        if _payload_size > 0xff:
            raise ValueError("Invalid size value EnumParent::_payload_: {_payload_size} > 0xff")
        _span.append((_payload_size << 0))
        _span.extend(payload or self.payload or [])
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.payload) + 3

@dataclass
class EmptyParent(ScalarParent):


    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['EmptyParent', bytes]:
        payload = span
        span = bytes([])
        fields['payload'] = payload
        try:
            child, remainder = AliasedChild_A.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception:
            pass
        try:
            child, remainder = AliasedChild_B.parse(fields.copy(), payload)
            if remainder:
                raise Exception('Unexpected parsing remainder')
            return child, span
        except Exception:
            pass
        return EmptyParent(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        return ScalarParent.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return len(self.payload)

@dataclass
class Packet_Scalar_Field(Packet):
    a: int = field(kw_only=True, default=0)
    c: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Scalar_Field', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:8], byteorder='little')
        fields['a'] = (value_ >> 0) & 0x7f
        fields['c'] = (value_ >> 7) & 0x1ffffffffffffff
        span = span[8:]
        return Packet_Scalar_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if self.a > 0x7f:
            raise ValueError("Invalid scalar value Packet_Scalar_Field::a: {self.a} > 0x7f")
        if self.c > 0x1ffffffffffffff:
            raise ValueError("Invalid scalar value Packet_Scalar_Field::c: {self.c} > 0x1ffffffffffffff")
        _value = (
            (self.a << 0) |
            (self.c << 7)
        )
        _span.extend(int.to_bytes(_value, length=8, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 8

@dataclass
class Packet_Enum_Field(Packet):
    a: Enum7 = field(kw_only=True, default=Enum7.A)
    c: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Enum_Field', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:8], byteorder='little')
        fields['a'] = Enum7.from_int((value_ >> 0) & 0x7f)
        fields['c'] = (value_ >> 7) & 0x1ffffffffffffff
        span = span[8:]
        return Packet_Enum_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if self.c > 0x1ffffffffffffff:
            raise ValueError("Invalid scalar value Packet_Enum_Field::c: {self.c} > 0x1ffffffffffffff")
        _value = (
            (self.a << 0) |
            (self.c << 7)
        )
        _span.extend(int.to_bytes(_value, length=8, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 8

@dataclass
class Packet_Reserved_Field(Packet):
    a: int = field(kw_only=True, default=0)
    c: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Reserved_Field', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:8], byteorder='little')
        fields['a'] = (value_ >> 0) & 0x7f
        fields['c'] = (value_ >> 9) & 0x7fffffffffffff
        span = span[8:]
        return Packet_Reserved_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if self.a > 0x7f:
            raise ValueError("Invalid scalar value Packet_Reserved_Field::a: {self.a} > 0x7f")
        if self.c > 0x7fffffffffffff:
            raise ValueError("Invalid scalar value Packet_Reserved_Field::c: {self.c} > 0x7fffffffffffff")
        _value = (
            (self.a << 0) |
            (self.c << 9)
        )
        _span.extend(int.to_bytes(_value, length=8, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 8

@dataclass
class Packet_Size_Field(Packet):
    a: int = field(kw_only=True, default=0)
    b: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Size_Field', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:8], byteorder='little')
        b_size = (value_ >> 0) & 0x7
        fields['a'] = (value_ >> 3) & 0x1fffffffffffffff
        span = span[8:]
        if len(span) < b_size:
            raise Exception('Invalid packet size')
        b = []
        for n in range(b_size):
            b.append(int.from_bytes(span[n:n + 1], byteorder='little'))
        fields['b'] = b
        span = span[b_size:]
        return Packet_Size_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        b_size = len(self.b)
        if b_size > 0x7:
            raise ValueError("Invalid size value Packet_Size_Field::b: {b_size} > 0x7")
        if self.a > 0x1fffffffffffffff:
            raise ValueError("Invalid scalar value Packet_Size_Field::a: {self.a} > 0x1fffffffffffffff")
        _value = (
            (b_size << 0) |
            (self.a << 3)
        )
        _span.extend(int.to_bytes(_value, length=8, byteorder='little'))
        _span.extend(self.b)
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.b) + 8

@dataclass
class Packet_Count_Field(Packet):
    a: int = field(kw_only=True, default=0)
    b: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Count_Field', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:8], byteorder='little')
        b_count = (value_ >> 0) & 0x7
        fields['a'] = (value_ >> 3) & 0x1fffffffffffffff
        span = span[8:]
        if len(span) < b_count:
            raise Exception('Invalid packet size')
        b = []
        for n in range(b_count):
            b.append(int.from_bytes(span[n:n + 1], byteorder='little'))
        fields['b'] = b
        span = span[b_count:]
        return Packet_Count_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if len(self.b) > 0x7:
            raise ValueError("Invalid count value Packet_Count_Field::b: {len(self.b)} > 0x7")
        if self.a > 0x1fffffffffffffff:
            raise ValueError("Invalid scalar value Packet_Count_Field::a: {self.a} > 0x1fffffffffffffff")
        _value = (
            (len(self.b) << 0) |
            (self.a << 3)
        )
        _span.extend(int.to_bytes(_value, length=8, byteorder='little'))
        _span.extend(self.b)
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.b) + 8

@dataclass
class Packet_FixedScalar_Field(Packet):
    b: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_FixedScalar_Field', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:8], byteorder='little')
        if (value_ >> 0) & 0x7f != 0x7:
            raise Exception('Unexpected fixed field value')
        fields['b'] = (value_ >> 7) & 0x1ffffffffffffff
        span = span[8:]
        return Packet_FixedScalar_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if self.b > 0x1ffffffffffffff:
            raise ValueError("Invalid scalar value Packet_FixedScalar_Field::b: {self.b} > 0x1ffffffffffffff")
        _value = (
            (0x7 << 0) |
            (self.b << 7)
        )
        _span.extend(int.to_bytes(_value, length=8, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 8

@dataclass
class Packet_FixedEnum_Field(Packet):
    b: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_FixedEnum_Field', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:8], byteorder='little')
        if (value_ >> 0) & 0x7f != Enum7.A:
            raise Exception('Unexpected fixed field value')
        fields['b'] = (value_ >> 7) & 0x1ffffffffffffff
        span = span[8:]
        return Packet_FixedEnum_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if self.b > 0x1ffffffffffffff:
            raise ValueError("Invalid scalar value Packet_FixedEnum_Field::b: {self.b} > 0x1ffffffffffffff")
        _value = (
            (Enum7.A << 0) |
            (self.b << 7)
        )
        _span.extend(int.to_bytes(_value, length=8, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 8

@dataclass
class Packet_Payload_Field_VariableSize(Packet):


    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Payload_Field_VariableSize', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        _payload__size = (span[0] >> 0) & 0x7
        span = span[1:]
        if len(span) < _payload__size:
            raise Exception('Invalid packet size')
        payload = span[:_payload__size]
        span = span[_payload__size:]
        fields['payload'] = payload
        return Packet_Payload_Field_VariableSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _payload_size = len(payload or self.payload or [])
        if _payload_size > 0x7:
            raise ValueError("Invalid size value Packet_Payload_Field_VariableSize::_payload_: {_payload_size} > 0x7")
        _span.append((_payload_size << 0))
        _span.extend(payload or self.payload or [])
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.payload) + 1

@dataclass
class Packet_Payload_Field_SizeModifier(Packet):


    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Payload_Field_SizeModifier', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        _payload__size = (span[0] >> 0) & 0x7
        span = span[1:]
        _payload__size -= +2
        if len(span) < _payload__size:
            raise Exception('Invalid packet size')
        payload = span[:_payload__size]
        span = span[_payload__size:]
        fields['payload'] = payload
        return Packet_Payload_Field_SizeModifier(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _payload_size = len(payload or self.payload or []) + +2
        if _payload_size > 0x7:
            raise ValueError("Invalid size value Packet_Payload_Field_SizeModifier::_payload_: {_payload_size} > 0x7")
        _span.append((_payload_size << 0))
        _span.extend(payload or self.payload or [])
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.payload) + 1

@dataclass
class Packet_Payload_Field_UnknownSize(Packet):
    a: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Payload_Field_UnknownSize', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        payload = span[:-2]
        span = span[-2:]
        fields['payload'] = payload
        if len(span) < 2:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        fields['a'] = value_
        span = span[2:]
        return Packet_Payload_Field_UnknownSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        if self.a > 0xffff:
            raise ValueError("Invalid scalar value Packet_Payload_Field_UnknownSize::a: {self.a} > 0xffff")
        _span.extend(int.to_bytes((self.a << 0), length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.payload) + 2

@dataclass
class Packet_Payload_Field_UnknownSize_Terminal(Packet):
    a: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Payload_Field_UnknownSize_Terminal', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        fields['a'] = value_
        span = span[2:]
        payload = span
        span = bytes([])
        fields['payload'] = payload
        return Packet_Payload_Field_UnknownSize_Terminal(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if self.a > 0xffff:
            raise ValueError("Invalid scalar value Packet_Payload_Field_UnknownSize_Terminal::a: {self.a} > 0xffff")
        _span.extend(int.to_bytes((self.a << 0), length=2, byteorder='little'))
        _span.extend(payload or self.payload or [])
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.payload) + 2

@dataclass
class Packet_Body_Field_VariableSize(Packet):


    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Body_Field_VariableSize', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        _body__size = (span[0] >> 0) & 0x7
        span = span[1:]
        if len(span) < _body__size:
            raise Exception('Invalid packet size')
        payload = span[:_body__size]
        span = span[_body__size:]
        fields['payload'] = payload
        return Packet_Body_Field_VariableSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _payload_size = len(payload or self.payload or [])
        if _payload_size > 0x7:
            raise ValueError("Invalid size value Packet_Body_Field_VariableSize::_body_: {_payload_size} > 0x7")
        _span.append((_payload_size << 0))
        _span.extend(payload or self.payload or [])
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.payload) + 1

@dataclass
class Packet_Body_Field_UnknownSize(Packet):
    a: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Body_Field_UnknownSize', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        payload = span[:-2]
        span = span[-2:]
        fields['payload'] = payload
        if len(span) < 2:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        fields['a'] = value_
        span = span[2:]
        return Packet_Body_Field_UnknownSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(payload or self.payload or [])
        if self.a > 0xffff:
            raise ValueError("Invalid scalar value Packet_Body_Field_UnknownSize::a: {self.a} > 0xffff")
        _span.extend(int.to_bytes((self.a << 0), length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.payload) + 2

@dataclass
class Packet_Body_Field_UnknownSize_Terminal(Packet):
    a: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Body_Field_UnknownSize_Terminal', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        fields['a'] = value_
        span = span[2:]
        payload = span
        span = bytes([])
        fields['payload'] = payload
        return Packet_Body_Field_UnknownSize_Terminal(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if self.a > 0xffff:
            raise ValueError("Invalid scalar value Packet_Body_Field_UnknownSize_Terminal::a: {self.a} > 0xffff")
        _span.extend(int.to_bytes((self.a << 0), length=2, byteorder='little'))
        _span.extend(payload or self.payload or [])
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.payload) + 2

@dataclass
class Packet_ScalarGroup_Field(Packet):


    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_ScalarGroup_Field', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        if value_ != 0x2a:
            raise Exception('Unexpected fixed field value')
        span = span[2:]
        return Packet_ScalarGroup_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(int.to_bytes((0x2a << 0), length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 2

@dataclass
class Packet_EnumGroup_Field(Packet):


    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_EnumGroup_Field', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        if value_ != Enum16.A:
            raise Exception('Unexpected fixed field value')
        span = span[2:]
        return Packet_EnumGroup_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(int.to_bytes((Enum16.A << 0), length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 2

@dataclass
class Packet_Checksum_Field_FromStart(Packet):
    a: int = field(kw_only=True, default=0)
    b: int = field(kw_only=True, default=0)
    crc: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Checksum_Field_FromStart', bytes]:
        fields = {'payload': None}
        if len(span) < 5:
            raise Exception('Invalid packet size')
        if len(span) < 5:
            raise Exception('Invalid packet size')
        crc = span[4]
        fields['crc'] = crc
        computed_crc = Checksum(span[:4])
        if computed_crc != crc:
            raise Exception(f'Invalid checksum computation: {computed_crc} != {crc}')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        fields['a'] = value_
        value_ = int.from_bytes(span[2:4], byteorder='little')
        fields['b'] = value_
        span = span[5:]
        return Packet_Checksum_Field_FromStart(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _checksum_start = len(_span)
        if self.a > 0xffff:
            raise ValueError("Invalid scalar value Packet_Checksum_Field_FromStart::a: {self.a} > 0xffff")
        _span.extend(int.to_bytes((self.a << 0), length=2, byteorder='little'))
        if self.b > 0xffff:
            raise ValueError("Invalid scalar value Packet_Checksum_Field_FromStart::b: {self.b} > 0xffff")
        _span.extend(int.to_bytes((self.b << 0), length=2, byteorder='little'))
        _checksum = Checksum(_span[_checksum_start:])
        _span.append(_checksum)
        return bytes(_span)

    @property
    def size(self) -> int:
        return 5

@dataclass
class Packet_Checksum_Field_FromEnd(Packet):
    crc: int = field(kw_only=True, default=0)
    a: int = field(kw_only=True, default=0)
    b: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Checksum_Field_FromEnd', bytes]:
        fields = {'payload': None}
        if len(span) < 0:
            raise Exception('Invalid packet size')
        if len(span) < 5:
            raise Exception('Invalid packet size')
        crc = span[-5]
        fields['crc'] = crc
        computed_crc = Checksum(span[:-5])
        if computed_crc != crc:
            raise Exception(f'Invalid checksum computation: {computed_crc} != {crc}')
        if len(span) < 5:
            raise Exception('Invalid packet size')
        payload = span[:-5]
        span = span[-5:]
        fields['payload'] = payload
        if len(span) < 5:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[1:3], byteorder='little')
        fields['a'] = value_
        value_ = int.from_bytes(span[3:5], byteorder='little')
        fields['b'] = value_
        span = span[5:]
        return Packet_Checksum_Field_FromEnd(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _checksum_start = len(_span)
        _span.extend(payload or self.payload or [])
        _checksum = Checksum(_span[_checksum_start:])
        _span.append(_checksum)
        if self.a > 0xffff:
            raise ValueError("Invalid scalar value Packet_Checksum_Field_FromEnd::a: {self.a} > 0xffff")
        _span.extend(int.to_bytes((self.a << 0), length=2, byteorder='little'))
        if self.b > 0xffff:
            raise ValueError("Invalid scalar value Packet_Checksum_Field_FromEnd::b: {self.b} > 0xffff")
        _span.extend(int.to_bytes((self.b << 0), length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.payload) + 5

@dataclass
class Packet_Struct_Field(Packet):
    a: SizedStruct = field(kw_only=True, default_factory=SizedStruct)
    b: UnsizedStruct = field(kw_only=True, default_factory=UnsizedStruct)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Struct_Field', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['a'] = SizedStruct.parse_all(span[0:1])
        span = span[1:]
        b, span = UnsizedStruct.parse(span)
        fields['b'] = b
        return Packet_Struct_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.a.serialize())
        _span.extend(self.b.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.b.size + 1

@dataclass
class Packet_Custom_Field_ConstantSize(Packet):
    a: SizedCustomField = field(kw_only=True, default_factory=SizedCustomField)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Custom_Field_ConstantSize', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['a'] = SizedCustomField.parse_all(span[0:1])
        span = span[1:]
        return Packet_Custom_Field_ConstantSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.a.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return 1

@dataclass
class Packet_Custom_Field_VariableSize(Packet):
    a: UnsizedCustomField = field(kw_only=True, default_factory=UnsizedCustomField)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Custom_Field_VariableSize', bytes]:
        fields = {'payload': None}
        a, span = UnsizedCustomField.parse(span)
        fields['a'] = a
        return Packet_Custom_Field_VariableSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.a.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.a.size

@dataclass
class Packet_Array_Field_ByteElement_ConstantSize(Packet):
    array: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_ByteElement_ConstantSize', bytes]:
        fields = {'payload': None}
        if len(span) < 4:
            raise Exception('Invalid packet size')
        array = []
        for n in range(4):
            array.append(int.from_bytes(span[n:n + 1], byteorder='little'))
        fields['array'] = array
        span = span[4:]
        return Packet_Array_Field_ByteElement_ConstantSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.array)
        return bytes(_span)

    @property
    def size(self) -> int:
        return 4

@dataclass
class Packet_Array_Field_ByteElement_VariableSize(Packet):
    array: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_ByteElement_VariableSize', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_size = (span[0] >> 0) & 0xf
        span = span[1:]
        if len(span) < array_size:
            raise Exception('Invalid packet size')
        array = []
        for n in range(array_size):
            array.append(int.from_bytes(span[n:n + 1], byteorder='little'))
        fields['array'] = array
        span = span[array_size:]
        return Packet_Array_Field_ByteElement_VariableSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        array_size = len(self.array)
        if array_size > 0xf:
            raise ValueError("Invalid size value Packet_Array_Field_ByteElement_VariableSize::array: {array_size} > 0xf")
        _span.append((array_size << 0))
        _span.extend(self.array)
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.array) + 1

@dataclass
class Packet_Array_Field_ByteElement_VariableCount(Packet):
    array: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_ByteElement_VariableCount', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_count = (span[0] >> 0) & 0xf
        span = span[1:]
        if len(span) < array_count:
            raise Exception('Invalid packet size')
        array = []
        for n in range(array_count):
            array.append(int.from_bytes(span[n:n + 1], byteorder='little'))
        fields['array'] = array
        span = span[array_count:]
        return Packet_Array_Field_ByteElement_VariableCount(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if len(self.array) > 0xf:
            raise ValueError("Invalid count value Packet_Array_Field_ByteElement_VariableCount::array: {len(self.array)} > 0xf")
        _span.append((len(self.array) << 0))
        _span.extend(self.array)
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.array) + 1

@dataclass
class Packet_Array_Field_ByteElement_UnknownSize(Packet):
    array: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_ByteElement_UnknownSize', bytes]:
        fields = {'payload': None}
        array = []
        for n in range(len(span)):
            array.append(int.from_bytes(span[n:n + 1], byteorder='little'))
        fields['array'] = array
        span = bytes()
        return Packet_Array_Field_ByteElement_UnknownSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.array)
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.array)

@dataclass
class Packet_Array_Field_ScalarElement_ConstantSize(Packet):
    array: List[int] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_ScalarElement_ConstantSize', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        array = []
        for n in range(4):
            array.append(int.from_bytes(span[n * 2:(n + 1) * 2], byteorder='little'))
        fields['array'] = array
        span = span[8:]
        return Packet_Array_Field_ScalarElement_ConstantSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        for elt in self.array:
            _span.extend(int.to_bytes(elt, length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 8

@dataclass
class Packet_Array_Field_ScalarElement_VariableSize(Packet):
    array: List[int] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_ScalarElement_VariableSize', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_size = (span[0] >> 0) & 0xf
        span = span[1:]
        if len(span) < array_size:
            raise Exception('Invalid packet size')
        if array_size % 2 != 0:
            raise Exception('Array size is not a multiple of the element size')
        array_count = int(array_size / 2)
        array = []
        for n in range(array_count):
            array.append(int.from_bytes(span[n * 2:(n + 1) * 2], byteorder='little'))
        fields['array'] = array
        span = span[array_size:]
        return Packet_Array_Field_ScalarElement_VariableSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        array_size = len(self.array) * 2
        if array_size > 0xf:
            raise ValueError("Invalid size value Packet_Array_Field_ScalarElement_VariableSize::array: {array_size} > 0xf")
        _span.append((array_size << 0))
        for elt in self.array:
            _span.extend(int.to_bytes(elt, length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.array) * 2 + 1

@dataclass
class Packet_Array_Field_ScalarElement_VariableCount(Packet):
    array: List[int] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_ScalarElement_VariableCount', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_count = (span[0] >> 0) & 0xf
        span = span[1:]
        if len(span) < 2 * array_count:
            raise Exception('Invalid packet size')
        array = []
        for n in range(array_count):
            array.append(int.from_bytes(span[n * 2:(n + 1) * 2], byteorder='little'))
        fields['array'] = array
        span = span[array_count * 2:]
        return Packet_Array_Field_ScalarElement_VariableCount(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if len(self.array) > 0xf:
            raise ValueError("Invalid count value Packet_Array_Field_ScalarElement_VariableCount::array: {len(self.array)} > 0xf")
        _span.append((len(self.array) << 0))
        for elt in self.array:
            _span.extend(int.to_bytes(elt, length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.array) * 2 + 1

@dataclass
class Packet_Array_Field_ScalarElement_UnknownSize(Packet):
    array: List[int] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_ScalarElement_UnknownSize', bytes]:
        fields = {'payload': None}
        if len(span) % 2 != 0:
            raise Exception('Array size is not a multiple of the element size')
        array_count = int(len(span) / 2)
        array = []
        for n in range(array_count):
            array.append(int.from_bytes(span[n * 2:(n + 1) * 2], byteorder='little'))
        fields['array'] = array
        span = bytes()
        return Packet_Array_Field_ScalarElement_UnknownSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        for elt in self.array:
            _span.extend(int.to_bytes(elt, length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.array) * 2

@dataclass
class Packet_Array_Field_EnumElement_ConstantSize(Packet):
    array: List[Enum16] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_EnumElement_ConstantSize', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        array = []
        for n in range(4):
            array.append(Enum16.from_int(int.from_bytes(span[n * 2:(n + 1) * 2], byteorder='little')))
        fields['array'] = array
        span = span[8:]
        return Packet_Array_Field_EnumElement_ConstantSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        for elt in self.array:
            _span.extend(int.to_bytes(elt, length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 8

@dataclass
class Packet_Array_Field_EnumElement_VariableSize(Packet):
    array: List[Enum16] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_EnumElement_VariableSize', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_size = (span[0] >> 0) & 0xf
        span = span[1:]
        if len(span) < array_size:
            raise Exception('Invalid packet size')
        if array_size % 2 != 0:
            raise Exception('Array size is not a multiple of the element size')
        array_count = int(array_size / 2)
        array = []
        for n in range(array_count):
            array.append(Enum16.from_int(int.from_bytes(span[n * 2:(n + 1) * 2], byteorder='little')))
        fields['array'] = array
        span = span[array_size:]
        return Packet_Array_Field_EnumElement_VariableSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        array_size = len(self.array) * 2
        if array_size > 0xf:
            raise ValueError("Invalid size value Packet_Array_Field_EnumElement_VariableSize::array: {array_size} > 0xf")
        _span.append((array_size << 0))
        for elt in self.array:
            _span.extend(int.to_bytes(elt, length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.array) * 2 + 1

@dataclass
class Packet_Array_Field_EnumElement_VariableCount(Packet):
    array: List[Enum16] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_EnumElement_VariableCount', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_count = (span[0] >> 0) & 0xf
        span = span[1:]
        if len(span) < 2 * array_count:
            raise Exception('Invalid packet size')
        array = []
        for n in range(array_count):
            array.append(Enum16.from_int(int.from_bytes(span[n * 2:(n + 1) * 2], byteorder='little')))
        fields['array'] = array
        span = span[array_count * 2:]
        return Packet_Array_Field_EnumElement_VariableCount(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if len(self.array) > 0xf:
            raise ValueError("Invalid count value Packet_Array_Field_EnumElement_VariableCount::array: {len(self.array)} > 0xf")
        _span.append((len(self.array) << 0))
        for elt in self.array:
            _span.extend(int.to_bytes(elt, length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.array) * 2 + 1

@dataclass
class Packet_Array_Field_EnumElement_UnknownSize(Packet):
    array: List[Enum16] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_EnumElement_UnknownSize', bytes]:
        fields = {'payload': None}
        if len(span) % 2 != 0:
            raise Exception('Array size is not a multiple of the element size')
        array_count = int(len(span) / 2)
        array = []
        for n in range(array_count):
            array.append(Enum16.from_int(int.from_bytes(span[n * 2:(n + 1) * 2], byteorder='little')))
        fields['array'] = array
        span = bytes()
        return Packet_Array_Field_EnumElement_UnknownSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        for elt in self.array:
            _span.extend(int.to_bytes(elt, length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.array) * 2

@dataclass
class Packet_Array_Field_SizedElement_ConstantSize(Packet):
    array: List[SizedStruct] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_SizedElement_ConstantSize', bytes]:
        fields = {'payload': None}
        if len(span) < 4:
            raise Exception('Invalid packet size')
        array = []
        for n in range(4):
            array.append(SizedStruct.parse_all(span[n:n + 1]))
        fields['array'] = array
        span = span[4:]
        return Packet_Array_Field_SizedElement_ConstantSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        for elt in self.array:
            _span.extend(elt.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return 4

@dataclass
class Packet_Array_Field_SizedElement_VariableSize(Packet):
    array: List[SizedStruct] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_SizedElement_VariableSize', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_size = (span[0] >> 0) & 0xf
        span = span[1:]
        if len(span) < array_size:
            raise Exception('Invalid packet size')
        array = []
        for n in range(array_size):
            array.append(SizedStruct.parse_all(span[n:n + 1]))
        fields['array'] = array
        span = span[array_size:]
        return Packet_Array_Field_SizedElement_VariableSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        array_size = len(self.array)
        if array_size > 0xf:
            raise ValueError("Invalid size value Packet_Array_Field_SizedElement_VariableSize::array: {array_size} > 0xf")
        _span.append((array_size << 0))
        for elt in self.array:
            _span.extend(elt.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.array]) + 1

@dataclass
class Packet_Array_Field_SizedElement_VariableCount(Packet):
    array: List[SizedStruct] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_SizedElement_VariableCount', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_count = (span[0] >> 0) & 0xf
        span = span[1:]
        if len(span) < array_count:
            raise Exception('Invalid packet size')
        array = []
        for n in range(array_count):
            array.append(SizedStruct.parse_all(span[n:n + 1]))
        fields['array'] = array
        span = span[array_count:]
        return Packet_Array_Field_SizedElement_VariableCount(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if len(self.array) > 0xf:
            raise ValueError("Invalid count value Packet_Array_Field_SizedElement_VariableCount::array: {len(self.array)} > 0xf")
        _span.append((len(self.array) << 0))
        for elt in self.array:
            _span.extend(elt.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.array]) + 1

@dataclass
class Packet_Array_Field_SizedElement_UnknownSize(Packet):
    array: List[SizedStruct] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_SizedElement_UnknownSize', bytes]:
        fields = {'payload': None}
        array = []
        for n in range(len(span)):
            array.append(SizedStruct.parse_all(span[n:n + 1]))
        fields['array'] = array
        span = bytes()
        return Packet_Array_Field_SizedElement_UnknownSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        for elt in self.array:
            _span.extend(elt.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.array])

@dataclass
class Packet_Array_Field_UnsizedElement_ConstantSize(Packet):
    array: List[UnsizedStruct] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_UnsizedElement_ConstantSize', bytes]:
        fields = {'payload': None}
        array = []
        for n in range(4):
            _elt, span = UnsizedStruct.parse(span)
            array.append(_elt)
        fields['array'] = array
        return Packet_Array_Field_UnsizedElement_ConstantSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        for elt in self.array:
            _span.extend(elt.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.array])

@dataclass
class Packet_Array_Field_UnsizedElement_VariableSize(Packet):
    array: List[UnsizedStruct] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_UnsizedElement_VariableSize', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_size = (span[0] >> 0) & 0xf
        span = span[1:]
        if len(span) < array_size:
            raise Exception('Invalid packet size')
        array_span = span[:array_size]
        array = []
        while len(array_span) > 0:
            _elt, array_span = UnsizedStruct.parse(array_span)
            array.append(_elt)
        fields['array'] = array
        span = span[array_size:]
        return Packet_Array_Field_UnsizedElement_VariableSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        array_size = sum(elt.size for elt in self.array)
        if array_size > 0xf:
            raise ValueError("Invalid size value Packet_Array_Field_UnsizedElement_VariableSize::array: {array_size} > 0xf")
        _span.append((array_size << 0))
        for elt in self.array:
            _span.extend(elt.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.array]) + 1

@dataclass
class Packet_Array_Field_UnsizedElement_VariableCount(Packet):
    array: List[UnsizedStruct] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_UnsizedElement_VariableCount', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_count = (span[0] >> 0) & 0xf
        span = span[1:]
        array = []
        for n in range(array_count):
            _elt, span = UnsizedStruct.parse(span)
            array.append(_elt)
        fields['array'] = array
        return Packet_Array_Field_UnsizedElement_VariableCount(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if len(self.array) > 0xf:
            raise ValueError("Invalid count value Packet_Array_Field_UnsizedElement_VariableCount::array: {len(self.array)} > 0xf")
        _span.append((len(self.array) << 0))
        for elt in self.array:
            _span.extend(elt.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.array]) + 1

@dataclass
class Packet_Array_Field_UnsizedElement_UnknownSize(Packet):
    array: List[UnsizedStruct] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_UnsizedElement_UnknownSize', bytes]:
        fields = {'payload': None}
        array = []
        while len(span) > 0:
            _elt, span = UnsizedStruct.parse(span)
            array.append(_elt)
        fields['array'] = array
        return Packet_Array_Field_UnsizedElement_UnknownSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        for elt in self.array:
            _span.extend(elt.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.array])

@dataclass
class Packet_Array_Field_UnsizedElement_SizeModifier(Packet):
    array: List[UnsizedStruct] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_UnsizedElement_SizeModifier', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_size = (span[0] >> 0) & 0xf
        span = span[1:]
        array_size = array_size - +2
        if len(span) < array_size:
            raise Exception('Invalid packet size')
        array_span = span[:array_size]
        array = []
        while len(array_span) > 0:
            _elt, array_span = UnsizedStruct.parse(array_span)
            array.append(_elt)
        fields['array'] = array
        span = span[array_size:]
        return Packet_Array_Field_UnsizedElement_SizeModifier(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        array_size = +2 + sum(elt.size for elt in self.array)
        if array_size > 0xf:
            raise ValueError("Invalid size value Packet_Array_Field_UnsizedElement_SizeModifier::array: {array_size} > 0xf")
        _span.append((array_size << 0))
        for elt in self.array:
            _span.extend(elt.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.array]) + 1

@dataclass
class Packet_Array_Field_SizedElement_VariableSize_Padded(Packet):
    array: List[int] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_SizedElement_VariableSize_Padded', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_size = (span[0] >> 0) & 0xf
        span = span[1:]
        if len(span) < 16:
            raise Exception('Invalid packet size')
        remaining_span = span[16:]
        span = span[:16]
        if len(span) < array_size:
            raise Exception('Invalid packet size')
        if array_size % 2 != 0:
            raise Exception('Array size is not a multiple of the element size')
        array_count = int(array_size / 2)
        array = []
        for n in range(array_count):
            array.append(int.from_bytes(span[n * 2:(n + 1) * 2], byteorder='little'))
        fields['array'] = array
        span = span[array_size:]
        span = remaining_span
        return Packet_Array_Field_SizedElement_VariableSize_Padded(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        array_size = len(self.array) * 2
        if array_size > 0xf:
            raise ValueError("Invalid size value Packet_Array_Field_SizedElement_VariableSize_Padded::array: {array_size} > 0xf")
        _span.append((array_size << 0))
        _array_start = len(_span)
        for elt in self.array:
            _span.extend(int.to_bytes(elt, length=2, byteorder='little'))
        _span.extend([0] * (16 - len(_span) + _array_start))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 17

@dataclass
class Packet_Array_Field_UnsizedElement_VariableCount_Padded(Packet):
    array: List[UnsizedStruct] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Array_Field_UnsizedElement_VariableCount_Padded', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_count = span[0]
        span = span[1:]
        if len(span) < 16:
            raise Exception('Invalid packet size')
        remaining_span = span[16:]
        span = span[:16]
        array = []
        for n in range(array_count):
            _elt, span = UnsizedStruct.parse(span)
            array.append(_elt)
        fields['array'] = array
        span = remaining_span
        return Packet_Array_Field_UnsizedElement_VariableCount_Padded(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if len(self.array) > 0xff:
            raise ValueError("Invalid count value Packet_Array_Field_UnsizedElement_VariableCount_Padded::array: {len(self.array)} > 0xff")
        _span.append((len(self.array) << 0))
        _array_start = len(_span)
        for elt in self.array:
            _span.extend(elt.serialize())
        _span.extend([0] * (16 - len(_span) + _array_start))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 17

@dataclass
class Packet_Optional_Scalar_Field(Packet):
    a: Optional[int] = field(kw_only=True, default=None)
    b: Optional[int] = field(kw_only=True, default=None)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Optional_Scalar_Field', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        c0 = (span[0] >> 0) & 0x1
        c1 = (span[0] >> 1) & 0x1
        span = span[1:]
        if c0 == 0:
            if len(span) < 3:
                raise Exception('Invalid packet size')
            fields['a'] = int.from_bytes(span[:3], byteorder='little')
            span = span[3:]
        if c1 == 1:
            if len(span) < 4:
                raise Exception('Invalid packet size')
            fields['b'] = int.from_bytes(span[:4], byteorder='little')
            span = span[4:]
        return Packet_Optional_Scalar_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _value = (
            ((1 if self.a is None else 0) << 0) |
            ((0 if self.b is None else 1) << 1)
        )
        _span.append(_value)
        if self.a is not None:
            _span.extend(int.to_bytes(self.a, length=3, byteorder='little'))
        if self.b is not None:
            _span.extend(int.to_bytes(self.b, length=4, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 1 + (
            (0 if self.a is None else 3) +
            (0 if self.b is None else 4)
        )

@dataclass
class Packet_Optional_Enum_Field(Packet):
    a: Optional[Enum16] = field(kw_only=True, default=None)
    b: Optional[Enum16] = field(kw_only=True, default=None)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Optional_Enum_Field', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        c0 = (span[0] >> 0) & 0x1
        c1 = (span[0] >> 1) & 0x1
        span = span[1:]
        if c0 == 0:
            if len(span) < 2:
                raise Exception('Invalid packet size')
            fields['a'] = Enum16(int.from_bytes(span[:2], byteorder='little'))
            span = span[2:]
        if c1 == 1:
            if len(span) < 2:
                raise Exception('Invalid packet size')
            fields['b'] = Enum16(int.from_bytes(span[:2], byteorder='little'))
            span = span[2:]
        return Packet_Optional_Enum_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _value = (
            ((1 if self.a is None else 0) << 0) |
            ((0 if self.b is None else 1) << 1)
        )
        _span.append(_value)
        if self.a is not None:
            _span.extend(int.to_bytes(self.a, length=2, byteorder='little'))
        if self.b is not None:
            _span.extend(int.to_bytes(self.b, length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 1 + (
            (0 if self.a is None else 2) +
            (0 if self.b is None else 2)
        )

@dataclass
class Packet_Optional_Struct_Field(Packet):
    a: Optional[SizedStruct] = field(kw_only=True, default=None)
    b: Optional[UnsizedStruct] = field(kw_only=True, default=None)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Packet_Optional_Struct_Field', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        c0 = (span[0] >> 0) & 0x1
        c1 = (span[0] >> 1) & 0x1
        span = span[1:]
        if c0 == 0:
            a, span = SizedStruct.parse(span)
            fields['a'] = a
        if c1 == 1:
            b, span = UnsizedStruct.parse(span)
            fields['b'] = b
        return Packet_Optional_Struct_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _value = (
            ((1 if self.a is None else 0) << 0) |
            ((0 if self.b is None else 1) << 1)
        )
        _span.append(_value)
        if self.a is not None:
            _span.extend(self.a.serialize())
        if self.b is not None:
            _span.extend(self.b.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return 1 + (
            (0 if self.a is None else self.a.size) +
            (0 if self.b is None else self.b.size)
        )

@dataclass
class ScalarChild_A(ScalarParent):
    b: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        self.a = 0

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['ScalarChild_A', bytes]:
        if fields['a'] != 0:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['b'] = span[0]
        span = span[1:]
        return ScalarChild_A(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if self.b > 0xff:
            raise ValueError("Invalid scalar value ScalarChild_A::b: {self.b} > 0xff")
        _span.append((self.b << 0))
        return ScalarParent.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

@dataclass
class ScalarChild_B(ScalarParent):
    c: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        self.a = 1

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['ScalarChild_B', bytes]:
        if fields['a'] != 1:
            raise Exception("Invalid constraint field values")
        if len(span) < 2:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        fields['c'] = value_
        span = span[2:]
        return ScalarChild_B(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if self.c > 0xffff:
            raise ValueError("Invalid scalar value ScalarChild_B::c: {self.c} > 0xffff")
        _span.extend(int.to_bytes((self.c << 0), length=2, byteorder='little'))
        return ScalarParent.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 2

@dataclass
class EnumChild_A(EnumParent):
    b: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        self.a = Enum16.A

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['EnumChild_A', bytes]:
        if fields['a'] != Enum16.A:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['b'] = span[0]
        span = span[1:]
        return EnumChild_A(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if self.b > 0xff:
            raise ValueError("Invalid scalar value EnumChild_A::b: {self.b} > 0xff")
        _span.append((self.b << 0))
        return EnumParent.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

@dataclass
class EnumChild_B(EnumParent):
    c: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        self.a = Enum16.B

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['EnumChild_B', bytes]:
        if fields['a'] != Enum16.B:
            raise Exception("Invalid constraint field values")
        if len(span) < 2:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        fields['c'] = value_
        span = span[2:]
        return EnumChild_B(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if self.c > 0xffff:
            raise ValueError("Invalid scalar value EnumChild_B::c: {self.c} > 0xffff")
        _span.extend(int.to_bytes((self.c << 0), length=2, byteorder='little'))
        return EnumParent.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 2

@dataclass
class AliasedChild_A(EmptyParent):
    b: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        self.a = 2

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AliasedChild_A', bytes]:
        if fields['a'] != 2:
            raise Exception("Invalid constraint field values")
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['b'] = span[0]
        span = span[1:]
        return AliasedChild_A(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if self.b > 0xff:
            raise ValueError("Invalid scalar value AliasedChild_A::b: {self.b} > 0xff")
        _span.append((self.b << 0))
        return EmptyParent.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 1

@dataclass
class AliasedChild_B(EmptyParent):
    c: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        self.a = 3

    @staticmethod
    def parse(fields: dict, span: bytes) -> Tuple['AliasedChild_B', bytes]:
        if fields['a'] != 3:
            raise Exception("Invalid constraint field values")
        if len(span) < 2:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        fields['c'] = value_
        span = span[2:]
        return AliasedChild_B(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if self.c > 0xffff:
            raise ValueError("Invalid scalar value AliasedChild_B::c: {self.c} > 0xffff")
        _span.extend(int.to_bytes((self.c << 0), length=2, byteorder='little'))
        return EmptyParent.serialize(self, payload = bytes(_span))

    @property
    def size(self) -> int:
        return 2

@dataclass
class Struct_Scalar_Field(Packet):
    a: int = field(kw_only=True, default=0)
    c: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Scalar_Field', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:8], byteorder='little')
        fields['a'] = (value_ >> 0) & 0x7f
        fields['c'] = (value_ >> 7) & 0x1ffffffffffffff
        span = span[8:]
        return Struct_Scalar_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if self.a > 0x7f:
            raise ValueError("Invalid scalar value Struct_Scalar_Field::a: {self.a} > 0x7f")
        if self.c > 0x1ffffffffffffff:
            raise ValueError("Invalid scalar value Struct_Scalar_Field::c: {self.c} > 0x1ffffffffffffff")
        _value = (
            (self.a << 0) |
            (self.c << 7)
        )
        _span.extend(int.to_bytes(_value, length=8, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 8

@dataclass
class Struct_Enum_Field_(Packet):
    a: Enum7 = field(kw_only=True, default=Enum7.A)
    c: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Enum_Field_', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:8], byteorder='little')
        fields['a'] = Enum7.from_int((value_ >> 0) & 0x7f)
        fields['c'] = (value_ >> 7) & 0x1ffffffffffffff
        span = span[8:]
        return Struct_Enum_Field_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if self.c > 0x1ffffffffffffff:
            raise ValueError("Invalid scalar value Struct_Enum_Field_::c: {self.c} > 0x1ffffffffffffff")
        _value = (
            (self.a << 0) |
            (self.c << 7)
        )
        _span.extend(int.to_bytes(_value, length=8, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 8

@dataclass
class Struct_Enum_Field(Packet):
    s: Struct_Enum_Field_ = field(kw_only=True, default_factory=Struct_Enum_Field_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Enum_Field', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        fields['s'] = Struct_Enum_Field_.parse_all(span[0:8])
        span = span[8:]
        return Struct_Enum_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return 8

@dataclass
class Struct_Reserved_Field_(Packet):
    a: int = field(kw_only=True, default=0)
    c: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Reserved_Field_', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:8], byteorder='little')
        fields['a'] = (value_ >> 0) & 0x7f
        fields['c'] = (value_ >> 9) & 0x7fffffffffffff
        span = span[8:]
        return Struct_Reserved_Field_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if self.a > 0x7f:
            raise ValueError("Invalid scalar value Struct_Reserved_Field_::a: {self.a} > 0x7f")
        if self.c > 0x7fffffffffffff:
            raise ValueError("Invalid scalar value Struct_Reserved_Field_::c: {self.c} > 0x7fffffffffffff")
        _value = (
            (self.a << 0) |
            (self.c << 9)
        )
        _span.extend(int.to_bytes(_value, length=8, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 8

@dataclass
class Struct_Reserved_Field(Packet):
    s: Struct_Reserved_Field_ = field(kw_only=True, default_factory=Struct_Reserved_Field_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Reserved_Field', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        fields['s'] = Struct_Reserved_Field_.parse_all(span[0:8])
        span = span[8:]
        return Struct_Reserved_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return 8

@dataclass
class Struct_Size_Field_(Packet):
    a: int = field(kw_only=True, default=0)
    b: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Size_Field_', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:8], byteorder='little')
        b_size = (value_ >> 0) & 0x7
        fields['a'] = (value_ >> 3) & 0x1fffffffffffffff
        span = span[8:]
        if len(span) < b_size:
            raise Exception('Invalid packet size')
        b = []
        for n in range(b_size):
            b.append(int.from_bytes(span[n:n + 1], byteorder='little'))
        fields['b'] = b
        span = span[b_size:]
        return Struct_Size_Field_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        b_size = len(self.b)
        if b_size > 0x7:
            raise ValueError("Invalid size value Struct_Size_Field_::b: {b_size} > 0x7")
        if self.a > 0x1fffffffffffffff:
            raise ValueError("Invalid scalar value Struct_Size_Field_::a: {self.a} > 0x1fffffffffffffff")
        _value = (
            (b_size << 0) |
            (self.a << 3)
        )
        _span.extend(int.to_bytes(_value, length=8, byteorder='little'))
        _span.extend(self.b)
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.b) + 8

@dataclass
class Struct_Size_Field(Packet):
    s: Struct_Size_Field_ = field(kw_only=True, default_factory=Struct_Size_Field_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Size_Field', bytes]:
        fields = {'payload': None}
        s, span = Struct_Size_Field_.parse(span)
        fields['s'] = s
        return Struct_Size_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_Count_Field_(Packet):
    a: int = field(kw_only=True, default=0)
    b: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Count_Field_', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:8], byteorder='little')
        b_count = (value_ >> 0) & 0x7
        fields['a'] = (value_ >> 3) & 0x1fffffffffffffff
        span = span[8:]
        if len(span) < b_count:
            raise Exception('Invalid packet size')
        b = []
        for n in range(b_count):
            b.append(int.from_bytes(span[n:n + 1], byteorder='little'))
        fields['b'] = b
        span = span[b_count:]
        return Struct_Count_Field_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if len(self.b) > 0x7:
            raise ValueError("Invalid count value Struct_Count_Field_::b: {len(self.b)} > 0x7")
        if self.a > 0x1fffffffffffffff:
            raise ValueError("Invalid scalar value Struct_Count_Field_::a: {self.a} > 0x1fffffffffffffff")
        _value = (
            (len(self.b) << 0) |
            (self.a << 3)
        )
        _span.extend(int.to_bytes(_value, length=8, byteorder='little'))
        _span.extend(self.b)
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.b) + 8

@dataclass
class Struct_Count_Field(Packet):
    s: Struct_Count_Field_ = field(kw_only=True, default_factory=Struct_Count_Field_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Count_Field', bytes]:
        fields = {'payload': None}
        s, span = Struct_Count_Field_.parse(span)
        fields['s'] = s
        return Struct_Count_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_FixedScalar_Field_(Packet):
    b: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_FixedScalar_Field_', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:8], byteorder='little')
        if (value_ >> 0) & 0x7f != 0x7:
            raise Exception('Unexpected fixed field value')
        fields['b'] = (value_ >> 7) & 0x1ffffffffffffff
        span = span[8:]
        return Struct_FixedScalar_Field_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if self.b > 0x1ffffffffffffff:
            raise ValueError("Invalid scalar value Struct_FixedScalar_Field_::b: {self.b} > 0x1ffffffffffffff")
        _value = (
            (0x7 << 0) |
            (self.b << 7)
        )
        _span.extend(int.to_bytes(_value, length=8, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 8

@dataclass
class Struct_FixedScalar_Field(Packet):
    s: Struct_FixedScalar_Field_ = field(kw_only=True, default_factory=Struct_FixedScalar_Field_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_FixedScalar_Field', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        fields['s'] = Struct_FixedScalar_Field_.parse_all(span[0:8])
        span = span[8:]
        return Struct_FixedScalar_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return 8

@dataclass
class Struct_FixedEnum_Field_(Packet):
    b: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_FixedEnum_Field_', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:8], byteorder='little')
        if (value_ >> 0) & 0x7f != Enum7.A:
            raise Exception('Unexpected fixed field value')
        fields['b'] = (value_ >> 7) & 0x1ffffffffffffff
        span = span[8:]
        return Struct_FixedEnum_Field_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if self.b > 0x1ffffffffffffff:
            raise ValueError("Invalid scalar value Struct_FixedEnum_Field_::b: {self.b} > 0x1ffffffffffffff")
        _value = (
            (Enum7.A << 0) |
            (self.b << 7)
        )
        _span.extend(int.to_bytes(_value, length=8, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 8

@dataclass
class Struct_FixedEnum_Field(Packet):
    s: Struct_FixedEnum_Field_ = field(kw_only=True, default_factory=Struct_FixedEnum_Field_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_FixedEnum_Field', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        fields['s'] = Struct_FixedEnum_Field_.parse_all(span[0:8])
        span = span[8:]
        return Struct_FixedEnum_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return 8

@dataclass
class Struct_ScalarGroup_Field_(Packet):


    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_ScalarGroup_Field_', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        if value_ != 0x2a:
            raise Exception('Unexpected fixed field value')
        span = span[2:]
        return Struct_ScalarGroup_Field_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(int.to_bytes((0x2a << 0), length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 2

@dataclass
class Struct_ScalarGroup_Field(Packet):
    s: Struct_ScalarGroup_Field_ = field(kw_only=True, default_factory=Struct_ScalarGroup_Field_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_ScalarGroup_Field', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['s'] = Struct_ScalarGroup_Field_.parse_all(span[0:2])
        span = span[2:]
        return Struct_ScalarGroup_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return 2

@dataclass
class Struct_EnumGroup_Field_(Packet):


    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_EnumGroup_Field_', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        if value_ != Enum16.A:
            raise Exception('Unexpected fixed field value')
        span = span[2:]
        return Struct_EnumGroup_Field_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(int.to_bytes((Enum16.A << 0), length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 2

@dataclass
class Struct_EnumGroup_Field(Packet):
    s: Struct_EnumGroup_Field_ = field(kw_only=True, default_factory=Struct_EnumGroup_Field_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_EnumGroup_Field', bytes]:
        fields = {'payload': None}
        if len(span) < 2:
            raise Exception('Invalid packet size')
        fields['s'] = Struct_EnumGroup_Field_.parse_all(span[0:2])
        span = span[2:]
        return Struct_EnumGroup_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return 2

@dataclass
class Struct_Checksum_Field_FromStart_(Packet):
    a: int = field(kw_only=True, default=0)
    b: int = field(kw_only=True, default=0)
    crc: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Checksum_Field_FromStart_', bytes]:
        fields = {'payload': None}
        if len(span) < 5:
            raise Exception('Invalid packet size')
        if len(span) < 5:
            raise Exception('Invalid packet size')
        crc = span[4]
        fields['crc'] = crc
        computed_crc = Checksum(span[:4])
        if computed_crc != crc:
            raise Exception(f'Invalid checksum computation: {computed_crc} != {crc}')
        value_ = int.from_bytes(span[0:2], byteorder='little')
        fields['a'] = value_
        value_ = int.from_bytes(span[2:4], byteorder='little')
        fields['b'] = value_
        span = span[5:]
        return Struct_Checksum_Field_FromStart_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _checksum_start = len(_span)
        if self.a > 0xffff:
            raise ValueError("Invalid scalar value Struct_Checksum_Field_FromStart_::a: {self.a} > 0xffff")
        _span.extend(int.to_bytes((self.a << 0), length=2, byteorder='little'))
        if self.b > 0xffff:
            raise ValueError("Invalid scalar value Struct_Checksum_Field_FromStart_::b: {self.b} > 0xffff")
        _span.extend(int.to_bytes((self.b << 0), length=2, byteorder='little'))
        _checksum = Checksum(_span[_checksum_start:])
        _span.append(_checksum)
        return bytes(_span)

    @property
    def size(self) -> int:
        return 5

@dataclass
class Struct_Checksum_Field_FromStart(Packet):
    s: Struct_Checksum_Field_FromStart_ = field(kw_only=True, default_factory=Struct_Checksum_Field_FromStart_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Checksum_Field_FromStart', bytes]:
        fields = {'payload': None}
        if len(span) < 5:
            raise Exception('Invalid packet size')
        fields['s'] = Struct_Checksum_Field_FromStart_.parse_all(span[0:5])
        span = span[5:]
        return Struct_Checksum_Field_FromStart(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return 5

@dataclass
class Struct_Checksum_Field_FromEnd_(Packet):
    crc: int = field(kw_only=True, default=0)
    a: int = field(kw_only=True, default=0)
    b: int = field(kw_only=True, default=0)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Checksum_Field_FromEnd_', bytes]:
        fields = {'payload': None}
        if len(span) < 0:
            raise Exception('Invalid packet size')
        if len(span) < 5:
            raise Exception('Invalid packet size')
        crc = span[-5]
        fields['crc'] = crc
        computed_crc = Checksum(span[:-5])
        if computed_crc != crc:
            raise Exception(f'Invalid checksum computation: {computed_crc} != {crc}')
        if len(span) < 5:
            raise Exception('Invalid packet size')
        payload = span[:-5]
        span = span[-5:]
        fields['payload'] = payload
        if len(span) < 5:
            raise Exception('Invalid packet size')
        value_ = int.from_bytes(span[1:3], byteorder='little')
        fields['a'] = value_
        value_ = int.from_bytes(span[3:5], byteorder='little')
        fields['b'] = value_
        span = span[5:]
        return Struct_Checksum_Field_FromEnd_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _checksum_start = len(_span)
        _span.extend(payload or self.payload or [])
        _checksum = Checksum(_span[_checksum_start:])
        _span.append(_checksum)
        if self.a > 0xffff:
            raise ValueError("Invalid scalar value Struct_Checksum_Field_FromEnd_::a: {self.a} > 0xffff")
        _span.extend(int.to_bytes((self.a << 0), length=2, byteorder='little'))
        if self.b > 0xffff:
            raise ValueError("Invalid scalar value Struct_Checksum_Field_FromEnd_::b: {self.b} > 0xffff")
        _span.extend(int.to_bytes((self.b << 0), length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.payload) + 5

@dataclass
class Struct_Checksum_Field_FromEnd(Packet):
    s: Struct_Checksum_Field_FromEnd_ = field(kw_only=True, default_factory=Struct_Checksum_Field_FromEnd_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Checksum_Field_FromEnd', bytes]:
        fields = {'payload': None}
        s, span = Struct_Checksum_Field_FromEnd_.parse(span)
        fields['s'] = s
        return Struct_Checksum_Field_FromEnd(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_Struct_Field(Packet):
    a: SizedStruct = field(kw_only=True, default_factory=SizedStruct)
    b: UnsizedStruct = field(kw_only=True, default_factory=UnsizedStruct)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Struct_Field', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['a'] = SizedStruct.parse_all(span[0:1])
        span = span[1:]
        b, span = UnsizedStruct.parse(span)
        fields['b'] = b
        return Struct_Struct_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.a.serialize())
        _span.extend(self.b.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.b.size + 1

@dataclass
class Struct_Custom_Field_ConstantSize_(Packet):
    a: SizedCustomField = field(kw_only=True, default_factory=SizedCustomField)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Custom_Field_ConstantSize_', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['a'] = SizedCustomField.parse_all(span[0:1])
        span = span[1:]
        return Struct_Custom_Field_ConstantSize_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.a.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return 1

@dataclass
class Struct_Custom_Field_ConstantSize(Packet):
    s: Struct_Custom_Field_ConstantSize_ = field(kw_only=True, default_factory=Struct_Custom_Field_ConstantSize_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Custom_Field_ConstantSize', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['s'] = Struct_Custom_Field_ConstantSize_.parse_all(span[0:1])
        span = span[1:]
        return Struct_Custom_Field_ConstantSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return 1

@dataclass
class Struct_Custom_Field_VariableSize_(Packet):
    a: UnsizedCustomField = field(kw_only=True, default_factory=UnsizedCustomField)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Custom_Field_VariableSize_', bytes]:
        fields = {'payload': None}
        a, span = UnsizedCustomField.parse(span)
        fields['a'] = a
        return Struct_Custom_Field_VariableSize_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.a.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.a.size

@dataclass
class Struct_Custom_Field_VariableSize(Packet):
    s: Struct_Custom_Field_VariableSize_ = field(kw_only=True, default_factory=Struct_Custom_Field_VariableSize_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Custom_Field_VariableSize', bytes]:
        fields = {'payload': None}
        s, span = Struct_Custom_Field_VariableSize_.parse(span)
        fields['s'] = s
        return Struct_Custom_Field_VariableSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_Array_Field_ByteElement_ConstantSize_(Packet):
    array: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_ByteElement_ConstantSize_', bytes]:
        fields = {'payload': None}
        if len(span) < 4:
            raise Exception('Invalid packet size')
        array = []
        for n in range(4):
            array.append(int.from_bytes(span[n:n + 1], byteorder='little'))
        fields['array'] = array
        span = span[4:]
        return Struct_Array_Field_ByteElement_ConstantSize_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.array)
        return bytes(_span)

    @property
    def size(self) -> int:
        return 4

@dataclass
class Struct_Array_Field_ByteElement_ConstantSize(Packet):
    s: Struct_Array_Field_ByteElement_ConstantSize_ = field(kw_only=True, default_factory=Struct_Array_Field_ByteElement_ConstantSize_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_ByteElement_ConstantSize', bytes]:
        fields = {'payload': None}
        if len(span) < 4:
            raise Exception('Invalid packet size')
        fields['s'] = Struct_Array_Field_ByteElement_ConstantSize_.parse_all(span[0:4])
        span = span[4:]
        return Struct_Array_Field_ByteElement_ConstantSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return 4

@dataclass
class Struct_Array_Field_ByteElement_VariableSize_(Packet):
    array: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_ByteElement_VariableSize_', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_size = (span[0] >> 0) & 0xf
        span = span[1:]
        if len(span) < array_size:
            raise Exception('Invalid packet size')
        array = []
        for n in range(array_size):
            array.append(int.from_bytes(span[n:n + 1], byteorder='little'))
        fields['array'] = array
        span = span[array_size:]
        return Struct_Array_Field_ByteElement_VariableSize_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        array_size = len(self.array)
        if array_size > 0xf:
            raise ValueError("Invalid size value Struct_Array_Field_ByteElement_VariableSize_::array: {array_size} > 0xf")
        _span.append((array_size << 0))
        _span.extend(self.array)
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.array) + 1

@dataclass
class Struct_Array_Field_ByteElement_VariableSize(Packet):
    s: Struct_Array_Field_ByteElement_VariableSize_ = field(kw_only=True, default_factory=Struct_Array_Field_ByteElement_VariableSize_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_ByteElement_VariableSize', bytes]:
        fields = {'payload': None}
        s, span = Struct_Array_Field_ByteElement_VariableSize_.parse(span)
        fields['s'] = s
        return Struct_Array_Field_ByteElement_VariableSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_Array_Field_ByteElement_VariableCount_(Packet):
    array: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_ByteElement_VariableCount_', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_count = (span[0] >> 0) & 0xf
        span = span[1:]
        if len(span) < array_count:
            raise Exception('Invalid packet size')
        array = []
        for n in range(array_count):
            array.append(int.from_bytes(span[n:n + 1], byteorder='little'))
        fields['array'] = array
        span = span[array_count:]
        return Struct_Array_Field_ByteElement_VariableCount_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if len(self.array) > 0xf:
            raise ValueError("Invalid count value Struct_Array_Field_ByteElement_VariableCount_::array: {len(self.array)} > 0xf")
        _span.append((len(self.array) << 0))
        _span.extend(self.array)
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.array) + 1

@dataclass
class Struct_Array_Field_ByteElement_VariableCount(Packet):
    s: Struct_Array_Field_ByteElement_VariableCount_ = field(kw_only=True, default_factory=Struct_Array_Field_ByteElement_VariableCount_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_ByteElement_VariableCount', bytes]:
        fields = {'payload': None}
        s, span = Struct_Array_Field_ByteElement_VariableCount_.parse(span)
        fields['s'] = s
        return Struct_Array_Field_ByteElement_VariableCount(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_Array_Field_ByteElement_UnknownSize_(Packet):
    array: bytearray = field(kw_only=True, default_factory=bytearray)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_ByteElement_UnknownSize_', bytes]:
        fields = {'payload': None}
        array = []
        for n in range(len(span)):
            array.append(int.from_bytes(span[n:n + 1], byteorder='little'))
        fields['array'] = array
        span = bytes()
        return Struct_Array_Field_ByteElement_UnknownSize_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.array)
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.array)

@dataclass
class Struct_Array_Field_ByteElement_UnknownSize(Packet):
    s: Struct_Array_Field_ByteElement_UnknownSize_ = field(kw_only=True, default_factory=Struct_Array_Field_ByteElement_UnknownSize_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_ByteElement_UnknownSize', bytes]:
        fields = {'payload': None}
        s, span = Struct_Array_Field_ByteElement_UnknownSize_.parse(span)
        fields['s'] = s
        return Struct_Array_Field_ByteElement_UnknownSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_Array_Field_ScalarElement_ConstantSize_(Packet):
    array: List[int] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_ScalarElement_ConstantSize_', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        array = []
        for n in range(4):
            array.append(int.from_bytes(span[n * 2:(n + 1) * 2], byteorder='little'))
        fields['array'] = array
        span = span[8:]
        return Struct_Array_Field_ScalarElement_ConstantSize_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        for elt in self.array:
            _span.extend(int.to_bytes(elt, length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 8

@dataclass
class Struct_Array_Field_ScalarElement_ConstantSize(Packet):
    s: Struct_Array_Field_ScalarElement_ConstantSize_ = field(kw_only=True, default_factory=Struct_Array_Field_ScalarElement_ConstantSize_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_ScalarElement_ConstantSize', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        fields['s'] = Struct_Array_Field_ScalarElement_ConstantSize_.parse_all(span[0:8])
        span = span[8:]
        return Struct_Array_Field_ScalarElement_ConstantSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return 8

@dataclass
class Struct_Array_Field_ScalarElement_VariableSize_(Packet):
    array: List[int] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_ScalarElement_VariableSize_', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_size = (span[0] >> 0) & 0xf
        span = span[1:]
        if len(span) < array_size:
            raise Exception('Invalid packet size')
        if array_size % 2 != 0:
            raise Exception('Array size is not a multiple of the element size')
        array_count = int(array_size / 2)
        array = []
        for n in range(array_count):
            array.append(int.from_bytes(span[n * 2:(n + 1) * 2], byteorder='little'))
        fields['array'] = array
        span = span[array_size:]
        return Struct_Array_Field_ScalarElement_VariableSize_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        array_size = len(self.array) * 2
        if array_size > 0xf:
            raise ValueError("Invalid size value Struct_Array_Field_ScalarElement_VariableSize_::array: {array_size} > 0xf")
        _span.append((array_size << 0))
        for elt in self.array:
            _span.extend(int.to_bytes(elt, length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.array) * 2 + 1

@dataclass
class Struct_Array_Field_ScalarElement_VariableSize(Packet):
    s: Struct_Array_Field_ScalarElement_VariableSize_ = field(kw_only=True, default_factory=Struct_Array_Field_ScalarElement_VariableSize_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_ScalarElement_VariableSize', bytes]:
        fields = {'payload': None}
        s, span = Struct_Array_Field_ScalarElement_VariableSize_.parse(span)
        fields['s'] = s
        return Struct_Array_Field_ScalarElement_VariableSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_Array_Field_ScalarElement_VariableCount_(Packet):
    array: List[int] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_ScalarElement_VariableCount_', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_count = (span[0] >> 0) & 0xf
        span = span[1:]
        if len(span) < 2 * array_count:
            raise Exception('Invalid packet size')
        array = []
        for n in range(array_count):
            array.append(int.from_bytes(span[n * 2:(n + 1) * 2], byteorder='little'))
        fields['array'] = array
        span = span[array_count * 2:]
        return Struct_Array_Field_ScalarElement_VariableCount_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if len(self.array) > 0xf:
            raise ValueError("Invalid count value Struct_Array_Field_ScalarElement_VariableCount_::array: {len(self.array)} > 0xf")
        _span.append((len(self.array) << 0))
        for elt in self.array:
            _span.extend(int.to_bytes(elt, length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.array) * 2 + 1

@dataclass
class Struct_Array_Field_ScalarElement_VariableCount(Packet):
    s: Struct_Array_Field_ScalarElement_VariableCount_ = field(kw_only=True, default_factory=Struct_Array_Field_ScalarElement_VariableCount_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_ScalarElement_VariableCount', bytes]:
        fields = {'payload': None}
        s, span = Struct_Array_Field_ScalarElement_VariableCount_.parse(span)
        fields['s'] = s
        return Struct_Array_Field_ScalarElement_VariableCount(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_Array_Field_ScalarElement_UnknownSize_(Packet):
    array: List[int] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_ScalarElement_UnknownSize_', bytes]:
        fields = {'payload': None}
        if len(span) % 2 != 0:
            raise Exception('Array size is not a multiple of the element size')
        array_count = int(len(span) / 2)
        array = []
        for n in range(array_count):
            array.append(int.from_bytes(span[n * 2:(n + 1) * 2], byteorder='little'))
        fields['array'] = array
        span = bytes()
        return Struct_Array_Field_ScalarElement_UnknownSize_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        for elt in self.array:
            _span.extend(int.to_bytes(elt, length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.array) * 2

@dataclass
class Struct_Array_Field_ScalarElement_UnknownSize(Packet):
    s: Struct_Array_Field_ScalarElement_UnknownSize_ = field(kw_only=True, default_factory=Struct_Array_Field_ScalarElement_UnknownSize_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_ScalarElement_UnknownSize', bytes]:
        fields = {'payload': None}
        s, span = Struct_Array_Field_ScalarElement_UnknownSize_.parse(span)
        fields['s'] = s
        return Struct_Array_Field_ScalarElement_UnknownSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_Array_Field_EnumElement_ConstantSize_(Packet):
    array: List[Enum16] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_EnumElement_ConstantSize_', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        array = []
        for n in range(4):
            array.append(Enum16.from_int(int.from_bytes(span[n * 2:(n + 1) * 2], byteorder='little')))
        fields['array'] = array
        span = span[8:]
        return Struct_Array_Field_EnumElement_ConstantSize_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        for elt in self.array:
            _span.extend(int.to_bytes(elt, length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 8

@dataclass
class Struct_Array_Field_EnumElement_ConstantSize(Packet):
    s: Struct_Array_Field_EnumElement_ConstantSize_ = field(kw_only=True, default_factory=Struct_Array_Field_EnumElement_ConstantSize_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_EnumElement_ConstantSize', bytes]:
        fields = {'payload': None}
        if len(span) < 8:
            raise Exception('Invalid packet size')
        fields['s'] = Struct_Array_Field_EnumElement_ConstantSize_.parse_all(span[0:8])
        span = span[8:]
        return Struct_Array_Field_EnumElement_ConstantSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return 8

@dataclass
class Struct_Array_Field_EnumElement_VariableSize_(Packet):
    array: List[Enum16] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_EnumElement_VariableSize_', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_size = (span[0] >> 0) & 0xf
        span = span[1:]
        if len(span) < array_size:
            raise Exception('Invalid packet size')
        if array_size % 2 != 0:
            raise Exception('Array size is not a multiple of the element size')
        array_count = int(array_size / 2)
        array = []
        for n in range(array_count):
            array.append(Enum16.from_int(int.from_bytes(span[n * 2:(n + 1) * 2], byteorder='little')))
        fields['array'] = array
        span = span[array_size:]
        return Struct_Array_Field_EnumElement_VariableSize_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        array_size = len(self.array) * 2
        if array_size > 0xf:
            raise ValueError("Invalid size value Struct_Array_Field_EnumElement_VariableSize_::array: {array_size} > 0xf")
        _span.append((array_size << 0))
        for elt in self.array:
            _span.extend(int.to_bytes(elt, length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.array) * 2 + 1

@dataclass
class Struct_Array_Field_EnumElement_VariableSize(Packet):
    s: Struct_Array_Field_EnumElement_VariableSize_ = field(kw_only=True, default_factory=Struct_Array_Field_EnumElement_VariableSize_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_EnumElement_VariableSize', bytes]:
        fields = {'payload': None}
        s, span = Struct_Array_Field_EnumElement_VariableSize_.parse(span)
        fields['s'] = s
        return Struct_Array_Field_EnumElement_VariableSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_Array_Field_EnumElement_VariableCount_(Packet):
    array: List[Enum16] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_EnumElement_VariableCount_', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_count = (span[0] >> 0) & 0xf
        span = span[1:]
        if len(span) < 2 * array_count:
            raise Exception('Invalid packet size')
        array = []
        for n in range(array_count):
            array.append(Enum16.from_int(int.from_bytes(span[n * 2:(n + 1) * 2], byteorder='little')))
        fields['array'] = array
        span = span[array_count * 2:]
        return Struct_Array_Field_EnumElement_VariableCount_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if len(self.array) > 0xf:
            raise ValueError("Invalid count value Struct_Array_Field_EnumElement_VariableCount_::array: {len(self.array)} > 0xf")
        _span.append((len(self.array) << 0))
        for elt in self.array:
            _span.extend(int.to_bytes(elt, length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.array) * 2 + 1

@dataclass
class Struct_Array_Field_EnumElement_VariableCount(Packet):
    s: Struct_Array_Field_EnumElement_VariableCount_ = field(kw_only=True, default_factory=Struct_Array_Field_EnumElement_VariableCount_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_EnumElement_VariableCount', bytes]:
        fields = {'payload': None}
        s, span = Struct_Array_Field_EnumElement_VariableCount_.parse(span)
        fields['s'] = s
        return Struct_Array_Field_EnumElement_VariableCount(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_Array_Field_EnumElement_UnknownSize_(Packet):
    array: List[Enum16] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_EnumElement_UnknownSize_', bytes]:
        fields = {'payload': None}
        if len(span) % 2 != 0:
            raise Exception('Array size is not a multiple of the element size')
        array_count = int(len(span) / 2)
        array = []
        for n in range(array_count):
            array.append(Enum16.from_int(int.from_bytes(span[n * 2:(n + 1) * 2], byteorder='little')))
        fields['array'] = array
        span = bytes()
        return Struct_Array_Field_EnumElement_UnknownSize_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        for elt in self.array:
            _span.extend(int.to_bytes(elt, length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return len(self.array) * 2

@dataclass
class Struct_Array_Field_EnumElement_UnknownSize(Packet):
    s: Struct_Array_Field_EnumElement_UnknownSize_ = field(kw_only=True, default_factory=Struct_Array_Field_EnumElement_UnknownSize_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_EnumElement_UnknownSize', bytes]:
        fields = {'payload': None}
        s, span = Struct_Array_Field_EnumElement_UnknownSize_.parse(span)
        fields['s'] = s
        return Struct_Array_Field_EnumElement_UnknownSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_Array_Field_SizedElement_ConstantSize_(Packet):
    array: List[SizedStruct] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_SizedElement_ConstantSize_', bytes]:
        fields = {'payload': None}
        if len(span) < 4:
            raise Exception('Invalid packet size')
        array = []
        for n in range(4):
            array.append(SizedStruct.parse_all(span[n:n + 1]))
        fields['array'] = array
        span = span[4:]
        return Struct_Array_Field_SizedElement_ConstantSize_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        for elt in self.array:
            _span.extend(elt.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return 4

@dataclass
class Struct_Array_Field_SizedElement_ConstantSize(Packet):
    s: Struct_Array_Field_SizedElement_ConstantSize_ = field(kw_only=True, default_factory=Struct_Array_Field_SizedElement_ConstantSize_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_SizedElement_ConstantSize', bytes]:
        fields = {'payload': None}
        if len(span) < 4:
            raise Exception('Invalid packet size')
        fields['s'] = Struct_Array_Field_SizedElement_ConstantSize_.parse_all(span[0:4])
        span = span[4:]
        return Struct_Array_Field_SizedElement_ConstantSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return 4

@dataclass
class Struct_Array_Field_SizedElement_VariableSize_(Packet):
    array: List[SizedStruct] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_SizedElement_VariableSize_', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_size = (span[0] >> 0) & 0xf
        span = span[1:]
        if len(span) < array_size:
            raise Exception('Invalid packet size')
        array = []
        for n in range(array_size):
            array.append(SizedStruct.parse_all(span[n:n + 1]))
        fields['array'] = array
        span = span[array_size:]
        return Struct_Array_Field_SizedElement_VariableSize_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        array_size = len(self.array)
        if array_size > 0xf:
            raise ValueError("Invalid size value Struct_Array_Field_SizedElement_VariableSize_::array: {array_size} > 0xf")
        _span.append((array_size << 0))
        for elt in self.array:
            _span.extend(elt.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.array]) + 1

@dataclass
class Struct_Array_Field_SizedElement_VariableSize(Packet):
    s: Struct_Array_Field_SizedElement_VariableSize_ = field(kw_only=True, default_factory=Struct_Array_Field_SizedElement_VariableSize_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_SizedElement_VariableSize', bytes]:
        fields = {'payload': None}
        s, span = Struct_Array_Field_SizedElement_VariableSize_.parse(span)
        fields['s'] = s
        return Struct_Array_Field_SizedElement_VariableSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_Array_Field_SizedElement_VariableCount_(Packet):
    array: List[SizedStruct] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_SizedElement_VariableCount_', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_count = (span[0] >> 0) & 0xf
        span = span[1:]
        if len(span) < array_count:
            raise Exception('Invalid packet size')
        array = []
        for n in range(array_count):
            array.append(SizedStruct.parse_all(span[n:n + 1]))
        fields['array'] = array
        span = span[array_count:]
        return Struct_Array_Field_SizedElement_VariableCount_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if len(self.array) > 0xf:
            raise ValueError("Invalid count value Struct_Array_Field_SizedElement_VariableCount_::array: {len(self.array)} > 0xf")
        _span.append((len(self.array) << 0))
        for elt in self.array:
            _span.extend(elt.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.array]) + 1

@dataclass
class Struct_Array_Field_SizedElement_VariableCount(Packet):
    s: Struct_Array_Field_SizedElement_VariableCount_ = field(kw_only=True, default_factory=Struct_Array_Field_SizedElement_VariableCount_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_SizedElement_VariableCount', bytes]:
        fields = {'payload': None}
        s, span = Struct_Array_Field_SizedElement_VariableCount_.parse(span)
        fields['s'] = s
        return Struct_Array_Field_SizedElement_VariableCount(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_Array_Field_SizedElement_UnknownSize_(Packet):
    array: List[SizedStruct] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_SizedElement_UnknownSize_', bytes]:
        fields = {'payload': None}
        array = []
        for n in range(len(span)):
            array.append(SizedStruct.parse_all(span[n:n + 1]))
        fields['array'] = array
        span = bytes()
        return Struct_Array_Field_SizedElement_UnknownSize_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        for elt in self.array:
            _span.extend(elt.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.array])

@dataclass
class Struct_Array_Field_SizedElement_UnknownSize(Packet):
    s: Struct_Array_Field_SizedElement_UnknownSize_ = field(kw_only=True, default_factory=Struct_Array_Field_SizedElement_UnknownSize_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_SizedElement_UnknownSize', bytes]:
        fields = {'payload': None}
        s, span = Struct_Array_Field_SizedElement_UnknownSize_.parse(span)
        fields['s'] = s
        return Struct_Array_Field_SizedElement_UnknownSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_Array_Field_UnsizedElement_ConstantSize_(Packet):
    array: List[UnsizedStruct] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_UnsizedElement_ConstantSize_', bytes]:
        fields = {'payload': None}
        array = []
        for n in range(4):
            _elt, span = UnsizedStruct.parse(span)
            array.append(_elt)
        fields['array'] = array
        return Struct_Array_Field_UnsizedElement_ConstantSize_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        for elt in self.array:
            _span.extend(elt.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.array])

@dataclass
class Struct_Array_Field_UnsizedElement_ConstantSize(Packet):
    s: Struct_Array_Field_UnsizedElement_ConstantSize_ = field(kw_only=True, default_factory=Struct_Array_Field_UnsizedElement_ConstantSize_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_UnsizedElement_ConstantSize', bytes]:
        fields = {'payload': None}
        s, span = Struct_Array_Field_UnsizedElement_ConstantSize_.parse(span)
        fields['s'] = s
        return Struct_Array_Field_UnsizedElement_ConstantSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_Array_Field_UnsizedElement_VariableSize_(Packet):
    array: List[UnsizedStruct] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_UnsizedElement_VariableSize_', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_size = (span[0] >> 0) & 0xf
        span = span[1:]
        if len(span) < array_size:
            raise Exception('Invalid packet size')
        array_span = span[:array_size]
        array = []
        while len(array_span) > 0:
            _elt, array_span = UnsizedStruct.parse(array_span)
            array.append(_elt)
        fields['array'] = array
        span = span[array_size:]
        return Struct_Array_Field_UnsizedElement_VariableSize_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        array_size = sum(elt.size for elt in self.array)
        if array_size > 0xf:
            raise ValueError("Invalid size value Struct_Array_Field_UnsizedElement_VariableSize_::array: {array_size} > 0xf")
        _span.append((array_size << 0))
        for elt in self.array:
            _span.extend(elt.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.array]) + 1

@dataclass
class Struct_Array_Field_UnsizedElement_VariableSize(Packet):
    s: Struct_Array_Field_UnsizedElement_VariableSize_ = field(kw_only=True, default_factory=Struct_Array_Field_UnsizedElement_VariableSize_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_UnsizedElement_VariableSize', bytes]:
        fields = {'payload': None}
        s, span = Struct_Array_Field_UnsizedElement_VariableSize_.parse(span)
        fields['s'] = s
        return Struct_Array_Field_UnsizedElement_VariableSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_Array_Field_UnsizedElement_VariableCount_(Packet):
    array: List[UnsizedStruct] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_UnsizedElement_VariableCount_', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_count = (span[0] >> 0) & 0xf
        span = span[1:]
        array = []
        for n in range(array_count):
            _elt, span = UnsizedStruct.parse(span)
            array.append(_elt)
        fields['array'] = array
        return Struct_Array_Field_UnsizedElement_VariableCount_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if len(self.array) > 0xf:
            raise ValueError("Invalid count value Struct_Array_Field_UnsizedElement_VariableCount_::array: {len(self.array)} > 0xf")
        _span.append((len(self.array) << 0))
        for elt in self.array:
            _span.extend(elt.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.array]) + 1

@dataclass
class Struct_Array_Field_UnsizedElement_VariableCount(Packet):
    s: Struct_Array_Field_UnsizedElement_VariableCount_ = field(kw_only=True, default_factory=Struct_Array_Field_UnsizedElement_VariableCount_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_UnsizedElement_VariableCount', bytes]:
        fields = {'payload': None}
        s, span = Struct_Array_Field_UnsizedElement_VariableCount_.parse(span)
        fields['s'] = s
        return Struct_Array_Field_UnsizedElement_VariableCount(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_Array_Field_UnsizedElement_UnknownSize_(Packet):
    array: List[UnsizedStruct] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_UnsizedElement_UnknownSize_', bytes]:
        fields = {'payload': None}
        array = []
        while len(span) > 0:
            _elt, span = UnsizedStruct.parse(span)
            array.append(_elt)
        fields['array'] = array
        return Struct_Array_Field_UnsizedElement_UnknownSize_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        for elt in self.array:
            _span.extend(elt.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.array])

@dataclass
class Struct_Array_Field_UnsizedElement_UnknownSize(Packet):
    s: Struct_Array_Field_UnsizedElement_UnknownSize_ = field(kw_only=True, default_factory=Struct_Array_Field_UnsizedElement_UnknownSize_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_UnsizedElement_UnknownSize', bytes]:
        fields = {'payload': None}
        s, span = Struct_Array_Field_UnsizedElement_UnknownSize_.parse(span)
        fields['s'] = s
        return Struct_Array_Field_UnsizedElement_UnknownSize(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_Array_Field_UnsizedElement_SizeModifier_(Packet):
    array: List[UnsizedStruct] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_UnsizedElement_SizeModifier_', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_size = (span[0] >> 0) & 0xf
        span = span[1:]
        array_size = array_size - +2
        if len(span) < array_size:
            raise Exception('Invalid packet size')
        array_span = span[:array_size]
        array = []
        while len(array_span) > 0:
            _elt, array_span = UnsizedStruct.parse(array_span)
            array.append(_elt)
        fields['array'] = array
        span = span[array_size:]
        return Struct_Array_Field_UnsizedElement_SizeModifier_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        array_size = +2 + sum(elt.size for elt in self.array)
        if array_size > 0xf:
            raise ValueError("Invalid size value Struct_Array_Field_UnsizedElement_SizeModifier_::array: {array_size} > 0xf")
        _span.append((array_size << 0))
        for elt in self.array:
            _span.extend(elt.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return sum([elt.size for elt in self.array]) + 1

@dataclass
class Struct_Array_Field_UnsizedElement_SizeModifier(Packet):
    s: Struct_Array_Field_UnsizedElement_SizeModifier_ = field(kw_only=True, default_factory=Struct_Array_Field_UnsizedElement_SizeModifier_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_UnsizedElement_SizeModifier', bytes]:
        fields = {'payload': None}
        s, span = Struct_Array_Field_UnsizedElement_SizeModifier_.parse(span)
        fields['s'] = s
        return Struct_Array_Field_UnsizedElement_SizeModifier(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_Array_Field_SizedElement_VariableSize_Padded_(Packet):
    array: List[int] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_SizedElement_VariableSize_Padded_', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_size = (span[0] >> 0) & 0xf
        span = span[1:]
        if len(span) < 16:
            raise Exception('Invalid packet size')
        remaining_span = span[16:]
        span = span[:16]
        if len(span) < array_size:
            raise Exception('Invalid packet size')
        if array_size % 2 != 0:
            raise Exception('Array size is not a multiple of the element size')
        array_count = int(array_size / 2)
        array = []
        for n in range(array_count):
            array.append(int.from_bytes(span[n * 2:(n + 1) * 2], byteorder='little'))
        fields['array'] = array
        span = span[array_size:]
        span = remaining_span
        return Struct_Array_Field_SizedElement_VariableSize_Padded_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        array_size = len(self.array) * 2
        if array_size > 0xf:
            raise ValueError("Invalid size value Struct_Array_Field_SizedElement_VariableSize_Padded_::array: {array_size} > 0xf")
        _span.append((array_size << 0))
        _array_start = len(_span)
        for elt in self.array:
            _span.extend(int.to_bytes(elt, length=2, byteorder='little'))
        _span.extend([0] * (16 - len(_span) + _array_start))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 17

@dataclass
class Struct_Array_Field_SizedElement_VariableSize_Padded(Packet):
    s: Struct_Array_Field_SizedElement_VariableSize_Padded_ = field(kw_only=True, default_factory=Struct_Array_Field_SizedElement_VariableSize_Padded_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_SizedElement_VariableSize_Padded', bytes]:
        fields = {'payload': None}
        if len(span) < 17:
            raise Exception('Invalid packet size')
        fields['s'] = Struct_Array_Field_SizedElement_VariableSize_Padded_.parse_all(span[0:17])
        span = span[17:]
        return Struct_Array_Field_SizedElement_VariableSize_Padded(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return 17

@dataclass
class Struct_Array_Field_UnsizedElement_VariableCount_Padded_(Packet):
    array: List[UnsizedStruct] = field(kw_only=True, default_factory=list)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_UnsizedElement_VariableCount_Padded_', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        array_count = span[0]
        span = span[1:]
        if len(span) < 16:
            raise Exception('Invalid packet size')
        remaining_span = span[16:]
        span = span[:16]
        array = []
        for n in range(array_count):
            _elt, span = UnsizedStruct.parse(span)
            array.append(_elt)
        fields['array'] = array
        span = remaining_span
        return Struct_Array_Field_UnsizedElement_VariableCount_Padded_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        if len(self.array) > 0xff:
            raise ValueError("Invalid count value Struct_Array_Field_UnsizedElement_VariableCount_Padded_::array: {len(self.array)} > 0xff")
        _span.append((len(self.array) << 0))
        _array_start = len(_span)
        for elt in self.array:
            _span.extend(elt.serialize())
        _span.extend([0] * (16 - len(_span) + _array_start))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 17

@dataclass
class Struct_Array_Field_UnsizedElement_VariableCount_Padded(Packet):
    s: Struct_Array_Field_UnsizedElement_VariableCount_Padded_ = field(kw_only=True, default_factory=Struct_Array_Field_UnsizedElement_VariableCount_Padded_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Array_Field_UnsizedElement_VariableCount_Padded', bytes]:
        fields = {'payload': None}
        if len(span) < 17:
            raise Exception('Invalid packet size')
        fields['s'] = Struct_Array_Field_UnsizedElement_VariableCount_Padded_.parse_all(span[0:17])
        span = span[17:]
        return Struct_Array_Field_UnsizedElement_VariableCount_Padded(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return 17

@dataclass
class Struct_Optional_Scalar_Field_(Packet):
    a: Optional[int] = field(kw_only=True, default=None)
    b: Optional[int] = field(kw_only=True, default=None)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Optional_Scalar_Field_', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        c0 = (span[0] >> 0) & 0x1
        c1 = (span[0] >> 1) & 0x1
        span = span[1:]
        if c0 == 0:
            if len(span) < 3:
                raise Exception('Invalid packet size')
            fields['a'] = int.from_bytes(span[:3], byteorder='little')
            span = span[3:]
        if c1 == 1:
            if len(span) < 4:
                raise Exception('Invalid packet size')
            fields['b'] = int.from_bytes(span[:4], byteorder='little')
            span = span[4:]
        return Struct_Optional_Scalar_Field_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _value = (
            ((1 if self.a is None else 0) << 0) |
            ((0 if self.b is None else 1) << 1)
        )
        _span.append(_value)
        if self.a is not None:
            _span.extend(int.to_bytes(self.a, length=3, byteorder='little'))
        if self.b is not None:
            _span.extend(int.to_bytes(self.b, length=4, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 1 + (
            (0 if self.a is None else 3) +
            (0 if self.b is None else 4)
        )

@dataclass
class Struct_Optional_Scalar_Field(Packet):
    s: Struct_Optional_Scalar_Field_ = field(kw_only=True, default_factory=Struct_Optional_Scalar_Field_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Optional_Scalar_Field', bytes]:
        fields = {'payload': None}
        s, span = Struct_Optional_Scalar_Field_.parse(span)
        fields['s'] = s
        return Struct_Optional_Scalar_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_Optional_Enum_Field_(Packet):
    a: Optional[Enum16] = field(kw_only=True, default=None)
    b: Optional[Enum16] = field(kw_only=True, default=None)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Optional_Enum_Field_', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        c0 = (span[0] >> 0) & 0x1
        c1 = (span[0] >> 1) & 0x1
        span = span[1:]
        if c0 == 0:
            if len(span) < 2:
                raise Exception('Invalid packet size')
            fields['a'] = Enum16(int.from_bytes(span[:2], byteorder='little'))
            span = span[2:]
        if c1 == 1:
            if len(span) < 2:
                raise Exception('Invalid packet size')
            fields['b'] = Enum16(int.from_bytes(span[:2], byteorder='little'))
            span = span[2:]
        return Struct_Optional_Enum_Field_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _value = (
            ((1 if self.a is None else 0) << 0) |
            ((0 if self.b is None else 1) << 1)
        )
        _span.append(_value)
        if self.a is not None:
            _span.extend(int.to_bytes(self.a, length=2, byteorder='little'))
        if self.b is not None:
            _span.extend(int.to_bytes(self.b, length=2, byteorder='little'))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 1 + (
            (0 if self.a is None else 2) +
            (0 if self.b is None else 2)
        )

@dataclass
class Struct_Optional_Enum_Field(Packet):
    s: Struct_Optional_Enum_Field_ = field(kw_only=True, default_factory=Struct_Optional_Enum_Field_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Optional_Enum_Field', bytes]:
        fields = {'payload': None}
        s, span = Struct_Optional_Enum_Field_.parse(span)
        fields['s'] = s
        return Struct_Optional_Enum_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

@dataclass
class Struct_Optional_Struct_Field_(Packet):
    a: Optional[SizedStruct] = field(kw_only=True, default=None)
    b: Optional[UnsizedStruct] = field(kw_only=True, default=None)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Optional_Struct_Field_', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        c0 = (span[0] >> 0) & 0x1
        c1 = (span[0] >> 1) & 0x1
        span = span[1:]
        if c0 == 0:
            a, span = SizedStruct.parse(span)
            fields['a'] = a
        if c1 == 1:
            b, span = UnsizedStruct.parse(span)
            fields['b'] = b
        return Struct_Optional_Struct_Field_(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _value = (
            ((1 if self.a is None else 0) << 0) |
            ((0 if self.b is None else 1) << 1)
        )
        _span.append(_value)
        if self.a is not None:
            _span.extend(self.a.serialize())
        if self.b is not None:
            _span.extend(self.b.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return 1 + (
            (0 if self.a is None else self.a.size) +
            (0 if self.b is None else self.b.size)
        )

@dataclass
class Struct_Optional_Struct_Field(Packet):
    s: Struct_Optional_Struct_Field_ = field(kw_only=True, default_factory=Struct_Optional_Struct_Field_)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Struct_Optional_Struct_Field', bytes]:
        fields = {'payload': None}
        s, span = Struct_Optional_Struct_Field_.parse(span)
        fields['s'] = s
        return Struct_Optional_Struct_Field(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.extend(self.s.serialize())
        return bytes(_span)

    @property
    def size(self) -> int:
        return self.s.size

class Enum_Incomplete_Truncated_Closed_(enum.IntEnum):
    A = 0x0
    B = 0x1

    @staticmethod
    def from_int(v: int) -> Union[int, 'Enum_Incomplete_Truncated_Closed_']:
        try:
            return Enum_Incomplete_Truncated_Closed_(v)
        except ValueError:
            raise ValueError('Invalid enum value')

@dataclass
class Enum_Incomplete_Truncated_Closed(Packet):
    e: Enum_Incomplete_Truncated_Closed_ = field(kw_only=True, default=Enum_Incomplete_Truncated_Closed_.A)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Enum_Incomplete_Truncated_Closed', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['e'] = Enum_Incomplete_Truncated_Closed_.from_int((span[0] >> 0) & 0x7)
        span = span[1:]
        return Enum_Incomplete_Truncated_Closed(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.append((self.e << 0))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 1

class Enum_Incomplete_Truncated_Open_(enum.IntEnum):
    A = 0x0
    B = 0x1

    @staticmethod
    def from_int(v: int) -> Union[int, 'Enum_Incomplete_Truncated_Open_']:
        try:
            return Enum_Incomplete_Truncated_Open_(v)
        except ValueError:
            return v

@dataclass
class Enum_Incomplete_Truncated_Open(Packet):
    e: Enum_Incomplete_Truncated_Open_ = field(kw_only=True, default=Enum_Incomplete_Truncated_Open_.A)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Enum_Incomplete_Truncated_Open', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['e'] = Enum_Incomplete_Truncated_Open_.from_int((span[0] >> 0) & 0x7)
        span = span[1:]
        return Enum_Incomplete_Truncated_Open(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.append((self.e << 0))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 1

class Enum_Incomplete_Truncated_Closed_WithRange_(enum.IntEnum):
    A = 0x0

    @staticmethod
    def from_int(v: int) -> Union[int, 'Enum_Incomplete_Truncated_Closed_WithRange_']:
        try:
            return Enum_Incomplete_Truncated_Closed_WithRange_(v)
        except ValueError:
            if v >= 0x1 and v <= 0x6:
                return v
            raise ValueError('Invalid enum value')

@dataclass
class Enum_Incomplete_Truncated_Closed_WithRange(Packet):
    e: Enum_Incomplete_Truncated_Closed_WithRange_ = field(kw_only=True, default=Enum_Incomplete_Truncated_Closed_WithRange_.A)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Enum_Incomplete_Truncated_Closed_WithRange', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['e'] = Enum_Incomplete_Truncated_Closed_WithRange_.from_int((span[0] >> 0) & 0x7)
        span = span[1:]
        return Enum_Incomplete_Truncated_Closed_WithRange(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.append((self.e << 0))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 1

class Enum_Incomplete_Truncated_Open_WithRange_(enum.IntEnum):
    A = 0x0

    @staticmethod
    def from_int(v: int) -> Union[int, 'Enum_Incomplete_Truncated_Open_WithRange_']:
        try:
            return Enum_Incomplete_Truncated_Open_WithRange_(v)
        except ValueError:
            return v

@dataclass
class Enum_Incomplete_Truncated_Open_WithRange(Packet):
    e: Enum_Incomplete_Truncated_Open_WithRange_ = field(kw_only=True, default=Enum_Incomplete_Truncated_Open_WithRange_.A)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Enum_Incomplete_Truncated_Open_WithRange', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['e'] = Enum_Incomplete_Truncated_Open_WithRange_.from_int((span[0] >> 0) & 0x7)
        span = span[1:]
        return Enum_Incomplete_Truncated_Open_WithRange(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.append((self.e << 0))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 1

class Enum_Complete_Truncated_(enum.IntEnum):
    A = 0x0
    B = 0x1
    C = 0x2
    D = 0x3
    E = 0x4
    F = 0x5
    G = 0x6
    H = 0x7

    @staticmethod
    def from_int(v: int) -> Union[int, 'Enum_Complete_Truncated_']:
        try:
            return Enum_Complete_Truncated_(v)
        except ValueError:
            raise ValueError('Invalid enum value')

@dataclass
class Enum_Complete_Truncated(Packet):
    e: Enum_Complete_Truncated_ = field(kw_only=True, default=Enum_Complete_Truncated_.A)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Enum_Complete_Truncated', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['e'] = Enum_Complete_Truncated_.from_int((span[0] >> 0) & 0x7)
        span = span[1:]
        return Enum_Complete_Truncated(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.append((self.e << 0))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 1

class Enum_Complete_Truncated_WithRange_(enum.IntEnum):
    A = 0x0

    @staticmethod
    def from_int(v: int) -> Union[int, 'Enum_Complete_Truncated_WithRange_']:
        try:
            return Enum_Complete_Truncated_WithRange_(v)
        except ValueError:
            if v >= 0x1 and v <= 0x7:
                return v
            raise ValueError('Invalid enum value')

@dataclass
class Enum_Complete_Truncated_WithRange(Packet):
    e: Enum_Complete_Truncated_WithRange_ = field(kw_only=True, default=Enum_Complete_Truncated_WithRange_.A)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Enum_Complete_Truncated_WithRange', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['e'] = Enum_Complete_Truncated_WithRange_.from_int((span[0] >> 0) & 0x7)
        span = span[1:]
        return Enum_Complete_Truncated_WithRange(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.append((self.e << 0))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 1

class Enum_Complete_WithRange_(enum.IntEnum):
    A = 0x0
    B = 0x1

    @staticmethod
    def from_int(v: int) -> Union[int, 'Enum_Complete_WithRange_']:
        try:
            return Enum_Complete_WithRange_(v)
        except ValueError:
            if v >= 0x2 and v <= 0xff:
                return v
            raise ValueError('Invalid enum value')

@dataclass
class Enum_Complete_WithRange(Packet):
    e: Enum_Complete_WithRange_ = field(kw_only=True, default=Enum_Complete_WithRange_.A)

    def __post_init__(self) -> None:
        pass

    @staticmethod
    def parse(span: bytes) -> Tuple['Enum_Complete_WithRange', bytes]:
        fields = {'payload': None}
        if len(span) < 1:
            raise Exception('Invalid packet size')
        fields['e'] = Enum_Complete_WithRange_.from_int(span[0])
        span = span[1:]
        return Enum_Complete_WithRange(**fields), span

    def serialize(self, payload: Optional[bytes] = None) -> bytes:
        _span = bytearray()
        _span.append((self.e << 0))
        return bytes(_span)

    @property
    def size(self) -> int:
        return 1
