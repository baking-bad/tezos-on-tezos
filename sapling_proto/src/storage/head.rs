// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

pub const DEFAULT_MEMO_SIZE: usize = 8;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SaplingHead {
    pub roots_pos: usize,
    pub nullifiers_size: usize,
    pub commitments_size: usize,
    pub memo_size: usize,
}

impl SaplingHead {
    pub fn new(memo_size: usize) -> Self {
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
