use tezos_core::{
    types::encoded::{Encoded, BlockHash},
};
use crate::Result;
use crate::constants::ZERO_BLOCK_HASH;

#[derive(Debug, Clone, PartialEq)]
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
        if data.len() != 44 {
            return Err(tezos_core::Error::InvalidBytes.into())
        }
        Ok(Self {
            level: i32::from_be_bytes([data[0], data[1], data[2], data[3]]),
            hash: BlockHash::from_bytes(&data[4..36])?,
            timestamp: i64::from_be_bytes([data[36], data[37], data[38], data[39], data[40], data[41], data[42], data[43]])
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_head_encoding() -> Result<()> {
        let head = Head::default();
        let raw = head.to_bytes()?;
        let res = Head::from_bytes(raw.as_slice())?;
        assert_eq!(head, res);
        Ok(())
    }
}