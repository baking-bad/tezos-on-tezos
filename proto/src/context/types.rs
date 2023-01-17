use derive_more::{From, TryInto};
use serde_json_wasm;
use std::ops::Deref;

use tezos_core::types::{
    encoded::{
        Address, BlockHash, ContractAddress, Encoded, ImplicitAddress, OperationHash, PublicKey,
    },
    mutez::Mutez,
    number::Nat,
};
use tezos_michelson::micheline::Micheline;
use tezos_rpc::models::operation::Operation as OperationReceipt;

use crate::{
    context::{checksum::Checksum, head::Head},
    producer::types::BatchReceipt,
    Result,
};

#[derive(Debug, Clone, From, TryInto)]
pub enum ContextNode {
    Mutez(Mutez),
    Nat(Nat),
    PublicKey(PublicKey),
    BlockHash(BlockHash),
    OperationHash(OperationHash),
    Checksum(Checksum),
    Head(Head),
    OperationReceipt(OperationReceipt),
    BatchReceipt(BatchReceipt),
    Micheline(Micheline),
}

impl ContextNode {
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        match self {
            Self::Head(value) => value.encode(),
            Self::Checksum(value) => value.encode(),
            Self::Mutez(value) => value.encode(),
            Self::Nat(value) => value.encode(),
            Self::PublicKey(value) => value.encode(),
            Self::BlockHash(value) => value.encode(),
            Self::OperationHash(value) => value.encode(),
            Self::OperationReceipt(value) => value.encode(),
            Self::BatchReceipt(value) => value.encode(),
            Self::Micheline(value) => value.encode(),
        }
    }
}

pub trait ContextNodeType: Clone {
    fn encode(&self) -> Result<Vec<u8>>;
    fn decode(bytes: &[u8]) -> Result<ContextNode>;
    fn unwrap(node: ContextNode) -> Self;
    fn wrap(self) -> ContextNode;
    fn max_bytes() -> usize;
}

macro_rules! context_node_type_core {
    ($ty: ty, $max_bytes: expr) => {
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

            fn unwrap(node: ContextNode) -> Self {
                node.try_into().unwrap()
            }

            fn wrap(self) -> ContextNode {
                self.into()
            }

            fn max_bytes() -> usize {
                $max_bytes
            }
        }
    };
}

context_node_type_core!(Mutez, 8);
context_node_type_core!(Nat, 32);
context_node_type_core!(PublicKey, 33);
context_node_type_core!(BlockHash, 32);
context_node_type_core!(OperationHash, 32);
context_node_type_core!(Head, 44);
context_node_type_core!(Checksum, 32);
context_node_type_core!(Micheline, 2048);

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

            fn unwrap(node: ContextNode) -> Self {
                node.try_into().unwrap()
            }

            fn wrap(self) -> ContextNode {
                self.into()
            }

            fn max_bytes() -> usize {
                2048
            }
        }
    };
}

context_node_type_rpc!(OperationReceipt);
context_node_type_rpc!(BatchReceipt);

pub trait TezosAddress {
    fn to_string(&self) -> &str;
}

impl TezosAddress for ImplicitAddress {
    fn to_string(&self) -> &str {
        self.value()
    }
}

impl TezosAddress for ContractAddress {
    fn to_string(&self) -> &str {
        self.value()
    }
}

impl TezosAddress for Address {
    fn to_string(&self) -> &str {
        self.value()
    }
}

impl TezosAddress for &str {
    fn to_string(&self) -> &str {
        self.deref()
    }
}
