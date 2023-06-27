// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

pub mod ciphertext;
pub mod head;
pub mod sapling;
pub mod store;

pub const MAX_ROOTS: usize = 120;
pub const MAX_HEIGHT: usize = 32;

pub use ciphertext::Ciphertext;
pub use head::SaplingHead;
pub use sapling::SaplingStorage;

pub type SaplingEphemeralStorage = layered_store::EphemeralStore;
