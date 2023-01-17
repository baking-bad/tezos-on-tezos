mod rpc;

use core::result;
use derive_more::{Display, Error, From};
use tezos_core::types::encoded::OperationHash;

pub use chrono::ParseError as TimestampParsingError;
pub use ibig::error::OutOfBoundsError as BigIntOutOfBoundsError;
pub use ibig::error::ParseError as BigIntParsingError;
pub use serde_json_wasm::de::Error as DeserializationError;
pub use serde_json_wasm::ser::Error as SerializationError;
pub use tezos_core::Error as TezosCoreError;
pub use tezos_michelson::Error as TezosMichelsonError;
pub use tezos_operation::Error as TezosOperationError;
pub use tezos_rpc::Error as TezosRpcError;
pub use vm::Error as InterpreterError;

pub use rpc::{RpcError, RpcErrors};

#[derive(Debug, From, Display, Error)]
pub enum Error {
    TezosRpcError(TezosRpcError),
    TezosCoreError(TezosCoreError),
    TezosOperationError(TezosOperationError),
    TezosMichelsonError(TezosMichelsonError),
    SerializationError(SerializationError),
    DeserializationError(DeserializationError),
    BigIntParsingError(BigIntParsingError),
    BigIntOutOfBoundsError(BigIntOutOfBoundsError),
    TimestampParsingError(TimestampParsingError),
    ContextUnstagedError,
    ExternalError {
        message: String,
    },
    OperationKindUnsupported,
    #[display(fmt = "operation {:?}, caused by {:?}", hash, inner)]
    ValidationError {
        hash: OperationHash,
        inner: RpcError,
    },
    InterpreterError(InterpreterError),
}

pub type Result<T> = result::Result<T, Error>;
