use derive_more::{From, Display, Error};
use proto::errors::{TezosCoreError, TezosRpcError, TezosOperationError, Error as TezosProtoError};

#[derive(Debug, From, Display, Error)]
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
    OperationParsingError
}

pub type Result<T> = std::result::Result<T, Error>;
