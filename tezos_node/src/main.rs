use actix_web::web::Data;
use tezos_node::{launch_node, rollup::rpc_client::RollupRpcClient, rollup::RollupClient};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut client = RollupRpcClient::default();
    client
        .initialize()
        .await
        .expect("Failed to initialize client");

    let data = Data::new(client);
    launch_node::<RollupRpcClient>(data).await
}
