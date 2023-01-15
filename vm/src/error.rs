use derive_more::{Display, Error};
use std::{
    backtrace::Backtrace,
    fmt::Display
};
use tezos_michelson::micheline::Micheline;

#[derive(Debug, Display)]
pub enum InternalKind {
    Parsing,
    Typechecking
}

#[derive(Debug)]
pub struct InternalError {
    pub kind: InternalKind,
    pub message: String,
    pub backtrace: Backtrace
}

impl InternalError {
    pub fn new(kind: InternalKind, message: String) -> Self {
        Self {
            kind,
            message,
            backtrace: Backtrace::capture()
        }
    }
}

impl Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} error\n{}\n", self.kind, self.message))?;
        self.backtrace.fmt(f)
    }
}

impl std::error::Error for InternalError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl PartialEq for InternalError {
    fn eq(&self, _: &Self) -> bool {
        unimplemented!()
    }
}

#[derive(Debug, Display, Error, PartialEq)]
pub enum Error {
    Internal(InternalError),
    UnsupportedPrimitive {
        prim: String
    },
    #[display(fmt = "{:?}", with)]
    ScriptFailed {
        with: Micheline
    },
    MissingScriptField {
        prim: String
    },
    ContractNotFound {
        address: String
    },
    EntrypointNotFound {
        name: String
    },
    ConflictingEntrypoints,
    BadStack {
        location: usize
    },
    BadReturn,
    InvalidArity {
        arity: usize
    },
    GeneralOverflow,
    MutezOverflow,
    MutezUnderflow,
    BigMapAccessDenied {
        ptr: i64,
    },
    BigMapNotAllocated {
        ptr: i64
    },
}

pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! err_mismatch {
    ($expected: expr, $found: expr) => {        
        Err($crate::Error::Internal(
            $crate::error::InternalError::new(
                $crate::error::InternalKind::Typechecking,
                format!("Expected:\t{}\nFound:\t\t{}", $expected, $found)
            )
        ))
    };
}

macro_rules! impl_parsing_error {
    ($inner_err_ty: ty) => {
        impl From<$inner_err_ty> for Error {
            fn from(error: $inner_err_ty) -> Self {
                Self::Internal(InternalError::new(
                    InternalKind::Parsing,
                    error.to_string()
                ))
            }
        }        
    };
}

impl_parsing_error!(tezos_core::Error);
impl_parsing_error!(tezos_michelson::Error);
impl_parsing_error!(ibig::error::ParseError);
impl_parsing_error!(chrono::ParseError);
impl_parsing_error!(serde_json_wasm::de::Error);

impl From<ibig::error::OutOfBoundsError> for Error {
    fn from(_: ibig::error::OutOfBoundsError) -> Self {
        Error::GeneralOverflow
    }
}