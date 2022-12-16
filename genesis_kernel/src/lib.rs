#![cfg_attr(all(not(test), not(feature = "std")), no_std)]

pub mod installer;

use installer::host;

const SEED_BALANCE_KEYS: [&[u8; 63]; 8] = [
    b"/context/contracts/tz1grSQDByRpnVs7sPtaprNZRp531ZKz6Jmm/balance",  // Pytezos built-in key
    b"/context/contracts/tz1VSUr8wwNhLAzempoch5d6hLRiTh8Cjcjb/balance",  // Alice from Flextesa
    b"/context/contracts/tz1TGu6TN5GSez2ndXXeDX6LgUDvLzPLqgYV/balance",  // Activator from Tezos sandbox
    b"/context/contracts/tz1KqTpEZ7Yob7QbPE4Hy4Wo8fHG8LhKxZSx/balance",  // Bootstrap 1 from Tezos sandbox
    b"/context/contracts/tz1gjaF81ZRRvdzjobyfVNsAeSC6PScjfQwN/balance",  // Bootstrap 2 from Tezos sandbox
    b"/context/contracts/tz1faswCTDciRzE4oJ9jn2Vm2dvjeyA9fUzU/balance",  // Bootstrap 3 from Tezos sandbox
    b"/context/contracts/tz1b7tUupMgCNw2cCLpKTkSD1NZzB5TkP2sv/balance",  // Bootstrap 4 from Tezos sandbox
    b"/context/contracts/tz1ddb9NMYHZi5UzPdzTZMYQQZoMub195zgv/balance",  // Bootstrap 5 from Tezos sandbox
];
const SEED_BALANCE_VALUE: &[u8] = b"\x80\xd0\xac\xf3\x0e";  // 4,000,000,000 mutez

fn seed_accounts(keys: &[&[u8; 63]], balance: &[u8]) {
    for path in keys.iter() {
        let size = unsafe { 
            host::store_write(
                path.as_ptr(), 
                path.len(), 0, 
                balance.as_ptr(), 
                balance.len()
            )
        };
        if size < 0 {
            panic!("Failed to write value at {:?}", path);
        }
    }
    debug_str!("Seed accounts successfully initialized");
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub extern "C" fn kernel_run() {
    use installer::install_kernel;
    debug_str!("Genesis kernel invoked");
    install_kernel(include_bytes!("../../.bin/wasm_2_0_0/root_hash.bin"));
    seed_accounts(SEED_BALANCE_KEYS.as_slice(), SEED_BALANCE_VALUE);
}

#[cfg_attr(all(target_arch = "wasm32", not(feature = "std")), panic_handler)]
#[no_mangle]
fn panic(info: &core::panic::PanicInfo) -> ! {
    if let Some(msg) = info.payload().downcast_ref::<&str>() {
        debug_str!(msg);
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tezos_core::types::mutez::Mutez;

    #[test]
    pub fn installer_seeds_balance() {
        seed_accounts(SEED_BALANCE_KEYS.as_slice(), SEED_BALANCE_VALUE);

        let balance: Vec<u8> = unsafe {
            installer::host::HOST
                .as_mut()
                .store
                .get_value("/durable/context/contracts/tz1ddb9NMYHZi5UzPdzTZMYQQZoMub195zgv/balance")
        };
        assert_eq!(Mutez::from(4000000000u32), Mutez::from_bytes(balance.as_slice()).unwrap());
    }
}
