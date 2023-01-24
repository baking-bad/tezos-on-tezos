mod runner;

use serde_json::json;
use runner::mock::MockClient;

#[test]
fn test_store_input() {
    let mut client = MockClient::default();
    let store_input = client
        .use_wallet("alice")
        .reveal()
        .originate_script("store_input.json", json!({"string": ""}), 100);

    client.bake();

    client
        .call(&store_input, "default", json!({"string": "abcdefg"}), 100)
        .inject();

    client.bake();

    assert_eq!(200, client.get_contract_balance(&store_input));
    assert!(client.get_contract_storage(&store_input).contains("abcdefg"));

    client
        .call(&store_input, "default", json!({"string": "xyz"}), 100)
        .inject();

    client.bake();

    assert_eq!(300, client.get_contract_balance(&store_input));
    assert!(client.get_contract_storage(&store_input).contains("xyz"));
}