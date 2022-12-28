mod rpc_errors;

use core::result;
use derive_more::{From, Display, Error};

pub use tezos_rpc::Error as TezosRpcError;
pub use tezos_core::Error as TezosCoreError;
pub use tezos_operation::Error as TezosOperationError;
pub use tezos_michelson::Error as TezosMichelsonError;
pub use serde_json_wasm::ser::Error as SerializationError;
pub use serde_json_wasm::de::Error as DeserializationError;
pub use ibig::error::ParseError as BigIntParsingError;
pub use chrono::ParseError as TimestampParsingError;

use tezos_core::types::encoded::{OperationHash, ContractHash};
use tezos_michelson::michelson::{
    types::Type,
    data::Instruction
};

pub use rpc_errors::{RpcErrors, RpcError};

#[derive(Debug, From, Display, Error)]
pub enum Error {
    TezosRpcError(TezosRpcError),
    TezosCoreError(TezosCoreError),
    TezosOperationError(TezosOperationError),
    TezosMichelsonError(TezosMichelsonError),
    SerializationError(SerializationError),
    DeserializationError(DeserializationError),
    BigIntParsingError(BigIntParsingError),
    TimestampParsingError(TimestampParsingError),
    ContextUnstagedError,
    ExternalError {
        message: String
    },
    #[display(fmt = "operation {:#?}, caused by {:#?}", hash, inner)]
    ValidationError {
        hash: OperationHash,
        inner: RpcError
    },
    #[display(fmt = "expected {}, found {}", expected, found)]
    MichelsonTypeError {
        expected: String,
        found: String
    },
    OperationKindUnsupported,
    #[display(fmt = "{:#?}", ty)]
    MichelsonTypeUnsupported {
        ty: Type
    },
    #[display(fmt = "{:#?}", instruction)]
    MichelsonInstructionUnsupported {
        instruction: Instruction
    },
    ScriptSectionMissing,
    UnexpectedPairArity,
    StackOutOfBounds,
    UnexpectedStackSize
}

pub type Result<T> = result::Result<T, Error>;