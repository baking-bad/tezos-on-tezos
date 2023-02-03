use static_init::dynamic;
use serial_test::serial;
use std::path::PathBuf;
use std::time::Duration;
use tezos_rpc::{client::TezosRpc, http::default::HttpClient, Result};
use tokio::time::sleep;
use std::process::{Command, Child};

struct TezosNode {
    proc: Child
}

impl Default for TezosNode {
    fn default() -> Self {
        let bin_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("target")
            .join("debug")
            .join("mock-node");  // This is a workaround for CARGO_BIN_EXE_mock-node

        Self {
            proc: Command::new(bin_path)
                .spawn()
                .expect("Failed to launch mock node")
        }
    }
}

impl Drop for TezosNode {
    fn drop(&mut self) {
        self.proc
            .kill()
            .expect("Failed to stop mock node");
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

#[tokio::test]
#[serial]
async fn integration_test() -> Result<()> {
    let rpc = connect("http://127.0.0.1:8732", 30).await;
    let hash = rpc.get_block_hash().send().await?;
    println!("{:?}", hash);
    Ok(())
}
