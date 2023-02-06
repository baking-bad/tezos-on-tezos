use actix_web::web::Data;
use std::time::Duration;
use tezos_node::{launch_node, rollup::mock_client::RollupMockClient, rollup::RollupClient};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut client = RollupMockClient::default();
    client
        .initialize()
        .await
        .expect("Failed to initialize client");

    let data = Data::new(client);
    let baker = data.clone();

    actix_web::rt::spawn(async move {
        let mut interval = actix_web::rt::time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            baker.bake().await.expect("Failed to produce block")
        }
    });

    launch_node::<RollupMockClient>(data).await
}
