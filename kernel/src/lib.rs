pub mod context;
pub mod error;
pub mod kernel;
pub mod inbox;
pub mod store;

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn kernel_run() {
    let mut context = crate::context::PVMContext::new(unsafe { host::wasm_host::WasmHost::new() });
    crate::kernel::kernel_run(&mut context);
}

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

#[macro_export]
macro_rules! debug_msg {
    ($($arg:tt)*) => {
        crate::debug_log(format!($($arg)*))
    };
}