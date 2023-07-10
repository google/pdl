Python Generated Code Guide
===========================

Usage
-----

.. sourcecode:: bash

    usage: generate_python_backend.py [-h] [--input INPUT] [--output OUTPUT] [--custom-type-location CUSTOM_TYPE_LOCATION]

    options:
      -h, --help            show this help message and exit
      --input INPUT         Input PDL-JSON source
      --output OUTPUT       Output Python file
      --custom-type-location CUSTOM_TYPE_LOCATION
                            Module of declaration of custom types

Example invocation:

.. sourcecode:: bash

    cargo run my-protocol.pdl --output-format json | \
        ./pdl-compiler/scripts/generate_python_backend.py > my-protocol.py

Language bindings
-----------------

The generator produces a pure python implementation of the parser and serializer
for the selected grammar, using only builtin features of the Python language.
The generated constructs are all type annotated and _should_ pass the type
validation.

All packets inherit either from their parent declaration or at the root
a blanket `Packet` class implementation.

.. sourcecode:: python

    @dataclass
    class Packet:
        payload: Optional[bytes] = field(repr=False, default_factory=bytes, compare=False)

Enum declarations
^^^^^^^^^^^^^^^^^

+---------------------------------------+---------------------------------------------------------------+
| ::                                    | .. sourcecode:: python                                        |
|                                       |                                                               |
|     enum TestEnum : 8 {               |     class TestEnum(enum.IntEnum):                             |
|         A = 1,                        |         A = 1                                                 |
|         B = 2..3,                     |         B_MIN = 2                                             |
|         C = 4,                        |         B_MAX = 3                                             |
|         OTHER = ..,                   |         C = 4                                                 |
|     }                                 |                                                               |
+---------------------------------------+---------------------------------------------------------------+

.. note::
    Python enums are open by construction, default cases in enum declarations are ignored.

Packet declarations
^^^^^^^^^^^^^^^^^^^

+---------------------------------------+---------------------------------------------------------------+
| ::                                    | .. sourcecode:: python                                        |
|                                       |                                                               |
|     packet TestPacket {               |     @dataclass                                                |
|         a: 8,                         |     packet TestPacket(Packet):                                |
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
