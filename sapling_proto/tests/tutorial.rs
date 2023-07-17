// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use anyhow::Result;
use hex;
use layered_store::EphemeralStore;
use sapling_proto::{
    executor::execute_transaction,
    storage::{run_migrations, SaplingStorage},
    //formatter::Formatter,
    types::SaplingTransaction,
    validator::validate_transaction,
};

// https://ghostnet.tzkt.io/KT1PwYL1B8hagFeCcByAcsN3KTQHmJFfDwnj
const SHIELDING_TX1_HEX: &'static str =
    include_str!("../tests/data/ong6gzsvydC8zgFn1KAM3HFFVymZbroRKqAH4tt1ejcYBcyvroy");
const SHIELDING_TX2_HEX: &'static str =
    include_str!("../tests/data/op4u8djpMUU5n2q1vuoSa4CApTa2cim3jjyXJD7pJeMQ9mH6Vxc");
const SAPLING_TX_HEX: &'static str =
    include_str!("../tests/data/opDWbJCeqTFayGipzgtczTxdvwVjVFYpC2qAbkvFJLLVcqF6rEx");
const UNSHIELDING_TX_HEX: &'static str =
    include_str!("../tests/data/ooCfUWEY9785vAiGxvCqS74qDyCd8vWPXikuLycHjexwe17ALAb");
pub const ANTI_REPLAY: &'static str = "KT1PwYL1B8hagFeCcByAcsN3KTQHmJFfDwnjNetXnHfVqm9iesp";

#[test]
fn run_tutorial() -> Result<()> {
    let mut storage = EphemeralStore::default();
    let head = storage.get_head()?;
    run_migrations(&mut storage, &head)?;

    for (_i, tx_hex) in [
        SHIELDING_TX1_HEX,
        SHIELDING_TX2_HEX,
        SAPLING_TX_HEX,
        UNSHIELDING_TX_HEX,
    ]
    .into_iter()
    .enumerate()
    {
        let payload = hex::decode(tx_hex)?;
        let transaction = SaplingTransaction::try_from(payload.as_slice())?;
        //println!("Tx {}\n{}\n", i, transaction.to_string());

        validate_transaction(&mut storage, &transaction, &ANTI_REPLAY.into())?;
        execute_transaction(&mut storage, &transaction)?;
    }

    Ok(())
}
