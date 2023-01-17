// SPDX-FileCopyrightText: 2022 TriliTech <contact@trili.tech>
// SPDX-FileCopyrightText: 2022 PK Lab <info@pklab.io>
// SPDX-License-Identifier: MIT

//! The *Preimage Installer* installs a kernel, that is too large to be originated directly.
//!
//! It does so leveraging both *DAC* and the [reveal preimage] mechanism.
//!
//! At its limit, 3 levels of DAC pages gives us enough content for >7GB, which is already
//! far in excess of the likely max limit on WASM modules that the PVM can support.
//! Store values alone are limited to ~2GB.
//!
//! [reveal preimage]: host::rollup_core::reveal_preimage
//!
const PREIMAGE_HASH_SIZE: usize = 33;
const MAX_PAGE_SIZE: usize = 4096;
const MAX_FILE_CHUNK_SIZE: usize = 2048;
const MAX_DAC_LEVELS: usize = 4;
const KERNEL_PATH: &[u8] = b"/kernel/boot.wasm";
const PREPARE_KERNEL_PATH: &[u8] = b"/installer/kernel/boot.wasm";
const REBOOT_PATH: &[u8] = b"/kernel/env/reboot";

#[cfg(not(test))]
pub mod host {
    #[link(wasm_import_module = "smart_rollup_core")]
    extern "C" {
        pub fn store_write(
            path: *const u8,
            path_len: usize,
            offset: usize,
            src: *const u8,
            num_bytes: usize,
        ) -> i32;

        pub fn store_move(
            from_path: *const u8,
            from_path_len: usize,
            to_path: *const u8,
            to_path_len: usize,
        ) -> i32;

        pub fn reveal_preimage(
            hash_addr: *const u8,
            hash_len: usize,
            destination_addr: *mut u8,
            max_bytes: usize,
        ) -> i32;

        pub fn write_debug(src: *const u8, num_bytes: usize);
    }
}

#[macro_export]
macro_rules! debug_str {
    ($msg: expr) => {
        unsafe {
            crate::installer::host::write_debug(($msg as &str).as_ptr(), ($msg as &str).len());
        }
    };
}

fn fetch_page<'a>(
    hash: &[u8; PREIMAGE_HASH_SIZE],
    buffer: &'a mut [u8],
) -> (u8, &'a mut [u8], &'a mut [u8]) {
    let page_size = unsafe {
        host::reveal_preimage(hash.as_ptr(), hash.len(), buffer.as_mut_ptr(), buffer.len())
    };
    if page_size < 0 {
        panic!("Fetch page: failed to reveal preimage {}", page_size);
    } else if page_size < 5 {
        // tag + prefix
        panic!("Fetch page: too small {}", page_size);
    }

    let (page, rest) = buffer.split_at_mut(MAX_PAGE_SIZE);
    if page[0] > 1 {
        panic!("Fetch page: invalid tag {}", page[0]);
    }

    let data_size = u32::from_be_bytes([page[1], page[2], page[3], page[4]]) as usize;
    let end_offset = 5 + data_size;

    if page_size < end_offset.try_into().unwrap()
        || (page[0] == 1 && data_size % PREIMAGE_HASH_SIZE != 0)
    {
        panic!("Fetch page: invalid size prefix");
    }
    (page[0], &mut page[5..end_offset], rest)
}

fn write_content(kernel_size: &mut usize, content: &[u8]) {
    use core::ops::AddAssign;
    let size = unsafe {
        host::store_write(
            PREPARE_KERNEL_PATH.as_ptr(),
            PREPARE_KERNEL_PATH.len(),
            *kernel_size,
            content.as_ptr(),
            content.len(),
        )
    };
    if size < 0 {
        panic!("Write content: failed {}", size);
    }
    kernel_size.add_assign(content.len());
}

fn reveal_loop(
    level: usize,
    hash: &[u8; PREIMAGE_HASH_SIZE],
    rest: &mut [u8],
    kernel_size: &mut usize,
) {
    if level >= MAX_DAC_LEVELS {
        panic!("Reveal loop: DAC preimage tree contains too many levels");
    }
    match fetch_page(hash, rest) {
        (0, content, _) => {
            for chunk in content.chunks(MAX_FILE_CHUNK_SIZE).into_iter() {
                write_content(kernel_size, chunk)
            }
        }
        (1, hashes, rest) => {
            for hash in hashes.chunks_exact(PREIMAGE_HASH_SIZE).into_iter() {
                reveal_loop(
                    level + 1,
                    hash.try_into().expect("Invalid preimage hash"),
                    rest,
                    kernel_size,
                );
            }
        }
        _ => panic!("Reveal loop: unexpected data"),
    }
}

pub fn install_kernel(root_hash: &[u8; PREIMAGE_HASH_SIZE]) {
    let mut buffer = [0; MAX_PAGE_SIZE * MAX_DAC_LEVELS];
    let mut kernel_size = 0;
    reveal_loop(0, root_hash, buffer.as_mut_slice(), &mut kernel_size);

    let size = unsafe {
        host::store_move(
            PREPARE_KERNEL_PATH.as_ptr(),
            PREPARE_KERNEL_PATH.len(),
            KERNEL_PATH.as_ptr(),
            KERNEL_PATH.len(),
        )
    };
    if size < 0 {
        panic!("Install kernel: failed to swap {}", size);
    }

    debug_str!("Kernel successfully installed, rebooting");
    unsafe {
        host::store_write(
            REBOOT_PATH.as_ptr(),
            REBOOT_PATH.len(),
            0,
            [0_u8].as_ptr(),
            1,
        );
    }
}

#[cfg(test)]
pub mod host {
    use super::PREIMAGE_HASH_SIZE;
    use core::slice::{from_raw_parts, from_raw_parts_mut};
    use mock_runtime::host::MockHost;
    use once_cell::sync::Lazy;
    pub static mut HOST: Lazy<MockHost> = Lazy::new(|| MockHost::default());

    pub unsafe fn store_write(
        path: *const u8,
        path_len: usize,
        offset: usize,
        src: *const u8,
        num_bytes: usize,
    ) -> i32 {
        let path = from_raw_parts(path, path_len);
        let bytes = from_raw_parts(src, num_bytes);
        HOST.as_mut().handle_store_write(path, offset, bytes)
    }

    pub unsafe fn store_move(
        from_path: *const u8,
        from_path_len: usize,
        to_path: *const u8,
        to_path_len: usize,
    ) -> i32 {
        let from_path = from_raw_parts(from_path, from_path_len);
        let to_path = from_raw_parts(to_path, to_path_len);
        HOST.as_mut().handle_store_move(from_path, to_path);
        0
    }

    pub unsafe fn reveal_preimage(
        hash_addr: *const u8,
        hash_len: usize,
        destination_addr: *mut u8,
        max_bytes: usize,
    ) -> i32 {
        let hash = from_raw_parts(hash_addr, hash_len)
            .try_into()
            .unwrap_or_else(|_| panic!("Hash is not {} bytes", PREIMAGE_HASH_SIZE));

        let preimage = HOST.as_mut().store.retrieve_preimage(hash);
        let bytes = if preimage.len() < max_bytes {
            &preimage
        } else {
            &preimage[0..max_bytes]
        };

        let slice = from_raw_parts_mut(destination_addr, bytes.len());
        slice.copy_from_slice(bytes);

        bytes.len().try_into().unwrap()
    }

    pub unsafe fn write_debug(src: *const u8, num_bytes: usize) {
        let msg = from_raw_parts(src, num_bytes).to_vec();
        eprintln!("[DEBUG] {}", String::from_utf8(msg).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;
    use mock_runtime::state::HostState;
    use tezos_encoding::enc::BinWriter;
    use tezos_rollup_encoding::dac::{Page, V0ContentPage, V0HashPage, MAX_PAGE_SIZE};

    fn prepare_preimages(state: &mut HostState, input: &[u8]) -> [u8; PREIMAGE_HASH_SIZE] {
        let content_pages = V0ContentPage::new_pages(input)
            .map(Page::V0ContentPage)
            .map(|page| {
                let mut buffer = Vec::with_capacity(MAX_PAGE_SIZE);
                page.bin_write(&mut buffer).expect("can serialize");
                buffer
            });

        let mut hashes = Vec::with_capacity(input.len() / V0ContentPage::MAX_CONTENT_SIZE + 1);

        for page in content_pages {
            assert!(page.len() <= 4096);
            let hash = state.set_preimage(page);
            hashes.push(hash);
        }

        let mut hash_pages: Vec<_> = V0HashPage::new_pages(&hashes).collect();
        assert_eq!(1, hash_pages.len(), "expected single hash page");

        let hash_page = hash_pages.remove(0);

        let mut root_page = Vec::with_capacity(MAX_PAGE_SIZE);
        Page::V0HashPage(hash_page)
            .bin_write(&mut root_page)
            .expect("cannot serialize hash page");

        let root_hash = state.set_preimage(root_page);
        root_hash
    }

    #[test]
    pub fn installer_sets_correct_kernel() {
        let mut kernel: Vec<u8> = Vec::with_capacity(40_000);
        for i in 0..10000 {
            kernel.extend_from_slice(u16::to_le_bytes(i).as_slice());
        }

        let root_hash = unsafe { prepare_preimages(host::HOST.as_mut(), &kernel) };

        let mut root_hex_hash = [0; PREIMAGE_HASH_SIZE * 2];
        hex::encode_to_slice(&root_hash, root_hex_hash.as_mut_slice())
            .expect("hex encoding should work");

        install_kernel(&root_hash);

        let installed_kernel: Vec<u8> = unsafe {
            host::HOST
                .as_mut()
                .store
                .get_value("/durable/kernel/boot.wasm")
        };
        assert_eq!(
            installed_kernel.len(),
            kernel.len(),
            "Expected same kernel size."
        );
        assert_eq!(installed_kernel, kernel, "Expected kernel to be installed.");
    }
}
