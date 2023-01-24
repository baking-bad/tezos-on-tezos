use context::EphemeralContext;
use serde_json::Value;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use tezos_core::types::encoded::Encoded;
use tezos_l2::executor::origination::originated_address;
use tezos_michelson::micheline::Micheline;

use crate::runner::client::Client;

pub type MockClient = Client<EphemeralContext>;

pub fn read_script(filename: &str) -> Micheline {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/data/scripts");
    path.push(filename);

    let mut file = File::open(path).expect("Failed to open Micheline file");

    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read Micheline file");

    serde_json::from_slice(buffer.as_slice()).expect("Failed to decode Micheline script")
}

impl MockClient {
    pub fn default() -> Self {
        let mut client = Self::new(EphemeralContext::new());
        client.migrate().bake();
        client.import_wallet(
            "pytezos",
            "edsk33N474hxzA4sKeWVM6iuGNGDpX2mGwHNxEA4UbWS8sW3Ta3NKH",
        );
        client.import_wallet(
            "alice",
            "edsk3QoqBuvdamxouPhin7swCvkQNgq4jP5KZPbwWNnwdZpSpJiEbq",
        );
        client.import_wallet(
            "dictator",
            "edsk31vznjHSSpGExDMHYASz45VZqXN4DPxvsa4hAyY8dHM28cZzp6",
        );
        client.import_wallet(
            "bootstrap1",
            "edsk3gUfUPyBSfrS9CCgmCiQsTCHGkviBDusMxDJstFtojtc1zcpsh",
        );
        client.import_wallet(
            "bootstrap2",
            "edsk39qAm1fiMjgmPkw1EgQYkMzkJezLNewd7PLNHTkr6w9XA2zdfo",
        );
        client.import_wallet(
            "bootstrap3",
            "edsk4ArLQgBTLWG5FJmnGnT689VKoqhXwmDPBuGx3z4cvwU9MmrPZZ",
        );
        client.import_wallet(
            "bootstrap4",
            "edsk2uqQB9AY4FvioK2YMdfmyMrer5R8mGFyuaLLFfSRo8EoyNdht3",
        );
        client.import_wallet(
            "bootstrap5",
            "edsk4QLrcijEffxV31gGdN2HU7UpyJjA8drFoNcmnB28n89YjPNRFm",
        );
        client
    }

    pub fn originate_script(
        &mut self,
        filename: &'static str,
        storage: Value,
        balance: u32,
    ) -> String {
        let script = read_script(filename);
        let opg_hash = self.originate(script, storage, balance.into()).inject();
        let address = originated_address(&opg_hash, 0).unwrap();
        address.into_string()
    }
}
