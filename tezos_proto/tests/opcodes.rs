// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

/// Ported from https://gitlab.com/tezos/tezos/-/blob/master/tests_python/tests_016/test_contract_onchain_opcodes.py
mod runner;

use runner::mock::MockClient;
use serde_json::json;
use tezos_core::types::encoded::Encoded;

#[test]
fn test_progress() {
    let mut client = MockClient::default();

    let head1 = client.bake();
    assert_eq!(head1.level, 1);

    let head2 = client.bake();
    assert_eq!(head2.level, 2);
    assert!(head2.timestamp > head1.timestamp);
    assert_ne!(head2.hash, head1.hash);
}

#[test]
fn test_store_input() {
    let mut client = MockClient::default();
    let store_input = client.use_wallet("alice").reveal().originate_script(
        "store_input.json",
        json!({"string": ""}),
        100,
    );

    client.bake();

    client
        .call(&store_input, "default", json!({"string": "abcdefg"}), 100)
        .inject();

    client.bake();

    assert_eq!(200, client.get_contract_balance(&store_input));
    assert!(client
        .get_contract_storage(&store_input)
        .contains("abcdefg"));

    client
        .call(&store_input, "default", json!({"string": "xyz"}), 100)
        .inject();

    client.bake();

    assert_eq!(300, client.get_contract_balance(&store_input));
    assert!(client.get_contract_storage(&store_input).contains("xyz"));
}

#[test]
fn test_transfer_amount() {
    let mut client = MockClient::default();
    let transfer_amount = client.use_wallet("alice").reveal().originate_script(
        "transfer_amount.json",
        json!({"int": "0"}),
        100,
    );

    client.bake();

    client
        .call(&transfer_amount, "default", json!({"prim": "Unit"}), 500)
        .inject();

    client.bake();

    assert_eq!(600, client.get_contract_balance(&transfer_amount));
    assert!(client
        .get_contract_storage(&transfer_amount)
        .contains("500"));
}

#[test]
fn test_store_now() {
    let mut client = MockClient::default();
    let store_now = client.use_wallet("alice").reveal().originate_script(
        "store_now.json",
        json!({"string": "2017-07-13T09:19:01Z"}),
        0,
    );

    client.bake();

    client
        .call(&store_now, "default", json!({"prim": "Unit"}), 0)
        .inject();

    client.bake();

    assert!(client
        .get_contract_storage(&store_now)
        .contains("1970-01-01T00:00:24Z"));
}

#[test]
fn test_transfer_tokens() {
    let mut client = MockClient::default();
    let noop = client.use_wallet("alice").reveal().originate_script(
        "noop.json",
        json!({"prim": "Unit"}),
        0,
    );

    client.bake();

    let transfer_tokens =
        client.originate_script("transfer_tokens.json", json!({"prim": "Unit"}), 50000000);

    client.bake();

    client
        .call(
            &transfer_tokens,
            "default",
            json!({ "string": noop }),
            50000000,
        )
        .inject();

    client.bake();

    assert_eq!(100000000, client.get_contract_balance(&noop));
}

#[test]
fn test_self() {
    let mut client = MockClient::default();
    let self_contract = client.use_wallet("alice").reveal().originate_script(
        "self.json",
        json!({"string": "tz1KqTpEZ7Yob7QbPE4Hy4Wo8fHG8LhKxZSx"}),
        0,
    );

    client.bake();

    client
        .call(&self_contract, "default", json!({"prim": "Unit"}), 0)
        .inject();

    client.bake();

    assert!(client
        .get_contract_storage(&self_contract)
        .contains(&self_contract));
}

#[test]
fn test_contract_fails() {
    let mut client = MockClient::default();
    let contract = client.use_wallet("alice").reveal().originate_script(
        "contract.json",
        json!({"prim": "Unit"}),
        0,
    );

    client.bake();

    let opg_hash = client
        .call(&contract, "default", json!({ "string": contract }), 0)
        .inject();

    client.bake();

    assert!(client
        .get_operation(opg_hash.value())
        .contains("michelson_v1.runtime_error"));
}

#[test]
fn test_source() {
    let mut client = MockClient::default();
    let source = client.use_wallet("alice").reveal().originate_script(
        "source.json",
        json!({"string": "tz1grSQDByRpnVs7sPtaprNZRp531ZKz6Jmm"}),
        0,
    );

    client.bake();

    client
        .call(&source, "default", json!({"prim": "Unit"}), 0)
        .inject();

    client.bake();

    assert!(client
        .get_contract_storage(&source)
        .contains("tz1VSUr8wwNhLAzempoch5d6hLRiTh8Cjcjb"));
}

#[test]
fn test_source_proxy() {
    let mut client = MockClient::default();
    let source = client.use_wallet("alice").reveal().originate_script(
        "source.json",
        json!({"string": "tz1grSQDByRpnVs7sPtaprNZRp531ZKz6Jmm"}),
        0,
    );

    client.bake();

    let proxy = client.originate_script("proxy.json", json!({"prim": "Unit"}), 0);

    client
        .call(&proxy, "default", json!({ "string": source }), 0)
        .inject();

    client.bake();

    assert!(client
        .get_contract_storage(&source)
        .contains("tz1VSUr8wwNhLAzempoch5d6hLRiTh8Cjcjb"));
}

#[test]
fn test_sender() {
    let mut client = MockClient::default();
    let sender = client.use_wallet("alice").reveal().originate_script(
        "sender.json",
        json!({"string": "tz1grSQDByRpnVs7sPtaprNZRp531ZKz6Jmm"}),
        0,
    );

    client.bake();

    client
        .call(&sender, "default", json!({"prim": "Unit"}), 0)
        .inject();

    client.bake();

    assert!(client
        .get_contract_storage(&sender)
        .contains("tz1VSUr8wwNhLAzempoch5d6hLRiTh8Cjcjb"));
}

#[test]
fn test_sender_proxy() {
    let mut client = MockClient::default();
    let sender = client.use_wallet("alice").reveal().originate_script(
        "sender.json",
        json!({"string": "tz1grSQDByRpnVs7sPtaprNZRp531ZKz6Jmm"}),
        0,
    );

    client.bake();

    let proxy = client.originate_script("proxy.json", json!({"prim": "Unit"}), 0);

    client
        .call(&proxy, "default", json!({ "string": sender }), 0)
        .inject();

    client.bake();

    assert!(client.get_contract_storage(&sender).contains(&proxy));
}

#[test]
fn test_big_map_to_self() {
    let mut client = MockClient::default();
    let big_map_to_self =
        client
            .use_wallet("alice")
            .reveal()
            .originate_script("big_map_to_self.json", json!([]), 0);

    client.bake();

    let opg_hash = client
        .call(&big_map_to_self, "default", json!({"prim": "Unit"}), 0)
        .inject();

    client.bake();

    // ATM not supporting big map move
    assert!(client
        .get_operation(opg_hash.value())
        .contains("michelson_v1.runtime_error"));
}
