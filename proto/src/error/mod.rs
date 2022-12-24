pub mod rpc_errors;

use core::result;
use tezos_core;
use tezos_operation;
use serde_json_wasm;
use derive_more::{From, Display, Error};

pub use tezos_rpc::Error as TezosRpcError;
pub use tezos_core::Error as TezosCoreError;
pub use tezos_operation::Error as TezosOperationError;
pub use tezos_michelson::Error as TezosMichelsonError;
pub use serde_json_wasm::ser::Error as SerializationError;
pub use serde_json_wasm::de::Error as DeserializationError;

pub use rpc_errors::{RpcErrors, RpcError};

#[derive(Debug, From, Display, Error)]
pub enum Error {
    TezosRpcError(TezosRpcError),
    TezosCoreError(TezosCoreError),
    TezosOperationError(TezosOperationError),
    TezosMichelsonError(TezosMichelsonError),
    SerializationError(SerializationError),
    DeserializationError(DeserializationError),
    OperationKindUnsupported,
    #[display(fmt = "operation {}, caused by {}", hash, inner)]
    ValidationError {
        hash: String,
        inner: RpcError
    },
    ContextUnstagedError,
    ExternalError {
        message: String
    },
}

pub type Result<T> = result::Result<T, Error>;
