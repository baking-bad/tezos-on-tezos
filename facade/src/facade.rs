use context::{ContextNode, BatchReceipt, BatchHeader, Head};
use tezos_core::types::{
    encoded::{ContractAddress, PublicKey, ScriptExprHash, BlockHash},
    mutez::Mutez,
    number::Nat,
};
use tezos_rpc::models::{
    operation::Operation,
    block::{Block, FullHeader, Header, Metadata}
};
use tezos_michelson::micheline::Micheline;

use crate::{
    client::{RollupClient, BlockId},
    Result
};

impl RollupClient {
    pub async fn get_context_node(&self, key: String, block_id: BlockId) -> Result<ContextNode> {
        // TODO: convert block_id, assuming head only for now
        let value = self.get_state_value(key, block_id).await?;
        Ok(ContextNode::from_vec(value)?)
    }

    pub async fn get_block_hash(&self, block_id: BlockId) -> Result<BlockHash> {
        let head: Head = self.get_context_node("/head".into(), block_id)
            .await?
            .try_into()?;
        Ok(head.hash)
    }
}