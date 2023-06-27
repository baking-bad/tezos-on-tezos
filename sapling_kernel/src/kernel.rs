// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use kernel_io::{
    inbox::{read_inbox, InboxMessage},
    KernelStore, KernelStoreAsHost,
};
use sapling_proto::{
    executor::execute_transaction,
    formatter::Formatter,
    storage::{SaplingHead, SaplingStorage},
    types::SaplingTransaction,
    validator::validate_transaction,
};
use tezos_smart_rollup_core::SmartRollupCore;

use crate::{payload::SaplingPayload, Result};

// For demo purpose only (replicate L1 contract state)
pub const ANTI_REPLAY: &'static str = "KT1PwYL1B8hagFeCcByAcsN3KTQHmJFfDwnjNetXnHfVqm9iesp";

pub fn apply_transaction(
    storage: &mut impl SaplingStorage,
    transaction: &SaplingTransaction,
) -> Result<SaplingHead> {
    validate_transaction(storage, &transaction, &ANTI_REPLAY.into())?;
    execute_transaction(storage, &transaction)?;
    let head = storage.get_head()?;
    Ok(head)
}

pub fn kernel_run<Host: SmartRollupCore>(host: &mut Host) {
    let mut context = KernelStore::attach(host);

    let mut head = context.get_head().expect("Failed to get head");
    context.log(format!("Kernel invoked, prev head: {}", head.to_string()));

    let res: Result<()> = loop {
        match read_inbox(context.as_host(), b"") {
            Ok(InboxMessage::BeginBlock(_)) => {}
            Ok(InboxMessage::LevelInfo(_)) => {}
            Ok(InboxMessage::Payload(SaplingPayload::Transaction(tx))) => {
                context.log(format!("Transaction pending: {}", tx.to_string()));
                match apply_transaction(&mut context, &tx) {
                    Ok(new_head) => {
                        head = new_head;
                        context.log(format!(
                            "Transaction applied, new head: {}",
                            head.to_string()
                        ));
                    }
                    Err(err) => {
                        context.rollback();
                        context.log(format!("Transaction failed: {}", err));
                    }
                }
            }
            Ok(InboxMessage::EndBlock(_)) => break Ok(()),
            Ok(InboxMessage::NoMoreData) => break Ok(()),
            Ok(InboxMessage::Foreign(id)) => context.log(format!("Foreign message #{}", id)),
            Ok(InboxMessage::Unknown(id)) => context.log(format!("Unknown message #{}", id)),
            Err(err) => context.log(err.to_string()),
        }
    };

    match res {
        Ok(_) => {
            context
                .as_mut()
                .persist()
                .expect("Failed to persist changes");
            context.log(format!("Kernel yields"));
        }
        Err(err) => {
            context.log(err.format());
            context.clear();
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::Write;

    use super::*;

    use hex;
    use kernel_io::{KernelStore, KernelStoreAsHost};
    use tezos_data_encoding::enc::{BinResult, BinWriter};
    use tezos_smart_rollup_mock::MockHost;

    struct ExternalMessage(Vec<u8>);

    impl ExternalMessage {
        pub fn from_hex(value: &str) -> Self {
            Self(hex::decode(value).unwrap())
        }
    }

    impl BinWriter for ExternalMessage {
        fn bin_write(&self, output: &mut Vec<u8>) -> BinResult {
            match output.write(self.0.as_slice()) {
                Ok(_) => BinResult::Ok(()),
                Err(err) => BinResult::Err(err.into()),
            }
        }
    }

    #[test]
    fn sapling_deposit() -> Result<()> {
        let mut host = MockHost::default();
        let mut context = KernelStore::<MockHost>::attach(&mut host);

        let message = ExternalMessage::from_hex(
            "00000000000001f3f1de6f589f17cda6e8811dd2fb5b2b78875d440de07f6964a2f06e4e26f99b25\
             b004c78eee562cbe5db1eb52042efda378c227c07d649a496121ed51155a322a6f768ee30ab0cfe3\
             7bb0b6cd238e325292b888683f720828a01615d126d847107e7ab1847ef07d3b19d7260318b457b7\
             951c0cb95f5c72a32b85d6fef303bb5011e4125162c16b4c9a845bab7b175e379debc044e98f161b\
             46cb6f955e9d03a6e708cc10df0e2c0a7fdab46cef61a07d837a2f287811f659b3410e3f9e78827b\
             b035b27df9a052fa82af43d4f68dca2a7d4a9576168f07ed419d33f7c30cd78cd363af03b3fcccd9\
             f24287b7f67ca79213a6a3a1bb8cb6a28cbed6027d829b92e177b6a94c8c1c5f1392d79f579d0349\
             e0fd981a8e641ca46d0aa521a0a2c3a40000004ff9db50a921e41f8acbcb0c0006504c20732add90\
             6bccba01e5f5b6ee50cddb5356a61029569c0a0adc8879467d7c5c393b8738d55876b41e2406818c\
             b68e07e6807c1e9411f8898c8feff9e55d5474a8634c173d9fa81abc1e442a1336a2855a3f6db56c\
             e44031230850008eca4d0a70d9ed8005d93fa5545de7b7bfbe53ae70f963ebcdb5e57218dadac436\
             adb2bad786311759780b786d4bcee0ed3f76801903ce3d41c6ded99145c8b276cf0109dce50c80ad\
             64808aaf789de3fa54ae752f75735905f15dd2f30b923de19d53bab0ec5d7886735824b2ddf15fc7\
             2a48313b90a9ddf6a4df7cc3fad8c3495da0a4f40809c3710af64718dcf7d6b9de7fd52955ab9b0c\
             53be54ca0c827dc52ed50cffffffffff676980fbc2f4300c01f0b7820d00e3347c8da4ee61467437\
             6cbc45359daa54f9b5493e00000000",
        );

        context.as_host().add_external(message);
        context.as_host().run_level(kernel_run);

        let head = context.get_head()?;
        println!("{}", head.to_string());
        assert_eq!(1, head.commitments_size);
        assert_eq!(0, head.nullifiers_size);
        assert_eq!(1, head.roots_pos);

        let root1 = context.get_root(1)?.expect("Failed to get root #1");
        assert_eq!(
            root1.to_string(),
            "69a1f12aea9ef4019a059e69e70d6317c35d936d3ea61181f9fa9fa297fe092f"
        );

        let cm1 = context
            .get_commitment(4294967296)?
            .expect("Failed to get commitment #4294967296");
        assert_eq!(
            cm1.to_string(),
            "f1de6f589f17cda6e8811dd2fb5b2b78875d440de07f6964a2f06e4e26f99b25"
        );

        Ok(())
    }
}
