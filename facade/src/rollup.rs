use serde::Serialize;
use async_trait::async_trait;
use context::{BatchReceipt, Head, ContextNode};
use tezos_vm::entrypoints::collect_entrypoints;
use tezos_core::types::{
    encoded::{
        ContractAddress, ImplicitAddress, PublicKey, OperationHash, ScriptExprHash, 
        Address, ChainId, BlockHash, ProtocolHash, Encoded
    },
    mutez::Mutez,
    number::Nat,
};
use tezos_rpc::models::{
    operation::{Operation},
    block::{Block, FullHeader, Metadata},
    contract::{ContractScript, ContractInfo, ContractEntrypoints}
};
use tezos_michelson::{
    micheline::Micheline,
    michelson::types::Type,
};
use ibig::IBig;
use std::collections::HashMap;

use crate::{
    Result,
    Error,
};

#[derive(Debug, Clone, PartialEq)]
pub enum BlockId {
    Head,
    Genesis,
    Level(i32),
    Hash(BlockHash)
}

impl TryFrom<&str> for BlockId {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "head" => Ok(Self::Head),
            "genesis" => Ok(Self::Genesis),
            hash if hash.len() == 51 => {
                match BlockHash::new(hash.into()) {
                    Ok(value) => Ok(Self::Hash(value)),
                    Err(err) => Err(Error::InvalidArguments { message: err.to_string() })
                }
            },
            level => {
                match i32::from_str_radix(level, 10) {
                    Ok(value) => Ok(Self::Level(value)),
                    Err(err) => Err(Error::InvalidArguments { message: err.to_string() })
                }
            }
        } 
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BlockProtocols {
    pub protocol: ProtocolHash,
    pub next_protocol: ProtocolHash
}

#[async_trait]
pub trait RollupClient {
    async fn get_state_value(&self, key: String, block_id: &BlockId) -> Result<ContextNode>;

    async fn get_block_hash(&self, block_id: &str) -> Result<BlockHash> {
        let block_id = BlockId::try_from(block_id)?;
        if let BlockId::Hash(hash) = block_id {
            return Ok(hash)
        }

        let head: Head = self
            .get_state_value("/head".into(), &block_id)
            .await?
            .try_into()?;
        Ok(head.hash)
    }

    async fn get_batch_receipt(&self, block_id: &str) -> Result<BatchReceipt> {
        let block_hash = self.get_block_hash(block_id).await?;
        let receipt: BatchReceipt = self
            .get_state_value(format!("/batches/{}", block_hash.value()), &BlockId::Head)
            .await?
            .try_into()?;
        Ok(receipt.into())
    }  

    async fn get_block_header(&self, block_id: &str) -> Result<FullHeader> {
        let receipt = self.get_batch_receipt(block_id).await?;
        Ok(receipt.into())
    }

    async fn get_block_metadata(&self, block_id: &str) -> Result<Metadata> {
        let receipt = self.get_batch_receipt(block_id).await?;
        Ok(receipt.into())
    }

    async fn get_block_protocols(&self, block_id: &str) -> Result<BlockProtocols> {
        let receipt = self.get_batch_receipt(block_id).await?;
        Ok(BlockProtocols {
            protocol: receipt.protocol.clone(),
            next_protocol: receipt.protocol
        })
    }

    async fn get_chain_id(&self) -> Result<ChainId> {
        let receipt = self.get_batch_receipt("head").await?;
        Ok(receipt.chain_id)
    }

    async fn get_contract_balance(&self, block_id: &str, address: &str) -> Result<Mutez> {
        let contract: Address = address.try_into()?;
        let balance: Mutez = self
            .get_state_value(
                format!("/context/contracts/{}/balance", contract.value()), 
                &block_id.try_into()?
            )
            .await?
            .try_into()?;
        Ok(balance)
    }

    async fn get_contract_counter(&self, block_id: &str, address: &str) -> Result<Nat> {
        let contract: Address = address.try_into()?;
        let counter: Nat = self
            .get_state_value(
                format!("/context/contracts/{}/counter", contract.value()), 
                &block_id.try_into()?
            )
            .await?
            .try_into()?;
        Ok(counter)
    }

    async fn get_contract_public_key(&self, block_id: &str, address: &str) -> Result<PublicKey> {
        let address: ImplicitAddress = address.try_into()?;
        let pubkey: PublicKey = self
            .get_state_value(
                format!("/context/contracts/{}/pubkey", address.value()), 
                &block_id.try_into()?
            )
            .await?
            .try_into()?;
        Ok(pubkey)
    }

    async fn get_contract_code(&self, block_id: &str, address: &str) -> Result<Micheline> {
        let contract: ContractAddress = address.try_into()?;
        let script: Micheline = self
            .get_state_value(
                format!("/context/contracts/{}/code", contract.value()), 
                &block_id.try_into()?
            )
            .await?
            .try_into()?;
        Ok(script)
    }

    async fn get_contract_storage(&self, block_id: &str, address: &str) -> Result<Micheline> {
        let contract: ContractAddress = address.try_into()?;
        let storage: Micheline = self
            .get_state_value(
                format!("/context/contracts/{}/storage", contract.value()), 
                &block_id.try_into()?
            )
            .await?
            .try_into()?;
        Ok(storage)
    }

    async fn get_contract_script(&self, block_id: &str, address: &str) -> Result<ContractScript> {
        let code = self.get_contract_code(block_id, address).await?;
        let storage = self.get_contract_storage(block_id, address).await?;
        Ok(ContractScript { code: code.try_into()?, storage })
    }

    async fn get_contract(&self, block_id: &str, address: &str) -> Result<ContractInfo> {
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

    async fn get_contract_entrypoints(&self, block_id: &str, address: &str) -> Result<ContractEntrypoints> {
        let contract: ContractAddress = address.try_into()?;
        let value: Micheline = self
            .get_state_value(
                format!("/context/contracts/{}/entrypoints", contract.value()), 
                &block_id.try_into()?
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

    async fn get_big_map_value(&self, block_id: &str, big_map_id: i64, key_hash: &str) -> Result<Micheline> {
        let key_hash: ScriptExprHash = key_hash.try_into()?;
        let value: Micheline = self
            .get_state_value(
                format!("/context/big_maps/{}/values/{}", big_map_id, key_hash.value()), 
                &block_id.try_into()?
            )
            .await?
            .try_into()?;
        Ok(value)
    }

    async fn get_operation_hash_list(&self, block_id: &str, pass: i32) -> Result<Vec<OperationHash>> {
        if pass != 3 {
            return Err(Error::KeyNotFound { key: format!("/blocks/{}/operations/{}", block_id, pass) })
        }
        let head: Head = self
            .get_state_value("/head".into(), &block_id.try_into()?)
            .await?
            .try_into()?;
        Ok(head.operations)
    }

    async fn get_operation_hash_list_list(&self, block_id: &str) -> Result<Vec<Vec<OperationHash>>> {
        let managers = self.get_operation_hash_list(block_id, 3).await?;
        Ok(vec![vec![], vec![], vec![], managers])
    }

    async fn get_operation_hash(&self, block_id: &str, pass: i32, index: i32) -> Result<OperationHash> {
        let index: usize = index.try_into()?;
        let mut hash_list = self.get_operation_hash_list(block_id, pass).await?;
        if index >= hash_list.len() {
            return Err(Error::InvalidArguments { message: format!("Index out of bounds ({}, {})", index, hash_list.len()) })
        }
        Ok(hash_list.remove(index))
    }

    async fn get_operation_receipt(&self, hash: &OperationHash) -> Result<Operation> {
        let operation: Operation = self
            .get_state_value(format!("/operations/{}", hash.value()), &BlockId::Head)
            .await?
            .try_into()?;
        Ok(operation)
    }

    async fn get_operation(&self, block_id: &str, pass: i32, index: i32) -> Result<Operation> {
        let hash = self.get_operation_hash(block_id, pass, index).await?;
        let operation = self.get_operation_receipt(&hash).await?;
        Ok(operation)
    }

    async fn get_operation_list(&self, block_id: &str, pass: i32) -> Result<Vec<Operation>> {
        let mut hash_list = self.get_operation_hash_list(block_id, pass).await?;
        let mut operations: Vec<Operation> = Vec::with_capacity(hash_list.len());
        for hash in hash_list.drain(..) {
            let operation = self.get_operation_receipt(&hash).await?;
            operations.push(operation);
        }
        Ok(operations)
    }

    async fn get_operation_list_list(&self, block_id: &str) -> Result<Vec<Vec<Operation>>> {
        let managers = self.get_operation_list(block_id, 3).await?;
        Ok(vec![vec![], vec![], vec![], managers])
    }

    async fn get_block(&self, block_id: &str) -> Result<Block> {
        let receipt = self.get_batch_receipt(block_id).await?;
        Ok(Block {
            hash: receipt.hash.clone(),
            chain_id: receipt.chain_id.clone(),
            protocol: receipt.protocol.clone(),
            header: receipt.header.clone().into(),
            metadata: Some(receipt.into()),
            operations: self.get_operation_list_list(block_id).await?
        })
    }

    async fn get_live_blocks(&self, block_id: &str) -> Result<Vec<BlockHash>> {
        let receipt = self.get_batch_receipt(block_id).await?;
        // TODO: ttl blocks
        Ok(vec![receipt.header.predecessor])
    }
}
