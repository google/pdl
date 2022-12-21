use std::path::Path;

use crate::quote_block;

/// Generate the file preamble.
pub fn generate(path: &Path) -> String {
    let mut code = String::new();
    let filename = path.file_name().unwrap().to_str().expect("non UTF-8 filename");
    code.push_str(&format!("// @generated rust packets from {filename}\n\n"));

    // TODO(mgeisler): make the generated code clean from warnings.
    code.push_str("#![allow(warnings, missing_docs)]\n\n");

    code.push_str(&quote_block! {
        use bytes::{Buf, BufMut, Bytes, BytesMut};
        use num_derive::{FromPrimitive, ToPrimitive};
        use num_traits::{FromPrimitive, ToPrimitive};
        use std::convert::{TryFrom, TryInto};
        use std::fmt;
        use std::sync::Arc;
        use thiserror::Error;
    });

    code.push_str(&quote_block! {
        type Result<T> = std::result::Result<T, Error>;
    });

    code.push_str(&quote_block! {
        #[derive(Debug, Error)]
        pub enum Error {
            #[error("Packet parsing failed")]
            InvalidPacketError,
            #[error("{field} was {value:x}, which is not known")]
            ConstraintOutOfBounds { field: String, value: u64 },
            #[error("when parsing {obj} needed length of {wanted} but got {got}")]
            InvalidLengthError { obj: String, wanted: usize, got: usize },
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
