pub mod context;
pub mod validator;
pub mod error;
pub mod executor;
pub mod crypto;

use host::{
    input::{Input},
    rollup_core::{RawRollupCore, MAX_INPUT_MESSAGE_SIZE},
    runtime::Runtime,
};

use crate::context::EphemeralContext;
use crate::validator::{parse_operation, validate_operation};
use crate::executor::execute_operation;


pub fn tez_kernel_run<Host: RawRollupCore>(host: &mut Host) {
    let mut context = EphemeralContext::new();

    match host.read_input(MAX_INPUT_MESSAGE_SIZE) {
        Ok(Some(Input::Message(message))) => {          
            if let Ok(opg) = parse_operation(message.as_ref()) {
                if validate_operation(host, &mut context, &opg).is_ok() {
                    match execute_operation(host, &mut context, &opg) {
                        Ok(res) => {
                            context.store_operation_receipt(&message.level, &message.id, &res);
                            context.commit(host);
                        },
                        Err(_) => context.rollback()
                    }
                }
            }
        }
        Ok(Some(Input::Slot(_message))) => todo!("handle slot message"),
        Ok(None) => {},
        Err(_) => todo!("handle runtime errors")
    }
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn kernel_run() {
    let mut host = unsafe { host::wasm_host::WasmHost::new() };
    tez_kernel_run(&mut host)
}

#[cfg(test)]
mod test {
    use crate::tez_kernel_run;
    use crate::context::EphemeralContext;

    use mock_runtime::host::{MockHost};
    use tezos_operation::operations::{SignedOperation, Transaction, Operation};
    use tezos_core::types::{
        encoded::{Encoded, PublicKey},
        mutez::Mutez,
        number::Nat
    };
    use host::rollup_core::Input;

    #[test]
    fn send_tez() {
        let mut host = MockHost::default();
        let mut context = EphemeralContext::new();

        let source = "tz1V3dHSCJnWPRdzDmZGCZaTMuiTmbtPakmU".try_into().unwrap();
        let public_key = PublicKey::try_from("edpktipCJ3SkjvtdcrwELhvupnyYJSmqoXu3kdzK1vL6fT5cY8FTEa").unwrap();
        let initial_balance: Mutez = 10000000u32.into();

        context.set_balance(&source, &initial_balance);
        context.set_public_key(&source, &public_key);
        context.set_counter(&source, &Nat::try_from("100000").unwrap());
        context.commit(&mut host);

        let message = SignedOperation::new(
            "BMNvSHmWUkdonkG2oFwwQKxHUdrYQhUXqxLaSRX9wjMGfLddURC".try_into().unwrap(),
            vec![
                Transaction::new(
                    source.clone(),
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
        host.as_mut().set_ready_for_input(level);
        host
            .as_mut()
            .add_next_inputs(10, vec![(Input::MessageData, input)].iter());

        tez_kernel_run(&mut host);

        // check_debug_log(|debug_log| {
        //     assert!(!debug_log.is_empty());
        // });

        let receipt = context.get_operation_receipt(&host, &10i32, &0i32);
        assert!(receipt.is_some());

        println!("{:?}", receipt);
    }
}