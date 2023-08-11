// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use layered_store::{error::err_into, Result, StoreType};
use tezos_core::types::encoded::{BlockHash, ChainId, ImplicitAddress, ProtocolHash};
use tezos_rpc::models::{
    balance_update::BalanceUpdate,
    block::{
        FullHeader, LevelInfo, Metadata, OperationListLength, TestChainStatus, TestChainStatusName,
    },
};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::batch::header::{BatchHeader, POW_NONCE};
use crate::protocol::constants::{Constants, ProtocolAlpha};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatchReceipt {
    pub chain_id: ChainId,
    pub protocol: ProtocolHash,
    pub hash: BlockHash,
    pub header: BatchHeader,
    pub balance_updates: Option<Vec<BalanceUpdate>>,
    pub batcher: Option<ImplicitAddress>,
}

impl StoreType for BatchReceipt {
    fn from_bytes(_bytes: &[u8]) -> Result<Self> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // This is a workaround to avoid floating point operations introduced by serde.
            // Since we do not need RPC models deserialization inside the kernel,
            // we can only enable that for tests and binaries that are not compiled to wasm.
            serde_json_wasm::de::from_slice(_bytes).map_err(err_into)
        }
        #[cfg(target_arch = "wasm32")]
        unimplemented!()
    }

    fn to_bytes(&self) -> Result<Vec<u8>> {
        serde_json_wasm::ser::to_vec(self).map_err(err_into)
    }
}

impl From<BatchReceipt> for Metadata {
    fn from(receipt: BatchReceipt) -> Self {
        // TODO: this code is only used in sequencer, need to move it there
        // Constants should be retrieved separately and then merged with batch receipts
        let constants = ProtocolAlpha::constants();
        Self {
            baker: receipt.batcher.clone(),
            proposer: receipt.batcher,
            balance_updates: receipt.balance_updates,
            // derived
            protocol: receipt.protocol.clone(),
            next_protocol: receipt.protocol,
            level_info: Some(LevelInfo {
                level: receipt.header.level,
                level_position: receipt.header.level - 1,
                cycle: receipt.header.level / constants.blocks_per_cycle,
                cycle_position: receipt.header.level % constants.blocks_per_cycle,
                expected_commitment: false,
            }),
            // default
            max_operations_ttl: constants.max_operations_time_to_live,
            max_operation_data_length: constants.max_operation_data_length,
            max_operation_list_length: vec![
                OperationListLength {
                    max_size: 0,
                    max_op: None,
                },
                OperationListLength {
                    max_size: 0,
                    max_op: None,
                },
                OperationListLength {
                    max_size: 0,
                    max_op: None,
                },
                OperationListLength {
                    max_size: constants.max_operations_list_length,
                    max_op: Some(
                        constants.max_operations_list_length * constants.max_operation_data_length,
                    ),
                },
            ],
            max_block_header_length: constants.max_block_header_length,
            // null / deprecated
            level: None,
            consumed_gas: None,
            deactivated: None,
            implicit_operations_results: None,
            voting_period_kind: None,
            voting_period_info: None,
            test_chain_status: TestChainStatus {
                status: TestChainStatusName::NotRunning,
                chain_id: None,
                genesis: None,
                protocol: None,
                expiration: None,
            },
            nonce_hash: None,
            liquidity_baking_toggle_ema: None,
            liquidity_baking_escape_ema: None,
        }
    }
}

// TODO: move to sequencer
impl From<BatchReceipt> for FullHeader {
    fn from(receipt: BatchReceipt) -> Self {
        Self {
            chain_id: receipt.chain_id,
            context: receipt.header.context,
            fitness: vec![],
            hash: receipt.hash,
            level: receipt.header.level,
            liquidity_baking_escape_vote: false,
            operations_hash: receipt.header.operations_hash,
            payload_hash: Some(receipt.header.payload_hash),
            payload_round: 0,
            predecessor: receipt.header.predecessor,
            priority: 0,
            proof_of_work_nonce: POW_NONCE.into(),
            proto: 0,
            protocol: receipt.protocol,
            seed_nonce_hash: None,
            signature: None,
            timestamp: NaiveDateTime::from_timestamp_opt(receipt.header.timestamp, 0).unwrap(),
            validation_pass: 4,
        }
    }
}
