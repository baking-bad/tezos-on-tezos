// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

pub mod ciphertext;
pub mod head;
pub mod sapling;
pub mod store;

pub const MAX_ROOTS: u64 = 120;
pub const MAX_HEIGHT: u8 = 32;

pub use ciphertext::Ciphertext;
pub use head::SaplingHead;
pub use sapling::SaplingStorage;

pub type SaplingEphemeralStorage = layered_store::EphemeralStore;
