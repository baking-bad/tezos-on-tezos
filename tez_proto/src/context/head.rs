use tezos_core::{
    types::encoded::{Encoded, BlockHash},
};
use crate::error::Result;

pub const ZERO_BLOCK_HASH: &str = "BKiHLREqU3JkXfzEDYAkmmfX48gBDtYhMrpA98s7Aq4SzbUAB6M";

#[derive(Debug, Clone)]
pub struct Head {
    pub level: i32,
    pub hash: BlockHash
}

impl Head {
    pub fn new(level: i32, hash: BlockHash) -> Self {
        Self { level, hash }
    }

    pub fn default() -> Self {
        Self {
            level: -1,
            hash: BlockHash::new(ZERO_BLOCK_HASH.into()).unwrap()
        }
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() != 4 + 32 {
            return Err(tezos_core::Error::InvalidBytes.into())
        }
        Ok(Self {
            level: i32::from_be_bytes([data[0], data[1], data[2], data[3]]),
            hash: BlockHash::from_bytes(&data[4..])?
        })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        Ok([self.level.to_be_bytes().to_vec(), self.hash.to_bytes()?].concat())
    }
}