use tezos_core::types::encoded::{BlockHash, OperationHash, Encoded};
use serde::{Serialize, Deserialize};

pub const ZERO_BLOCK_HASH: &str = "BKiHLREqU3JkXfzEDYAkmmfX48gBDtYhMrpA98s7Aq4SzbUAB6M";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Head {
    pub level: i32,
    pub hash: BlockHash,
    pub timestamp: i64,
    pub operations: Vec<OperationHash>,
}

impl Head {
    pub fn new(level: i32, hash: BlockHash, timestamp: i64, operations: Vec<OperationHash>) -> Self {
        Self {
            level,
            hash,
            timestamp,
            operations,
        }
    }

    pub fn default() -> Self {
        Self {
            level: -1,
            hash: BlockHash::new(ZERO_BLOCK_HASH.into()).unwrap(),
            timestamp: 0,
            operations: Vec::new(),
        }
    }
}

impl std::fmt::Display for Head {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "lvl: {}, ts: {}, hash: {}, #ops: {}",
            self.level,
            self.timestamp,
            self.hash.value(),
            self.operations.len(),
        ))
    }
}
