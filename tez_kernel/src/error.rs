use tezos_core;
use tezos_operation;
use std::result;
use serde_json;
use host::runtime;

#[derive(Debug)]
pub enum Error {
    TezosCoreError {
        source: tezos_core::Error
    },
    TezosOperationError {
        source: tezos_operation::Error
    },
    ValidationError {
        message: String
    },
    ExecutionError {
        message: String
    },
    SerializationError {
        source: serde_json::Error
    },
    WasmHostError {
        source: host::runtime::RuntimeError
    },
    StorageError {
        message: String
    }
}

impl From<tezos_core::Error> for Error {
    fn from(error: tezos_core::Error) -> Self {
        Self::TezosCoreError { source: error }
    }
}

impl From<tezos_operation::Error> for Error {
    fn from(error: tezos_operation::Error) -> Self {
        Self::TezosOperationError { source: error }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::SerializationError { source: error }
    }
}

impl From<runtime::RuntimeError> for Error {
    fn from(error: runtime::RuntimeError) -> Self {
        Self::WasmHostError { source: error }
    }
}

pub type Result<T> = result::Result<T, Error>;

#[macro_export]
macro_rules! validation_error {
    ($($arg:tt)*) => {
        Err(crate::error::Error::ValidationError { message: format!($($arg)*) })
    };
}

#[macro_export]
macro_rules! execution_error {
    ($($arg:tt)*) => {
        Err(crate::error::Error::ExecutionError { message: format!($($arg)*) })
    };
}

#[macro_export]
macro_rules! storage_error {
    ($($arg:tt)*) => {
        Err(crate::error::Error::StorageError { message: format!($($arg)*) })
    };
}