// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use tezos_core::types::encoded::{ChainId, OperationHash, Encoded};
use tezos_operation::operations::SignedOperation;
use tezos_proto::{batcher::apply_batch, context::TezosContext};
use tezos_smart_rollup_core::SmartRollupCore;
use tezos_smart_rollup_host::runtime::Runtime;
use kernel_io::{
    inbox::{read_inbox, InboxMessage},
    KernelStore, KernelStoreAsHost,
};

use crate::{payload::TezosPayload, Error, Result};

pub fn kernel_run<Host: SmartRollupCore>(host: &mut Host) {
    let mut context = KernelStore::attach(host);

    let metadata = Runtime::reveal_metadata(context.as_host());
    let mut head = context.get_head().expect("Failed to get head");
    head.chain_id =
        ChainId::from_bytes(&metadata.raw_rollup_address[..4]).expect("Failed to decode chain ID");

    context.log(format!("Kernel invoked, prev head: {}", head));

    let mut batch_payload: Vec<(OperationHash, SignedOperation)> = Vec::new();
    let res: Result<()> = loop {
        match read_inbox(context.as_host(), &metadata.raw_rollup_address[..4]) {
            Ok(InboxMessage::BeginBlock(inbox_level)) => {
                // Origination level is the one before kernel is first time invoked
                // We assume single rollup block per inbox block here
                // Note that head level is the one prior to the current block
                let expected = inbox_level - metadata.origination_level as i32 - 2;
                if head.level != expected {
                    break Err(Error::InconsistentHeadLevel {
                        expected,
                        found: head.level,
                    });
                }
            }
            Ok(InboxMessage::LevelInfo(info)) => {
                head.timestamp = info.predecessor_timestamp;
            }
            Ok(InboxMessage::Payload(TezosPayload::Operation { hash, opg })) => {
                context.log(format!("Operation pending: {}", &hash.value()));
                batch_payload.push((hash, opg));
            }
            Ok(InboxMessage::EndBlock(_)) => {
                match apply_batch(&mut context, head.clone(), batch_payload, false) {
                    Ok(new_head) => {
                        head = new_head;
                        context.log(format!("Batch applied: {}", head));
                        break Ok(());
                    }
                    Err(err) => break Err(err.into()),
                }
            }
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
    use tezos_proto::context::TezosContext;
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
    fn send_tez() -> Result<()> {
        let mut host = MockHost::default();
        let mut context = KernelStore::<MockHost>::attach(&mut host);
        // default rollup address is sr163Lv22CdE8QagCwf48PWDTquk6isQwv57Head
        // chain_id is first 4 bytes (00000000)
        // the rest is the operation payload
        let message = ExternalMessage::from_hex(
            "0000000062fd30ac16979d9b88aca559e8fd8b97abd2519bebe09ad8a269d60df0b17ddc6b\
            00e8b36c80efb51ec85a14562426049aa182a3ce38f902e18a18e807000017143f62ff9c2f41b30ee00b8c64d233fda43adf05\
            eb829cfd2e733ee9a8f44b6c00e8b36c80efb51ec85a14562426049aa182a3ce3800e28a18ab0b8102c0843d00006b82198cb1\
            79e8306c1bedd08f12dc863f32888600b2014573fd63d27895841ea6ca9d45e23e1e3b836298801b5e390b3b0a0b412003af89\
            c08e63b6d8cf6847300e627c4ce0882ce4e2b842295309de3a0bd6260f",
        );

        context.as_host().add_external(message);
        context.as_host().run_level(kernel_run);

        let head = context.get_head()?;
        println!("{:?}", head);
        assert_eq!(0, head.level);
        assert_eq!(1, head.operations.len());

        let batch_receipt = context.get_batch_receipt(head.hash.value())?;
        assert_eq!(batch_receipt.hash, head.hash);

        for opg_hash in head.operations.iter() {
            let opg_receipt = context.get_operation_receipt(opg_hash.value())?;
            // println!("Receipt: {:#?}", receipt);
            assert!(opg_receipt.hash.is_some(), "Expected operation hash");
        }

        Ok(())
    }
}
