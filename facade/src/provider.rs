use std::fmt::Display;

use async_trait::async_trait;
use ibig::IBig;
use tezos_rpc::models::{
    block::{Block, FullHeader, Metadata as BlockMetadata},
    operation::Operation, 
    contract::{ContractScript, ContractInfo, ContractEntrypoints},
    constants::Constants,
};
use tezos_core::types::{
    encoded::{OperationHash, BlockHash, Address, ImplicitAddress, ChainId, ScriptExprHash, Encoded},
};
use tezos_michelson::micheline::Micheline;

use crate::error::{Result, Error};

#[derive(Debug, Clone, PartialEq)]
pub enum BlockId {
    Head,
    Genesis,
    Hash(BlockHash),
    Level(i32),
}

#[async_trait]
pub trait RPCProvider {
    // async fn get_contract(&self, block_id: BlockId, address: Address) -> Result<ContractInfo>;
    // async fn get_contract_balance(&self, block_id: BlockId, address: Address) -> Result<IBig>;
    // async fn get_contract_counter(&self, block_id: BlockId, address: Address) -> Result<IBig>;
    // async fn get_contract_delegate(&self, block_id: BlockId, address: Address) -> Result<Option<ImplicitAddress>>;
    // async fn get_contract_script(&self, block_id: BlockId, address: Address) -> Result<ContractScript>;
    // async fn get_contract_storage(&self, block_id: BlockId, address: Address) -> Result<Micheline>;
    // async fn get_contract_entrypoints(&self, block_id: BlockId, address: Address) -> Result<ContractEntrypoints>;
    // async fn get_big_map_value(&self, block_id: BlockId, id: i64, key_hash: ScriptExprHash) -> Result<Micheline>;
    // async fn get_block(&self, block_id: BlockId) -> Result<Block>;
    async fn get_block_hash(&self, block_id: BlockId) -> Result<BlockHash>;
    // async fn get_block_header(&self, block_id: BlockId) -> Result<FullHeader>;
    // async fn get_block_metadata(&self, block_id: BlockId) -> Result<BlockMetadata>;
    // async fn get_block_protocols(&self, block_id: BlockId) -> Result<()>;
    // async fn get_block_branch(&self, block_id: BlockId) -> Result<Vec<BlockHash>>;
    // async fn get_operation_list_list(&self, block_id: BlockId) -> Result<Vec<Vec<Operation>>>;
    // async fn get_operation_list(&self, block_id: BlockId, pass: u32) -> Result<Vec<Operation>>;
    // async fn get_operation(&self, block_id: BlockId, pass: u32, index: u32) -> Result<Operation>;
    // async fn get_operation_hashes_list_list(&self, block_id: BlockId) -> Result<Vec<Vec<OperationHash>>>;
    // async fn get_operation_hashes_list(&self, block_id: BlockId, pass: u32) -> Result<Vec<OperationHash>>;
    // async fn get_operation_hash(&self, block_id: BlockId, pass: u32, index: u32) -> Result<OperationHash>;
    // async fn get_protocol_constants(&self, block_id: BlockId) -> Result<Constants>;
    // async fn get_chain_id(&self) -> Result<ChainId>;
}

impl TryFrom<&str> for BlockId {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "head" => Ok(Self::Head),
            "genesis" => Ok(Self::Genesis),
            hash if hash.len() == 51 => Ok(Self::Hash(hash.try_into()?)),
            level => Ok(Self::Level(i32::from_str_radix(level, 10)?))
        }
    }
}

impl Display for BlockId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Genesis => f.write_str("genesis"),
            Self::Head => f.write_str("head"),
            Self::Hash(hash) => f.write_str(hash.value()),
            Self::Level(level) => f.write_str(&level.to_string())
        }
    }
}