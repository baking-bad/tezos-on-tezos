pub use tezos_core::types::{
    encoded::{
        OperationHash,
        OperationListListHash,
        BlockPayloadHash,
        BlockHash,
        Signature,
        Encoded
    },
    hex_string::HexString
};
pub use tezos_operation::{
    block_header::{BlockHeader, LiquidityBakingToggleVote},
    block_header
};
pub use tezos_rpc::models::{
    block::{
        Metadata as BlockMetadata,
        FullHeader
    },
    balance_update::BalanceUpdate,
    balance_update
};

use crate::{
    error::Result,
    context::Context,
    validator::ManagerOperation,
    executor::execute_operation
};

pub fn execute_block(
    context: &mut impl Context,
    header: BlockHeader,
    operations: Vec<ManagerOperation>,
    implicit_updates: Option<Vec<BalanceUpdate>>
) -> Result<BlockHash> {
    todo!()
}