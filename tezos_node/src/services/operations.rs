use actix_web::{
    http::StatusCode,
    web::{Data, Path},
    HttpResponse, Responder, Result,
};

use crate::rollup::TezosFacade;

pub async fn operation<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String, i32, i32)>,
) -> Result<impl Responder> {
    let value = client
        .get_operation(&path.0.as_str().try_into()?, path.1, path.2)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

pub async fn operation_list<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String, i32)>,
) -> Result<impl Responder> {
    let value = client
        .get_operation_list(&path.0.as_str().try_into()?, path.1)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

pub async fn operation_list_list<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String,)>,
) -> Result<impl Responder> {
    let value = client
        .get_operation_list_list(&path.0.as_str().try_into()?)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

pub async fn operation_hash<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String, i32, i32)>,
) -> Result<impl Responder> {
    let value = client
        .get_operation_hash(&path.0.as_str().try_into()?, path.1, path.2)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

pub async fn operation_hash_list<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String, i32)>,
) -> Result<impl Responder> {
    let value = client
        .get_operation_hash_list(&path.0.as_str().try_into()?, path.1)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

pub async fn operation_hash_list_list<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String,)>,
) -> Result<impl Responder> {
    let value = client
        .get_operation_hash_list_list(&path.0.as_str().try_into()?)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}
