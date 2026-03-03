Python Generated Code Guide
===========================

Usage
-----

The Python backend is integrated into the ``pdlc`` tool.

Example invocation:

.. sourcecode:: bash

    pdlc --output-format python my-protocol.pdl > my-protocol.py

If your PDL file uses custom types or checksums, you must provide the module
where they are defined using the ``--custom-field`` option:

.. sourcecode:: bash

    pdlc --output-format python --custom-field my_protocol.custom_types my-protocol.pdl > my-protocol.py

Language bindings
-----------------

The generator produces a pure Python implementation of the parser and serializer
for the selected grammar, using only builtin features of the Python language.
The generated constructs are all type annotated and pass ``ruff`` validation.

All packets inherit either from their parent declaration or at the root
a blanket ``Packet`` class implementation.

.. sourcecode:: python

    @dataclass
    class Packet:
        payload: Optional[bytes] = field(repr=False, default_factory=bytes, compare=False)

        @classmethod
        def parse_all(cls, span: bytes) -> 'Packet':
            ...

        @property
        def size(self) -> int:
            return 0

        def show(self, prefix: str = '') -> None:
            ...

Enum declarations
^^^^^^^^^^^^^^^^^

+---------------------------------------+---------------------------------------------------------------+
| ::                                    | .. sourcecode:: python                                        |
|                                       |                                                               |
|     enum TestEnum : 8 {               |     class TestEnum(enum.IntEnum):                             |
|         A = 1,                        |         A = 1                                                 |
|         B = 2..3,                     |         C = 4                                                 |
|         C = 4,                        |                                                               |
|         OTHER = ..,                   |         @staticmethod                                         |
|     }                                 |         def from_int(v: int) -> Union[int, 'TestEnum']:       |
|                                       |             pass                                              |
+---------------------------------------+---------------------------------------------------------------+

.. note::
    Python enums are closed by construction, default cases in enum declarations are ignored.
    The static method ``from_int`` provides validation for enum tag ranges.

Packet declarations
^^^^^^^^^^^^^^^^^^^

+---------------------------------------+---------------------------------------------------------------+
| ::                                    | .. sourcecode:: python                                        |
|                                       |                                                               |
|     packet TestPacket {               |     @dataclass                                                |
|         a: 8,                         |     class TestPacket(Packet):                                 |
|         b: TestEnum,                  |         a: int = field(kw_only=True, default=0)               |
|     }                                 |         b: TestEnum = field(kw_only=True, default=TestEnum.A) |
|                                       |                                                               |
|                                       |         def __post_init__(self) -> None:                      |
|                                       |             pass                                              |
|                                       |                                                               |
|                                       |         @staticmethod                                         |
|                                       |         def parse(span: bytes) -> Tuple['TestPacket', bytes]: |
|                                       |             pass                                              |
|                                       |                                                               |
|                                       |         def serialize(self, payload: Optional[bytes] = None)  |
|                                       |                 -> bytes:                                     |
|                                       |             pass                                              |
|                                       |                                                               |
|                                       |         @property                                             |
|                                       |         def size(self) -> int:                                |
|                                       |             pass                                              |
+---------------------------------------+---------------------------------------------------------------+
| ::                                    | .. sourcecode:: python                                        |
|                                       |                                                               |
|     packet TestPacket: ParentPacket { |     @dataclass                                                |
|         a: 8,                         |     packet TestPacket(ParentPacket):                          |
|         b: TestEnum,                  |         a: int = field(kw_only=True, default=0)               |
|     }                                 |         b: TestEnum = field(kw_only=True, default=TestEnum.A) |
|                                       |                                                               |
|                                       |         @staticmethod                                         |
|                                       |         def parse(span: bytes) -> Tuple['TestPacket', bytes]: |
|                                       |             pass                                              |
|                                       |                                                               |
|                                       |         def serialize(self, payload: bytes = None) -> bytes:  |
|                                       |             pass                                              |
|                                       |                                                               |
|                                       |         @property                                             |
|                                       |         def size(self) -> int:                                |
|                                       |             pass                                              |
+---------------------------------------+---------------------------------------------------------------+

Field declarations
^^^^^^^^^^^^^^^^^^

Fields without a binding name do not have a concrete representation in the
generated class, but are nonetheless validated during parsing or implicitely
generated during serialization.

+---------------------------------------+---------------------------------------------------------------+
| ::                                    | .. sourcecode:: python                                        |
|                                       |                                                               |
|     a: 8                              |     a: int = field(kw_only=True, default=0)                   |
+---------------------------------------+---------------------------------------------------------------+
| ::                                    | .. sourcecode:: python                                        |
|                                       |                                                               |
|     a: TestEnum,                      |     a: TestEnum = field(kw_only=True, default=TestEnum.A)     |
|     b: TestStruct                     |     b: TestStruct = field(kw_only=True,                       |
|                                       |                           default_factory=TestStruct)         |
+---------------------------------------+---------------------------------------------------------------+
| ::                                    | .. sourcecode:: python                                        |
|                                       |                                                               |
|     a: 8[],                           |     a: List[int] = field(kw_only=True, default_factory=list)  |
|     b: 16[128],                       |     b: List[int] = field(kw_only=True, default_factory=list)  |
|     c: TestEnum[],                    |     c: List[TestEnum] = field(kw_only=True,                   |
|     d: TestStruct[]                   |                               default_factory=list)           |
|                                       |     d: List[TestStruct] = field(kw_only=True,                 |
|                                       |                                 default_factory=list)         |
+---------------------------------------+---------------------------------------------------------------+
| ::                                    | .. sourcecode:: python                                        |
|                                       |                                                               |
|     a: 8 if c_a = 1,                  |     a: Optional[int] = field(kw_only=True, default=None)      |
|     b: TestEnum if c_b = 1,           |     b: Optional[TestEnum] = field(kw_only=True, default=None) |
|     c: TestStruct if c_c = 1,         |     c: Optional[TestStruct] = field(kw_only=True,             |
|                                       |                                     default=None)             |
+---------------------------------------+---------------------------------------------------------------+

Runtime error reporting
-----------------------

Decoding and encoding errors are reported with ``DecodeError`` and ``EncodeError`` exception
variants. The following is an exhaustive list of error conditions and the reported exception.

Decoding errors
^^^^^^^^^^^^^^^

* ``DecodeError`` Parent class for all decoding exceptions.
* ``EnumValueError`` The decoded value for an enum field does not match any of the declared enum tags or tag ranges.
* ``FixedValueError`` The decoded value for a fixed scalar or enum field does not match the expected value.
* ``ConstraintValueError`` The decoded value for a scalar or enum field does not match the constraint value.
* ``LengthError`` Field decoding triggers an overflow on the input bytes.
* ``ArraySizeError`` The decoded size of an array is not a multiple of the known element size.
* ``TrailingBytesError`` Input bytes remain after decoding the entire packet.

Encoding errors
^^^^^^^^^^^^^^^

