pub mod ephemeral;
pub mod error;
pub mod generic;
pub mod store;

#[cfg(feature = "kernel")]
pub mod kernel;

#[cfg(feature = "tezos")]
pub mod tezos;

pub use crate::{
    error::{Error, Result},
    store::{LayeredStore, StoreBackend, StoreType},
};

pub type EphemeralStore = LayeredStore<ephemeral::EphemeralBackend>;

#[cfg(feature = "kernel")]
pub type KernelStore<'rt, Host> = LayeredStore<kernel::KernelBackend<'rt, Host>>;
