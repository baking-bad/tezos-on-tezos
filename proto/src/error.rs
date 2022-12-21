use core::result;
use tezos_core;
use tezos_operation;
use serde_json_wasm;
use derive_more::{From, Display, Error};

pub use tezos_rpc::Error as TezosRpcError;
pub use tezos_core::Error as TezosCoreError;
pub use tezos_operation::Error as TezosOperationError;
pub use serde_json_wasm::ser::Error as SerializationError;
pub use serde_json_wasm::de::Error as DeserializationError;

#[derive(Debug, Display)]
pub enum ErrorKind {
    Validation,
    Migration,
    Execution,
    Context
}

#[derive(Debug, From, Display, Error)]
pub enum Error {
    TezosRpcError(TezosRpcError),
    TezosCoreError(TezosCoreError),
    TezosOperationError(TezosOperationError),
    SerializationError(SerializationError),
    DeserializationError(DeserializationError),
    #[display(fmt = "{} ({})", message, kind)]
    InternalError {
        kind: ErrorKind,
        message: String
    },
}

pub type Result<T> = result::Result<T, Error>;

#[macro_export]
macro_rules! validation_error {
    ($($arg:tt)*) => {
        Err(crate::error::Error::InternalError {
            kind: crate::error::ErrorKind::Validation,
            message: format!($($arg)*)
        })
    };
}

#[macro_export]
macro_rules! execution_error {
    ($($arg:tt)*) => {
        Err(crate::error::Error::InternalError {
            kind: crate::error::ErrorKind::Execution,
            message: format!($($arg)*)
        })
    };
}

#[macro_export]
macro_rules! migration_error {
    ($($arg:tt)*) => {
        Err(crate::error::Error::InternalError {
            kind: crate::error::ErrorKind::Migration,
            message: format!($($arg)*)
        })
    };
}
