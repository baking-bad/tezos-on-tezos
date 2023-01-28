use actix_web::{
    get,
    Responder,
    Result,
    HttpResponse,
    web::{Path, Data},
    http::StatusCode,
};

use context::Config;

use crate::{
    client::RollupClient,
};

#[get("/chains/main/chain_id")]
async fn chain_id(client: Data<RollupClient>) -> Result<impl Responder> {
    let value = client.get_chain_id().await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/hash")]
async fn block_hash(client: Data<RollupClient>, path: Path<(String,)>) -> Result<impl Responder> {
    let value = client.get_block_hash(path.0.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/header")]
async fn block_header(client: Data<RollupClient>, path: Path<(String,)>) -> Result<impl Responder> {
    let value = client.get_block_header(path.0.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/metadata")]
async fn block_metadata(client: Data<RollupClient>, path: Path<(String,)>) -> Result<impl Responder> {
    let value = client.get_block_metadata(path.0.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/protocols")]
async fn block_protocols(client: Data<RollupClient>, path: Path<(String,)>) -> Result<impl Responder> {
    let value = client.get_block_protocols(path.0.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/context/constants")]
async fn constants(_client: Data<RollupClient>, _path: Path<(String,)>) -> Result<impl Responder> {
    Ok(HttpResponse::build(StatusCode::OK).json(Config::default()))
}

#[get("/chains/main/blocks/{block_id}/context/delegates")]
async fn delegates(_client: Data<RollupClient>, _path: Path<(String,)>) -> Result<impl Responder> {
    Ok(HttpResponse::build(StatusCode::OK).json(Vec::<String>::new()))
}

#[get("/chains/main/blocks/{block_id}/context/delegates/{delegate_id}")]
async fn delegate(_client: Data<RollupClient>, _path: Path<(String, String)>) -> Result<impl Responder> {
    Ok(HttpResponse::build(StatusCode::NOT_FOUND).finish())
}

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}/balance")]
async fn contract_balance(client: Data<RollupClient>, path: Path<(String, String)>) -> Result<impl Responder> {
    let value = client.get_contract_balance(path.0.as_str(), path.1.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}/counter")]
async fn contract_counter(client: Data<RollupClient>, path: Path<(String, String)>) -> Result<impl Responder> {
    let value = client.get_contract_counter(path.0.as_str(), path.1.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}/delegate")]
async fn contract_delegate(_client: Data<RollupClient>, _path: Path<(String, String)>) -> Result<impl Responder> {
    Ok(HttpResponse::build(StatusCode::NOT_FOUND).finish())
}

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}/storage")]
async fn contract_storage(client: Data<RollupClient>, path: Path<(String, String)>) -> Result<impl Responder> {
    let value = client.get_contract_storage(path.0.as_str(), path.1.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}/script")]
async fn contract_script(client: Data<RollupClient>, path: Path<(String, String)>) -> Result<impl Responder> {
    let value = client.get_contract_script(path.0.as_str(), path.1.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}/entrypoints")]
async fn contract_entrypoints(client: Data<RollupClient>, path: Path<(String, String)>) -> Result<impl Responder> {
    let value = client.get_contract_entrypoints(path.0.as_str(), path.1.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}")]
async fn contract(client: Data<RollupClient>, path: Path<(String, String)>) -> Result<impl Responder> {
    let value = client.get_contract(path.0.as_str(), path.1.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}
