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

/// Generate the file preamble.
pub fn generate(path: &Path) -> proc_macro2::TokenStream {
    // TODO(mgeisler): Make the  generated code free from warnings.
    //
    // The code either needs
    //
    // clippy_lints: "none",
    // lints: "none",
    //
    // in the Android.bp file, or we need to add
    //
    // #![allow(warnings, missing_docs)]
    //
    // to the generated code. We cannot add the module-level attribute
    // here because of how the generated code is used with include! in
    // lmp/src/packets.rs.
    let filename = path.file_name().unwrap().to_str().expect("non UTF-8 filename");
    let module_doc_string = format!(" @generated rust packets from {filename}.");
    // TODO(mgeisler): the doc comment below should be an outer
    // comment (#![doc = ...]). However, people include the generated
    // code in the middle of another module via include_str!:
    //
    // fn before() {}
    // include_str!("generated.rs")
    // fn after() {}
    //
    // It is illegal to have a //! comment in the middle of a file. We
    // should refactor such usages to instead look like this:
    //
    // fn before() {}
    // mod foo { include_str!("generated.rs") }
    // use foo::*;
    // fn after() {}
    quote! {
        #[doc = #module_doc_string]

        use bytes::{Buf, BufMut, Bytes, BytesMut};
        use std::convert::{TryFrom, TryInto};
        use std::cell::Cell;
        use std::fmt;
        use thiserror::Error;

        type Result<T> = std::result::Result<T, Error>;

        /// Private prevents users from creating arbitrary scalar values
        /// in situations where the value needs to be validated.
        /// Users can freely deref the value, but only the backend
        /// may create it.
        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub struct Private<T>(T);

        impl<T> std::ops::Deref for Private<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        #[derive(Debug, Error)]
        pub enum Error {
            #[error("Packet parsing failed")]
            InvalidPacketError,
            #[error("{field} was {value:x}, which is not known")]
            ConstraintOutOfBounds { field: String, value: u64 },
            #[error("Got {actual:x}, expected {expected:x}")]
            InvalidFixedValue { expected: u64, actual: u64 },
            #[error("when parsing {obj} needed length of {wanted} but got {got}")]
            InvalidLengthError { obj: String, wanted: usize, got: usize },
            #[error("array size ({array} bytes) is not a multiple of the element size ({element} bytes)")]
            InvalidArraySize { array: usize, element: usize },
            #[error("Due to size restrictions a struct could not be parsed.")]
            ImpossibleStructError,
            #[error("when parsing field {obj}.{field}, {value} is not a valid {type_} value")]
            InvalidEnumValueError { obj: String, field: String, value: u64, type_: String },
            #[error("expected child {expected}, got {actual}")]
            InvalidChildError { expected: &'static str, actual: String },
        }

        pub trait Packet {
            fn to_bytes(self) -> Bytes;
            fn to_vec(self) -> Vec<u8>;
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
        assert_snapshot_eq("tests/generated/preamble.rs", &format_rust(&actual_code));
    }
}
