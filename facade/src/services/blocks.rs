use actix_web::{
    get,
    http::StatusCode,
    web::{Data, Path},
    HttpResponse, Responder, Result,
};

use crate::{rollup::TezosFacade, Client};

#[get("/chains/main/blocks/{block_id}/hash")]
async fn block_hash(client: Data<Client>, path: Path<(String,)>) -> Result<impl Responder> {
    let value = client.get_block_hash(&path.0.as_str().try_into()?).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/header")]
async fn block_header(client: Data<Client>, path: Path<(String,)>) -> Result<impl Responder> {
    let value = client
        .get_block_header(&path.0.as_str().try_into()?)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/metadata")]
async fn block_metadata(client: Data<Client>, path: Path<(String,)>) -> Result<impl Responder> {
    let value = client
        .get_block_metadata(&path.0.as_str().try_into()?)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/protocols")]
async fn block_protocols(client: Data<Client>, path: Path<(String,)>) -> Result<impl Responder> {
    let value = client
        .get_block_protocols(&path.0.as_str().try_into()?)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/live_blocks")]
async fn live_blocks(client: Data<Client>, path: Path<(String,)>) -> Result<impl Responder> {
    let value = client.get_live_blocks(&path.0.as_str().try_into()?).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}")]
async fn block(client: Data<Client>, path: Path<(String,)>) -> Result<impl Responder> {
    let value = client.get_block(&path.0.as_str().try_into()?).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}
