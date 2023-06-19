use derive_more::{From, TryInto};
use tezos_core::types::{
    encoded::{ContractAddress, Encoded, OperationHash, PublicKey},
    mutez::Mutez,
    number::Nat,
};
use tezos_michelson::micheline::Micheline;
use tezos_rpc::models::operation::Operation;
use layered_store::{StoreType, error::err_into, Result, internal_error};

use crate::{context::batch::BatchReceipt, context::head::Head};

#[derive(Debug, Clone, From, TryInto)]
pub enum TezosStoreType {
    Head(Head),
    Int(i64),
    Operation(Operation),
    Batch(BatchReceipt),
    PublicKey(PublicKey),
    OperationHash(OperationHash),
    ContractAddress(ContractAddress),
    Micheline(Micheline),
    Nat(Nat),
    Mutez(Mutez),
}

impl StoreType for TezosStoreType {
    fn to_vec(&self) -> Result<Vec<u8>> {
        let (prefix, payload) = match self {
            Self::Head(value) => (b'\x01', value.encode()?),
            Self::Int(value) => (b'\x02', value.encode()?),
            Self::Operation(value) => (b'\x03', value.encode()?),
            Self::Batch(value) => (b'\x04', value.encode()?),
            Self::PublicKey(value) => (b'\x05', value.encode()?),
            Self::OperationHash(value) => (b'\x06', value.encode()?),
            Self::ContractAddress(value) => (b'\x07', value.encode()?),
            Self::Micheline(value) => (b'\x08', value.encode()?),
            Self::Nat(value) => (b'\x09', value.encode()?),
            Self::Mutez(value) => (b'\x0A', value.encode()?),
        };
        Ok([vec![prefix], payload].concat())
    }

    fn from_vec(value: Vec<u8>) -> Result<Self> {
        match value.as_slice() {
            [b'\x01', bytes @ ..] => Head::decode(bytes),
            [b'\x02', bytes @ ..] => i64::decode(bytes),
            [b'\x03', bytes @ ..] => Operation::decode(bytes),
            [b'\x04', bytes @ ..] => BatchReceipt::decode(bytes),
            [b'\x05', bytes @ ..] => PublicKey::decode(bytes),
            [b'\x06', bytes @ ..] => OperationHash::decode(bytes),
            [b'\x07', bytes @ ..] => ContractAddress::decode(bytes),
            [b'\x08', bytes @ ..] => Micheline::decode(bytes),
            [b'\x09', bytes @ ..] => Nat::decode(bytes),
            [b'\x0A', bytes @ ..] => Mutez::decode(bytes),
            _ => Err(internal_error!("Invalid context value prefix")),
        }
    }
}

pub trait StoreValue: Clone {
    fn encode(&self) -> Result<Vec<u8>>;
    fn decode(bytes: &[u8]) -> Result<TezosStoreType>;
}

macro_rules! context_node_type_core {
    ($ty: ty) => {
        impl StoreValue for $ty {
            fn decode(bytes: &[u8]) -> Result<TezosStoreType> {
                match Self::from_bytes(bytes) {
                    Ok(value) => Ok(value.into()),
                    Err(error) => Err(err_into(error)),
                }
            }

            fn encode(&self) -> Result<Vec<u8>> {
                self.to_bytes().map_err(err_into)
            }
        }
    };
}

context_node_type_core!(PublicKey);
context_node_type_core!(OperationHash);
context_node_type_core!(ContractAddress);
context_node_type_core!(Micheline);
context_node_type_core!(Mutez);
context_node_type_core!(Nat);

macro_rules! context_node_type_rpc {
    ($ty: ty) => {
        impl StoreValue for $ty {
            fn decode(_bytes: &[u8]) -> Result<TezosStoreType> {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    // This is a workaround to avoid floating point operations introduced by serde.
                    // Since we do not need RPC models deserialization inside the kernel,
                    // we can only enable that for tests and binaries that are not compiled to wasm.
                    let value: $ty = serde_json_wasm::de::from_slice(_bytes).map_err(err_into)?;
                    return Ok(value.into());
                }
                #[cfg(target_arch = "wasm32")]
                unimplemented!()
            }

            fn encode(&self) -> Result<Vec<u8>> {
                Ok(serde_json_wasm::ser::to_vec(self).map_err(err_into)?)
            }
        }
    };
}

context_node_type_rpc!(Operation);
context_node_type_rpc!(BatchReceipt);

impl StoreValue for i64 {
    fn decode(bytes: &[u8]) -> Result<TezosStoreType> {
        if bytes.len() == 8 {
            let value = i64::from_be_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]);
            Ok(value.into())
        } else {
            Err(internal_error!("Failed to decode i64"))
        }
    }

    fn encode(&self) -> Result<Vec<u8>> {
        Ok(self.to_be_bytes().to_vec())
    }
}

impl StoreValue for Head {
    fn decode(bytes: &[u8]) -> Result<TezosStoreType> {
        let value: Head = serde_json_wasm::de::from_slice(bytes).map_err(err_into)?;
        Ok(value.into())
    }

    fn encode(&self) -> Result<Vec<u8>> {
        Ok(serde_json_wasm::ser::to_vec(self).map_err(err_into)?)
    }
}
