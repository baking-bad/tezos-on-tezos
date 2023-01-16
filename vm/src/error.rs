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

    pub fn print(&self) {
        println!("{} error\n{}\nStacktrace:\n{}", self.kind, self.message, self.backtrace)
    }
}

impl Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} error", self.kind))
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
    #[display(fmt = "ScriptFailed")]
    ScriptFailed {
        with: Micheline
    },
    #[display(fmt = "ContractNotFound: {}", address)]
    ContractNotFound {
        address: String
    },
    #[display(fmt = "EntrypointNotFound: {}", name)]
    EntrypointNotFound {
        name: String
    },
    #[display(fmt = "ConflictingEntrypoints: {}", address)]
    ConflictingEntrypoints {
        address: String
    },
    #[display(fmt = "BadStack at: {}", location)]
    BadStack {
        location: usize
    },
    #[display(fmt = "BadReturn")]
    BadReturn,
    #[display(fmt = "BigMapAccessDenied for: {}", ptr)]
    BigMapAccessDenied {
        ptr: i64,
    },
    #[display(fmt = "BigMapNotAllocated: {}", ptr)]
    BigMapNotAllocated {
        ptr: i64
    },
    #[display(fmt = "MutezOverflow")]
    MutezOverflow,
    #[display(fmt = "MutezUnderflow")]
    MutezUnderflow,
    #[display(fmt = "GeneralOverflow")]
    GeneralOverflow,
}

pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! internal_error {
    ($kind: ident, $($arg:tt)*) => {
        $crate::Error::Internal(
            $crate::error::InternalError::new(
                $crate::error::InternalKind::$kind,
                format!($($arg)*)
            )
        )
    };
}

#[macro_export]
macro_rules! err_mismatch {
    ($expected: expr, $found: expr) => {
        Err($crate::internal_error!(Typechecking, "Expected: {}\nFound: {}", $expected, $found))
    };
}

#[macro_export]
macro_rules! err_unsupported {
    ($prim: expr) => {
        Err($crate::internal_error!(Parsing, "`{}` unsupported", $prim))
    };
}

macro_rules! impl_parsing_error {
    ($inner_err_ty: ty) => {
        impl From<$inner_err_ty> for Error {
            fn from(error: $inner_err_ty) -> Self {
                $crate::internal_error!(Parsing, "{} (caused by {})", error.to_string(), stringify!($inner_err_ty))
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

impl Error {
    pub fn print(&self) {
        match self {
            Self::Internal(internal) => internal.print(),
            Self::ScriptFailed { with } => {
                let msg = match serde_json_wasm::to_string(with) {
                    Ok(res) => res,
                    Err(err) => err.to_string()
                };
                println!("Script failed\nWith: {}", msg);
            },
            err => println!("{:#?}", err),
        }
    }
}