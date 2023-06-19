use derive_more::{Display, Error};
use std::backtrace::Backtrace;
use tezos_core::types::mutez::Mutez;

#[derive(Debug)]
pub struct InternalError {
    pub message: String,
    pub backtrace: Backtrace,
}

impl InternalError {
    pub fn new(message: String) -> Self {
        Self {
            message,
            backtrace: Backtrace::capture(),
        }
    }

    pub fn format(&self) -> String {
        format!(
            "Tezos proto error\n{}\nStacktrace:\n{}",
            self.message, self.backtrace
        )
    }
}

impl std::fmt::Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Tezos proto error, {}",
            self.message.replace("\n", " ")
        ))
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
    BalanceTooLow { balance: Mutez },
    ContractCodeMissing { address: String },
    ContractStorageMissing { address: String },
    InconsistentSources,
    ContentsListError,
    UnrevealedPublicKey,
    InvalidSignature,
    EmptyImplicitContract,
    CounterInThePast { counter: String },
}

pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! internal_error {
    ($($arg:tt)*) => {
        $crate::Error::Internal(
            $crate::error::InternalError::new(format!($($arg)*))
        )
    };
}

macro_rules! impl_from_error {
    ($inner_err_ty: ty, $kind: ident) => {
        impl From<$inner_err_ty> for Error {
            fn from(error: $inner_err_ty) -> Self {
                $crate::internal_error!("{:?}", error)
            }
        }
    };
}

impl_from_error!(tezos_rpc::Error, TezosEncoding);
impl_from_error!(tezos_core::Error, TezosEncoding);
impl_from_error!(tezos_operation::Error, TezosEncoding);
impl_from_error!(tezos_michelson::Error, TezosEncoding);
impl_from_error!(serde_json_wasm::ser::Error, Encoding);
impl_from_error!(serde_json_wasm::de::Error, Encoding);
impl_from_error!(ibig::error::OutOfBoundsError, Encoding);
impl_from_error!(ibig::error::ParseError, Encoding);
impl_from_error!(chrono::ParseError, Encoding);
impl_from_error!(&str, Encoding);

impl From<michelson_vm::Error> for Error {
    fn from(error: michelson_vm::Error) -> Self {
        internal_error!("> Michelson VM: {}", error)
    }
}

impl From<layered_store::Error> for Error {
    fn from(error: layered_store::Error) -> Self {
        internal_error!("> Layered storage: {}", error)
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
