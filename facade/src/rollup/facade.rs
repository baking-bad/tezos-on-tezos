use async_trait::async_trait;
use ibig::IBig;
use std::collections::HashMap;
use tezos_core::types::encoded::{
    Address, BlockHash, ContractAddress, Encoded, ImplicitAddress, 
    OperationHash, PublicKey, ScriptExprHash, Signature,
};
use tezos_core::types::{mutez::Mutez, number::Nat};
use tezos_michelson::micheline::Micheline;
use tezos_michelson::michelson::types::Type;
use tezos_operation::operations::{SignedOperation, UnsignedOperation};
use tezos_rpc::models::{
    block::{Block, FullHeader, Metadata},
    contract::{ContractEntrypoints, ContractInfo, ContractScript},
    operation::Operation,
};
use tezos_vm::entrypoints::collect_entrypoints;

use crate::{
    rollup::{BlockId, BlockProtocols, RollupClient, TezosFacade},
    Error, Result,
};

pub fn parse_operation(payload: &[u8]) -> Result<OperationHash> {
    const SIGNATURE_SIZE: usize = 64;
    if payload.len() < SIGNATURE_SIZE {
        return Err(Error::InvalidArguments {
            message: format!("Payload too short"),
        });
    }

    // validate encoding
    UnsignedOperation::from_forged_bytes(&payload[..payload.len() - SIGNATURE_SIZE])?;
    Signature::from_bytes(&payload[payload.len() - SIGNATURE_SIZE..])?;

    let hash = SignedOperation::operation_hash(payload)?;
    Ok(hash)
}

#[async_trait]
impl<T: RollupClient + Sync + Send> TezosFacade for T {
    async fn get_block_hash(&self, block_id: &BlockId) -> Result<BlockHash> {
        match block_id {
            BlockId::Hash(hash) => Ok(hash.clone()),
            _ => Ok(self.get_batch_head(block_id).await?.hash),
        }
    }

    async fn get_block_header(&self, block_id: &BlockId) -> Result<FullHeader> {
        let receipt = self.get_batch_receipt(block_id).await?;
        Ok(receipt.into())
    }

    async fn get_block_metadata(&self, block_id: &BlockId) -> Result<Metadata> {
        let receipt = self.get_batch_receipt(block_id).await?;
        Ok(receipt.into())
    }

    async fn get_block_protocols(&self, block_id: &BlockId) -> Result<BlockProtocols> {
        let receipt = self.get_batch_receipt(block_id).await?;
        Ok(BlockProtocols {
            protocol: receipt.protocol.clone(),
            next_protocol: receipt.protocol,
        })
    }

    async fn get_contract_balance(&self, block_id: &BlockId, address: &Address) -> Result<Mutez> {
        let balance: Mutez = self
            .get_state_value(
                format!("/context/contracts/{}/balance", address.value()),
                block_id,
            )
            .await?
            .try_into()?;
        Ok(balance)
    }

    async fn get_contract_counter(
        &self,
        block_id: &BlockId,
        address: &ImplicitAddress,
    ) -> Result<Nat> {
        let counter: Nat = self
            .get_state_value(
                format!("/context/contracts/{}/counter", address.value()),
                block_id,
            )
            .await?
            .try_into()?;
        Ok(counter)
    }

    async fn get_contract_public_key(
        &self,
        block_id: &BlockId,
        address: &ImplicitAddress,
    ) -> Result<PublicKey> {
        let pubkey: PublicKey = self
            .get_state_value(
                format!("/context/contracts/{}/pubkey", address.value()),
                block_id,
            )
            .await?
            .try_into()?;
        Ok(pubkey)
    }

    async fn get_contract_code(
        &self,
        block_id: &BlockId,
        address: &ContractAddress,
    ) -> Result<Micheline> {
        let script: Micheline = self
            .get_state_value(
                format!("/context/contracts/{}/code", address.value()),
                block_id,
            )
            .await?
            .try_into()?;
        Ok(script)
    }

    async fn get_contract_storage(
        &self,
        block_id: &BlockId,
        address: &ContractAddress,
    ) -> Result<Micheline> {
        let storage: Micheline = self
            .get_state_value(
                format!("/context/contracts/{}/storage", address.value()),
                block_id,
            )
            .await?
            .try_into()?;
        Ok(storage)
    }

    async fn get_contract_script(
        &self,
        block_id: &BlockId,
        address: &ContractAddress,
    ) -> Result<ContractScript> {
        let code = self.get_contract_code(block_id, address).await?;
        let storage = self.get_contract_storage(block_id, address).await?;
        Ok(ContractScript {
            code: code.try_into()?,
            storage,
        })
    }

    async fn get_contract(&self, block_id: &BlockId, address: &Address) -> Result<ContractInfo> {
        let balance = self.get_contract_balance(block_id, address).await?;
        let (counter, script) = match address {
            Address::Implicit(tz) => {
                let counter = self.get_contract_counter(block_id, tz).await?;
                (Some(IBig::from_str_radix(counter.to_str(), 10)?), None)
            }
            Address::Originated(kt) => {
                let script = self.get_contract_script(block_id, kt).await?;
                (None, Some(script))
            }
        };
        Ok(ContractInfo {
            balance: IBig::from_str_radix(&balance.to_string(), 10)?,
            counter,
            script,
            delegate: None,
        })
    }

    async fn get_contract_entrypoints(
        &self,
        block_id: &BlockId,
        address: &ContractAddress,
    ) -> Result<ContractEntrypoints> {
        let value: Micheline = self
            .get_state_value(
                format!("/context/contracts/{}/entrypoints", address.value()),
                block_id,
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
                .collect(),
        })
    }

    async fn get_big_map_value(
        &self,
        block_id: &BlockId,
        big_map_id: i64,
        key_hash: &ScriptExprHash,
    ) -> Result<Micheline> {
        let value: Micheline = self
            .get_state_value(
                format!(
                    "/context/big_maps/{}/values/{}",
                    big_map_id,
                    key_hash.value()
                ),
                block_id,
            )
            .await?
            .try_into()?;
        Ok(value)
    }

    async fn get_operation_hash_list(
        &self,
        block_id: &BlockId,
        pass: i32,
    ) -> Result<Vec<OperationHash>> {
        if pass != 3 {
            return Err(Error::KeyNotFound {
                key: format!("/operations/{}", pass),
            });
        }
        let head = self.get_batch_head(block_id).await?;
        Ok(head.operations)
    }

    async fn get_operation_hash_list_list(
        &self,
        block_id: &BlockId,
    ) -> Result<Vec<Vec<OperationHash>>> {
        let managers = self.get_operation_hash_list(block_id, 3).await?;
        Ok(vec![vec![], vec![], vec![], managers])
    }

    async fn get_operation_hash(
        &self,
        block_id: &BlockId,
        pass: i32,
        index: i32,
    ) -> Result<OperationHash> {
        let index: usize = index.try_into()?;
        let mut hash_list = self.get_operation_hash_list(block_id, pass).await?;
        if index >= hash_list.len() {
            return Err(Error::InvalidArguments {
                message: format!("Index out of bounds ({}, {})", index, hash_list.len()),
            });
        }
        Ok(hash_list.remove(index))
    }

    async fn get_operation(&self, block_id: &BlockId, pass: i32, index: i32) -> Result<Operation> {
        let hash = self.get_operation_hash(block_id, pass, index).await?;
        let operation = self.get_operation_receipt(&hash).await?;
        Ok(operation)
    }

    async fn get_operation_list(&self, block_id: &BlockId, pass: i32) -> Result<Vec<Operation>> {
        let mut hash_list = self.get_operation_hash_list(block_id, pass).await?;
        let mut operations: Vec<Operation> = Vec::with_capacity(hash_list.len());
        for hash in hash_list.drain(..) {
            let operation = self.get_operation_receipt(&hash).await?;
            operations.push(operation);
        }
        Ok(operations)
    }

    async fn get_operation_list_list(&self, block_id: &BlockId) -> Result<Vec<Vec<Operation>>> {
        let managers = self.get_operation_list(block_id, 3).await?;
        Ok(vec![vec![], vec![], vec![], managers])
    }

    async fn get_block(&self, block_id: &BlockId) -> Result<Block> {
        let receipt = self.get_batch_receipt(block_id).await?;
        Ok(Block {
            hash: receipt.hash.clone(),
            chain_id: receipt.chain_id.clone(),
            protocol: receipt.protocol.clone(),
            header: receipt.header.clone().into(),
            metadata: Some(receipt.into()),
            operations: self.get_operation_list_list(block_id).await?,
        })
    }

    async fn get_live_blocks(&self, block_id: &BlockId) -> Result<Vec<BlockHash>> {
        let receipt = self.get_batch_receipt(block_id).await?;
        // TODO: ttl blocks
        Ok(vec![receipt.header.predecessor])
    }

    async fn inject_operation(&self, payload: Vec<u8>) -> Result<OperationHash> {
        let hash = parse_operation(payload.as_slice())?;
        let chain_id = self.get_chain_id().await?;
        let message = [chain_id.to_bytes()?, payload].concat();
        self.inject_batch(vec![message]).await?;
        Ok(hash)
    }
}
