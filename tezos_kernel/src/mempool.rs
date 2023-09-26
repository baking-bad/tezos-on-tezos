use std::mem::size_of;

use layered_store::StoreType;
use tezos_core::types::encoded::OperationHash;
use tezos_operation::operations::SignedOperation;

use crate::Result;

#[derive(Debug, Clone, Default)]
pub struct MempoolState(Vec<u8>);

impl StoreType for MempoolState {
    fn to_bytes(&self) -> layered_store::Result<Vec<u8>> {
        Ok(self.0.clone())
    }

    fn from_bytes(bytes: &[u8]) -> layered_store::Result<Self> {
        Ok(Self(bytes.to_vec()))
    }
}

impl MempoolState {
    const HASH_SIZE: usize = 32;

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len() / Self::HASH_SIZE
    }

    fn find(&self, term: &[u8]) -> Option<usize> {
        let mut ptr = 0;
        while ptr + Self::HASH_SIZE <= self.0.len() {
            if &self.0[ptr..ptr+Self::HASH_SIZE] == term {
                return Some(ptr);
            }
            ptr += Self::HASH_SIZE;
        }
        None
    }

    pub fn add(&mut self, hash: &OperationHash) -> Result<bool> {
        let mut term = StoreType::to_bytes(hash)?;
        if self.find(term.as_slice()).is_none() {
            self.0.append(&mut term);
            return Ok(true);
        }
        Ok(false)
    }

    pub fn remove(&mut self, hash: &OperationHash) -> Result<bool> {
        let term = StoreType::to_bytes(hash)?;
        if let Some(ptr) = self.find(term.as_slice()) {
            self.0.drain(ptr..ptr+Self::HASH_SIZE);
            return Ok(true);
        }
        Ok(false)
    }

    pub fn to_vec(&self) -> Result<Vec<OperationHash>> {
        let res = self.0
            .chunks_exact(Self::HASH_SIZE)
            .map(|chunk| StoreType::from_bytes(chunk))
            .collect::<layered_store::Result<Vec<OperationHash>>>();
        Ok(res?)
    }
}
