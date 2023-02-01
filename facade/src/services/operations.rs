use actix_web::{
    get,
    http::StatusCode,
    web::{Data, Path},
    HttpResponse, Responder, Result,
};

use crate::{rollup::TezosFacade, Client};

#[get("/chains/main/blocks/{block_id}/operations/{pass}/{index}")]
async fn operation(client: Data<Client>, path: Path<(String, i32, i32)>) -> Result<impl Responder> {
    let value = client
        .get_operation(&path.0.as_str().try_into()?, path.1, path.2)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/operations/{pass}")]
async fn operation_list(client: Data<Client>, path: Path<(String, i32)>) -> Result<impl Responder> {
    let value = client
        .get_operation_list(&path.0.as_str().try_into()?, path.1)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/operations")]
async fn operation_list_list(
    client: Data<Client>,
    path: Path<(String,)>,
) -> Result<impl Responder> {
    let value = client
        .get_operation_list_list(&path.0.as_str().try_into()?)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/operation_hashes/{pass}/{index}")]
async fn operation_hash(
    client: Data<Client>,
    path: Path<(String, i32, i32)>,
) -> Result<impl Responder> {
    let value = client
        .get_operation_hash(&path.0.as_str().try_into()?, path.1, path.2)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/operation_hashes/{pass}")]
async fn operation_hash_list(
    client: Data<Client>,
    path: Path<(String, i32)>,
) -> Result<impl Responder> {
    let value = client
        .get_operation_hash_list(&path.0.as_str().try_into()?, path.1)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

#[get("/chains/main/blocks/{block_id}/operation_hashes")]
async fn operation_hash_list_list(
    client: Data<Client>,
    path: Path<(String,)>,
) -> Result<impl Responder> {
    let value = client
        .get_operation_hash_list_list(&path.0.as_str().try_into()?)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}
