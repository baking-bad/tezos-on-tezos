// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use actix_web::web::Bytes;
use async_trait::async_trait;
use michelson_vm::entrypoints::collect_entrypoints;
use std::collections::{HashMap, VecDeque};
use tezos_core::types::encoded::{
    Address, BlockHash, ContractAddress, Encoded, ImplicitAddress, OperationHash, PublicKey,
    ScriptExprHash,
};
use tezos_core::types::{mutez::Mutez, number::Nat};
use tezos_michelson::micheline::Micheline;
use tezos_michelson::michelson::types::Type;
use tezos_rpc::models::{
    block::{Block, FullHeader, Metadata},
    contract::{ContractEntrypoints, ContractInfo, ContractScript},
    operation::Operation,
};
use tokio::sync::mpsc::Receiver;

use crate::{
    rollup::{BlockId, BlockProtocols, RollupClient, TezosFacade},
    Error, Result,
};

#[async_trait]
impl<T: RollupClient + Send + Sync> TezosFacade for T {
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
        let balance: Mutez = match self
            .store_get(
                format!("/context/contracts/{}/balance", address.value()),
                block_id,
            )
            .await
        {
            Ok(val) => val,
            Err(Error::KeyNotFound { key: _ }) => 0u32.into(),
            Err(err) => return Err(err),
        };
        Ok(balance)
    }

    async fn get_contract_counter(
        &self,
        block_id: &BlockId,
        address: &ImplicitAddress,
    ) -> Result<Nat> {
        let counter: Nat = match self
            .store_get(
                format!("/context/contracts/{}/counter", address.value()),
                block_id,
            )
            .await
        {
            Ok(val) => val,
            Err(Error::KeyNotFound { key: _ }) => 0u32.into(),
            Err(err) => return Err(err),
        };
        Ok(counter)
    }

    async fn get_contract_public_key(
        &self,
        block_id: &BlockId,
        address: &ImplicitAddress,
    ) -> Result<Option<PublicKey>> {
        let pubkey: Option<PublicKey> = match self
            .store_get(
                format!("/context/contracts/{}/pubkey", address.value()),
                block_id,
            )
            .await
        {
            Ok(val) => Some(val),
            Err(Error::KeyNotFound { key: _ }) => None,
            Err(err) => return Err(err),
        };
        Ok(pubkey)
    }

    async fn get_contract_code(
        &self,
        block_id: &BlockId,
        address: &ContractAddress,
    ) -> Result<Micheline> {
        let script: Micheline = self
            .store_get(
                format!("/context/contracts/{}/code", address.value()),
                block_id,
            )
            .await?;
        Ok(script)
    }

    async fn get_contract_storage(
        &self,
        block_id: &BlockId,
        address: &ContractAddress,
    ) -> Result<Micheline> {
        let storage: Micheline = self
            .store_get(
                format!("/context/contracts/{}/storage", address.value()),
                block_id,
            )
            .await?;
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
                (Some(counter), None)
            }
            Address::Originated(kt) => {
                let script = self.get_contract_script(block_id, kt).await?;
                (None, Some(script))
            }
        };
        Ok(ContractInfo {
            balance,
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
            .store_get(
                format!("/context/contracts/{}/entrypoints", address.value()),
                block_id,
            )
            .await?;

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
            .store_get(
                format!(
                    "/context/bigmaps/{}/values/{}",
                    big_map_id,
                    key_hash.value()
                ),
                block_id,
            )
            .await?;
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

    async fn get_live_blocks(&self, _block_id: &BlockId) -> Result<VecDeque<BlockHash>> {
        let live_blocks_ptr = self.get_ttl_blocks().unwrap();
        let live_blocks = live_blocks_ptr.lock().unwrap();
        // TODO: remove hashes after block_id?
        //let block_hash = self.get_block_hash(block_id).await.unwrap();
        Ok(live_blocks.clone())
    }

    async fn get_heads_main_channel(&self) -> Result<Receiver<Result<Bytes>>> {
        self.create_channel()
    }
}
