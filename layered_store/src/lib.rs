pub mod store;
pub mod error;
pub mod ephemeral;

#[cfg(any(test, feature = "kernel"))]
pub mod kernel;

pub use crate::{
    store::{LayeredStore, StoreType},
    ephemeral::EphemeralStore,
    error::{Error, Result}
};
