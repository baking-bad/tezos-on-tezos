// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

pub mod ephemeral;
pub mod error;
pub mod store;
pub mod types;

pub use crate::{
    error::{Error, Result},
    store::{LayeredStore, StoreBackend, StoreType},
};

pub type EphemeralStore = LayeredStore<ephemeral::EphemeralBackend>;
