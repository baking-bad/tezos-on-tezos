// pub mod context;
pub mod provider;
pub mod rollup;
pub mod error;
pub mod facade;

use actix_web::{App, HttpServer};

use crate::{
    facade::{block_hash},
    rollup::RollupRpcClient,
};
pub use error::{Error, Result};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new()
        .app_data(RollupRpcClient::default())
        .service(block_hash)
    );
    server
        .bind(("127.0.0.1", 8732))?
        .run()
        .await
}
