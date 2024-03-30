// Copyright 2023 Google LLC
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

use proc_macro2::Ident;
use quote::format_ident;

pub fn get_integer_type(width: usize) -> Ident {
    let best_width = [8, 16, 32, 64]
        .into_iter()
        .filter(|x| *x >= width)
        .min()
        .unwrap_or_else(|| panic!("width {width} is too large"));
    format_ident!("u{best_width}")
}

/// Generate a block of code.
///
/// Like `quote!`, but the code block will be followed by an empty
/// line of code. This makes the generated code more readable.
#[macro_export]
macro_rules! quote_block {
    ($($tt:tt)*) => {
        format!("{}\n\n", ::quote::quote!($($tt)*))
    }
}
