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
    rollup::RollupClient,
    Client
};

#[get("/chains/main/chain_id")]
async fn chain_id(client: Data<Client>) -> Result<impl Responder> {
    let value = client.get_chain_id().await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/hash")]
async fn block_hash(client: Data<Client>, path: Path<(String,)>) -> Result<impl Responder> {
    let value = client.get_block_hash(path.0.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/header")]
async fn block_header(client: Data<Client>, path: Path<(String,)>) -> Result<impl Responder> {
    let value = client.get_block_header(path.0.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/metadata")]
async fn block_metadata(client: Data<Client>, path: Path<(String,)>) -> Result<impl Responder> {
    let value = client.get_block_metadata(path.0.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/protocols")]
async fn block_protocols(client: Data<Client>, path: Path<(String,)>) -> Result<impl Responder> {
    let value = client.get_block_protocols(path.0.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

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

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}/balance")]
async fn contract_balance(client: Data<Client>, path: Path<(String, String)>) -> Result<impl Responder> {
    let value = client.get_contract_balance(path.0.as_str(), path.1.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}/counter")]
async fn contract_counter(client: Data<Client>, path: Path<(String, String)>) -> Result<impl Responder> {
    let value = client.get_contract_counter(path.0.as_str(), path.1.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}/manager_key")]
async fn contract_public_key(client: Data<Client>, path: Path<(String, String)>) -> Result<impl Responder> {
    let value = client.get_contract_public_key(path.0.as_str(), path.1.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}/delegate")]
async fn contract_delegate(_client: Data<Client>, _path: Path<(String, String)>) -> Result<impl Responder> {
    Ok(HttpResponse::build(StatusCode::NOT_FOUND).finish())
}

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}/storage")]
async fn contract_storage(client: Data<Client>, path: Path<(String, String)>) -> Result<impl Responder> {
    let value = client.get_contract_storage(path.0.as_str(), path.1.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}/script")]
async fn contract_script(client: Data<Client>, path: Path<(String, String)>) -> Result<impl Responder> {
    let value = client.get_contract_script(path.0.as_str(), path.1.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}/entrypoints")]
async fn contract_entrypoints(client: Data<Client>, path: Path<(String, String)>) -> Result<impl Responder> {
    let value = client.get_contract_entrypoints(path.0.as_str(), path.1.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}")]
async fn contract(client: Data<Client>, path: Path<(String, String)>) -> Result<impl Responder> {
    let value = client.get_contract(path.0.as_str(), path.1.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/context/big_maps/{big_map_id}/values/{key_hash}")]
async fn big_map_value(client: Data<Client>, path: Path<(String, i64, String)>) -> Result<impl Responder> {
    let value = client.get_big_map_value(path.0.as_str(), path.1, path.2.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/operations/{pass}/{index}")]
async fn operation(client: Data<Client>, path: Path<(String, i32, i32)>) -> Result<impl Responder> {
    let value = client.get_operation(path.0.as_str(), path.1, path.2).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/operations/{pass}")]
async fn operation_list(client: Data<Client>, path: Path<(String, i32)>) -> Result<impl Responder> {
    let value = client.get_operation_list(path.0.as_str(), path.1).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/operations")]
async fn operation_list_list(client: Data<Client>, path: Path<(String,)>) -> Result<impl Responder> {
    let value = client.get_operation_list_list(path.0.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/operation_hashes/{pass}/{index}")]
async fn operation_hash(client: Data<Client>, path: Path<(String, i32, i32)>) -> Result<impl Responder> {
    let value = client.get_operation_hash(path.0.as_str(), path.1, path.2).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/operation_hashes/{pass}")]
async fn operation_hash_list(client: Data<Client>, path: Path<(String, i32)>) -> Result<impl Responder> {
    let value = client.get_operation_hash_list(path.0.as_str(), path.1).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/operation_hashes")]
async fn operation_hash_list_list(client: Data<Client>, path: Path<(String,)>) -> Result<impl Responder> {
    let value = client.get_operation_hash_list_list(path.0.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}")]
async fn block(client: Data<Client>, path: Path<(String,)>) -> Result<impl Responder> {
    let value = client.get_block(path.0.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/live_blocks")]
async fn live_blocks(client: Data<Client>, path: Path<(String,)>) -> Result<impl Responder> {
    let value = client.get_live_blocks(path.0.as_str()).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}
