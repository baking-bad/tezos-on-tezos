// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use zcash_primitives::sapling::{redjubjub::PublicKey, value::ValueCommitment};

pub const PAYLOAD_OUT_SIZE: usize = 80;
pub const NONCE_SIZE: usize = 24;

#[derive(Clone, Debug)]
pub struct Ciphertext {
    pub cv: ValueCommitment,
    pub epk: PublicKey,
    pub payload_enc: Vec<u8>,
    pub nonce_enc: [u8; NONCE_SIZE],
    pub payload_out: [u8; PAYLOAD_OUT_SIZE],
    pub nonce_out: [u8; NONCE_SIZE],
}

impl Ciphertext {
    // Payload contains fixed length fields and a variable size memo.
    // The fixed length things are diversifier, amount, rcm and a message
    // authentication code and 4 bytes added by the encoding for the length
    // of the variable length field memo.
    pub fn get_memo_size(&self) -> usize {
        const DIVERSIFIER_SIZE: usize = 11;
        const AMOUNT_SIZE: usize = 8;
        const RCM_SIZE: usize = 32;
        const TAG_SIZE: usize = 16;
        self.payload_enc.len() - (DIVERSIFIER_SIZE + AMOUNT_SIZE + RCM_SIZE + TAG_SIZE + 4)
    }
}
