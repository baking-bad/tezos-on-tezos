// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use tezos_core::types::{encoded::ProtocolHash, number::Nat};

const PROTOCOL: &str = "ProtoALphaALphaALphaALphaALphaALphaALphaALphaDdp3zK";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProtocolConstants {
    pub idle_time: i64,
    pub minimal_block_delay: i32,
    pub blocks_per_cycle: i32,
    pub max_operations_time_to_live: i32,
    pub max_block_header_length: i32,
    pub max_operation_data_length: i32,
    pub max_operations_list_length: i32,
    pub hard_gas_limit_per_operation: Nat,
    pub hard_storage_limit_per_operation: Nat,
    pub hard_gas_limit_per_block: Nat,
    pub cost_per_byte: Nat,
}

pub trait Constants {
    fn protocol() -> ProtocolHash;
    fn constants() -> ProtocolConstants;
}

pub struct ConstantsAlpha {}

impl Constants for ConstantsAlpha {
    fn protocol() -> ProtocolHash {
        PROTOCOL.try_into().unwrap()
    }

    fn constants() -> ProtocolConstants {
        ProtocolConstants {
            idle_time: 1,
            minimal_block_delay: 1,
            blocks_per_cycle: 512,
            max_operations_time_to_live: 240,
            max_block_header_length: 2048,
            max_operation_data_length: 86400,
            max_operations_list_length: 1024,
            hard_gas_limit_per_operation: 1040000u64.into(),
            hard_storage_limit_per_operation: 60000u64.into(),
            hard_gas_limit_per_block: 5200000u64.into(),
            cost_per_byte: 250u32.into(),
        }
    }
}
