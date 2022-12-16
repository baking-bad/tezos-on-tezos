pub mod context;

use host::{
    input::Input,
    rollup_core::MAX_INPUT_MESSAGE_SIZE,
    runtime::Runtime,
};
use tez_proto::{
    context::Context,
    validator::{parse_operation, validate_operation},
    executor::execute_operation,
    crypto::operation_hash,
    error::Result
};

use crate::context::PVMContext;

pub fn debug_log(message: String) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        host::rollup_core::write_debug(message.as_ptr(), message.len())
    };
    #[cfg(not(target_arch = "wasm32"))]
    {
        eprintln!("[DEBUG] {}", message);
    };
}

macro_rules! debug_msg {
    ($($arg:tt)*) => {
        debug_log(format!($($arg)*))
    };
}

fn process_message<'a>( 
    context: &mut impl Context, 
    payload: &'a [u8],
    level: &i32,
    index: &i32
) -> Result<String> {
    let opg = parse_operation(payload)?;
    let opg = validate_operation(context, opg)?;
    let mut receipt = execute_operation(context, &opg)?;
    let opg_hash = operation_hash(payload)?;
    receipt.hash = Some(opg_hash.clone());
    context.store_operation_receipt(level, index, &receipt)?;
    context.commit()?;
    Ok(opg_hash.into())
}

pub fn tez_kernel_run<Host: Runtime>(context: &mut PVMContext<Host>) {
    #[cfg(any(test, feature = "repl"))]
    {
        use host::path::RefPath;
        const SEED_BALANCE_VALUE: &[u8] = b"\x80\xd0\xac\xf3\x0e";

        context.as_mut().store_write(
            &RefPath::assert_from(b"/context/contracts/tz1grSQDByRpnVs7sPtaprNZRp531ZKz6Jmm/balance"), 
            SEED_BALANCE_VALUE, 
            0
        ).expect("Failed to initialize seed account");
        debug_msg!("Seed account tz1grSQDByRpnVs7sPtaprNZRp531ZKz6Jmm initialized");
    }

    let mut index = 0;
    let res = loop {
        match context.as_mut().read_input(MAX_INPUT_MESSAGE_SIZE) {
            Ok(Some(Input::Message(message))) => {
                match message.as_ref() {
                    b"\x00\x01" => (),  // Start of level
                    b"\x00\x02" => break Ok(()),  // End of level
                    [b'\x01', payload @ ..] => {
                        match process_message(context, payload, &message.level, &index) {
                            Err(error) => {
                                debug_msg!("Failed to process message #{} at level {}: {:?}", message.id, message.level, error);
                                context.clear();
                            },
                            Ok(hash) => {
                                debug_msg!("Operation {} included in block {} with index {}", hash, message.level, index);
                                index += 1;
                            }
                        }
                    },
                    _ => debug_msg!("Foreign message #{}", message.id)
                }
            },
            Ok(Some(Input::Slot(_message))) => todo!("handle slot message"),
            Ok(None) => break Ok(()),
            Err(err) => break Err(err)
        }
    };
    if let Err(error) = res {
        debug_msg!("{:?}", error);
    }
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn kernel_run() {
    let mut context = PVMContext::new(unsafe { host::wasm_host::WasmHost::new() });
    tez_kernel_run(&mut context);
}

#[cfg(test)]
mod test {
    use crate::tez_kernel_run;
    use crate::context::PVMContext;

    use tez_proto::context::Context;
    use tez_proto::error::Result;
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
            .add_next_inputs(10, vec![(Input::MessageData, input)].iter());

        tez_kernel_run(&mut context);

        let receipt = context.get_operation_receipt(&level, &0i32)?;
        // println!("Receipt: {:#?}", receipt);
        assert!(receipt.is_some(), "Expected operation receipt");
        assert!(receipt.unwrap().hash.is_some(), "Expected operation hash");

        Ok(())
    }
}