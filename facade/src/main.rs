pub mod error;
pub mod routes;
pub mod rollup;
pub mod rpc;
pub mod mock;

use actix_web::{App, HttpServer, web::Data, middleware::Logger};

use crate::{
    routes::{
        block_hash, block_header, block_metadata, block_protocols, block, live_blocks,
        delegate, delegates, constants, chain_id,
        contract_balance, contract_counter, contract_delegate, contract_public_key,
        contract_storage, contract_script, contract_entrypoints, contract, big_map_value,
        operation_hash, operation_hash_list, operation_hash_list_list,
        operation, operation_list, operation_list_list,
    },
};

pub use error::{Error, Result};
pub type Client = rpc::RPCClient;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| App::new()
        .wrap(Logger::default())
        .app_data(Data::new(Client::default()))
        .service(chain_id)
        .service(live_blocks)
        .service(big_map_value)
        .service(block)
        .service(block_hash)
        .service(block_header)
        .service(block_metadata)
        .service(block_protocols)
        .service(delegates)
        .service(delegate)
        .service(constants)
        .service(contract)
        .service(contract_public_key)
        .service(contract_balance)
        .service(contract_counter)
        .service(contract_delegate)
        .service(contract_storage)
        .service(contract_script)
        .service(contract_entrypoints)
        .service(operation)
        .service(operation_list)
        .service(operation_list_list)
        .service(operation_hash)
        .service(operation_hash_list)
        .service(operation_hash_list_list)
    );
    server
        .bind(("127.0.0.1", 8732))?
        .run()
        .await
}