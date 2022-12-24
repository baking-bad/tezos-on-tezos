use tezos_core::types::encoded::{ContextHash, Encoded};
use tezos_core::internal::crypto::blake2b;
use crate::Result;

const ZERO_32: &[u8; 32] = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

#[derive(Debug, Clone, PartialEq)]
pub struct Checksum([u8; 32]);

impl Checksum {
    pub fn default() -> Self {
        Self {
            0: ZERO_32.to_owned()
        }
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        Ok(Self {
            0: data.try_into().map_err(|_| tezos_core::Error::InvalidBytes)?
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok(self.0.to_vec())
    }

    pub fn update(&mut self, value: &Vec<u8>) -> Result<()> {
        let digest = blake2b(value, 32)?;
        self.0.iter_mut().zip(digest.iter()).for_each(|(a, b)| *a ^= *b);
        Ok(())
    }

    pub fn hash(&self) -> Result<ContextHash> {
        ContextHash::from_bytes(self.0.as_slice()).map_err(|e| e.into())
    }
}