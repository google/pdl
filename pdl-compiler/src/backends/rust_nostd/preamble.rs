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

use quote::quote;
use std::path::Path;

/// Generate the file preamble for `rust_nostd`.
pub fn generate(path: &Path) -> proc_macro2::TokenStream {
    let filename = path.file_name().unwrap().to_str().expect("non UTF-8 filename");
    let module_doc_string = format!(" @generated rust_nostd packets from {filename}.");
    quote! {
        #[doc = #module_doc_string]

        extern crate alloc;

        use alloc::vec::Vec;
        use bytes::{Buf, BufMut};
        use core::convert::{TryFrom, TryInto};
        use core::fmt;
        use core::result::Result;
        use pdl_runtime::nostd::Packet;
        use pdl_runtime::{Array, DecodeError, EncodeError};

        /// Private prevents users from creating arbitrary scalar values
        /// in situations where the value needs to be validated.
        /// Users can freely deref the value, but only the backend
        /// may create it.
        #[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub struct Private<T>(T);

        impl<T> core::ops::Deref for Private<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T: core::fmt::Debug> core::fmt::Debug for Private<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                T::fmt(&self.0, f)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{assert_snapshot_eq, format_rust};

    #[test]
    fn test_generate_preamble() {
        let actual_code = generate(Path::new("some/path/foo.pdl")).to_string();
        assert_snapshot_eq("tests/generated/preamble_nostd.rs", &format_rust(&actual_code));
    }
}
