use actix_web::{
    http::StatusCode,
    web::{Data, Path},
    HttpResponse, Responder, Result,
};

use crate::rollup::TezosFacade;

pub async fn block_hash<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String,)>,
) -> Result<impl Responder> {
    let value = client.get_block_hash(&path.0.as_str().try_into()?).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

pub async fn block_header<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String,)>,
) -> Result<impl Responder> {
    let value = client
        .get_block_header(&path.0.as_str().try_into()?)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

pub async fn block_metadata<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String,)>,
) -> Result<impl Responder> {
    let value = client
        .get_block_metadata(&path.0.as_str().try_into()?)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

pub async fn block_protocols<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String,)>,
) -> Result<impl Responder> {
    let value = client
        .get_block_protocols(&path.0.as_str().try_into()?)
        .await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

pub async fn live_blocks<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String,)>,
) -> Result<impl Responder> {
    let value = client.get_live_blocks(&path.0.as_str().try_into()?).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}

pub async fn block<T: TezosFacade>(
    client: Data<T>,
    path: Path<(String,)>,
) -> Result<impl Responder> {
    let value = client.get_block(&path.0.as_str().try_into()?).await?;
    Ok(HttpResponse::build(StatusCode::OK).json(value))
}
