pub mod context;
pub mod validator;
pub mod error;
pub mod executor;
pub mod crypto;

use host::{
    input::Input,
    rollup_core::{RawRollupCore, MAX_INPUT_MESSAGE_SIZE},
    runtime::Runtime,
};
use debug::debug_str;

use crate::context::EphemeralContext;
use crate::validator::{parse_operation, validate_operation};
use crate::executor::execute_operation;
use crate::crypto::operation_hash;
use crate::error::{Result, Error};

fn process_message<'a>(
    host: &mut impl Runtime, 
    context: &mut EphemeralContext, 
    payload: &'a [u8],
    level: &i32,
    index: &i32
) -> Result<()> {
    let opg = parse_operation(payload)?;
    let opg = validate_operation(host, context, opg)?;
    let mut receipt = execute_operation(host, context, &opg)?;
    receipt.hash = Some(operation_hash(payload)?);
    context.store_operation_receipt(level, index, &receipt)?;
    context.commit(host)?;
    Ok(())
}

pub fn tez_kernel_run<Host: RawRollupCore>(host: &mut Host) {
    #[cfg(any(test, feature = "repl"))]
    {
        use host::path::RefPath;
        use tezos_core::types::mutez::Mutez;

        host.store_write(
            &RefPath::assert_from(b"/context/contracts/tz1grSQDByRpnVs7sPtaprNZRp531ZKz6Jmm/balance"), 
            Mutez::from(4_000_000_000u32).to_bytes().unwrap().as_slice(), 
            0
        ).expect("Failed to initialize seed account");
        debug_str!(Host, "Seed account tz1grSQDByRpnVs7sPtaprNZRp531ZKz6Jmm initialized");
    }
    let mut context = EphemeralContext::new();
    let res = loop {
        match host.read_input(MAX_INPUT_MESSAGE_SIZE) {
            Ok(Some(Input::Message(message))) => {
                match message.as_ref() {
                    b"\x00\x01" => (),  // Start of level
                    b"\x00\x02" => break Ok(()),  // End of level
                    [b'\x01', payload @ ..] => {
                        if let Err(error) = process_message(
                            host, &mut context, 
                            payload,
                            &message.level,
                            &message.id
                        ) {
                            debug_str!(Host, error.to_string().as_str());
                            context.clear();
                        }
                    },
                    _ => break validation_error!("Unexpected input")
                }
            },
            Ok(Some(Input::Slot(_message))) => todo!("handle slot message"),
            Ok(None) => break Ok(()),
            Err(err) => break Err(Error::from(err))
        }
    };
    if let Err(error) = res {
        debug_str!(Host, error.to_string().as_str());
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
    use crate::error::Result;

    use mock_runtime::host::{MockHost, check_debug_log};
    use host::rollup_core::Input;
    use hex;

    #[test]
    fn send_tez() -> Result<()> {
        let mut host = MockHost::default();
        let mut context = EphemeralContext::new();
        let input = hex::decode("62fd30ac16979d9b88aca559e8fd8b97abd2519bebe09ad8a269d60df0b17ddc6b\
            00e8b36c80efb51ec85a14562426049aa182a3ce38f902e18a18e807000017143f62ff9c2f41b30ee00b8c64d233fda43adf05\
            eb829cfd2e733ee9a8f44b6c00e8b36c80efb51ec85a14562426049aa182a3ce3800e28a18ab0b8102c0843d00006b82198cb1\
            79e8306c1bedd08f12dc863f32888600b2014573fd63d27895841ea6ca9d45e23e1e3b836298801b5e390b3b0a0b412003af89\
            c08e63b6d8cf6847300e627c4ce0882ce4e2b842295309de3a0bd6260f").unwrap();
        let level = 10;
        host.as_mut().set_ready_for_input(level);
        host
            .as_mut()
            .add_next_inputs(10, vec![(Input::MessageData, input)].iter());

        tez_kernel_run(&mut host);

        check_debug_log(|debug_log| {
            assert!(!debug_log.is_empty());
        });

        let receipt = context.get_operation_receipt(&host, &level, &0i32)?;
        // println!("Receipt: {:#?}", receipt);
        assert!(receipt.is_some(), "Expected operation receipt");
        assert!(receipt.unwrap().hash.is_some(), "Expected operation hash");

        Ok(())
    }
}