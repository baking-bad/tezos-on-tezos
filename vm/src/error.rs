use derive_more::{Display, Error};
use tezos_michelson::micheline::Micheline;
use tezos_michelson::michelson::{
    types::Type,
    data::Instruction
};

#[derive(Debug, Display, Error, PartialEq)]
pub enum Error {
    ParsingError {
        inner: String
    },
    #[display(fmt = "expected {}, found {}", expected, found)]
    TypeMismatch {
        expected: String,
        found: String
    },
    #[display(fmt = "{:?}", ty)]
    MichelsonTypeUnsupported {
        ty: Type
    },
    #[display(fmt = "{:?}", instruction)]
    MichelsonInstructionUnsupported {
        instruction: Instruction
    },
    #[display(fmt = "{:?}", with)]
    ScriptFailed {
        with: Micheline
    },
    MissingScriptField {
        prim: String
    },
    ContractNotFound,
    EntrypointNotFound {
        name: String
    },
    BigMapNotAllocated {
        ptr: i64
    },
    ConflictingEntrypoints,
    BadStack {
        location: usize
    },
    BadReturn,
    #[display(fmt = "expected {}, found: {}", expected, found)]
    InvalidArity {
        expected: usize,
        found: usize
    },
    GeneralOverflow,
    MutezOverflow,
    MutezUnderflow,
    #[display(fmt = "owner {}, offender: {} (ptr: {})", owner, offender, ptr)]
    BigMapAccessDenied {
        ptr: i64,
        owner: String,
        offender: String
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! err {
    ($err: expr) => {
        Err($err.into())
    };
}

#[macro_export]
macro_rules! err_type {
    ($expected: expr, $found: expr) => {
        $crate::err!($crate::Error::TypeMismatch {
            expected: format!("{:?}", $expected),
            found: format!("{:?}", $found)
        })
    };
}

impl From<tezos_core::Error> for Error {
    fn from(error: tezos_core::Error) -> Self {
        Self::ParsingError { inner: error.to_string() }
    }
}

impl From<tezos_michelson::Error> for Error {
    fn from(error: tezos_michelson::Error) -> Self {
        Self::ParsingError { inner: error.to_string() }
    }
}

impl From<ibig::error::ParseError> for Error {
    fn from(error: ibig::error::ParseError) -> Self {
        Self::ParsingError { inner: error.to_string() }
    }
}

impl From<chrono::ParseError> for Error {
    fn from(error: chrono::ParseError) -> Self {
        Self::ParsingError { inner: error.to_string() }
    }
}

impl From<serde_json_wasm::de::Error> for Error {
    fn from(error: serde_json_wasm::de::Error) -> Self {
        Self::ParsingError { inner: error.to_string() }
    }
}

impl From<ibig::error::OutOfBoundsError> for Error {
    fn from(_: ibig::error::OutOfBoundsError) -> Self {
        Error::GeneralOverflow
    }
}