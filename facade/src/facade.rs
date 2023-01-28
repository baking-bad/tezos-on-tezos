use context::{ContextNode, BatchReceipt, BatchHeader, Head};
use tezos_vm::entrypoints::collect_entrypoints;
use serde::Serialize;
use tezos_core::types::{
    encoded::{ContractAddress, PublicKey, ScriptExprHash, Address, ChainId, BlockHash, ProtocolHash, Encoded},
    mutez::Mutez,
    number::Nat,
};
use tezos_rpc::models::{
    operation::Operation,
    block::{Block, FullHeader, Header, Metadata},
    contract::{ContractScript, ContractInfo, ContractEntrypoints}
};
use tezos_michelson::{
    micheline::Micheline,
    michelson::types::Type,
};
use ibig::IBig;
use std::collections::HashMap;

use crate::{
    client::{RollupClient, BlockId},
    Error,
    Result
};

// fn get_contract_entrypoints(&self, block_id: BlockId, address: Address) -> Result<ContractEntrypoints>;
// fn get_big_map_value(&self, block_id: BlockId, id: i64, key_hash: ScriptExprHash) -> Result<Micheline>;
// fn get_block(&self, block_id: BlockId) -> Result<Block>;
// fn get_block_branch(&self, block_id: BlockId) -> Result<Vec<BlockHash>>;
// fn get_operation_list_list(&self, block_id: BlockId) -> Result<Vec<Vec<Operation>>>;
// fn get_operation_list(&self, block_id: BlockId, pass: u32) -> Result<Vec<Operation>>;
// fn get_operation(&self, block_id: BlockId, pass: u32, index: u32) -> Result<Operation>;
// fn get_operation_hashes_list_list(&self, block_id: BlockId) -> Result<Vec<Vec<OperationHash>>>;
// fn get_operation_hashes_list(&self, block_id: BlockId, pass: u32) -> Result<Vec<OperationHash>>;
// fn get_operation_hash(&self, block_id: BlockId, pass: u32, index: u32) -> Result<OperationHash>;
#[derive(Debug, Clone, Serialize)]
pub struct BlockProtocols {
    pub protocol: ProtocolHash,
    pub next_protocol: ProtocolHash
}

impl RollupClient {
    pub async fn get_context_node(&self, key: String, block_id: BlockId) -> Result<ContextNode> {
        // TODO: convert block_id, assuming head only for now
        let value = self.get_state_value(key, block_id).await?;
        Ok(ContextNode::from_vec(value)?)
    }

    pub async fn get_block_hash(&self, block_id: &str) -> Result<BlockHash> {
        let receipt: BatchReceipt = self
            .get_context_node("/batch".into(), block_id.try_into()?)
            .await?
            .try_into()?;
        Ok(receipt.hash)
    }

    pub async fn get_block_header(&self, block_id: &str) -> Result<FullHeader> {
        let receipt: BatchReceipt = self
            .get_context_node("/batch".into(), block_id.try_into()?)
            .await?
            .try_into()?;
        Ok(receipt.into())
    }

    pub async fn get_block_metadata(&self, block_id: &str) -> Result<Metadata> {
        let receipt: BatchReceipt = self
            .get_context_node("/batch".into(), block_id.try_into()?)
            .await?
            .try_into()?;
        Ok(receipt.into())
    }

    pub async fn get_block_protocols(&self, block_id: &str) -> Result<BlockProtocols> {
        let receipt: BatchReceipt = self
            .get_context_node("/batch".into(), block_id.try_into()?)
            .await?
            .try_into()?;

        Ok(BlockProtocols {
            protocol: receipt.protocol.clone(),
            next_protocol: receipt.protocol
        })
    }

    pub async fn get_chain_id(&self) -> Result<ChainId> {
        let receipt: BatchReceipt = self
            .get_context_node("/batch".into(), BlockId::Head)
            .await?
            .try_into()?;
        Ok(receipt.chain_id)
    }

    pub async fn get_contract_balance(&self, block_id: &str, address: &str) -> Result<Mutez> {
        let contract: Address = address.try_into()?;
        let balance: Mutez = self
            .get_context_node(
                format!("/context/contracts/{}/balance", contract.value()), 
                block_id.try_into()?
            )
            .await?
            .try_into()?;
        Ok(balance)
    }

    pub async fn get_contract_counter(&self, block_id: &str, address: &str) -> Result<Nat> {
        let contract: Address = address.try_into()?;
        let counter: Nat = self
            .get_context_node(
                format!("/context/contracts/{}/counter", contract.value()), 
                block_id.try_into()?
            )
            .await?
            .try_into()?;
        Ok(counter)
    }

    pub async fn get_contract_code(&self, block_id: &str, address: &str) -> Result<Micheline> {
        let contract: ContractAddress = address.try_into()?;
        let script: Micheline = self
            .get_context_node(
                format!("/context/contracts/{}/code", contract.value()), 
                block_id.try_into()?
            )
            .await?
            .try_into()?;
        Ok(script)
    }

    pub async fn get_contract_storage(&self, block_id: &str, address: &str) -> Result<Micheline> {
        let contract: ContractAddress = address.try_into()?;
        let storage: Micheline = self
            .get_context_node(
                format!("/context/contracts/{}/storage", contract.value()), 
                block_id.try_into()?
            )
            .await?
            .try_into()?;
        Ok(storage)
    }

    pub async fn get_contract_script(&self, block_id: &str, address: &str) -> Result<ContractScript> {
        let code = self.get_contract_code(block_id, address).await?;
        let storage = self.get_contract_storage(block_id, address).await?;
        Ok(ContractScript { code: code.try_into()?, storage })
    }

    pub async fn get_contract(&self, block_id: &str, address: &str) -> Result<ContractInfo> {
        let contract: Address = address.try_into()?;
        let balance = self.get_contract_balance(block_id, address).await?;

        let counter = match self.get_contract_counter(block_id, address).await {
            Ok(value) => Some(IBig::from_str_radix(value.to_str(), 10)?),
            Err(Error::KeyNotFound { key: _ }) => None,
            Err(err) => return Err(err.into())
        };

        let script = match &contract {
            Address::Implicit(_) => None,
            Address::Originated(_) => {
                let script = self.get_contract_script(block_id, address).await?;
                Some(script)
            }
        };

        Ok(ContractInfo {
            balance: IBig::from_str_radix(&balance.to_string(), 10)?,
            counter,
            delegate: None,
            script
        })
    }

    // TODO: store entrypoints in context
    pub async fn get_contract_entrypoints(&self, block_id: &str, address: &str) -> Result<ContractEntrypoints> {
        let contract: ContractAddress = address.try_into()?;
        let value: Micheline = self
            .get_context_node(
                format!("/context/contracts/{}/entrypoints", contract.value()), 
                block_id.try_into()?
            )
            .await?
            .try_into()?;

        let param_type: Type = value.try_into()?;
        let mut entrypoints: HashMap<String, Type> = HashMap::new();
        collect_entrypoints(param_type, &mut entrypoints, 0)?;

        Ok(ContractEntrypoints {
            entrypoints: entrypoints
                .into_iter()
                .map(|(k, v)| (k, Micheline::from(v)))
                .collect()
        })
    }
}