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
    web::Data,
    App, HttpServer,
};

pub async fn launch_node<T: RollupClient + TezosFacade + TezosHelpers + Send + Sync + 'static>(
    data: Data<T>,
    addr: &str,
    port: u16,
) -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(config::<T>)
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
    });
    server.bind((addr, port))?.run().await
}
