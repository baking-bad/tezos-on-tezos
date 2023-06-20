pub mod error;
pub mod inbox;
pub mod kernel;

pub use error::{Error, Result};

tezos_smart_rollup::kernel_entry!(crate::kernel::kernel_run);
