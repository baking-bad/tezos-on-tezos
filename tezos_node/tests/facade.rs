mod runner;

use actix_web::{test, web::Data, App};
use runner::mock::RollupMockClient;
use tezos_core::types::encoded::{BlockHash, Encoded};
use tezos_node::services::config;

#[actix_web::test]
async fn test_block_hash() {
    let client = RollupMockClient::default();
    client
        .initialize()
        .await
        .expect("Failed to initialize client");

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let app = test::init_service(
        App::new()
            .configure(config::<RollupMockClient>)
            .app_data(Data::new(client)),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/chains/main/blocks/head/hash")
        .to_request();
    let resp: BlockHash = test::call_and_read_body_json(&app, req).await;
    assert_ne!(
        "BKiHLREqU3JkXfzEDYAkmmfX48gBDtYhMrpA98s7Aq4SzbUAB6M",
        resp.value()
    );
}
