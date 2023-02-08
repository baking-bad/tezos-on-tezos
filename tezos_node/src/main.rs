use actix_web::web::Data;
use clap::Parser;
use tezos_node::{launch_node, rollup::rpc_client::RollupRpcClient, rollup::RollupClient};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value_t = String::from("http://localhost:8932"))]
    endpoint: String,

    #[arg(short, long, default_value_t = String::from("127.0.0.1"))]
    rpc_addr: String,

    #[arg(short, long, default_value_t = 8732)]
    port: u16,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let args = Args::parse();

    let mut client = RollupRpcClient::new(&args.endpoint);
    client
        .initialize()
        .await
        .expect("Failed to initialize client");

    let data = Data::new(client);
    launch_node::<RollupRpcClient>(data, &args.rpc_addr, args.port).await
}
