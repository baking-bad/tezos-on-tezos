use tezos_node::{launch_node, rollup::rpc_client::RollupRpcClient};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    launch_node::<RollupRpcClient>().await
}
