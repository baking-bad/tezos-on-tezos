use tezos_node::{launch_node, rollup::mock_client::RollupMockClient};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    launch_node::<RollupMockClient>().await
}
