use actix_web::{
    web::{Data, Json},
    Responder, Result,
};
use hex;
use tezos_rpc::models::bootstrapped_status::{BootstrappedStatus, ChainStatus};

use crate::{
    json_response,
    rollup::{RollupClient, TezosHelpers},
    Error,
};

pub async fn chain_id<T: RollupClient>(client: Data<T>) -> Result<impl Responder> {
    let value = client.get_chain_id().await?;
    Ok(json_response!(value))
}

pub async fn inject_operation<T: TezosHelpers>(
    client: Data<T>,
    request: Json<String>,
) -> Result<impl Responder> {
    let payload = hex::decode(request.0).map_err(Error::from)?;
    let value = client.inject_operation(payload).await?;
    Ok(json_response!(value))
}

pub async fn is_bootstrapped<T: RollupClient>(client: Data<T>) -> Result<impl Responder> {
    let synced = client.is_chain_synced().await?;
    let value = BootstrappedStatus {
        bootstrapped: synced,
        sync_state: if synced {
            ChainStatus::Synced
        } else {
            ChainStatus::Unsynced
        },
    };
    Ok(json_response!(value))
}
