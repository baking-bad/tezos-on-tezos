extern crate kernel;
extern crate debug;
extern crate kernel_core;

use host::rollup_core::{ RawRollupCore };
use debug::debug_msg;
use kernel::kernel_entry;
use kernel_core::memory::{ Memory };

pub fn tez_kernel_run<Host: RawRollupCore>(host: &mut Host) {
    let mut memory = Memory::load_memory(host);
    debug_msg!(Host, "Hello, Peter");
    memory.snapshot(host);
}

kernel_entry!(tez_kernel_run);
