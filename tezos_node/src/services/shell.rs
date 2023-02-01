use actix_web::{
    http::StatusCode,
    web::{Data, Json},
    HttpResponse, Responder, Result,
};
use hex;

use crate::{
    rollup::{RollupClient, TezosFacade},
    Error,
};

pub async fn chain_id<T: RollupClient>(client: Data<T>) -> Result<impl Responder> {
    let value = client.get_chain_id().await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

pub async fn inject_operation<T: TezosFacade>(
    client: Data<T>,
    request: Json<String>,
) -> Result<impl Responder> {
    let payload = hex::decode(request.0).map_err(Error::from)?;
    let value = client.inject_operation(payload).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}
