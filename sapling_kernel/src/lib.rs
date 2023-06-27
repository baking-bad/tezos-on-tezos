// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use sapling_proto::{
    executor::execute_transaction, types::SaplingTransaction, validator::validate_transaction,
};
use tezos_smart_rollup::{inbox::InboxMessage, kernel_entry, michelson::MichelsonUnit, prelude::*};

#[derive(Debug)]
pub enum Error {}

pub type Result<T> = std::result::Result<T, Error>;

fn kernel_run(host: &mut impl Runtime) {
    loop {
        match host.read_input() {
            Ok(Some(message)) => match InboxMessage::<MichelsonUnit>::parse(message.as_ref()) {
                Ok((_, InboxMessage::External(payload))) => {
                    if let Ok(tx) = SaplingTransaction::try_from(payload) {}
                }
                Ok(_) => continue,
                Err(_) => continue,
            },
            Ok(None) => return,
            Err(_) => continue,
        }
    }
}

pub fn entry(host: &mut impl Runtime) {
    kernel_run(host);
    host.mark_for_reboot().unwrap();
}

kernel_entry!(entry);
