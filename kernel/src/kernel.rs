use host::{rollup_core::RawRollupCore, runtime::Runtime};
use tezos_core::types::encoded::{ChainId, Encoded, OperationHash};
use tezos_ctx::{ExecutorContext, GenericContext};
use tezos_l2::{batcher::apply_batch, constants};
use tezos_operation::operations::SignedOperation;

use crate::{
    context::PVMContext,
    inbox::{read_inbox, InboxMessage},
    Error, Result,
};

pub fn kernel_run<Host: RawRollupCore>(context: &mut PVMContext<Host>) {
    let metadata = Runtime::reveal_metadata(context.as_mut()).expect("Failed to reveal metadata");
    let mut head = context.get_head().expect("Failed to get head");
    head.chain_id =
        ChainId::from_bytes(&metadata.raw_rollup_address[..4]).expect("Failed to decode chain ID");

    context.log(format!("Kernel invoked, prev head: {}", head));

    let mut batch_payload: Vec<(OperationHash, SignedOperation)> = Vec::new();
    let res: Result<()> = loop {
        match read_inbox(context.as_mut(), &metadata.raw_rollup_address[..4]) {
            Ok(InboxMessage::BeginBlock(inbox_level)) => {
                // Origination level is the one before kernel is first time invoked
                // We assume single rollup block per inbox block here
                // Note that head level is the one prior to the current block
                let expected = inbox_level - metadata.origination_level - 2;
                if head.level != expected {
                    break Err(Error::InconsistentHeadLevel {
                        expected,
                        found: head.level,
                    });
                }
            }
            Ok(InboxMessage::LevelInfo(info)) => {
                if head.timestamp == 0 {
                    head.timestamp = info.predecessor_timestamp;
                } else {
                    let upper_bound = info.predecessor_timestamp + constants::BLOCK_TIME;
                    if head.timestamp > upper_bound {
                        break Err(Error::InconsistentHeadTimestamp {
                            upper_bound,
                            found: head.timestamp,
                        });
                    }
                }
            }
            Ok(InboxMessage::L2Operation { hash, opg }) => {
                context.log(format!("Operation pending: {}", &hash.value()));
                batch_payload.push((hash, opg));
            }
            Ok(InboxMessage::EndBlock(_)) => {
                match apply_batch(context, head.clone(), batch_payload, false) {
                    Ok(new_head) => {
                        head = new_head;
                        context.log(format!("Batch applied: {}", head));
                        break Ok(());
                    }
                    Err(err) => break Err(err.into()),
                }
            }
            Ok(InboxMessage::NoMoreData) => break Ok(()),
            Ok(InboxMessage::Unknown(id)) => context.log(format!("Unknown message #{}", id)),
            Err(err) => context.log(err.to_string()),
        }
    };

    match res {
        Ok(_) => {
            context.persist().expect("Failed to persist changes");
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
    use super::*;
    use crate::context::PVMContext;

    use hex;
    use host::rollup_core::Input;
    use mock_runtime::host::MockHost;
    use tezos_ctx::{ExecutorContext, Result};

    #[test]
    fn send_tez() -> Result<()> {
        let mut context = PVMContext::new(MockHost::default());
        // default rollup address is sr1V6huFSUBUujzubUCg9nNXqpzfG9t4XD1h
        // chain_id is first 4 bytes (fceda8e6)
        // 01 is inbox prefix for external messages
        // the rest is the operation payload
        let input = hex::decode(
            "01fceda8e662fd30ac16979d9b88aca559e8fd8b97abd2519bebe09ad8a269d60df0b17ddc6b\
            00e8b36c80efb51ec85a14562426049aa182a3ce38f902e18a18e807000017143f62ff9c2f41b30ee00b8c64d233fda43adf05\
            eb829cfd2e733ee9a8f44b6c00e8b36c80efb51ec85a14562426049aa182a3ce3800e28a18ab0b8102c0843d00006b82198cb1\
            79e8306c1bedd08f12dc863f32888600b2014573fd63d27895841ea6ca9d45e23e1e3b836298801b5e390b3b0a0b412003af89\
            c08e63b6d8cf6847300e627c4ce0882ce4e2b842295309de3a0bd6260f",
        )
        .unwrap();
        let level = 1;
        context.as_mut().as_mut().set_ready_for_input(level);
        context.as_mut().as_mut().add_next_inputs(
            level,
            vec![
                (Input::MessageData, b"\x00\x01".to_vec()),
                (Input::MessageData, input),
                (Input::MessageData, b"\x00\x02".to_vec()),
            ]
            .iter(),
        );

        kernel_run(&mut context);

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
