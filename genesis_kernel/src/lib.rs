#![cfg_attr(all(not(test), not(feature = "std")), no_std)]

pub mod installer;

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn kernel_run() {
    use installer::install_kernel;
    install_kernel(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
}

#[cfg_attr(all(target_arch = "wasm32", not(feature = "std")), panic_handler)]
#[no_mangle]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    panic!()
}