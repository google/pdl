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

pub trait Symbol: Clone + Debug + Eq {}
impl<T: Clone + Debug + Eq> Symbol for T {}

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

/// A byte-aligned chunk.
/// Because a chunk is byte aligned, it should be easy to encode/decode in your language.
#[derive(Debug, Clone)]
pub enum Chunk<S: Symbol> {
    /// A chunk comprised of one or more bitpacked fields.
    BitPacked { fields: Vec<Field<S>>, width: usize },
    /// An opaque payload whose size is an unspecified even multiple of 8 bits.
    Payload,
}

/// A data structure that packs a set of fields, which may not be byte-aligned, into a sequence of byte algined chunks.
/// This is useful for generating encoding/decoding code in your language.
#[derive(Debug)]
pub struct ByteAligner<S: Symbol> {
    max_chunk_width: usize,
    chunks: Vec<Chunk<S>>,
}

impl<S: Symbol> ByteAligner<S> {
    pub fn new(max_chunk_width: usize) -> Self {
        Self { max_chunk_width, chunks: vec![] }
    }

    /// Get the generated chunks.
    ///
    /// Each `Chunk` within the vec begins and ends at a byte boundary, so the returned data structure
    /// represents a straightforward way to encode and decode the fields in your language.
    pub fn align(self) -> Result<Alignment<S>, &'static str> {
        match self.chunks.last() {
            Some(Chunk::BitPacked { width, .. }) if width % 8 != 0 => {
                Err("Provided fields could not be byte aligned")
            }
            None => Err("No fields provided"),
            _ => Ok(Alignment(self.chunks)),
        }
    }

    pub fn add_field(&mut self, symbol: S, width: usize) {
        if width > self.max_chunk_width {
            panic!("Field too wide");
        } else {
            self.add_offset_field(symbol, width, 0, false);
        }
    }

    pub fn add_payload(&mut self) {
        match self.chunks.last() {
            Some(Chunk::BitPacked { width, .. }) if width % 8 != 0 => {
                panic!("Bytes must start on a byte boundary")
            }
            _ => self.chunks.push(Chunk::Payload),
        };
    }

    fn add_offset_field(
        &mut self,
        symbol: S,
        width: usize,
        symbol_offset: usize,
        is_partial: bool,
    ) {
        match self.chunks.last_mut() {
            Some(Chunk::BitPacked { fields, width: chunk_width }) => {
                if *chunk_width == self.max_chunk_width {
                    self.add_field_to_new_chunk(symbol, width, symbol_offset, is_partial);
                } else if *chunk_width + width <= self.max_chunk_width {
                    fields.push(Field {
                        chunk_offset: *chunk_width,
                        symbol,
                        width,
                        symbol_offset,
                        is_partial,
                    });
                    *chunk_width += width;
                } else {
                    let width_for_next_chunk = *chunk_width + width - self.max_chunk_width;
                    let width_for_this_chunk = width - width_for_next_chunk;

                    fields.push(Field {
                        chunk_offset: *chunk_width,
                        symbol: symbol.clone(),
                        width: width_for_this_chunk,
                        symbol_offset,
                        is_partial: true,
                    });
                    *chunk_width += width_for_this_chunk;

                    self.add_offset_field(
                        symbol,
                        width_for_next_chunk,
                        symbol_offset + width_for_this_chunk,
                        true,
                    );
                }
            }
            _ => self.add_field_to_new_chunk(symbol, width, symbol_offset, is_partial),
        }
    }

    fn add_field_to_new_chunk(
        &mut self,
        symbol: S,
        width: usize,
        symbol_offset: usize,
        is_partial: bool,
    ) {
        self.chunks.push(Chunk::BitPacked {
            fields: vec![Field { chunk_offset: 0, symbol, width, symbol_offset, is_partial }],
            width,
        });
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

impl<S: Symbol> Alignment<S> {
    pub fn payload_offset(&self) -> Option<usize> {
        let mut offset = 0;
        for chunk in self.iter() {
            if let Chunk::BitPacked { width, .. } = chunk {
                offset += width;
            } else {
                return Some(offset);
            }
        }
        None
    }
}
