pub mod context;
pub mod error;
pub mod inbox;
pub mod kernel;
pub mod store;

pub use error::{Error, Result};

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn kernel_run() {
    let mut context = crate::context::PVMContext::new(unsafe { host::wasm_host::WasmHost::new() });
    crate::kernel::kernel_run(&mut context);
}
