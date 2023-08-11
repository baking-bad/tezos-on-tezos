// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use layered_store::{error::err_into, Result, StoreType};
use serde::{Deserialize, Serialize};
use tezos_core::types::encoded::{BlockHash, ChainId, Encoded, OperationHash};

pub const ZERO_BLOCK_HASH: &str = "BKiHLREqU3JkXfzEDYAkmmfX48gBDtYhMrpA98s7Aq4SzbUAB6M";
pub const ZERO_CHAIN_ID: &str = "NetXH12Aer3be93";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Head {
    pub chain_id: ChainId,
    pub level: i32,
    pub hash: BlockHash,
    pub timestamp: i64,
    pub operations: Vec<OperationHash>,
}

impl Head {
    pub fn new(
        chain_id: ChainId,
        level: i32,
        hash: BlockHash,
        timestamp: i64,
        operations: Vec<OperationHash>,
    ) -> Self {
        Self {
            chain_id,
            level,
            hash,
            timestamp,
            operations,
        }
    }

    pub fn default() -> Self {
        Self {
            chain_id: ZERO_CHAIN_ID.try_into().unwrap(),
            level: -1,
            hash: ZERO_BLOCK_HASH.try_into().unwrap(),
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

impl StoreType for Head {
    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        serde_json_wasm::de::from_slice(bytes).map_err(err_into)
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        serde_json_wasm::ser::to_vec(self).map_err(err_into)
    }
}
