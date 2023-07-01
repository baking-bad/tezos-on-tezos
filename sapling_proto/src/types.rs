// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use crate::storage::Ciphertext;
pub use zcash_primitives::sapling::{
    note::ExtractedNoteCommitment as Commitment,
    redjubjub::{PublicKey, Signature},
    value::ValueCommitment,
    Node as CommitmentNode, Nullifier,
};

pub const HASH_SIZE: usize = 32;

pub type Hash = [u8; HASH_SIZE];
pub type Proof = bellman::groth16::Proof<bls12_381::Bls12>;

#[derive(Clone, Debug)]
pub struct Input {
    pub cv: ValueCommitment,
    pub nf: Nullifier,
    pub rk: PublicKey,
    pub proof_i: Proof,
    pub signature: Signature,
    pub sig_payload: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct Output {
    pub cm: Commitment,
    pub proof_o: Proof,
    pub ciphertext: Ciphertext,
}

#[derive(Clone, Debug)]
pub struct SaplingTransaction {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub binding_sig: Signature,
    pub balance: i64,
    pub root: Hash,          // aka "anchor"
    pub bound_data: Vec<u8>, // packed arbitrary Micheline expression
    pub sig_payload: Vec<u8>,
}
