use host::runtime::Runtime;
use proto::{
    producer::{apply_batch, types::{OperationHash, SignedOperation}},
    context::Context,
    debug_msg
};
use hex;

use crate::{
    context::PVMContext,
    inbox::{InboxMessage, read_inbox}
};

pub fn kernel_run<Host: Runtime>(context: &mut PVMContext<Host>) {
    let mut head = context.get_head().expect("Failed to get head");
    debug_msg!(context, "Kernel invoked: {:?}", head);

    let mut batch_payload: Vec<(OperationHash, SignedOperation)> = Vec::new();
    let res = loop {
        match read_inbox(context.as_mut()) {
            Ok(InboxMessage::BeginBlock(_level)) => {
                // TODO: validate head against inbox_level - origination_level (revealed metadata)
            },
            Ok(InboxMessage::L2Operation { hash, opg }) => {
                debug_msg!(context, "Operation pending: {:?}", &hash);
                batch_payload.push((hash, opg));
            },
            Ok(InboxMessage::LevelInfo(info)) => {
                debug_msg!(context, "Info message: {}", hex::encode(info));
                head.timestamp += 1;  // TODO: validate and adjust timestamp if necessary
            },
            Ok(InboxMessage::EndBlock(_level)) => {
                match apply_batch(context, head.clone(), batch_payload) {
                    Ok(new_head) => {
                        head = new_head;
                        debug_msg!(context, "Batch applied: {:?}", head);
                        break Ok(())
                    },
                    Err(err) => break Err(err.into())
                }
            },
            Ok(InboxMessage::Unknown(id)) => debug_msg!(context, "Unknown message #{}", id),
            Ok(InboxMessage::NoMoreData) => break Ok(()),
            Err(err) => break Err(err)
        }
    };
    
    match res {
        Ok(_) => {
            context.clear();
            debug_msg!(context, "Kernel yields");
        },
        Err(err) => {
            debug_msg!(context, "Unrecoverable error: {:?}", err);
            // TODO: rewind state
        }
    }    
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::context::PVMContext;

    use proto::context::Context;
    use proto::errors::Result;
    use mock_runtime::host::MockHost;
    use host::rollup_core::Input;
    use hex;

    #[test]
    fn send_tez() -> Result<()> {
        let mut context = PVMContext::new(MockHost::default());
        let input = hex::decode("0162fd30ac16979d9b88aca559e8fd8b97abd2519bebe09ad8a269d60df0b17ddc6b\
            00e8b36c80efb51ec85a14562426049aa182a3ce38f902e18a18e807000017143f62ff9c2f41b30ee00b8c64d233fda43adf05\
            eb829cfd2e733ee9a8f44b6c00e8b36c80efb51ec85a14562426049aa182a3ce3800e28a18ab0b8102c0843d00006b82198cb1\
            79e8306c1bedd08f12dc863f32888600b2014573fd63d27895841ea6ca9d45e23e1e3b836298801b5e390b3b0a0b412003af89\
            c08e63b6d8cf6847300e627c4ce0882ce4e2b842295309de3a0bd6260f").unwrap();
        let level = 10;
        context
            .as_mut()
            .as_mut()
            .set_ready_for_input(level);
        context
            .as_mut()
            .as_mut()
            .add_next_inputs(level, vec![
                (Input::MessageData, b"\x00\x01".to_vec()),
                (Input::MessageData, input),
                (Input::MessageData, b"\x00\x02".to_vec())
            ].iter());

        kernel_run(&mut context);

        let opg_receipt = context.get_operation_receipt(0i32, 0i32)?;
        // println!("Receipt: {:#?}", receipt);
        assert!(opg_receipt.is_some(), "Expected operation receipt");
        assert!(opg_receipt.unwrap().hash.is_some(), "Expected operation hash");

        let batch_receipt = context.get_batch_receipt(0i32)?;
        assert!(batch_receipt.is_some(), "Expected batch receipt");

        let head = context.get_head()?;
        assert_eq!(0, head.level);

        Ok(())
    }
}