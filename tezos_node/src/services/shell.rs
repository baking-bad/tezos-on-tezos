// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use actix_web::{
    web::{Data, Json},
    Responder, Result,
};
use hex;
use serde::{Deserialize, Serialize};
use tezos_rpc::models::bootstrapped_status::{BootstrappedStatus, ChainStatus};

use crate::{
    json_response,
    rollup::{RollupClient, TezosHelpers},
    Error,
};

// TODO: describe mempool operation models
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PendingOperations {
    pub applied: Vec<String>,
    pub refused: Vec<String>,
    pub outdated: Vec<String>,
    pub branch_refused: Vec<String>,
    pub branch_delayed: Vec<String>,
    pub unprocessed: Vec<String>,
}

pub async fn version<T: RollupClient>(client: Data<T>) -> Result<impl Responder> {
    let value = client.get_version().await?;
    Ok(json_response!(value))
}

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

pub async fn pending_operations<T: RollupClient>() -> Result<impl Responder> {
    let value = PendingOperations::default();
    Ok(json_response!(value))
}

#[cfg(test)]
mod test {
    use actix_web::{test, web::Data, App};
    use tezos_rpc::models::version::VersionInfo;

    use crate::{rollup::mock_client::RollupMockClient, services::config, Result};

    #[actix_web::test]
    async fn test_version() -> Result<()> {
        let client = RollupMockClient::default();

        let app = test::init_service(
            App::new()
                .configure(config::<RollupMockClient>)
                .app_data(Data::new(client)),
        )
        .await;

        let req = test::TestRequest::get().uri("/version").to_request();
        let res: VersionInfo = test::call_and_read_body_json(&app, req).await;
        assert_eq!(0, res.version.major);
        Ok(())
    }
}
