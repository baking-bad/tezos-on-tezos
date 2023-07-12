// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

pub mod error;
pub mod rollup;
pub mod services;

pub use error::{Error, Result};

use crate::{
    rollup::{RollupClient, TezosFacade, TezosHelpers},
    services::config,
};
use actix_web::{
    middleware::{Logger, NormalizePath},
    web::{get, resource, Data},
    App, HttpServer, Responder,
};
use serde_json::json;

// Follows the https://teztnets.xyz/teztnets.json file format so that BCD can index a resettable chain
pub async fn teztnets(rpc_url: Data<String>) -> Result<impl Responder> {
    Ok(json_response!(json!({"rollupnet": {"rpc_url": rpc_url}})))
}

pub async fn launch_node<T: RollupClient + TezosFacade + TezosHelpers + Send + Sync + 'static>(
    data: Data<T>,
    addr: &str,
    port: u16,
    host: Data<String>,
) -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(config::<T>)
            .service(
                resource("/teztnets.json")
                    .app_data(host.clone())
                    .route(get().to(teztnets)),
            )
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
    });
    server.bind((addr, port))?.run().await
}
