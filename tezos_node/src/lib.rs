pub mod error;
pub mod rollup;
pub mod services;

pub use error::{Error, Result};

use crate::{
    rollup::{RollupClient, TezosFacade, TezosHelpers},
    services::config,
};
use actix_web::{middleware::Logger, web::Data, App, HttpServer};

pub async fn launch_node<
    T: Default + RollupClient + TezosFacade + TezosHelpers + Send + Sync + 'static,
>(
    data: Data<T>,
) -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .configure(config::<T>)
            .wrap(Logger::default())
    });
    server.bind(("127.0.0.1", 8732))?.run().await
}
