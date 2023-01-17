pub use tezos_core::types::{
    encoded::{
        BlockHash, BlockPayloadHash, ChainId, ContextHash, Encoded, ImplicitAddress, OperationHash,
        OperationListListHash, ProtocolHash, Signature,
    },
    hex_string::HexString,
};
pub use tezos_operation::{
    block_header,
    operations::{SignedOperation, UnsignedOperation},
};
pub use tezos_rpc::models::{
    balance_update,
    balance_update::BalanceUpdate,
    block::{
        Block, FullHeader, Header, LevelInfo, LiquidityBakingToggleVote, Metadata,
        OperationListLength, TestChainStatus, TestChainStatusName,
    },
    operation::Operation as OperationReceipt,
};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{constants::*, Result};

#[macro_export]
macro_rules! ts2dt {
    ($ts: expr) => {
        NaiveDateTime::from_timestamp_opt($ts, 0).unwrap()
    };
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatchHeader {
    pub predecessor: BlockHash,
    pub level: i32,
    pub timestamp: i64,
    pub operations_hash: OperationListListHash,
    pub payload_hash: BlockPayloadHash,
    pub context: ContextHash,
    // pub signature: Option<Signature>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatchReceipt {
    pub chain_id: ChainId,
    pub protocol: ProtocolHash,
    pub hash: BlockHash,
    pub header: BatchHeader,
    pub balance_updates: Option<Vec<BalanceUpdate>>,
    // pub batcher: ImplicitAddress
}

impl BatchHeader {
    pub fn hash(&self) -> Result<BlockHash> {
        let _header = block_header::BlockHeader::from(self.to_owned());
        // TODO: use BlockHeader::hash() once it in the SDK
        Ok(ZERO_BLOCK_HASH.try_into().unwrap())
    }
}

impl From<BatchHeader> for block_header::BlockHeader {
    fn from(header: BatchHeader) -> Self {
        Self {
            context: header.context,
            fitness: vec![],
            level: header.level,
            liquidity_baking_toggle_vote: block_header::LiquidityBakingToggleVote::Off,
            operations_hash: header.operations_hash,
            payload_hash: header.payload_hash,
            payload_round: 0,
            predecessor: header.predecessor,
            proof_of_work_nonce: POW_NONCE.try_into().unwrap(), // TODO: repo commit hash
            proto: 0,
            seed_nonce_hash: None,
            signature: ZERO_SIGNATURE.try_into().unwrap(), // TODO: sign with revealed preimage key
            timestamp: ts2dt!(header.timestamp),
            validation_pass: 4,
        }
    }
}

impl From<BatchHeader> for Header {
    fn from(header: BatchHeader) -> Self {
        Self {
            context: header.context,
            fitness: vec![],
            level: header.level,
            liquidity_baking_escape_vote: false,
            liquidity_baking_toggle_vote: LiquidityBakingToggleVote::Off,
            operations_hash: header.operations_hash,
            payload_hash: Some(header.payload_hash),
            payload_round: 0,
            predecessor: header.predecessor,
            priority: 0,
            proof_of_work_nonce: POW_NONCE.into(),
            proto: 0,
            seed_nonce_hash: None,
            signature: None,
            timestamp: ts2dt!(header.timestamp),
            validation_pass: 4,
        }
    }
}

impl From<BatchReceipt> for Metadata {
    fn from(receipt: BatchReceipt) -> Self {
        Self {
            baker: None, // TODO: + signature
            balance_updates: receipt.balance_updates,
            // derived
            protocol: receipt.protocol.to_owned(),
            next_protocol: receipt.protocol,
            level_info: Some(LevelInfo {
                level: receipt.header.level,
                level_position: receipt.header.level - 1,
                cycle: receipt.header.level / CYCLE_BLOCKS,
                cycle_position: receipt.header.level % CYCLE_BLOCKS,
                expected_commitment: false,
            }),
            // default
            max_operations_ttl: MAX_OPERATIONS_TTL,
            max_operation_data_length: MAX_OPERATION_DATA_LENGTH,
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
                    max_size: MAX_OPERATION_LIST_LENGTH,
                    max_op: Some(MAX_TOTAL_OPERATION_DATA_LENGTH),
                },
            ],
            max_block_header_length: MAX_BLOCK_HEADER_LENGTH,
            // null
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
            proposer: None,
            nonce_hash: None,
            liquidity_baking_toggle_ema: None,
            liquidity_baking_escape_ema: None,
        }
    }
}

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
            timestamp: ts2dt!(receipt.header.timestamp),
            validation_pass: 4,
        }
    }
}
