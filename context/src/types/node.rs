use derive_more::{From, TryInto};
use tezos_core::types::{
    encoded::{BlockHash, ContractAddress, Encoded, OperationHash, PublicKey},
    mutez::Mutez,
    number::Nat,
};
use tezos_michelson::micheline::Micheline;

use crate::{internal_error, Head, Result};

#[derive(Debug, Clone, From, TryInto)]
pub enum ContextNode {
    Head(Head),
    Mutez(Mutez),
    Nat(Nat),
    PublicKey(PublicKey),
    BlockHash(BlockHash),
    OperationHash(OperationHash),
    ContractAddress(ContractAddress),
    Micheline(Micheline),
    Raw(Vec<u8>),
    I64(i64),
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
            Self::ContractAddress(value) => (b'\x07', value.encode()?),
            Self::Micheline(value) => (b'\x08', value.encode()?),
            Self::Raw(value) => (b'\x09', value.clone()),
            Self::I64(value) => (b'\x0A', value.to_be_bytes().to_vec()),
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
            [b'\x07', bytes @ ..] => ContractAddress::decode(bytes),
            [b'\x08', bytes @ ..] => Micheline::decode(bytes),
            [b'\x09', bytes @ ..] => Ok(bytes.to_vec().into()),
            [b'\x0A', bytes @ ..] => decode_i64(bytes),
            _ => Err(internal_error!(Encoding, "Invalid context node prefix")),
        }
    }
}

pub fn decode_i64(bytes: &[u8]) -> Result<ContextNode> {
    if bytes.len() == 8 {
        let value = i64::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]);
        Ok(value.into())
    } else {
        Err(internal_error!(Encoding, ""))
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
context_node_type_core!(ContractAddress);
context_node_type_core!(Head);
context_node_type_core!(Micheline);
