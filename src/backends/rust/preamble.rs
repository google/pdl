use std::path::Path;

use crate::quote_block;

/// Generate the file preamble.
pub fn generate(path: &Path) -> String {
    let mut code = String::new();
    let filename = path.file_name().unwrap().to_str().expect("non UTF-8 filename");
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
    code.push_str(&format!("// @generated rust packets from {filename}\n\n"));

    code.push_str(&quote_block! {
        use bytes::{Buf, BufMut, Bytes, BytesMut};
        use std::convert::{TryFrom, TryInto};
        use std::cell::Cell;
        use std::fmt;
        use std::sync::Arc;
        use thiserror::Error;
    });

    code.push_str(&quote_block! {
        type Result<T> = std::result::Result<T, Error>;
    });

    code.push_str(&quote_block! {
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
    });

    code.push_str(&quote_block! {
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
        }
    });

    code.push_str(&quote_block! {
        #[derive(Debug, Error)]
        #[error("{0}")]
        pub struct TryFromError(&'static str);
    });

    code.push_str(&quote_block! {
        pub trait Packet {
            fn to_bytes(self) -> Bytes;
            fn to_vec(self) -> Vec<u8>;
        }
    });

    code
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{assert_snapshot_eq, rustfmt};

    #[test]
    fn test_generate_preamble() {
        let actual_code = generate(Path::new("some/path/foo.pdl"));
        assert_snapshot_eq("tests/generated/preamble.rs", &rustfmt(&actual_code));
    }
}
