pub mod error;
pub mod routes;
pub mod client;
pub mod facade;

use actix_web::{App, HttpServer, web::Data, middleware::Logger};

use crate::{
    routes::{
        block_hash, block_header, block_metadata, block_protocols, 
        delegate, delegates, constants, chain_id,
        contract_balance, contract_counter, contract_delegate,
        contract_storage, contract_script, contract_entrypoints, contract
    },
    client::RollupClient,
};
pub use error::{Error, Result};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new()
        .wrap(Logger::default())
        .app_data(Data::new(RollupClient::default()))
        .service(chain_id)
        .service(block_hash)
        .service(block_header)
        .service(block_metadata)
        .service(block_protocols)
        .service(delegates)
        .service(delegate)
        .service(constants)
        .service(contract_balance)
        .service(contract_counter)
        .service(contract_delegate)
        .service(contract_storage)
        .service(contract_script)
        .service(contract_entrypoints)
        .service(contract)
    );
    server
        .bind(("127.0.0.1", 8732))?
        .run()
        .await
}