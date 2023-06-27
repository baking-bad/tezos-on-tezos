// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

pub mod error;
pub mod kernel;
pub mod payload;

pub use error::{Error, Result};

tezos_smart_rollup_entrypoint::kernel_entry!(crate::kernel::kernel_run);
