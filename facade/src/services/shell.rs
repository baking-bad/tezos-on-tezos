use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{Data, Json},
    HttpResponse, Responder, Result,
};
use hex;

use crate::{
    rollup::{RollupClient, TezosFacade},
    Client, Error,
};

#[get("/chains/main/chain_id")]
async fn chain_id(client: Data<Client>) -> Result<impl Responder> {
    let value = client.get_chain_id().await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[post("/chains/main/injection/operation")]
async fn inject_operation(client: Data<Client>, request: Json<String>) -> Result<impl Responder> {
    let payload = hex::decode(request.0).map_err(Error::from)?;
    let value = client.inject_operation(payload).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}
