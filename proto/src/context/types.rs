use derive_more::{From, TryInto};
use serde_json_wasm;

use tezos_core::types::{
    encoded::{
        BlockHash, Encoded, OperationHash, PublicKey,
    },
    mutez::Mutez,
    number::Nat,
};
use tezos_michelson::micheline::Micheline;
use tezos_rpc::models::operation::Operation as OperationReceipt;

use crate::{
    context::{head::Head},
    producer::types::BatchReceipt,
    Result,
    internal_error
};

#[derive(Debug, Clone, From, TryInto)]
pub enum ContextNode {
    Head(Head),
    Mutez(Mutez),
    Nat(Nat),
    PublicKey(PublicKey),
    BlockHash(BlockHash),
    OperationHash(OperationHash),
    OperationReceipt(OperationReceipt),
    BatchReceipt(BatchReceipt),
    Micheline(Micheline),
}

impl ContextNode {
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        let (prefix, payload) = match self {
            Self::Head(value) => (b'\x01', value.encode()?),
            Self::Mutez(value) => (b'\x02', value.encode()?),
            Self::Nat(value) => (b'\x03', value.encode()?),
            Self::PublicKey(value) => (b'\x04', value.encode()?),
            Self::BlockHash(value) => (b'\x05', value.encode()?),
            Self::OperationHash(value) => (b'\x06', value.encode()?),
            Self::OperationReceipt(value) => (b'\x07', value.encode()?),
            Self::BatchReceipt(value) => (b'\x08', value.encode()?),
            Self::Micheline(value) => (b'\x09', value.encode()?),
        };
        Ok([vec![prefix], payload].concat())
    }

    pub fn from_vec(value: Vec<u8>) -> Result<Self> {
        match value.as_slice() {
            [b'\x01', bytes @ ..] => Head::decode(bytes),
            [b'\x02', bytes @ ..] => Mutez::decode(bytes),
            [b'\x03', bytes @ ..] => Nat::decode(bytes),
            [b'\x04', bytes @ ..] => PublicKey::decode(bytes),
            [b'\x05', bytes @ ..] => BlockHash::decode(bytes),
            [b'\x06', bytes @ ..] => OperationHash::decode(bytes),
            [b'\x07', bytes @ ..] => OperationReceipt::decode(bytes),
            [b'\x08', bytes @ ..] => BatchReceipt::decode(bytes),
            [b'\x09', bytes @ ..] => Micheline::decode(bytes),
            _ => Err(internal_error!(Encoding, "Invalid context node prefix"))
        }
    }
}

pub trait ContextNodeType: Clone {
    fn encode(&self) -> Result<Vec<u8>>;
    fn decode(bytes: &[u8]) -> Result<ContextNode>;
}

macro_rules! context_node_type_core {
    ($ty: ty) => {
        impl ContextNodeType for $ty {
            fn decode(bytes: &[u8]) -> Result<ContextNode> {
                match Self::from_bytes(bytes) {
                    Ok(value) => Ok(value.into()),
                    Err(error) => Err(error.into()),
                }
            }

            fn encode(&self) -> Result<Vec<u8>> {
                self.to_bytes().map_err(|e| e.into())
            }
        }
    };
}

context_node_type_core!(Mutez);
context_node_type_core!(Nat);
context_node_type_core!(PublicKey);
context_node_type_core!(BlockHash);
context_node_type_core!(OperationHash);
context_node_type_core!(Head);
context_node_type_core!(Micheline);

macro_rules! context_node_type_rpc {
    ($ty: ty) => {
        impl ContextNodeType for $ty {
            fn decode(bytes: &[u8]) -> Result<ContextNode> {
                match serde_json_wasm::from_slice::<$ty>(bytes) {
                    Ok(value) => Ok(value.into()),
                    Err(error) => Err(error.into()),
                }
            }

            fn encode(&self) -> Result<Vec<u8>> {
                serde_json_wasm::to_vec(self).map_err(|e| e.into())
            }
        }
    };
}

context_node_type_rpc!(OperationReceipt);
context_node_type_rpc!(BatchReceipt);
