pub mod context;
pub mod error;
pub mod inbox;
pub mod kernel;
pub mod store;

pub use error::{Error, Result};

#[cfg(target_arch = "wasm32")]
fn panic_hook(info: &core::panic::PanicInfo) {
    let message =
        if let Some(message) = info.payload().downcast_ref::<std::string::String>() {
            format!("Kernel panic {:?} at {:?}", message, info.location())
        } else {
            let message = info.payload().downcast_ref::<&str>();
            format!("Kernel panic {:?} at {:?}", message, info.location())
        };
    
    unsafe {
        host::rollup_core::write_debug(message.as_ptr(), message.len());
    }

    std::process::abort();
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn kernel_run() {
    let mut context = crate::context::PVMContext::new(unsafe { host::wasm_host::WasmHost::new() });
    std::panic::set_hook(Box::new(panic_hook));
    crate::kernel::kernel_run(&mut context);
}
