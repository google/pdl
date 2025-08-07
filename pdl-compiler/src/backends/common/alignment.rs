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

pub trait Symbol: Clone + Debug + Eq {}

impl<S> Symbol for S where S: Clone + Debug + Eq {}

/// A field that contains a partial or complete value. May not be byte aligned.
#[derive(Debug, Clone)]
pub struct Field<S: Symbol> {
    /// Offset into the chunk where this field starts.
    pub chunk_offset: usize,
    /// Language-specific symbol (variable, function call, etc.) which holds the value to encode.
    pub symbol: S,
    /// Width of the encodable value in bits.
    pub width: usize,
    /// Offset into the symbol at which the encodable value starts.
    pub symbol_offset: usize,
    /// Whether this field contains a partial value.
    pub is_partial: bool,
}

/// A field that contains a partial value.
#[derive(Debug, Clone)]
pub struct Partial {
    pub width: usize,
    pub offset: usize,
}

/// A byte-aligned chunk.
/// Because a chunk is byte aligned, it should be easy to encode/decode in your language.
#[derive(Debug, Clone)]
pub enum Chunk<S: Symbol> {
    /// A chunk comprised of bitpacked fields.
    Bitpack { fields: Vec<Field<S>>, width: usize },
    /// A chunk whose width is a whole multiple of 8 bits.
    SizedBytes { symbol: S, alignment: Vec<Partial>, width: usize },
    /// A chunk whose width is an unspecified whole multiple of 8 bits.
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

/// A data structure that packs a set of fields, which may not be byte-aligned, into a sequence of byte algined chunks.
/// This is useful for generating encoding/decoding code in your language.
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
    /// Each `Chunk` within the vec begins and ends at a byte boundary, so the returned data structure
    /// represents a straightforward way to encode and decode the fields in your language.
    pub fn align(mut self) -> Result<Alignment<S>, &'static str> {
        match self.chunks.last() {
            Some(Chunk::Bitpack { width, .. }) if *width % 8 != 0 => {
                dbg!(self);
                Err("Provided fields could not be aligned to the allowed chunk widths")
            }
            None => Err("No fields provided"),
            _ => {
                self.repack_last_chunk();
                Ok(Alignment(self.chunks))
            }
        }
    }

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
                // Add field to new chunk
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

    fn repack_last_chunk(&mut self) {
        if let Some(Chunk::Bitpack { width, .. }) = self.chunks.last() {
            if !self.allowed_chunk_widths.contains(width) {
                let (old_fields, old_chunk_width) =
                    self.chunks.pop().unwrap().get_bitpack().unwrap();

                let mut cur_width = 0;
                while cur_width != old_chunk_width {
                    for chunk_width in self.allowed_chunk_widths.iter() {
                        if *chunk_width <= (old_chunk_width - cur_width) {
                            self.chunks.push(Chunk::Bitpack {
                                fields: Self::rechunk_to_range(
                                    &old_fields,
                                    cur_width..(cur_width + *chunk_width),
                                ),
                                width: *chunk_width,
                            });
                            cur_width += *chunk_width
                        }
                    }
                }
            }
        }
    }

    fn rechunk_to_range(fields: &Vec<Field<S>>, range: Range<usize>) -> Vec<Field<S>> {
        let mut in_range = Vec::new();

        for field in fields {
            let field_range = field.chunk_offset..(field.chunk_offset + field.width);

            if range.contains(&field_range.start) && range.contains(&(field_range.end - 1)) {
                in_range.push(field.clone());
            } else if field_range.contains(&range.start) && field_range.contains(&(range.end - 1)) {
                in_range.push(Field {
                    chunk_offset: range.start,
                    symbol: field.symbol.clone(),
                    width: range.end - range.start,
                    symbol_offset: range.start - field_range.start,
                    is_partial: true,
                });
            } else if range.contains(&field_range.start) {
                in_range.push(Field {
                    chunk_offset: field_range.start,
                    symbol: field.symbol.clone(),
                    width: range.end - field_range.start,
                    symbol_offset: 0,
                    is_partial: true,
                });
            } else if range.contains(&(field_range.end - 1)) {
                in_range.push(Field {
                    chunk_offset: range.start,
                    symbol: field.symbol.clone(),
                    width: field_range.end - range.start,
                    symbol_offset: range.start - field_range.start,
                    is_partial: true,
                });
            }
        }

        in_range
    }

    pub fn add_sized_bytes(&mut self, symbol: S, width: usize) {
        if !self.is_aligned() {
            panic!("sized fields must start at a byte boundary")
        }

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

    pub fn add_bytes(&mut self, symbol: S) {
        if !self.is_aligned() {
            panic!("Bytes must start a byte boundary")
        }

        self.chunks.push(Chunk::UnsizedBytes(symbol));
    }

    fn is_aligned(&self) -> bool {
        self.chunks.last().is_none_or(
            |chunk| !matches!(chunk, Chunk::Bitpack { width, .. } if !self.allowed_chunk_widths.contains(width)))
    }
}

#[derive(Debug, Clone)]
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
