// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use tezos_core::types::number::Nat;

pub const PROTOCOL: &str = "ProtoALphaALphaALphaALphaALphaALphaALphaALphaDdp3zK";
pub const BLOCK_TIME: i64 = 8;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub blocks_per_cycle: i32,
    pub max_operations_time_to_live: i32,
    pub max_block_header_length: i32,
    pub max_operation_data_length: i32,
    pub max_operations_list_length: i32,
    pub pow_nonce: String,
    pub hard_gas_limit_per_operation: Nat,
    pub hard_storage_limit_per_operation: Nat,
    pub hard_gas_limit_per_block: Nat,
    pub cost_per_byte: Nat,
}

impl Config {
    pub fn default() -> Self {
        Self {
            blocks_per_cycle: 8096,
            max_operations_time_to_live: 240,
            max_block_header_length: 2048,
            max_operation_data_length: 86400,
            max_operations_list_length: 1024,
            pow_nonce: "deadbeef".into(),
            hard_gas_limit_per_operation: 1040000u64.into(),
            hard_storage_limit_per_operation: 60000u64.into(),
            hard_gas_limit_per_block: 5200000u64.into(),
            cost_per_byte: 250u32.into(),
        }
    }
}
