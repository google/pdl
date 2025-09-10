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
    fmt::Debug,
    ops::{Deref, DerefMut},
};

/// A target-language-specific representation of a PDL field.
pub trait Symbol: Clone + Debug + Eq {}

impl<S> Symbol for S where S: Clone + Debug + Eq {}

/// A PDL field that may not be byte aligned.
#[derive(Debug, Clone, PartialEq)]
pub struct Field<S: Symbol> {
    /// A symbol that represents a PDL field in your target language.
    pub symbol: S,
    /// Width of the symbol in bits.
    pub width: usize,
    /// Offset into the chunk where this field starts.
    pub offset: usize,
}

/// A byte-aligned grouping of one or more symbols.
#[derive(Debug, Clone, PartialEq)]
pub enum Chunk<S: Symbol> {
    /// A sequence of one or more symbols packed into a byte-aligned group.
    /// This is used to align symbols that may not be byte aligned, such as scalars and enums.
    ///
    /// For example,
    /// ```pdl
    /// packet MyPacket {
    ///     a: 1,
    ///     b: 7,
    /// }
    /// ```
    /// Aligns to:
    ///
    /// | Fields               | Width |
    /// |----------------------|-------|
    /// | a\[0..1\], b\[0..7\] | 8     |
    Bitpack { fields: Vec<Field<S>>, width: usize },
    /// A sequence of bytes with known size (specified in bits). This is useful for aligning PDL arrays whose size is known at compile time.
    Bytes { symbol: S, width: usize },
    /// A sequence of bytes with unknown size. This is useful for aligning PDL fields whose size is not known at compile time.
    DynBytes(S),
}

/// Packs symbols of various sizes, which may not be byte-aligned, into a sequence of byte-aligned chunks.
#[derive(Debug, Clone)]
pub struct ByteAligner<S: Symbol> {
    staged_chunk: Option<(Vec<Field<S>>, usize)>,
    chunks: Vec<Chunk<S>>,
}

impl<S: Symbol> ByteAligner<S> {
    pub const MAX_CHUNK_WIDTH: usize = 64;

    pub fn new() -> Self {
        Self { staged_chunk: None, chunks: vec![] }
    }

    /// Get the generated chunks.
    ///
    /// Each chunk within the Alignment will be byte-aligned.
    pub fn align(self) -> Result<Alignment<S>, &'static str> {
        if self.is_aligned() {
            Ok(Alignment(self.chunks))
        } else {
            Err("provided fields could not be byte aligned")
        }
    }

    /// Add a [`Symbol`] to the alignment.
    /// This symbol can have any width <= [`Self::MAX_CHUNK_WIDTH`].
    pub fn add_bitfield(&mut self, symbol: S, width: usize) {
        if let Some((fields, chunk_offset)) = self.staged_chunk.as_mut() {
            fields.push(Field { offset: *chunk_offset, symbol, width });
            *chunk_offset += width;
            if *chunk_offset > Self::MAX_CHUNK_WIDTH {
                panic!("total field width grew beyond max chunk width of {} before aligning to a byte boundary", Self::MAX_CHUNK_WIDTH)
            }
        } else {
            self.staged_chunk = Some((vec![Field { offset: 0, symbol, width }], width))
        }
        self.try_commit_staged_chunk();
    }

    /// Add a [`Symbol`] to the alignment.
    /// This symbol's width must satisfy `width % 8 == 0`.
    pub fn add_bytes(&mut self, symbol: S, width: usize) {
        if !self.is_aligned() {
            panic!("sized bytes must start at a byte boundary")
        }
        if width % 8 != 0 {
            panic!("width must be byte-aligned")
        }
        if width > Self::MAX_CHUNK_WIDTH {
            panic!("width can't be larger than max chunk width of {}", Self::MAX_CHUNK_WIDTH)
        }

        self.chunks.push(Chunk::Bytes { symbol, width });
    }

    /// Add a [`Symbol`] to the alignment.
    /// This symbol's width does not need to be exactly known at compile-time, although it must be known to be byte-divisible.
    pub fn add_dyn_bytes(&mut self, symbol: S) {
        if !self.is_aligned() {
            panic!("dynamic bytes must start at a byte boundary")
        }
        self.chunks.push(Chunk::DynBytes(symbol));
    }

    fn is_aligned(&self) -> bool {
        self.staged_chunk.is_none()
    }

    fn try_commit_staged_chunk(&mut self) -> bool {
        if self.staged_chunk.as_ref().is_some_and(|(_, width)| width % 8 == 0) {
            let (fields, width) = self.staged_chunk.take().unwrap();
            self.chunks.push(Chunk::Bitpack { fields, width });
            true
        } else {
            false
        }
    }
}

/// An alignment for a sequence of PDL fields, generated from a [`ByteAligner`].
/// To generate (de)serialization code from an [`Alignment`], you must iterate over it and handle the 3 different [`Chunk`] variants:
/// 1) [`Chunk::Bitpack`]
/// 2) [`Chunk::Bytes`]
/// 3) [`Chunk::DynBytes`]
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
    fn pack_field_into_56_chunk() {
        let mut a = ByteAligner::<&'static str>::new();
        a.add_bitfield("a", 56);

        assert_eq!(
            a.align().unwrap(),
            Alignment(vec![Chunk::Bitpack {
                fields: vec![Field { symbol: "a", width: 56, offset: 0 }],
                width: 56
            }])
        )
    }

    #[test]
    fn pack_multiple_fields_into_40_chunk() {
        let mut a = ByteAligner::<&'static str>::new();
        a.add_bitfield("a", 9);
        a.add_bitfield("b", 1);
        a.add_bitfield("c", 21);
        a.add_bitfield("d", 9);

        assert_eq!(
            a.align().unwrap(),
            Alignment(vec![Chunk::Bitpack {
                fields: vec![
                    Field { symbol: "a", width: 9, offset: 0 },
                    Field { symbol: "b", width: 1, offset: 9 },
                    Field { symbol: "c", width: 21, offset: 10 },
                    Field { symbol: "d", width: 9, offset: 31 }
                ],
                width: 40
            },])
        )
    }

    #[test]
    fn pack_multiple_fields_into_64_chunk() {
        let mut a = ByteAligner::<&'static str>::new();
        a.add_bitfield("a", 13);
        a.add_bitfield("b", 51);

        assert_eq!(
            a.align().unwrap(),
            Alignment(vec![Chunk::Bitpack {
                fields: vec![
                    Field { symbol: "a", width: 13, offset: 0 },
                    Field { symbol: "b", width: 51, offset: 13 }
                ],
                width: 64
            }])
        )
    }

    #[test]
    fn pack_multiple_fields_into_multiple_chunks() {
        let mut a = ByteAligner::<&'static str>::new();
        a.add_bitfield("a", 1);
        a.add_bitfield("b", 15);
        a.add_bitfield("c", 3);
        a.add_bitfield("d", 5);

        assert_eq!(
            a.align().unwrap(),
            Alignment(vec![
                Chunk::Bitpack {
                    fields: vec![
                        Field { symbol: "a", width: 1, offset: 0 },
                        Field { symbol: "b", width: 15, offset: 1 },
                    ],
                    width: 16
                },
                Chunk::Bitpack {
                    fields: vec![
                        Field { symbol: "c", width: 3, offset: 0 },
                        Field { symbol: "d", width: 5, offset: 3 }
                    ],
                    width: 8
                }
            ])
        );
    }

    #[test]
    fn bitfields_separated_by_dynamic_bytes() {
        let mut a = ByteAligner::<&'static str>::new();
        a.add_bitfield("a", 24);
        a.add_dyn_bytes("b");
        a.add_bitfield("c", 9);
        a.add_bitfield("d", 7);

        assert_eq!(
            a.align().unwrap(),
            Alignment(vec![
                Chunk::Bitpack {
                    fields: vec![Field { symbol: "a", width: 24, offset: 0 },],
                    width: 24
                },
                Chunk::DynBytes("b"),
                Chunk::Bitpack {
                    fields: vec![
                        Field { symbol: "c", width: 9, offset: 0 },
                        Field { symbol: "d", width: 7, offset: 9 }
                    ],
                    width: 16
                },
            ])
        )
    }

    #[test]
    #[should_panic]
    fn unalignable_fields() {
        let mut a = ByteAligner::<&'static str>::new();
        a.add_bitfield("a", 63);
        a.add_bitfield("b", 2);
        a.add_bitfield("c", 7);

        a.align().unwrap();
    }
}
