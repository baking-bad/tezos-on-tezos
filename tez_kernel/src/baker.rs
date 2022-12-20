use tez_proto::{
    context::Context,
    executor::block::{
        OperationHash, Signature, Encoded,
        BlockPayloadHash, OperationListListHash, HexString,
        block_header::{BlockHeader, LiquidityBakingToggleVote},
    },
};
use chrono::NaiveDateTime;

use crate::error::Result;

pub const ZERO_SIGNATURE: &str = "sigMzJ4GVAvXEd2RjsKGfG2H9QvqTSKCZsuB2KiHbZRGFz72XgF6KaKADznh674fQgBatxw3xdHqTtMHUZAGRprxy64wg1aq";
pub const POW_NONCE: &str = "0xdeadbeef";
pub const BLOCK_TIME: i64 = 15; 

pub fn wrap_rollup_block(
    context: &mut impl Context, 
    operation_hashes: Vec<OperationHash>,
    pred_timestamp: i64
) -> Result<BlockHeader> {
    let head = context.get_head()?;  // NOTE: validated at the migrations phase
    Ok(BlockHeader {
        signature: Signature::new(ZERO_SIGNATURE.into())?,  // TODO: sign with revealed preimage key
        level: head.level + 1,
        predecessor: head.hash.to_owned(),
        payload_hash: BlockPayloadHash::from_parts(
            head.hash, 
            0, 
            operation_hashes.to_owned()
        )?,
        operations_hash: OperationListListHash::try_from(
            vec![vec![], vec![], vec![], operation_hashes]
        )?,
        context: context.get_checksum()?.hash()?,
        timestamp: NaiveDateTime::from_timestamp_opt(pred_timestamp + BLOCK_TIME, 0).unwrap(),
        proof_of_work_nonce: HexString::new(POW_NONCE.into())?,  // TODO: repo commit hash
        // Default values
        proto: 0,
        payload_round: 0,
        validation_pass: 4,
        seed_nonce_hash: None,
        liquidity_baking_toggle_vote: LiquidityBakingToggleVote::Off,
        fitness: vec![],
    })
}