use serde_json::json;
use static_init::dynamic;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::time::Duration;
use tezos_contract::ContractFetcher;
use tezos_core::types::encoded::{Encoded, ImplicitAddress, OperationHash, SecretKey};
use tezos_core::types::number::Nat;
use tezos_michelson::michelson::data;
use tezos_operation::operations::{
    OperationContent, Origination, Reveal, Script, Transaction, UnsignedOperation,
};
use tezos_proto::executor::origination::originated_address;
use tezos_rpc::{client::TezosRpc, http::default::HttpClient, Result};
use tokio::time::sleep;

const ACTIVATOR_ADDRESS: &str = "tz1TGu6TN5GSez2ndXXeDX6LgUDvLzPLqgYV";
const ACTIVATOR_SECRET_KEY: &str = "edskRhxswacLW6jF6ULavDdzwqnKJVS4UcDTNiCyiH6H8ZNnn2pmNviL7pRNz9kRxxaWQFzEQEcZExGHKbwmuaAcoMegj5T99z";
const ACTIVATOR_PUBLIC_KEY: &str = "edpkuSLWfVU1Vq7Jg9FucPyKmma6otcMHac9zG4oU1KMHSTBpJuGQ2";

struct TezosNode {
    proc: Child,
}

impl Default for TezosNode {
    fn default() -> Self {
        let bin_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("target")
            .join("debug")
            .join("mock-node"); // This is a workaround for CARGO_BIN_EXE_mock-node

        Self {
            proc: Command::new(bin_path)
                .spawn()
                .expect("Failed to launch mock node"),
        }
    }
}

impl Drop for TezosNode {
    fn drop(&mut self) {
        self.proc.kill().expect("Failed to stop mock node");
    }
}

#[dynamic(drop)]
static mut NODE: TezosNode = TezosNode::default();

async fn connect(uri: &str, timeout_sec: i32) -> TezosRpc<HttpClient> {
    let rpc = TezosRpc::new(uri.into());
    for _ in 0..(timeout_sec * 10) {
        match rpc.is_bootstrapped().send().await {
            Ok(_) => break,
            Err(_) => sleep(Duration::from_millis(100)).await,
        };
    }
    rpc
}

async fn wait_operation(
    rpc: &TezosRpc<HttpClient>,
    hash: &OperationHash,
    timeout_sec: i32,
) -> Result<()> {
    for _ in 0..(timeout_sec * 2) {
        let block = rpc.get_block().send().await?;
        if block.operations[3]
            .iter()
            .any(|o| o.hash.as_ref().unwrap() == hash)
        {
            return Ok(());
        }
        sleep(Duration::from_millis(500)).await;
    }
    Err(tezos_rpc::Error::RpcErrorPlain {
        description: format!("Operation not found: {}", hash.value()),
    })
}

async fn send_operation<F: FnOnce(ImplicitAddress, Nat) -> OperationContent>(
    rpc: &TezosRpc<HttpClient>,
    make_content: F,
) -> Result<OperationHash> {
    let source: ImplicitAddress = ACTIVATOR_ADDRESS.try_into()?;
    let secret_key: SecretKey = ACTIVATOR_SECRET_KEY.try_into()?;

    let branch = rpc.get_block_hash().send().await?;
    let counter = rpc
        .get_contract_counter(&source.clone().into())
        .send()
        .await?
        + 1u32.into();
    let operation = UnsignedOperation::new(branch, vec![make_content(source, counter)]);
    // println!("{:#?}", operation);
    let operation_with_fee = rpc.min_fee(operation, None).await?;
    let signed_operation = operation_with_fee.into_signed_operation(&secret_key)?;
    let operation_hash = rpc
        .inject_operation(signed_operation.to_injectable_string()?.as_str())
        .send()
        .await?;

    wait_operation(rpc, &operation_hash, 5).await?;
    Ok(operation_hash)
}

async fn test_00_check_balance(rpc: &TezosRpc<HttpClient>) -> Result<()> {
    let contract = rpc
        .get_contract(&ACTIVATOR_ADDRESS.try_into().unwrap())
        .send()
        .await?;
    assert!(contract.balance > 0u32.into());
    Ok(())
}

async fn test_01_reveal_key(rpc: &TezosRpc<HttpClient>) -> Result<OperationHash> {
    send_operation(&rpc, |source, counter| -> OperationContent {
        Reveal::new(
            source,
            0u8.into(),
            counter.into(),
            0u8.into(),
            0u8.into(),
            ACTIVATOR_PUBLIC_KEY.try_into().unwrap(),
        )
        .into()
    })
    .await
}

async fn test_02_send_tez(rpc: &TezosRpc<HttpClient>) -> Result<OperationHash> {
    send_operation(&rpc, |source, counter| -> OperationContent {
        Transaction::new(
            source,
            0u8.into(),
            counter.into(),
            0u8.into(),
            0u8.into(),
            1000u16.into(),
            "tz2AjVPbMHdDF1XwHVhUrTg6ZvqY83AYhJEy".try_into().unwrap(),
            None,
        )
        .into()
    })
    .await
}

async fn test_03_deploy(rpc: &TezosRpc<HttpClient>) -> Result<OperationHash> {
    let code = json!([
        {"prim": "parameter", "args": [{"prim": "pair", "args": [{"prim": "string"}, {"prim": "int"}]}]},
        {"prim": "storage", "args": [{"prim": "big_map", "args": [{"prim": "string"}, {"prim": "int"}]}]},
        {"prim": "code", "args": [[
            {"prim": "UNPAIR"},
            {"prim": "UNPAIR"},
            {"prim": "DIP", "args": [[{"prim": "SOME"}]]},
            {"prim": "UPDATE"},
            {"prim": "NIL", "args": [{"prim": "operation"}]},
            {"prim": "PAIR"}
        ]]}
    ]);
    let storage = json!([]);
    send_operation(&rpc, move |source, counter| {
        Origination::new(
            source,
            0u8.into(),
            counter.into(),
            0u8.into(),
            0u8.into(),
            0u8.into(),
            None,
            Script {
                code: serde_json::from_value(code).unwrap(),
                storage: serde_json::from_value(storage).unwrap(),
            },
        )
        .into()
    })
    .await
}

async fn test_04_storage(rpc: &TezosRpc<HttpClient>, address: &str) -> Result<()> {
    let contract = rpc
        .contract_at(address.try_into().unwrap(), None)
        .await
        .expect("Failed to load contract");
    let storage = contract
        .storage()
        .get_at_index(0)
        .expect("Failed to unwrap storage");
    assert_eq!(storage, data::int(0));
    Ok(())
}

async fn test_05_call_contract(rpc: &TezosRpc<HttpClient>, address: &str) -> Result<OperationHash> {
    let contract = rpc
        .contract_at(address.try_into().unwrap(), None)
        .await
        .expect("Failed to load contract");
    let partial_tx = contract
        .call(
            "default".into(),
            vec![(
                "",
                data::pair(vec![data::try_string("GM").unwrap(), data::int(42)]),
            )],
        )
        .expect("Failed to prepare contract call");
    send_operation(&rpc, move |source, counter| -> OperationContent {
        partial_tx
            .complete_with(source, counter.into(), Some(0u8.into()), Some(0u8.into()))
            .into()
    })
    .await
}

async fn test_06_big_map(rpc: &TezosRpc<HttpClient>, address: &str) -> Result<()> {
    let contract = rpc
        .contract_at(address.try_into().unwrap(), None)
        .await
        .expect("Failed to load contract");
    let big_map = contract
        .storage()
        .big_maps()
        .get_by_index(0)
        .expect("Failed to select big_map");
    let big_map_value = big_map
        .get_value(data::try_string("GM").unwrap(), None)
        .await
        .expect("Failed to get big_map value");
    assert_eq!(big_map_value, data::int(42));
    Ok(())
}

#[tokio::test]
async fn quickstart() -> Result<()> {
    let rpc = connect("http://127.0.0.1:8732", 30).await;
    test_00_check_balance(&rpc).await?;
    test_01_reveal_key(&rpc).await?;
    test_02_send_tez(&rpc).await?;
    let hash = test_03_deploy(&rpc).await?;
    let address = originated_address(&hash, 0).unwrap();
    test_04_storage(&rpc, address.value()).await?;
    test_05_call_contract(&rpc, address.value()).await?;
    test_06_big_map(&rpc, address.value()).await?;
    Ok(())
}
