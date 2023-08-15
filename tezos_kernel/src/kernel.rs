// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use kernel_io::{
    inbox::{read_inbox, InboxMessage},
    KernelStore, KernelStoreAsHost,
};
use tezos_core::types::encoded::{ChainId, Encoded};
use tezos_proto::{config::{DefaultConfig, Config}, protocol};
use tezos_operation::operations::SignedOperation;
use tezos_smart_rollup_core::SmartRollupCore;
use tezos_smart_rollup_host::runtime::Runtime;

use crate::{payload::TezosPayload, Result};

pub enum TezosPayload {
    Operation(SignedOperation),
    Batch(BatchPayload),
}

impl PayloadType for TezosPayload {
    fn from_external_message(message: &[u8]) -> kernel_io::Result<Self> {
        let opg = SignedOperation::from_bytes(message).map_err(err_into)?;
        Ok(TezosPayload::Operation(opg))
    }
}

pub fn kernel_dispatch<Host: SmartRollupCore, Cfg: Config>(context: &mut KernelStore<Host>, prefix: &[u8]) -> Result<(bool, bool)> {
    match read_inbox(context.as_host(), prefix) {
        Ok(InboxMessage::BeginBlock(_)) => {
            let chain_id = ChainId::from_bytes(&prefix[..4])
                .expect("Failed to decode chain ID");
            let init = protocol::initialize::<Cfg>(context, chain_id)?;
            Ok((init, init))
        },
        Ok(InboxMessage::LevelInfo(info)) => {
            context.set("/time".into(), Some(info.predecessor_timestamp))?;
            context.commit()?;
            Ok((true, false))
        }
        Ok(InboxMessage::Payload(TezosPayload::Operation(operation))) => {
            protocol::inject_operation::<Cfg>(context, operation)?;
            context.as_host().mark_for_reboot()?;
            Ok((true, true))
        }
        Ok(InboxMessage::Payload(TezosPayload::Batch(batch))) => {
            protocol::inject_batch::<Cfg>(context, batch)?;
            context.as_host().mark_for_reboot()?;
            Ok((true, true))
        }
        Ok(InboxMessage::EndBlock(_)) => {
            let timestamp: i64 = context.get("/time".into())?.unwrap_or(0i64);
            protocol::finalize::<Cfg>(context, timestamp)?;
            Ok((true, true))
        }
        Ok(InboxMessage::NoMoreData) => Ok((false, true)),
        Ok(InboxMessage::Foreign(id)) => {
            context.log(format!("Foreign message #{}", id));
            Ok((false, false))
        },
        Ok(InboxMessage::Unknown(id)) => {
            context.log(format!("Unknown message #{}", id));
            Ok((false, false))
        },
        Err(err) => Err(err.into())
    }
}

pub fn kernel_run<Host: SmartRollupCore>(host: &mut Host) {
    let mut context = KernelStore::attach(host);
    context.log(format!("Kernel boots"));
    context.clear();

    let metadata = Runtime::reveal_metadata(context.as_host());
    loop {
        match kernel_dispatch::<Host, DefaultConfig>(&mut context, &metadata.raw_rollup_address) {
            Ok((save, exit)) => {
                if save {
                    context
                        .as_mut()
                        .persist()
                        .expect("Failed to persist changes");
                }
                if exit {
                    break;
                }
            },
            Err(err) => {
                context.clear();
                context.log(err.format());
            }
        }
    };

    context.log(format!("Kernel yields"));
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

        context.as_host().run_level(|_| {}); // Add StartOfLevel & InfoPerLevel
        context.as_host().add_external(message);
        context.as_host().run_level(kernel_run);

        let head = context.get_head()?;
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
