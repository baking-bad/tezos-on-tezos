use actix_web::{
    get,
    http::StatusCode,
    web::{Data, Path},
    HttpResponse, Responder, Result,
};
use context::Config;
use tezos_core::types::encoded::ScriptExprHash;

use crate::{rollup::TezosFacade, Client, Error};

#[get("/chains/main/blocks/{block_id}/context/constants")]
async fn constants(_client: Data<Client>, _path: Path<(String,)>) -> Result<impl Responder> {
    Ok(HttpResponse::build(StatusCode::OK).json(Config::default()))
}

#[get("/chains/main/blocks/{block_id}/context/delegates")]
async fn delegates(_client: Data<Client>, _path: Path<(String,)>) -> Result<impl Responder> {
    Ok(HttpResponse::build(StatusCode::OK).json(Vec::<String>::new()))
}

#[get("/chains/main/blocks/{block_id}/context/delegates/{delegate_id}")]
async fn delegate(_client: Data<Client>, _path: Path<(String, String)>) -> Result<impl Responder> {
    Ok(HttpResponse::build(StatusCode::NOT_FOUND).finish())
}

#[get("/chains/main/blocks/{block_id}/context/big_maps/{big_map_id}/values/{key_hash}")]
async fn big_map_value(
    client: Data<Client>,
    path: Path<(String, i64, String)>,
) -> Result<impl Responder> {
    let key_hash: ScriptExprHash = path.2.as_str().try_into().map_err(Error::from)?;
    let value = client
        .get_big_map_value(&path.0.as_str().try_into()?, path.1, &key_hash)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}
