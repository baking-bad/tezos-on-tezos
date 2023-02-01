use actix_web::{
    http::StatusCode,
    web::{Data, Path},
    HttpResponse, Responder, Result,
};
use context::Config;
use tezos_core::types::encoded::ScriptExprHash;

use crate::{rollup::TezosFacade, Error};

pub async fn constants() -> Result<impl Responder> {
    Ok(HttpResponse::build(StatusCode::OK).json(Config::default()))
}

pub async fn delegates() -> Result<impl Responder> {
    Ok(HttpResponse::build(StatusCode::OK).json(Vec::<String>::new()))
}

pub async fn delegate() -> Result<impl Responder> {
    Ok(HttpResponse::build(StatusCode::NOT_FOUND).finish())
}

pub async fn big_map_value<T: TezosFacade>(client: Data<T>, path: Path<(String, i64, String)>) -> Result<impl Responder> {
    let key_hash: ScriptExprHash = path.2.as_str().try_into().map_err(Error::from)?;
    let value = client
        .get_big_map_value(&path.0.as_str().try_into()?, path.1, &key_hash)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}
