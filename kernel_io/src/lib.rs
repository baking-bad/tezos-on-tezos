// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

pub mod error;
pub mod inbox;
pub mod store;

pub use crate::error::{Error, Result};

pub use store::KernelBackendAsHost as KernelStoreAsHost;
pub type KernelStore<'rt, Host> = layered_store::LayeredStore<store::KernelBackend<'rt, Host>>;
