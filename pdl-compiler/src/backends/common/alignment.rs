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

use std::{cmp, fmt::Debug};

/// A field that contains one or more bitpacked values. May not be byte aligned.
pub struct PackedField<S> {
    /// Offset into the chunk where this field starts.
    pub chunk_offset: usize,
    // Language-specific symbol (variable, function call, etc.) which holds the value to encode.
    pub symbol: S,
    // Width of the encodable value in bits.
    pub width: usize,
    // Offset into the symbol at which the encodable value starts.
    pub symbol_offset: usize,
}

/// A contiguous chunk that may contain one or more fields. Must be byte aligned.
/// Because a chunk is byte aligned, it should be easy to encode/decode in your language.
#[derive(Default)]
pub struct Chunk<S> {
    /// The fields that comprise this chunk.
    pub fields: Vec<PackedField<S>>,
    /// The total width of the chunk.
    pub width: usize,
}

impl<S> Chunk<S> {
    pub fn new(symbol: S, width: usize, offset: usize) -> Self {
        Chunk {
            fields: vec![PackedField { chunk_offset: 0, symbol, width, symbol_offset: offset }],
            width,
        }
    }

    pub fn pack_symbol(&mut self, symbol: S, width: usize, offset: usize) {
        let field = PackedField { chunk_offset: self.width, symbol, width, symbol_offset: offset };
        self.width += field.width;
        self.fields.push(field);
    }
}

/// A data structure that packs a set of fields, which may not be byte-aligned, into a sequence of byte algined chunks.
/// This is useful for generating encoding/decoding code in your language.
pub struct ByteAligner<S: Clone + Debug> {
    max_chunk_width: usize,
    chunks: Vec<Chunk<S>>,
}

impl<S> ByteAligner<S>
where
    S: Clone + Debug,
{
    pub fn new(max_chunk_width: usize) -> Self {
        Self { max_chunk_width, chunks: vec![] }
    }

    /// Get the generated chunks.
    ///
    /// Each `Chunk` within the vec begins and ends on a byte boundary, so the returned data structure
    /// represents a straightforward way to encode and decode the fields in your language.
    pub fn into_aligned_chunks(self) -> Result<Vec<Chunk<S>>, &'static str> {
        match self.chunks.last() {
            Some(Chunk { width, .. }) if *width != 0 && *width % 8 == 0 => Ok(self.chunks),
            _ => Err("No fields provided, or provided fields could not be byte aligned"),
        }
    }

    pub fn add_field(&mut self, symbol: S, width: usize) {
        assert!(
            width <= self.max_chunk_width,
            "Fields cannot be wider than {}",
            self.max_chunk_width
        );
        self.add_field_with_value_offset(symbol, width, 0);
    }

    fn add_field_with_value_offset(&mut self, symbol: S, width: usize, value_offset: usize) {
        match self.chunks.last_mut() {
            Some(chunk) if chunk.width + width <= self.max_chunk_width => {
                chunk.pack_symbol(symbol.clone(), width, value_offset);
            }
            Some(chunk)
                if chunk.width < self.max_chunk_width
                    && chunk.width + width > self.max_chunk_width =>
            {
                let width_for_this_chunk = cmp::min(self.max_chunk_width - chunk.width, width);
                let width_for_next_chunk = width - width_for_this_chunk;

                chunk.pack_symbol(symbol.clone(), width_for_this_chunk, value_offset);
                self.add_field_with_value_offset(
                    symbol,
                    width_for_next_chunk,
                    value_offset + width_for_this_chunk,
                );
            }
            _ => {
                self.chunks.push(Chunk::new(symbol, width, value_offset));
            }
        }
    }
}
