use derive_more::{Display, Error};
use std::backtrace::Backtrace;
use tezos_core::types::{mutez::Mutez};

pub use chrono::ParseError as TimestampParsingError;
pub use ibig::error::OutOfBoundsError as BigIntOutOfBoundsError;
pub use ibig::error::ParseError as BigIntParsingError;
pub use serde_json_wasm::de::Error as DeserializationError;
pub use serde_json_wasm::ser::Error as SerializationError;
pub use tezos_core::Error as TezosCoreError;
pub use tezos_michelson::Error as TezosMichelsonError;
pub use tezos_operation::Error as TezosOperationError;
pub use tezos_rpc::Error as TezosRpcError;
pub use tezos_vm::Error as InterpreterError;
pub use context::Error as ContextError;

#[derive(Debug, Display)]
pub enum InternalKind {
    Context,
    Encoding,
    Interpreter,
    TezosEncoding,
}

#[derive(Debug)]
pub struct InternalError {
    pub kind: InternalKind,
    pub message: String,
    pub backtrace: Backtrace,
}

impl InternalError {
    pub fn new(kind: InternalKind, message: String) -> Self {
        Self {
            kind,
            message,
            backtrace: Backtrace::capture(),
        }
    }

    pub fn format(&self) -> String {
        format!(
            "{} error\n{}\nStacktrace:\n{}",
            self.kind, self.message, self.backtrace
        )
    }
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} error", self.kind))
    }
}

impl std::error::Error for InternalError {
    fn description(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Display, Error)]
pub enum Error {
    Internal(InternalError),
    OperationKindUnsupported,
    BalanceNotInitialized,
    BalanceTooLow {
        balance: Mutez,
    },
    ContractCodeMissing {
        address: String,
    },
    ContractStorageMissing {
        address: String,
    },
    InconsistentSources,
    ContentsListError,
    UnrevealedPublicKey,
    InvalidSignature,
    EmptyImplicitContract,
    #[display(fmt = "expected {}, found {}", expected, found)]
    CounterInThePast {
        expected: u64,
        found: u64
    }
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

macro_rules! impl_from_error {
    ($inner_err_ty: ty, $kind: ident) => {
        impl From<$inner_err_ty> for Error {
            fn from(error: $inner_err_ty) -> Self {
                $crate::internal_error!($kind, "{:?}", error)
            }
        }
    };
}

impl_from_error!(ContextError, Context);
impl_from_error!(TezosRpcError, TezosEncoding);
impl_from_error!(TezosCoreError, TezosEncoding);
impl_from_error!(TezosOperationError, TezosEncoding);
impl_from_error!(TezosMichelsonError, TezosEncoding);
impl_from_error!(SerializationError, Encoding);
impl_from_error!(DeserializationError, Encoding);
impl_from_error!(BigIntOutOfBoundsError, Encoding);
impl_from_error!(BigIntParsingError, Encoding);
impl_from_error!(TimestampParsingError, Encoding);
impl_from_error!(&str, Encoding);

impl From<InterpreterError> for Error {
    fn from(error: InterpreterError) -> Self {
        internal_error!(Interpreter, "Caused by: {}", error)
    }
}

impl Error {
    pub fn format(&self) -> String {
        match self {
            Self::Internal(internal) => internal.format(),
            err => format!("{:#?}", err),
        }
    }
}
