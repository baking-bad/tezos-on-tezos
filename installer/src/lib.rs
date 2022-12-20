#![cfg_attr(all(not(test), not(feature = "std")), no_std)]

pub mod installer;

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn kernel_run() {
    debug_str!("Installer kernel invoked");
    crate::installer::install_kernel(include_bytes!("../../.bin/wasm_2_0_0/root_hash.bin"));
}

#[cfg_attr(all(target_arch = "wasm32", not(feature = "std")), panic_handler)]
#[no_mangle]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(msg) = info.payload().downcast_ref::<&str>() {
        debug_str!(msg);
    }
    panic!()
}
