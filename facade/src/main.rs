pub mod error;
pub mod routes;
pub mod client;
pub mod facade;

use actix_web::{App, HttpServer, web::Data};

use crate::{
    routes::{block_hash},
    client::RollupClient,
};
pub use error::{Error, Result};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new()
        .app_data(Data::new(RollupClient::default()))
        .service(block_hash)
    );
    server
        .bind(("127.0.0.1", 8732))?
        .run()
        .await
}