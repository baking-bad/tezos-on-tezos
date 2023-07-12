// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

pub const DEFAULT_MEMO_SIZE: u8 = 8;

#[derive(Clone, Debug, PartialEq)]
pub struct SaplingHead {
    pub roots_pos: u64,
    pub nullifiers_size: u64,  // FIXME: bigint
    pub commitments_size: u64, // FIXME: bigint
    pub memo_size: u8,
}

impl SaplingHead {
    pub fn new(memo_size: u8) -> Self {
        Self {
            roots_pos: 0,
            nullifiers_size: 0,
            commitments_size: 0,
            memo_size,
        }
    }
}

impl Default for SaplingHead {
    fn default() -> Self {
        Self::new(DEFAULT_MEMO_SIZE)
    }
}
