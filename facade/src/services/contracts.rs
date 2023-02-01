use actix_web::{
    get,
    http::StatusCode,
    web::{Data, Path},
    HttpResponse, Responder, Result,
};
use tezos_core::types::encoded::{Address, ContractAddress, ImplicitAddress};

use crate::{rollup::TezosFacade, Client, Error};

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}/balance")]
async fn contract_balance(
    client: Data<Client>,
    path: Path<(String, String)>,
) -> Result<impl Responder> {
    let address: Address = path.1.as_str().try_into().map_err(Error::from)?;
    let value = client
        .get_contract_balance(&path.0.as_str().try_into()?, &address)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}/counter")]
async fn contract_counter(
    client: Data<Client>,
    path: Path<(String, String)>,
) -> Result<impl Responder> {
    let address: ImplicitAddress = path.1.as_str().try_into().map_err(Error::from)?;
    let value = client
        .get_contract_counter(&path.0.as_str().try_into()?, &address)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}/manager_key")]
async fn contract_public_key(
    client: Data<Client>,
    path: Path<(String, String)>,
) -> Result<impl Responder> {
    let address: ImplicitAddress = path.1.as_str().try_into().map_err(Error::from)?;
    let value = client
        .get_contract_public_key(&path.0.as_str().try_into()?, &address)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}/delegate")]
async fn contract_delegate(
    _client: Data<Client>,
    _path: Path<(String, String)>,
) -> Result<impl Responder> {
    Ok(HttpResponse::build(StatusCode::NOT_FOUND).finish())
}

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}/storage")]
async fn contract_storage(
    client: Data<Client>,
    path: Path<(String, String)>,
) -> Result<impl Responder> {
    let address: ContractAddress = path.1.as_str().try_into().map_err(Error::from)?;
    let value = client
        .get_contract_storage(&path.0.as_str().try_into()?, &address)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}/script")]
async fn contract_script(
    client: Data<Client>,
    path: Path<(String, String)>,
) -> Result<impl Responder> {
    let address: ContractAddress = path.1.as_str().try_into().map_err(Error::from)?;
    let value = client
        .get_contract_script(&path.0.as_str().try_into()?, &address)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}/entrypoints")]
async fn contract_entrypoints(
    client: Data<Client>,
    path: Path<(String, String)>,
) -> Result<impl Responder> {
    let address: ContractAddress = path.1.as_str().try_into().map_err(Error::from)?;
    let value = client
        .get_contract_entrypoints(&path.0.as_str().try_into()?, &address)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/context/contracts/{contract_id}")]
async fn contract(client: Data<Client>, path: Path<(String, String)>) -> Result<impl Responder> {
    let address: Address = path.1.as_str().try_into().map_err(Error::from)?;
    let value = client
        .get_contract(&path.0.as_str().try_into()?, &address)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}
