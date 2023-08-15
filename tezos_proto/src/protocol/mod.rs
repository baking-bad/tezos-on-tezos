pub mod constants;
pub mod migrations;

use tezos_core::types::encoded::{ChainId, OperationHash, BlockHash};
use tezos_operation::operations::SignedOperation;

use crate::{
    protocol::{
        constants::{Constants, ConstantsAlpha},
        migrations::{Migrations, SandboxSeed}
    },
    batch::payload::BatchPayload,
    context::TezosContext,
    Result
};

pub trait Protocol {
    type Constants: Constants;
    type Migrations: Migrations;
    // type Fees: Fees;
    // type Bridge: Bridge;
    // type Contracts: Contracts;
}

pub struct ProtocolAlpha {}

impl Protocol for ProtocolAlpha {
    type Constants = ConstantsAlpha;
    type Migrations = SandboxSeed;
}


pub fn initialize<Proto: Protocol>(context: &mut impl TezosContext, chain_id: ChainId) -> Result<bool> {
    let mut head = context.get_head()?;
    context.log(format!("Protocol initialized: {}", head));

    if head.chain_id != chain_id {
        head.chain_id = chain_id;
        context.set_head(head.clone())?;
        context.commit()?;
    }

    Ok(false)
}

pub fn inject_operation<Proto: Protocol>(context: &mut impl TezosContext, operation: SignedOperation) -> Result<OperationHash> {
    // TODO: validate and add operations to mempool
    todo!()
}

pub fn inject_batch<Proto: Protocol>(context: &mut impl TezosContext, batch: BatchPayload) -> Result<BlockHash> {
    // validate and execute batch
    // remove included mempool operations
    todo!()
}

pub fn finalize<Proto: Protocol>(context: &mut impl TezosContext, timestamp: i64) -> Result<()> {
    let mut head = context.get_head()?;

    let constants = Proto::Constants::constants();
    if timestamp - head.timestamp > (constants.minimal_block_delay * constants.blocks_per_cycle) as i64 {
        head.timestamp = timestamp;
        context.set_head(head.clone())?;
        context.commit()?;
    }

    // Make an implicit batch in case:
    // - there are mempool operations that about to expire
    // - more than N seconds passed since the latest block

    // remove invalid mempool operations (due to balance/counter changes)

    Ok(())
}
