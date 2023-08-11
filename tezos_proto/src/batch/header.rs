// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use tezos_core::{
    internal::coder::Encoder,
    internal::crypto::blake2b,
    types::encoded::{
        BlockHash, BlockPayloadHash, ContextHash, Encoded, OperationHash, OperationListListHash,
        Signature,
    },
};
use tezos_operation::{
    block_header, internal::coder::operation_content_bytes_coder::OperationContentBytesCoder,
};
use tezos_rpc::models::block::{Header, LiquidityBakingToggleVote};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{context::head::Head, Result};

// TODO: make it meaningful so that TzKT can understand + embed git commit
pub const POW_NONCE: &str = "deadbeef";
pub const ZERO_SIGNATURE: &str =
    "sigMzJ4GVAvXEd2RjsKGfG2H9QvqTSKCZsuB2KiHbZRGFz72XgF6KaKADznh674fQgBatxw3xdHqTtMHUZAGRprxy64wg1aq";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatchHeader {
    pub predecessor: BlockHash,
    pub level: i32,
    pub timestamp: i64,
    pub operations_hash: OperationListListHash,
    pub payload_hash: BlockPayloadHash,
    pub context: ContextHash,
    pub signature: Option<Signature>,
}

impl BatchHeader {
    pub fn block_hash(&self) -> Result<BlockHash> {
        let header = block_header::BlockHeader::from(self.clone());
        let payload = OperationContentBytesCoder::encode(&header)?;
        let hash = blake2b(payload.as_slice(), 32)?;
        Ok(BlockHash::from_bytes(&hash)?)
    }

    pub fn implicit(
        prev_head: &Head,
        operation_hashes: Vec<OperationHash>,
        context_hash: ContextHash,
    ) -> Result<Self> {
        Ok(BatchHeader {
            level: prev_head.level + 1,
            predecessor: prev_head.hash.clone(),
            payload_hash: BlockPayloadHash::from_parts(
                prev_head.hash.clone(),
                0,
                operation_hashes.clone(),
            )?,
            operations_hash: OperationListListHash::try_from(vec![
                vec![],
                vec![],
                vec![],
                operation_hashes,
            ])?,
            context: context_hash,
            timestamp: prev_head.timestamp + 1, // Minimal possible delta, cannot go faster :D
            signature: None,
        })
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
            proof_of_work_nonce: POW_NONCE.try_into().unwrap(),
            proto: 0,
            seed_nonce_hash: None,
            signature: header
                .signature
                .unwrap_or_else(|| ZERO_SIGNATURE.try_into().unwrap()),
            timestamp: NaiveDateTime::from_timestamp_opt(header.timestamp, 0).unwrap(),
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
            timestamp: NaiveDateTime::from_timestamp_opt(header.timestamp, 0).unwrap(),
            validation_pass: 4,
        }
    }
}
