// fn get_contract(&self, block_id: BlockId, address: Address) -> Result<ContractInfo>;
// fn get_contract_balance(&self, block_id: BlockId, address: Address) -> Result<IBig>;
// fn get_contract_counter(&self, block_id: BlockId, address: Address) -> Result<IBig>;
// fn get_contract_delegate(&self, block_id: BlockId, address: Address) -> Result<Option<ImplicitAddress>>;
// fn get_contract_script(&self, block_id: BlockId, address: Address) -> Result<ContractScript>;
// fn get_contract_storage(&self, block_id: BlockId, address: Address) -> Result<Micheline>;
// fn get_contract_entrypoints(&self, block_id: BlockId, address: Address) -> Result<ContractEntrypoints>;
// fn get_big_map_value(&self, block_id: BlockId, id: i64, key_hash: ScriptExprHash) -> Result<Micheline>;
// fn get_block(&self, block_id: BlockId) -> Result<Block>;
// fn get_block_hash(&mut self, block_id: BlockId) -> Result<Option<BlockHash>>;
// fn get_block_header(&self, block_id: BlockId) -> Result<FullHeader>;
// fn get_block_metadata(&self, block_id: BlockId) -> Result<BlockMetadata>;
// fn get_block_protocols(&self, block_id: BlockId) -> Result<()>;
// fn get_block_branch(&self, block_id: BlockId) -> Result<Vec<BlockHash>>;
// fn get_operation_list_list(&self, block_id: BlockId) -> Result<Vec<Vec<Operation>>>;
// fn get_operation_list(&self, block_id: BlockId, pass: u32) -> Result<Vec<Operation>>;
// fn get_operation(&self, block_id: BlockId, pass: u32, index: u32) -> Result<Operation>;
// fn get_operation_hashes_list_list(&self, block_id: BlockId) -> Result<Vec<Vec<OperationHash>>>;
// fn get_operation_hashes_list(&self, block_id: BlockId, pass: u32) -> Result<Vec<OperationHash>>;
// fn get_operation_hash(&self, block_id: BlockId, pass: u32, index: u32) -> Result<OperationHash>;
// fn get_protocol_constants(&self, block_id: BlockId) -> Result<Constants>;
// fn get_chain_id(&self) -> Result<ChainId>;

use serde_json;
use actix_web::{
    get,
    Responder,
    Result,
    web::{Path, Data},
    error::{ErrorInternalServerError, ErrorNotFound, ErrorBadRequest}
};

use crate::{
    client::RollupClient,
};

#[get("/chains/main/chain_id")]
async fn chain_id() -> Result<impl Responder> {
    Ok(format!("todo"))
}

#[get("/chains/main/blocks/{block_id}/hash")]
async fn block_hash(client: Data<RollupClient>, path: Path<(String,)>) -> Result<impl Responder> {
    let block_id = path.0.as_str().try_into().map_err(ErrorBadRequest)?;
    match client.get_block_hash(block_id).await {
        Ok(value) => serde_json::to_string(&value).map_err(ErrorInternalServerError),
        Err(err) => Err(ErrorInternalServerError(err))
    }
}

#[get("/chains/main/blocks/{block_id}/context/delegates")]
async fn delegates() -> Result<impl Responder> {
    Ok(vec![])
}

#[get("/chains/main/blocks/{block_id}/context/delegates/{delegate_id}")]
async fn delegate() -> Result<String> {
    Err(ErrorNotFound(format!("No delegates")))
}