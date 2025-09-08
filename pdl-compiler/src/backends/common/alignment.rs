// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    cmp::Reverse,
    fmt::Debug,
    ops::{Deref, DerefMut, Range},
};

/// A language-specific representation of the value of a PDL field.
pub trait Symbol: Clone + Debug + Eq {}

impl<S> Symbol for S where S: Clone + Debug + Eq {}

/// A field that contains a partial or complete value. May not be byte aligned.
#[derive(Debug, Clone, PartialEq)]
pub struct Field<S: Symbol> {
    /// Offset into the chunk where this field starts.
    pub chunk_offset: usize,
    /// Symbol that holds the value to encode.
    pub symbol: S,
    /// Width of the value to encode in bits.
    /// If the value is partial, this is the width of the portion to encode.
    pub width: usize,
    /// Offset into the symbol at which the partial value to encode starts.
    pub symbol_offset: usize,
    /// Whether this field contains a partial value.
    pub is_partial: bool,
}

/// A field that contains a partial value.
#[derive(Debug, Clone, PartialEq)]
pub struct Partial {
    pub width: usize,
    pub offset: usize,
}

/// A chunk of bytes with a client-specified width. Each chunk maps to a language-specific representation of a PDL field's value.
/// Chunks should be easy to en/decode in your language.
#[derive(Debug, Clone, PartialEq)]
pub enum Chunk<S: Symbol> {
    /// A sequence of *field*s, where each *field* maps to a contiguous slice of a (or a complete) field defined within the PDL.
    /// This is used to align PDL fields that may not be byte divisible, such as scalars and enums.
    ///
    /// For example, assuming `allowed_chunk_widths` := {8, 16, 32, 64}:
    /// ```pdl
    /// packet MyPacket {
    ///     a: 56,
    /// }
    /// ```
    /// Will align to the following:
    ///
    /// | Chunk | Fields      | Width |
    /// |-------|-------------|-------|
    /// | 0     | a\[0..32\]  | 32    |
    /// | 1     | a\[32..48\] | 16    |
    /// | 2     | a\[48..56\] | 8     |
    ///
    /// Sometimes, a chunk will contain multiple fields. For example:
    /// ```pdl
    /// packet MyPacket {
    ///     a: 1,
    ///     b: 7,
    /// }
    /// ```
    /// Aligns to:
    ///
    /// |Chunk | Fields               | Width |
    /// |------|----------------------|-------|
    /// |0     | a\[0..1\], b\[0..7\] | 8     |
    Bitpack { fields: Vec<Field<S>>, width: usize },
    /// A sequence of bytes with known size (specified in bits). This is useful for aligning PDL arrays whose size is known at compile time.
    SizedBytes {
        symbol: S,
        alignment: Vec<Partial>,
        #[allow(dead_code)]
        width: usize,
    },
    /// A sequence of bytes with unknown size. This is useful for aligning PDL fields whose size is not known at compile time.
    UnsizedBytes(S),
}

impl<S: Symbol> Chunk<S> {
    fn get_bitpack(self) -> Option<(Vec<Field<S>>, usize)> {
        match self {
            Chunk::Bitpack { fields, width } => Some((fields, width)),
            _ => None,
        }
    }
}

/// Packs PDL fields of various sizes, which may not be byte-divisible, into a sequence of chunks.
/// A chunk is a group of bytes that can be easily en/decoded in your language. The widths of these groups are configurable via `allowed_chunk_widths`.
///
/// For example, many languages offer a (de)serialization library that can easily en/decode fields of size 8, 16, 32, 64. These will be your `allowed_chunk_widths`.
#[derive(Debug, Clone)]
pub struct ByteAligner<S: Symbol> {
    /// Sorted in descending order.
    allowed_chunk_widths: Vec<usize>,
    chunks: Vec<Chunk<S>>,
}

impl<S: Symbol> ByteAligner<S> {
    /// All elements of `allowed_chunk_widths` must satisfy `width % 8 == 0`, and `allowed_chunk_widths` **must**
    /// contain `8`.
    pub fn new(allowed_chunk_widths: &'static [usize]) -> Self {
        if !allowed_chunk_widths.iter().all(|width| width % 8 == 0) {
            panic!("All allowed chunk widths must be byte alignable")
        }
        if !allowed_chunk_widths.contains(&8) {
            panic!("Must allow byte-sized chunks")
        }
        let mut allowed_chunk_widths = allowed_chunk_widths.to_vec();
        allowed_chunk_widths.sort_by_key(|&width| Reverse(width));
        Self { allowed_chunk_widths, chunks: vec![] }
    }

    /// Get the generated chunks.
    ///
    /// Each chunk within the vec will be sized according to the `allowed_chunk_widths`,
    /// so the returned data structure should represent a straightforward way to en/decode the fields in your language.
    pub fn align(mut self) -> Result<Alignment<S>, &'static str> {
        match self.chunks.last() {
            Some(Chunk::Bitpack { width, .. }) if *width % 8 != 0 => {
                Err("Provided fields could not be aligned to the allowed chunk widths")
            }
            None => Err("No fields provided"),
            _ => {
                self.repack_last_chunk();
                Ok(Alignment(self.chunks))
            }
        }
    }

    /// Add a PDL field to the alignment. The `symbol` is a language-specific construct that represents the field's value.
    /// This field can have any width.
    pub fn add_bitfield(&mut self, symbol: S, width: usize) {
        match self.chunks.last_mut() {
            Some(Chunk::Bitpack { fields, width: chunk_width }) if *chunk_width % 8 != 0 => {
                // Update this chunk with field
                if *chunk_width + width > *self.allowed_chunk_widths.first().unwrap() {
                    panic!("total field width grew beyond maximum chunk width before aligning to a byte boundary")
                }
                fields.push(Field {
                    chunk_offset: *chunk_width,
                    symbol,
                    width,
                    symbol_offset: 0,
                    is_partial: false,
                });
                *chunk_width += width;
            }
            _ => {
                // Add field to a new chunk.
                self.repack_last_chunk();
                self.chunks.push(Chunk::Bitpack {
                    fields: vec![Field {
                        chunk_offset: 0,
                        symbol: symbol.clone(),
                        width,
                        symbol_offset: 0,
                        is_partial: false,
                    }],
                    width,
                });
            }
        }
    }

    /// If the last chunk does not have an `allowed_chunk_width`, split it into several that do.
    fn repack_last_chunk(&mut self) {
        if let Some(Chunk::Bitpack { width, .. }) = self.chunks.last() {
            if !self.allowed_chunk_widths.contains(width) {
                let (old_fields, old_chunk_width) =
                    self.chunks.pop().unwrap().get_bitpack().unwrap();

                let mut repacked_width = 0;
                while repacked_width != old_chunk_width {
                    for chunk_width in self.allowed_chunk_widths.iter() {
                        if *chunk_width <= (old_chunk_width - repacked_width) {
                            self.chunks.push(Chunk::Bitpack {
                                fields: Self::slice_fields(
                                    &old_fields,
                                    repacked_width..(repacked_width + *chunk_width),
                                ),
                                width: *chunk_width,
                            });
                            repacked_width += *chunk_width
                        }
                    }
                }
            }
        }
    }

    /// Extract out all fields that lie within the specified range, splitting them when necessary.
    fn slice_fields(fields: &Vec<Field<S>>, range: Range<usize>) -> Vec<Field<S>> {
        let mut in_range = Vec::<Field<S>>::new();

        let mut chunk_offset = 0;
        for field in fields {
            let field_range = field.chunk_offset..(field.chunk_offset + field.width);

            if range.contains(&field_range.start) && range.contains(&(field_range.end - 1)) {
                // The field lies entirely within the specified range.
                in_range.push(Field {
                    chunk_offset,
                    symbol: field.symbol.clone(),
                    width: field.width,
                    symbol_offset: field.symbol_offset,
                    is_partial: field.is_partial,
                });
            } else if field_range.contains(&range.start) && field_range.contains(&(range.end - 1)) {
                // The specified range lies entirely within the field.
                in_range.push(Field {
                    chunk_offset,
                    symbol: field.symbol.clone(),
                    width: range.end - range.start,
                    symbol_offset: range.start - field_range.start,
                    is_partial: true,
                });
            } else if range.contains(&field_range.start) {
                // The field starts within the specified range, but ends outside of it.
                in_range.push(Field {
                    chunk_offset,
                    symbol: field.symbol.clone(),
                    width: range.end - field_range.start,
                    symbol_offset: 0,
                    is_partial: true,
                });
            } else if range.contains(&(field_range.end - 1)) {
                // The field starts outside the specified range and ends inside it.
                in_range.push(Field {
                    chunk_offset,
                    symbol: field.symbol.clone(),
                    width: field_range.end - range.start,
                    symbol_offset: range.start - field_range.start,
                    is_partial: true,
                });
            }
            chunk_offset += in_range.last().map(|field| field.width).unwrap_or(0);
        }

        in_range
    }

    /// Add a PDL field to the alignment. The `symbol` is a language-specific construct that represents the field's value.
    /// This field's width must satisfy `width % 8 == 0`.
    pub fn add_sized_bytes(&mut self, symbol: S, width: usize) {
        if width % 8 != 0 {
            panic!("width must be byte-divisible")
        }
        self.repack_last_chunk();

        let mut alignment = Vec::new();

        let mut remaining_width = width;
        while remaining_width != 0 {
            for chunk_width in self.allowed_chunk_widths.iter() {
                if remaining_width >= *chunk_width {
                    let offset = alignment
                        .last()
                        .map(|Partial { width, offset }| offset + width)
                        .unwrap_or(0);

                    alignment.push(Partial { width: *chunk_width, offset });
                    remaining_width -= *chunk_width;
                }
            }
        }

        self.chunks.push(Chunk::SizedBytes { symbol, alignment, width });
    }

    /// Add a PDL field to the alignment. The `symbol` is a language-specific construct that represents the field's value.
    /// This field's width does not need to be known at compile-time, although it must be byte-divisible.
    pub fn add_bytes(&mut self, symbol: S) {
        self.repack_last_chunk();
        self.chunks.push(Chunk::UnsizedBytes(symbol));
    }
}

/// An alignment for a sequence of PDL fields, generated from a [`ByteAligner`].
/// To generate (de)serialization code from an `Alignment`, you must iterate over it and handle the 3 different Chunk variants:
/// 1) [`Chunk::Bitpack`]
/// 2) [`Chunk::SizedBytes`]
/// 3) [`Chunk::UnsizedBytes`]
#[derive(Debug, Clone, PartialEq)]
pub struct Alignment<S: Symbol>(Vec<Chunk<S>>);

impl<S: Symbol> Deref for Alignment<S> {
    type Target = Vec<Chunk<S>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<S: Symbol> DerefMut for Alignment<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_multiple_fields_into_one_chunk() {
        let mut a = ByteAligner::<&'static str>::new(&[8, 64]);
        a.add_bitfield("a", 13);
        a.add_bitfield("b", 51);

        assert_eq!(
            a.align().unwrap(),
            Alignment(vec![Chunk::Bitpack {
                fields: vec![
                    Field {
                        chunk_offset: 0,
                        symbol: "a",
                        width: 13,
                        symbol_offset: 0,
                        is_partial: false
                    },
                    Field {
                        chunk_offset: 13,
                        symbol: "b",
                        width: 51,
                        symbol_offset: 0,
                        is_partial: false
                    }
                ],
                width: 64
            }])
        )
    }

    #[test]
    fn pack_multiple_fields_into_multiple_chunks() {
        let mut a = ByteAligner::<&'static str>::new(&[8, 16]);
        a.add_bitfield("a", 1);
        a.add_bitfield("b", 15);
        a.add_bitfield("c", 3);
        a.add_bitfield("d", 5);

        assert_eq!(
            a.align().unwrap(),
            Alignment(vec![
                Chunk::Bitpack {
                    fields: vec![
                        Field {
                            chunk_offset: 0,
                            symbol: "a",
                            width: 1,
                            symbol_offset: 0,
                            is_partial: false
                        },
                        Field {
                            chunk_offset: 1,
                            symbol: "b",
                            width: 15,
                            symbol_offset: 0,
                            is_partial: false
                        }
                    ],
                    width: 16
                },
                Chunk::Bitpack {
                    fields: vec![
                        Field {
                            chunk_offset: 0,
                            symbol: "c",
                            width: 3,
                            symbol_offset: 0,
                            is_partial: false
                        },
                        Field {
                            chunk_offset: 3,
                            symbol: "d",
                            width: 5,
                            symbol_offset: 0,
                            is_partial: false
                        }
                    ],
                    width: 8
                },
            ])
        );
    }

    #[test]
    fn split_single_field_into_multiple_chunks() {
        let mut a = ByteAligner::<&'static str>::new(&[8, 16, 32]);
        a.add_bitfield("a", 56);

        assert_eq!(
            a.align().unwrap(),
            Alignment(vec![
                Chunk::Bitpack {
                    fields: vec![Field {
                        chunk_offset: 0,
                        symbol: "a",
                        width: 32,
                        symbol_offset: 0,
                        is_partial: true
                    }],
                    width: 32
                },
                Chunk::Bitpack {
                    fields: vec![Field {
                        chunk_offset: 0,
                        symbol: "a",
                        width: 16,
                        symbol_offset: 32,
                        is_partial: true
                    }],
                    width: 16
                },
                Chunk::Bitpack {
                    fields: vec![Field {
                        chunk_offset: 0,
                        symbol: "a",
                        width: 8,
                        symbol_offset: 48,
                        is_partial: true
                    }],
                    width: 8
                }
            ])
        )
    }

    #[test]
    fn split_multiple_fields_into_multiple_chunks() {
        let mut a = ByteAligner::<&'static str>::new(&[8, 16, 32, 64]);
        a.add_bitfield("a", 9);
        a.add_bitfield("b", 1);
        a.add_bitfield("c", 21);
        a.add_bitfield("d", 9);

        assert_eq!(
            a.align().unwrap(),
            Alignment(vec![
                Chunk::Bitpack {
                    fields: vec![
                        Field {
                            chunk_offset: 0,
                            symbol: "a",
                            width: 9,
                            symbol_offset: 0,
                            is_partial: false
                        },
                        Field {
                            chunk_offset: 9,
                            symbol: "b",
                            width: 1,
                            symbol_offset: 0,
                            is_partial: false
                        },
                        Field {
                            chunk_offset: 10,
                            symbol: "c",
                            width: 21,
                            symbol_offset: 0,
                            is_partial: false
                        },
                        Field {
                            chunk_offset: 31,
                            symbol: "d",
                            width: 1,
                            symbol_offset: 0,
                            is_partial: true
                        }
                    ],
                    width: 32
                },
                Chunk::Bitpack {
                    fields: vec![Field {
                        chunk_offset: 0,
                        symbol: "d",
                        width: 8,
                        symbol_offset: 1,
                        is_partial: true
                    }],
                    width: 8
                }
            ])
        )
    }

    #[test]
    fn split_sized_bytes_into_multiple_partials() {
        let mut a = ByteAligner::<&'static str>::new(&[8, 16, 32, 64]);
        a.add_sized_bytes("a", 56);

        assert_eq!(
            a.align().unwrap(),
            Alignment(vec![Chunk::SizedBytes {
                symbol: "a",
                alignment: vec![
                    Partial { width: 32, offset: 0 },
                    Partial { width: 16, offset: 32 },
                    Partial { width: 8, offset: 48 },
                ],
                width: 56
            }])
        )
    }

    #[test]
    fn bitfields_separated_by_dynamic_bytes() {
        let mut a = ByteAligner::<&'static str>::new(&[8, 16, 32, 64]);
        a.add_bitfield("a", 24);
        a.add_bytes("b");
        a.add_bitfield("c", 9);
        a.add_bitfield("d", 7);

        assert_eq!(
            a.align().unwrap(),
            Alignment(vec![
                Chunk::Bitpack {
                    fields: vec![Field {
                        chunk_offset: 0,
                        symbol: "a",
                        width: 16,
                        symbol_offset: 0,
                        is_partial: true
                    }],
                    width: 16
                },
                Chunk::Bitpack {
                    fields: vec![Field {
                        chunk_offset: 0,
                        symbol: "a",
                        width: 8,
                        symbol_offset: 16,
                        is_partial: true
                    }],
                    width: 8
                },
                Chunk::UnsizedBytes("b"),
                Chunk::Bitpack {
                    fields: vec![
                        Field {
                            chunk_offset: 0,
                            symbol: "c",
                            width: 9,
                            symbol_offset: 0,
                            is_partial: false
                        },
                        Field {
                            chunk_offset: 9,
                            symbol: "d",
                            width: 7,
                            symbol_offset: 0,
                            is_partial: false
                        }
                    ],
                    width: 16
                },
            ])
        )
    }

    #[test]
    #[should_panic]
    fn unalignable_fields() {
        let mut a = ByteAligner::<&'static str>::new(&[8, 16, 32, 64]);
        a.add_bitfield("a", 63);
        a.add_bitfield("b", 2);
        a.add_bitfield("c", 7);

        a.align().unwrap();
    }
}
