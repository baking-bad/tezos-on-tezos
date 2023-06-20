pub mod ephemeral;
pub mod error;
pub mod store;

#[cfg(any(test, feature = "kernel"))]
pub mod kernel;

pub use crate::{
    ephemeral::EphemeralStore,
    error::{Error, Result},
    store::{LayeredStore, StoreType},
};
