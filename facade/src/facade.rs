use serde_json;
use actix_web::{
    get,
    Responder,
    Result,
    web::{Path, Data},
    error::{ErrorInternalServerError, ErrorNotFound, ErrorBadRequest}
};

use crate::provider::RPCProvider;

#[get("/chains/main/chain_id")]
async fn chain_id() -> Result<impl Responder> {
    Ok(format!("todo"))
}

// #[get("/chains/main/blocks/{block_id}")]
// async fn block(provider: Data<dyn RPCProvider>, path: Path<(String,)>) -> Result<impl Responder> {
//     let block_id = path.0.as_str().try_into().map_err(ErrorBadRequest)?;
//     match provider.get_block(block_id).await {
//         Ok(value) => serde_json::to_string(&value).map_err(ErrorInternalServerError),
//         Err(err) => Err(ErrorInternalServerError(err))
//     }
// }

#[get("/chains/main/blocks/{block_id}/hash")]
async fn block_hash(provider: Data<dyn RPCProvider>, path: Path<(String,)>) -> Result<impl Responder> {
    let block_id = path.0.as_str().try_into().map_err(ErrorBadRequest)?;
    match provider.get_block_hash(block_id).await {
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
