# Packet Description Language

[TOC]

## Notation

|    Notation   |            Example           |                        Meaning                       |
|:-------------:|:----------------------------:|:----------------------------------------------------:|
| __ANY__       | __ANY__                      | Any character                                        |
| CAPITAL       | IDENTIFIER, INT              | A token production                                   |
| snake_case    | declaration, constraint      | A syntactical production                             |
| `string`      | `enum`, `=`                  | The exact character(s)                               |
| \x            | \n, \r, \t, \0               | The character represented by this escape             |
| x?            | `,`?                         | An optional item                                     |
| x*            | ALPHANUM*                    | 0 or more of x                                       |
| x+            | HEXDIGIT+                    | 1 or more of x                                       |
| x \| y        | ALPHA \| DIGIT, `0x` \| `0X` | Either x or y                                        |
| [x-y]         | [`a`-`z`]                    | Any of the characters in the range from x to y       |
| !x            | !\n                          | Negative Predicate (lookahead), do not consume input |
| ()            | (`,` enum_tag)               | Groups items                                         |


[WHITESPACE](#Whitespace) and [COMMENT](#Comment) are implicitly inserted between every item
and repetitions in syntactical rules (snake_case).

```
file: endianess declaration*
```
behaves like:
```
file: (WHITESPACE | COMMENT)* endianess (WHITESPACE | COMMENT)* (declaration | WHITESPACE | COMMENT)*
```

## File

> file:\
> &nbsp;&nbsp; endianess [declaration](#declarations)*
>
> endianess:\
> &nbsp;&nbsp; `little_endian_packets` | `big_endian_packets`

The structure of a `.pdl`file is:
1. A declaration of the protocol endianess: `little_endian_packets` or `big_endian_packets`. Followed by
2. Declarations describing the structure of the protocol.

```
// The protocol is little endian
little_endian_packets

// Brew a coffee
packet Brew {
  pot: 8, // Output Pot: 8bit, 0-255
  additions: CoffeeAddition[2] // Coffee Additions: array of 2 CoffeeAddition
}
```

The endianess affects how fields of fractional byte sizes (hence named
bit-fields) are parsed or serialized. Such fields are grouped together to the
next byte boundary, least significant bit first, and then byte-swapped to the
required endianess before being written to memory, or after being read from
memory.

```
packet Coffee {
  a: 1,
  b: 15,
  c: 3,
  d: 5,
}

// The first two field are laid out as a single
// integer of 16-bits
//     MSB                                   LSB
//     16                  8                 0
//     +---------------------------------------+
//     | b14 ..                        .. b0 |a|
//     +---------------------------------------+
//
// The file endianness is applied to this integer
// to obtain the byte layout of the packet fields.
//
// Little endian layout
//     MSB                                   LSB
//     7    6    5    4    3    2    1    0
//     +---------------------------------------+
//  0  |            b[6:0]                | a  |
//     +---------------------------------------+
//  1  |               b[14:7]                 |
//     +---------------------------------------+
//  2  |          d             |       c      |
//     +---------------------------------------+
//
// Big endian layout
//     MSB                                   LSB
//     7    6    5    4    3    2    1    0
//     +---------------------------------------+
//  0  |               b[14:7]                 |
//     +---------------------------------------+
//  1  |            b[6:0]                | a  |
//     +---------------------------------------+
//  2  |          d             |       c      |
//     +---------------------------------------+
```

Fields which qualify as bit-fields are:
- [Scalar](#fields-scalar) fields
- [Size](#fields-size) fields
- [Count](#fields-count) fields
- [Fixed](#fields-fixed) fields
- [Reserved](#fields-reserved) fields
- [Typedef](#fields-typedef) fields, when the field type is an
  [Enum](#enum)

Fields that do not qualify as bit-fields _must_ start and end on a byte boundary.

## Identifiers

- Identifiers can denote a field; an enumeration tag; or a declared type.

- Field identifiers declared in a [packet](#packet) (resp. [struct](#struct)) belong to the _scope_ that extends
  to the packet (resp. struct), and all derived packets (resp. structs).

- Field identifiers declared in a [group](#group) belong to the _scope_ that
  extends to the packets declaring a [group field](#group_field) for this group.

- Two fields may not be declared with the same identifier in any packet scope.

- Two types may not be declared width the same identifier.

## Declarations

> declaration: {#declaration}\
> &nbsp;&nbsp; [enum_declaration](#enum) |\
> &nbsp;&nbsp; [packet_declaration](#packet) |\
> &nbsp;&nbsp; [struct_declaration](#struct) |\
> &nbsp;&nbsp; [group_declaration](#group) |\
> &nbsp;&nbsp; [checksum_declaration](#checksum) |\
> &nbsp;&nbsp; [custom_field_declaration](#custom-field) |\
> &nbsp;&nbsp; [test_declaration](#test)

A *declaration* defines a type inside a `.pdl` file. A declaration can reference
another declaration appearing later in the file.

A declaration is either:
- an [Enum](#enum) declaration
- a [Packet](#packet) declaration
- a [Struct](#struct) declaration
- a [Group](#group) declaration
- a [Checksum](#checksum) declaration
- a [Custom Field](#custom-field) declaration
- a [Test](#test) declaration

### Enum

> enum_declaration:\
> &nbsp;&nbsp; `enum` [IDENTIFIER](#identifier) `:` [INTEGER](#integer) `{`\
> &nbsp;&nbsp;&nbsp;&nbsp; enum_tag_list\
> &nbsp;&nbsp; `}`
>
> enum_tag_list:\
> &nbsp;&nbsp; enum_tag (`,` enum_tag)* `,`?
>
> enum_tag:\
> &nbsp;&nbsp; [IDENTIFIER](#identifier) `=` [INTEGER](#integer)

An *enumeration* or for short *enum*, is a declaration of a set of named [integer](#integer) constants.

The [integer](#integer) following the name specifies the bit size of the values.

```
enum CoffeeAddition: 3 {
  Empty = 0,
  Cream = 1,
  Vanilla = 2,
  Chocolate = 3,
  Whisky = 4,
  Rum = 5,
  Kahlua = 6,
  Aquavit = 7
}
```

### Packet

> packet_declaration:\
> &nbsp;&nbsp; `packet` [IDENTIFIER](#identifier)\
> &nbsp;&nbsp;&nbsp;&nbsp; (`:` [IDENTIFIER](#identifier)\
> &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; (`(` [constraint_list](#constraints) `)`)?\
> &nbsp;&nbsp;&nbsp;&nbsp; )?\
> &nbsp;&nbsp; `{`\
> &nbsp;&nbsp;&nbsp;&nbsp; [field_list](#fields)?\
> &nbsp;&nbsp; `}`

A *packet* is a declaration of a sequence of [fields](#fields). While packets
can contain bit-fields, the size of the whole packet must be a multiple of 8
bits.

A *packet* can optionally inherit from another *packet* declaration. In this case the packet
inherits the parent's fields and the child's fields replace the
[*\_payload\_*](#fields-payload) or [*\_body\_*](#fields-body) field of the parent.

When inheriting, you can use constraints to set values on parent fields.
See [constraints](#constraints) for more details.

```
packet Error {
  code: 32,
  _payload_
}

packet ImATeapot: Error(code = 418) {
  brand_id: 8
}
```

### Struct

> struct_declaration:\
> &nbsp;&nbsp; `struct` [IDENTIFIER](#identifier)\
> &nbsp;&nbsp;&nbsp;&nbsp; (`:` [IDENTIFIER](#identifier)\
> &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; (`(` [constraint_list](#constraints) `)`)?\
> &nbsp;&nbsp;&nbsp;&nbsp; )?\
> &nbsp;&nbsp; `{`\
> &nbsp;&nbsp;&nbsp;&nbsp; [field_list](#fields)?\
> &nbsp;&nbsp; `}`

A *struct* follows the same rules as a [*packet*](#packet) with the following differences:
- It inherits from a *struct* declaration instead of *packet* declaration.
- A [typedef](#fields-typedef) field can reference a *struct*.

### Group

> group_declaration:\
> &nbsp;&nbsp; `group` [IDENTIFIER](#identifier) `{`\
> &nbsp;&nbsp;&nbsp;&nbsp; [field_list](#fields)\
> &nbsp;&nbsp; `}`

A *group* is a sequence of [fields](#fields) that expand in a
[packet](#packet) or [struct](#struct) when used.

See also the [Group field](#fields-group).

```
group Paged {
  offset: 8,
  limit: 8
}

packet AskBrewHistory {
  pot: 8, // Coffee Pot
  Paged
}
```
behaves like:
```
packet AskBrewHistory {
  pot: 8, // Coffee Pot
  offset: 8,
  limit: 8
}
```

### Checksum

> checksum_declaration:\
> &nbsp;&nbsp; `checksum` [IDENTIFIER](#identifier) `:` [INTEGER](#integer) [STRING](#string)

A *checksum* is a native type (not implemented in PDL). See your generator documentation
for more information on how to use it.

The [integer](#integer) following the name specify the bit size of the checksum value.
The [string](#string) following the size is a value defined by the generator implementation.

```
checksum CRC16: 16 "crc16"
```

### Custom Field

> custom_field_declaration:\
> &nbsp;&nbsp; `custom_field` [IDENTIFIER](#identifier) (`:` [INTEGER](#integer))? [STRING](#string)

A *custom field* is a native type (not implemented in PDL). See your generator documentation for more
information on how to use it.

If present, the [integer](#integer) following the name specify the bit size of the value.
The [string](#string) following the size is a value defined by the generator implementation.

```
custom_field URL "url"
```

### Test

> test_declaration:\
> &nbsp;&nbsp; `test` [IDENTIFIER](#identifier) `{`\
> &nbsp;&nbsp;&nbsp;&nbsp; test_case_list\
> &nbsp;&nbsp; `}`
>
> test_case_list:\
> &nbsp;&nbsp; test_case (`,` test_case)* `,`?
>
> test_case:\
> &nbsp;&nbsp; [STRING](#string)

A *test* declares a set of valid octet representations of a packet identified by its name.
The generator implementation defines how to use the test data.

A test passes if the packet parser accepts the input; if you want to test
the values returned for each field, you may specify a derived packet with field values enforced using
constraints.

```
packet Brew {
  pot: 8,
  addition: CoffeeAddition
}

test Brew {
  "\x00\x00",
  "\x00\x04"
}

// Fully Constrained Packet
packet IrishCoffeeBrew: Brew(pot = 0, additions_list = Whisky) {}

test IrishCoffeeBrew {
  "\x00\x04"
}
```

## Constraints

> constraint:\
> &nbsp;&nbsp; [IDENTIFIER](#identifier) `=` [IDENTIFIER](#identifier) | [INTEGER](#integer)
>
> constraint_list:\
> &nbsp;&nbsp; constraint (`,` constraint)* `,`?

A *constraint* defines the value of a parent field.
The value can either be an [enum](#enum) tag or an [integer](#integer).

```
group Additionable {
  addition: CoffeAddition
}

packet IrishCoffeeBrew {
  pot: 8,
  Additionable {
    addition = Whisky
  }
}

packet Pot0IrishCoffeeBrew: IrishCoffeeBrew(pot = 0) {}
```

## Fields

> field_list:\
> &nbsp;&nbsp; field (`,` field)* `,`?
>
> field:\
> &nbsp;&nbsp; [checksum_field](#fields-checksum) |\
> &nbsp;&nbsp; [padding_field](#fields-padding) |\
> &nbsp;&nbsp; [size_field](#fields-size) |\
> &nbsp;&nbsp; [count_field](#fields-count) |\
> &nbsp;&nbsp; [payload_field](#fields-payload) |\
> &nbsp;&nbsp; [body_field](#fields-body) |\
> &nbsp;&nbsp; [fixed_field](#fields-fixed) |\
> &nbsp;&nbsp; [reserved_field](#fields-reserved) |\
> &nbsp;&nbsp; [array_field](#fields-array) |\
> &nbsp;&nbsp; [scalar_field](#fields-scalar) |\
> &nbsp;&nbsp; [typedef_field](#fields-typedef) |\
> &nbsp;&nbsp; [group_field](#fields-group)

A field is either:
- a [Scalar](#fields-scalar) field
- a [Typedef](#fields-typedef) field
- a [Group](#fields-group) field
- an [Array](#fields-array) field
- a [Size](#fields-size) field
- a [Count](#fields-count) field
- a [Payload](#fields-payload) field
- a [Body](#fields-body) field
- a [Fixed](#fields-fixed) field
- a [Checksum](#fields-checksum) field
- a [Padding](#fields-padding) field
- a [Reserved](#fields-reserved) field

### Scalar {#fields-scalar}

> scalar_field:\
> &nbsp;&nbsp; [IDENTIFIER](#identifier) `:` [INTEGER](#integer)

A *scalar* field defines a numeric value with a bit size.

```
struct Coffee {
  temperature: 8
}
```

### Typedef {#fields-typedef}

> typedef_field:\
> &nbsp;&nbsp; [IDENTIFIER](#identifier) `:` [IDENTIFIER](#identifier)

A *typedef* field defines a field taking as value either an [enum](#enum), [struct](#struct),
[checksum](#checksum) or a [custom_field](#custom-field).

```
packet LastTimeModification {
  coffee: Coffee,
  addition: CoffeeAddition
}
```

### Array {#fields-array}

> array_field:\
> &nbsp;&nbsp; [IDENTIFIER](#identifier) `:` [INTEGER](#integer) | [IDENTIFIER](#identifier) `[`\
> &nbsp;&nbsp;&nbsp;&nbsp; [SIZE_MODIFIER](#size-modifier) | [INTEGER](#integer)\
> &nbsp;&nbsp; `]`

An *array* field defines a sequence of `N` elements of type `T`.

`N` can be:
- An [integer](#integer) value.
- A [size modifier](#size-modifier).
- Unspecified: In this case the array is dynamically sized using a
[*\_size\_*](#fields-size) or a [*\_count\_*](#fields-count).

`T` can be:
- An [integer](#integer) denoting the bit size of one element.
- An [identifier](#identifier) referencing an [enum](#enum), a [struct](#struct)
or a [custom field](#custom-field) type.

The size of `T` must always be a multiple of 8 bits, that is, the array elements
must start at byte boundaries.

```
packet Brew {
   pots: 8[2],
   additions: CoffeeAddition[2],
   extra_additions: CoffeeAddition[],
}
```

### Group {#fields-group}

> group_field:\
> &nbsp;&nbsp; [IDENTIFIER](#identifier) (`{` [constraint_list](#constraints) `}`)?

A *group* field inlines all the fields defined in the referenced group.

If a [constraint list](#constraints) constrains a [scalar](#fields-scalar) field
or [typedef](#fields-typedef) field with an [enum](#enum) type, the field will
become a [fixed](#fields-fixed) field.
The [fixed](#fields-fixed) field inherits the type or size of the original field and the
value from the constraint list.

See [Group Declaration](#group) for more information.

### Size {#fields-size}

> size_field:\
> &nbsp;&nbsp; `_size_` `(` [IDENTIFIER](#identifier) | `_payload_` | `_body_` `)` `:` [INTEGER](#integer)

A *\_size\_* field is a [scalar](#fields-scalar) field with as value the size in octet of the designated
[array](#fields-array), [*\_payload\_*](#fields-payload) or [*\_body\_*](#fields-body).

```
packet Parent {
  _size_(_payload_): 2,
  _payload_
}

packet Brew {
  pot: 8,
  _size_(additions): 8,
  additions: CoffeeAddition[]
}
```

### Count {#fields-count}

> count_field:\
> &nbsp;&nbsp; `_count_` `(` [IDENTIFIER](#identifier) `)` `:` [INTEGER](#integer)

A *\_count\_* field is a [*scalar*](#fields-scalar) field with as value the number of elements of the designated
[array](#fields-array).

```
packet Brew {
  pot: 8,
  _count_(additions): 8,
  additions: CoffeeAddition[]
}
```

### Payload {#fields-payload}

> payload_field:\
> &nbsp;&nbsp; `_payload_` (`:` `[` [SIZE_MODIFIER](#size-modifier) `]` )?

A *\_payload\_* field is a dynamically sized array of octets.

It declares where to parse the definition of a child [packet](#packet) or [struct](#struct).

A [*\_size\_*](#fields-size) or a [*\_count\_*](#fields-count) field referencing
the payload induce its size.

If used, a [size modifier](#size-modifier) can alter the octet size.

### Body {#fields-body}

> body_field:\
> &nbsp;&nbsp; `_body_`

A *\_body\_* field is like a [*\_payload\_*](#fields-payload) field with the following differences:
- The body field is private to the packet definition, it's accessible only when inheriting.
- The body does not accept a size modifier.

### Fixed {#fields-fixed}

> fixed_field:\
> &nbsp;&nbsp; `_fixed_` `=` \
> &nbsp;&nbsp;&nbsp;&nbsp; ( [INTEGER](#integer) `:` [INTEGER](#integer) ) |\
> &nbsp;&nbsp;&nbsp;&nbsp; ( [IDENTIFIER](#identifier) `:` [IDENTIFIER](#identifier) )

A *\_fixed\_* field defines a constant with a known bit size.
The constant can be either:
- An [integer](#integer) value
- An [enum](#enum) tag

```
packet Teapot {
  _fixed_ = 42: 8,
  _fixed_ = Empty: CoffeeAddition
}
```

### Checksum {#fields-checksum}

> checksum_field:\
> &nbsp;&nbsp; `_checksum_start_` `(` [IDENTIFIER](#identifier) `)`

A *\_checksum_start\_* field is a zero sized field that acts as a marker for the beginning of
the fields covered by a checksum.

The *\_checksum_start\_* references a [typedef](#fields-typedef) field
with a [checksum](#checksum) type that stores the checksum value and selects the algorithm
for the checksum.

```
checksum CRC16: 16 "crc16"

packet CRCedBrew {
  crc: CRC16,
  _checksum_start_(crc),
  pot: 8,
}
```

### Padding {#fields-padding}

> padding_field:\
> &nbsp;&nbsp; `_padding_` `[` [INTEGER](#integer) `]`

A *\_padding\_* field immediately following an array field pads the array field with `0`s to the
specified number of **octets**.

```
packet PaddedCoffee {
  additions: CoffeeAddition[],
  _padding_[100]
}
```

### Reserved {#fields-reserved}

> reserved_field:\
> &nbsp;&nbsp; `_reserved_` `:` [INTEGER](#integer)

A *\_reserved\_* field adds reserved bits.

```
packet DeloreanCoffee {
  _reserved_: 2014
}
```

## Tokens

### Integer

> INTEGER:\
> &nbsp;&nbsp; HEXVALUE | INTVALUE
>
> HEXVALUE:\
> &nbsp;&nbsp; `0x` | `0X` HEXDIGIT<sup>+</sup>
>
> INTVALUE:\
> &nbsp;&nbsp; DIGIT<sup>+</sup>
>
> HEXDIGIT:\
> &nbsp;&nbsp; DIGIT | [`a`-`f`] | [`A`-`F`]
>
> DIGIT:\
> &nbsp;&nbsp; [`0`-`9`]

A integer is a number in base 10 (decimal) or in base 16 (hexadecimal) with
the prefix `0x`

### String

> STRING:\
> &nbsp;&nbsp; `"` (!`"` __ANY__)* `"`

A string is sequence of character. It can be multi-line.

### Identifier

> IDENTIFIER: \
> &nbsp;&nbsp; ALPHA (ALPHANUM | `_`)*
>
> ALPHA:\
> &nbsp;&nbsp; [`a`-`z`] | [`A`-`Z`]
>
> ALPHANUM:\
> &nbsp;&nbsp; ALPHA | DIGIT

An identifier is a sequence of alphanumeric or `_` characters
starting with a letter.

### Size Modifier

> SIZE_MODIFIER:\
> &nbsp;&nbsp; `+` INTVALUE

A size modifier alters the octet size of the field it is attached to.
For example, `+ 2` defines that the size is 2 octet bigger than the
actual field size.

### Comment

> COMMENT:\
> &nbsp;&nbsp; BLOCK_COMMENT | LINE_COMMENT
>
> BLOCK_COMMENT:\
> &nbsp;&nbsp; `/*` (!`*/` ANY) `*/`
>
> LINE_COMMENT:\
> &nbsp;&nbsp; `//` (!\n ANY) `//`

### Whitespace

> WHITESPACE:\
> &nbsp;&nbsp; ` ` | `\t` | `\n`
