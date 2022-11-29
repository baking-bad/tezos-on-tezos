pub mod tezos;

use debug::debug_msg;
use kernel::kernel_entry;
use host::{
    input::{Input},
    rollup_core::{RawRollupCore, MAX_INPUT_MESSAGE_SIZE},
    runtime::Runtime,
};
use hex;
use tezos_operation::operations::{UnsignedOperation};


pub fn tez_kernel_run<Host: RawRollupCore>(host: &mut Host) {
    match host.read_input(MAX_INPUT_MESSAGE_SIZE) {
        Ok(Some(Input::Message(message))) => {
            debug_msg!(
                Host,
                "Processing MessageData {} at level {}",
                message.id,
                message.level
            );
            
            if let Err(err) = process_payload(host, message.as_ref()) {
                debug_msg!(Host, "Error processing payload {}", err);
            }
        }
        Ok(Some(Input::Slot(_message))) => todo!("handle slot message"),
        Ok(None) => {},
        Err(_) => todo!("handle runtime errors")
    }
}

fn process_payload<'a, Host: RawRollupCore>(host: &mut Host, payload: &'a [u8]) -> Result<(), tezos_operation::Error> {
    println!("payload: {}", hex::encode(payload));

    let signature = &payload[payload.len() - 64..].to_vec();
    println!("signature: {:?}", hex::encode(signature));

    let body = &payload[..payload.len() - 64].to_vec();
    println!("body: {:?}", hex::encode(signature));

    let opg = UnsignedOperation::from_forged_bytes(body)?;
    println!("branch: {:?}", opg.branch);

    for content in opg.contents.iter() {
        println!("{:?}", content);
    }

    Ok(())
}

kernel_entry!(tez_kernel_run);

#[cfg(test)]
mod test {
    use crate::tez_kernel_run;
    use mock_runtime::host::{MockHost, check_debug_log};
    use tezos_operation::operations::{SignedOperation, Transaction, Operation};
    use tezos_core::types::{
        encoded::{Encoded}
    };
    use host::rollup_core::Input;

    #[test]
    fn send_tez() {
        let mut mock_runtime = MockHost::default();

        let message = SignedOperation::new(
            "BMNvSHmWUkdonkG2oFwwQKxHUdrYQhUXqxLaSRX9wjMGfLddURC".try_into().unwrap(),
            vec![
                Transaction::new(
                    "tz1V3dHSCJnWPRdzDmZGCZaTMuiTmbtPakmU".try_into().unwrap(),
                    417u32.into(),
                    2336132u32.into(),
                    1527u32.into(),
                    357u32.into(),
                    498719u32.into(),
                    "tz1d5Dr3gjsxQo5XNbjAj558mLy3nGGQgMFA".try_into().unwrap(),
                    None
                ).into()
            ],
            "sigw1WNdYweqz1c7zKcvZFHQ18swSv4HBWje5quRmixxitPk7z8jtY63qXgKLPVfTM6XGxExPatBWJP44Bknyu3hDHDKJZgY".try_into().unwrap()
        );
        
        let forged_bytes = message.to_forged_bytes();
        let signature_bytes = message.signature.to_bytes();
        let input: Vec<u8> = [forged_bytes.unwrap(), signature_bytes.unwrap()].concat();

        let level = 10;
        mock_runtime.as_mut().set_ready_for_input(level);
        mock_runtime
            .as_mut()
            .add_next_inputs(10, vec![(Input::MessageData, input)].iter());

        tez_kernel_run(&mut mock_runtime);

        check_debug_log(|debug_log| {
            assert!(!debug_log.is_empty());
        });
    }
}