use derive_more::{From, Display};
use proto::error::{TezosCoreError, TezosRpcError, TezosOperationError, Error as TezosProtoError};

#[derive(Debug, Display)]
pub enum ErrorKind {
    Parsing,
}

#[derive(Debug, From, Display)]
pub enum Error {
    TezosProtoError(TezosProtoError),
    TezosOperationError(TezosOperationError),
    TezosCoreError(TezosCoreError),
    TezosRpcError(TezosRpcError),
    #[display(fmt = "{:?}", internal)]
    WasmHostError{
        internal: host::runtime::RuntimeError
    },
    #[display(fmt = "{:?}", internal)]
    HostPathError{
        internal: host::path::PathError
    },
    #[display(fmt = "{} ({})", message, kind)]
    InternalError {
        kind: ErrorKind,
        message: String
    },
}

pub type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! parsing_error {
    ($($arg:tt)*) => {
        Err(crate::error::Error::InternalError {
            kind: crate::error::ErrorKind::Parsing,
            message: format!($($arg)*)
        })
    };
}
