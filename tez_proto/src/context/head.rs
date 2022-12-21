use tezos_core::{
    types::encoded::{Encoded, BlockHash},
};
use crate::error::Result;
use crate::constants::ZERO_BLOCK_HASH;

#[derive(Debug, Clone)]
pub struct Head {
    pub level: i32,
    pub hash: BlockHash,
    pub timestamp: i64
}

impl Head {
    pub fn new(level: i32, hash: BlockHash, timestamp: i64) -> Self {
        Self { level, hash, timestamp }
    }

    pub fn default() -> Self {
        Self {
            level: -1,
            hash: BlockHash::new(ZERO_BLOCK_HASH.into()).unwrap(),
            timestamp: 0
        }
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() != 4 + 32 + 8 {
            return Err(tezos_core::Error::InvalidBytes.into())
        }
        Ok(Self {
            level: i32::from_be_bytes([data[0], data[1], data[2], data[3]]),
            hash: BlockHash::from_bytes(&data[4..])?,
            timestamp: i64::from_be_bytes([data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]])
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok([
            self.level.to_be_bytes().to_vec(),
            self.hash.to_bytes()?,
            self.timestamp.to_be_bytes().to_vec()
        ].concat())
    }
}