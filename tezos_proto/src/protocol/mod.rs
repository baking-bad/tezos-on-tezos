pub mod constants;
pub mod migrations;

use tezos_core::types::encoded::ChainId;
use tezos_operation::operations::SignedOperation;

use crate::{batch::payload::BatchPayload, config::Config, context::TezosContext, protocol::constants::Constants, Result};

pub fn initialize<C: Config>(context: &mut impl TezosContext, chain_id: ChainId) -> Result<()> {
    let mut head = context.get_head()?;

    if head.chain_id != chain_id {
        head.chain_id = chain_id;
        context.set_head(head.clone())?;
        context.commit()?;
    }

    Ok(())
}

pub fn inject_operation<C: Config>(context: &mut impl TezosContext, operation: SignedOperation) -> Result<()> {
    // TODO: validate and add operations to mempool
    Ok(())
}

pub fn inject_batch<C: Config>(context: &mut impl TezosContext, batch: BatchPayload) -> Result<()> {
    // remove included mempool operations
    // validate and execute batch
    Ok(())
}

pub fn finalize<C: Config>(context: &mut impl TezosContext, timestamp: i64) -> Result<()> {
    let mut head = context.get_head()?;

    let constants = C::Constants::constants();
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
