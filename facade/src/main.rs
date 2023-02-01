pub mod error;
pub mod rollup;
pub mod services;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};

use crate::services::{
    blocks::{block, block_hash, block_header, block_metadata, block_protocols, live_blocks},
    context::{big_map_value, constants, delegate, delegates},
    contracts::{
        contract, contract_balance, contract_counter, contract_delegate, contract_entrypoints,
        contract_public_key, contract_script, contract_storage,
    },
    operations::{
        operation, operation_hash, operation_hash_list, operation_hash_list_list, operation_list,
        operation_list_list,
    },
    shell::{chain_id, inject_operation},
};

pub use error::{Error, Result};

type Client = rollup::rpc_client::RollupRpcClient;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut client = Client::default();
    client
        .initialize()
        .await
        .expect("Failed to initialize client");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(client.clone()))
            .service(chain_id)
            .service(inject_operation)
            .service(block)
            .service(block_hash)
            .service(block_header)
            .service(block_metadata)
            .service(block_protocols)
            .service(live_blocks)
            .service(delegates)
            .service(delegate)
            .service(constants)
            .service(big_map_value)
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
    });
    server.bind(("127.0.0.1", 8732))?.run().await
}
