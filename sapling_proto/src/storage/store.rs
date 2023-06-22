use derive_more::{From, TryInto};
use layered_store::{error::err_into, internal_error, Result, StoreType};
use zcash_primitives::merkle_tree::HashSer;

use crate::{
    storage::SaplingHead,
    types::{Ciphertext, CommitmentNode, Hash, Nullifier},
};

#[derive(Debug, Clone, From, TryInto)]
pub enum SaplingStoreType {
    SaplingHead(SaplingHead),
    Hash(Hash),
    Nullifier(Nullifier),
    CommitmentNode(CommitmentNode),
    Ciphertext(Ciphertext),
}

impl StoreType for SaplingStoreType {
    fn to_vec(&self) -> Result<Vec<u8>> {
        let (prefix, payload) = match self {
            Self::SaplingHead(value) => (b'\x01', value.encode()?),
            Self::Hash(value) => (b'\x02', value.encode()?),
            Self::Nullifier(value) => (b'\x03', value.encode()?),
            Self::CommitmentNode(value) => (b'\x04', value.encode()?),
            Self::Ciphertext(value) => (b'\x05', value.encode()?),
        };
        Ok([vec![prefix], payload].concat())
    }

    fn from_vec(value: Vec<u8>) -> Result<Self> {
        match value.as_slice() {
            [b'\x01', bytes @ ..] => SaplingHead::decode(bytes),
            [b'\x02', bytes @ ..] => Hash::decode(bytes),
            [b'\x03', bytes @ ..] => Nullifier::decode(bytes),
            [b'\x04', bytes @ ..] => CommitmentNode::decode(bytes),
            [b'\x05', bytes @ ..] => Ciphertext::decode(bytes),
            _ => Err(internal_error!("Invalid context value prefix")),
        }
    }
}

pub trait StoreValue: Clone {
    fn encode(&self) -> Result<Vec<u8>>;
    fn decode(bytes: &[u8]) -> Result<SaplingStoreType>;
}

impl StoreValue for SaplingHead {
    fn decode(bytes: &[u8]) -> Result<SaplingStoreType> {
        let value: SaplingHead = serde_json_wasm::de::from_slice(bytes).map_err(err_into)?;
        Ok(value.into())
    }

    fn encode(&self) -> Result<Vec<u8>> {
        Ok(serde_json_wasm::ser::to_vec(self).map_err(err_into)?)
    }
}

impl StoreValue for Hash {
    fn decode(bytes: &[u8]) -> Result<SaplingStoreType> {
        let value: Hash = bytes.try_into().map_err(err_into)?;
        Ok(value.into())
    }

    fn encode(&self) -> Result<Vec<u8>> {
        Ok(self.to_vec())
    }
}

impl StoreValue for Nullifier {
    fn decode(bytes: &[u8]) -> Result<SaplingStoreType> {
        let value = Nullifier(bytes.try_into().map_err(err_into)?);
        Ok(value.into())
    }

    fn encode(&self) -> Result<Vec<u8>> {
        Ok(self.0.to_vec())
    }
}

impl StoreValue for CommitmentNode {
    fn decode(bytes: &[u8]) -> Result<SaplingStoreType> {
        let value = CommitmentNode::read(bytes).map_err(err_into)?;
        Ok(value.into())
    }

    fn encode(&self) -> Result<Vec<u8>> {
        let mut res: Vec<u8> = Vec::new();
        self.write(&mut res).map_err(err_into)?;
        Ok(res)
    }
}

impl StoreValue for Ciphertext {
    fn decode(bytes: &[u8]) -> Result<SaplingStoreType> {
        let value = Ciphertext::try_from(bytes).map_err(err_into)?;
        Ok(value.into())
    }

    fn encode(&self) -> Result<Vec<u8>> {
        let res = self.try_into().map_err(err_into)?;
        Ok(res)
    }
}
