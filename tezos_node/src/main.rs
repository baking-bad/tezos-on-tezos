use actix_web::{middleware::Logger, App, HttpServer};
use tezos_node::{
    rollup::rpc_client::RollupRpcClient,
    services::config
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut client = RollupRpcClient::default();
    client.initialize()
        .await
        .expect("Failed to initialize client");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .configure(config::<RollupRpcClient>)
            .wrap(Logger::default())
    });
    server.bind(("127.0.0.1", 8732))?.run().await
}
