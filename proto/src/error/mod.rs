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
pub use ibig::error::OutOfBoundsError as BigIntOutOfBoundsError;
pub use chrono::ParseError as TimestampParsingError;

use tezos_core::types::encoded::{OperationHash};
use tezos_michelson::michelson::{
    types::Type,
    data::Instruction
};

pub use rpc_errors::{RpcErrors, RpcError};
pub use crate::vm::StackItem;

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
    #[display(fmt = "{:#?}", with)]
    MichelsonScriptError {
        with: StackItem
    },
    ComparisonError,
    ScriptSectionMissing,
    UnexpectedPairArity,
    StackOutOfBounds,
    UnexpectedStackSize,
    ListOutOfBounds,
    ShiftOverflow
}

pub type Result<T> = result::Result<T, Error>;

#[macro_export]
macro_rules! err_type {
    ($expected: expr, $found: expr) => {
        Err(Error::MichelsonTypeError {
            expected: format!("{:?}", $expected),
            found: format!("{:?}", $found)
        })
    };
}