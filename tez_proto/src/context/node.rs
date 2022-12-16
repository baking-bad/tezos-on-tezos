use std::ops::Deref;
use tezos_core::{
    types::{
        encoded::{Encoded, PublicKey, Address, ImplicitAddress},
        mutez::Mutez,
        number::Nat
    }
};
use tezos_rpc::models::operation::{
    Operation as OperationReceipt
};
use serde_json_wasm;

use crate::error::Result;

#[derive(Debug, Clone)]
pub enum ContextNode {
    Mutez(Mutez),
    Nat(Nat),
    PublicKey(PublicKey),
    OperationReceipt(OperationReceipt)
}

impl ContextNode {
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        match self {
            Self::Mutez(value) => value.to_bytes().map_err(|e| e.into()),
            Self::Nat(value) => value.to_bytes().map_err(|e| e.into()),
            Self::PublicKey(value) => value.to_bytes().map_err(|e| e.into()),
            Self::OperationReceipt(value) => serde_json_wasm::to_vec(value).map_err(|e| e.into())
        }
    }
}

pub trait NodeType {
    fn parse(bytes: &[u8]) -> Result<ContextNode>;
    fn unwrap(node: &ContextNode) -> Self;
    fn wrap(value: &Self) -> ContextNode;
}

impl NodeType for Mutez {
    fn parse(bytes: &[u8]) -> Result<ContextNode> {
        match Mutez::from_bytes(bytes) {
            Ok(value) => Ok(ContextNode::Mutez(value)),
            Err(error) => Err(error.into())
        }
    }

    fn unwrap(node: &ContextNode) -> Self {
        match node {
            ContextNode::Mutez(value) => value.clone(),
            node => panic!("Type mismatch (expected Mutez, got {:?})", node)
        }
    }

    fn wrap(value: &Self) -> ContextNode {  
        ContextNode::Mutez(value.clone())
    }
}

impl NodeType for Nat {
    fn parse(bytes: &[u8]) -> Result<ContextNode> {
        match Nat::from_bytes(bytes) {
            Ok(value) => Ok(ContextNode::Nat(value)),
            Err(error) => Err(error.into())
        }
    }

    fn unwrap(node: &ContextNode) -> Self {
        match node {
            ContextNode::Nat(value) => value.clone(),
            node => panic!("Type mismatch (expected Nat, got {:?})", node)
        }
    }

    fn wrap(value: &Self) -> ContextNode {
        ContextNode::Nat(value.clone())
    }
}

impl NodeType for PublicKey {
    fn parse(bytes: &[u8]) -> Result<ContextNode> {
        match PublicKey::from_bytes(bytes) {
            Ok(value) => Ok(ContextNode::PublicKey(value)),
            Err(error) => Err(error.into())
        }
    }

    fn unwrap(node: &ContextNode) -> Self {
        match node {
            ContextNode::PublicKey(value) => value.clone(),
            node => panic!("Type mismatch (expected PublicKey, got {:?})", node)
        }
    }

    fn wrap(value: &Self) -> ContextNode {
        ContextNode::PublicKey(value.clone())
    }
}

impl NodeType for OperationReceipt {
    fn parse(bytes: &[u8]) -> Result<ContextNode> {
        match serde_json_wasm::from_slice(bytes) {
            Ok(value) => Ok(ContextNode::OperationReceipt(value)),
            Err(error) => Err(error.into())
        }
    }

    fn unwrap(node: &ContextNode) -> Self {
        match node {
            ContextNode::OperationReceipt(value) => value.clone(),
            node => panic!("Type mismatch (expected OperationReceipt, got {:?})", node)
        }
    }

    fn wrap(value: &Self) -> ContextNode {
        ContextNode::OperationReceipt(value.clone())
    }
}

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
