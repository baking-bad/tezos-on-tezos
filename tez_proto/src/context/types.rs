use std::ops::Deref;
use serde_json_wasm;
use derive_more::{From, TryInto};

pub use tezos_core::{
    types::{
        encoded::{Encoded, PublicKey, Address, ImplicitAddress, ContextHash, BlockHash},
        mutez::Mutez,
        number::Nat
    }
};
pub use tezos_rpc::models::{
    operation::Operation as OperationReceipt,
    block::FullHeader as BlockHeader,
    block::Metadata as BlockMetadata
};

use crate::context::{
    head::Head,
    checksum::Checksum
};
use crate::error::Result;

#[derive(Debug, Clone, From, TryInto)]
pub enum ContextNode {
    Mutez(Mutez),
    Nat(Nat),
    PublicKey(PublicKey),
    Checksum(Checksum),
    OperationReceipt(OperationReceipt),
    BlockHeader(BlockHeader),
    BlockMetadata(BlockMetadata),
    Head(Head)
}

impl ContextNode {
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        match self {
            Self::Head(value) => value.encode(),
            Self::Checksum(value) => value.encode(),
            Self::Mutez(value) => value.encode(),
            Self::Nat(value) => value.encode(),
            Self::PublicKey(value) => value.encode(),
            Self::OperationReceipt(value) => value.encode(),
            Self::BlockHeader(value) => value.encode(),
            Self::BlockMetadata(value) => value.encode(),
        }
    }
}

pub trait ContextNodeType : Clone {
    fn encode(&self) -> Result<Vec<u8>>;
    fn decode(bytes: &[u8]) -> Result<ContextNode>;
    fn unwrap(node: ContextNode) -> Self;
    fn wrap(self) -> ContextNode;
}

macro_rules! context_node_type_core {
    ($ty: ty) => {
        impl ContextNodeType for $ty {
            fn decode(bytes: &[u8]) -> Result<ContextNode> {
                match Self::from_bytes(bytes) {
                    Ok(value) => Ok(value.into()),
                    Err(error) => Err(error.into())
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
        }
    };
}

context_node_type_core!(Mutez);
context_node_type_core!(Nat);
context_node_type_core!(PublicKey);
context_node_type_core!(Head);
context_node_type_core!(Checksum);

macro_rules! context_node_type_rpc {
    ($ty: ty) => {
        impl ContextNodeType for $ty {
            fn decode(bytes: &[u8]) -> Result<ContextNode> {
                match serde_json_wasm::from_slice::<$ty>(bytes) {
                    Ok(value) => Ok(value.into()),
                    Err(error) => Err(error.into())
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
        }
    };
}

context_node_type_rpc!(OperationReceipt);
context_node_type_rpc!(BlockHeader);
context_node_type_rpc!(BlockMetadata);

pub trait TezosAddress {
    fn to_string(&self) -> &str;
}

impl TezosAddress for ImplicitAddress {
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
