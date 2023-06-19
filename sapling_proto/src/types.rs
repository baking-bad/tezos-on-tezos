pub use zcash_primitives::sapling::{
    note::ExtractedNoteCommitment as Commitment,
    redjubjub::{PublicKey, Signature},
    value::ValueCommitment,
    Nullifier,
};

pub const PAYLOAD_OUT_SIZE: usize = 80;
pub const NONCE_SIZE: usize = 24;
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
pub struct Ciphertext {
    pub cv: ValueCommitment,
    pub epk: PublicKey,
    pub payload_enc: Vec<u8>,
    pub nonce_enc: [u8; NONCE_SIZE],
    pub payload_out: [u8; PAYLOAD_OUT_SIZE],
    pub nonce_out: [u8; NONCE_SIZE],
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
    pub root: Hash, // aka "anchor"
    pub bound_data: String,
    pub sig_payload: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct SaplingDiff {
    pub commitments_and_ciphertexts: Vec<(Commitment, Ciphertext)>,
    pub nullifiers: Vec<Nullifier>,
}

#[derive(Clone, Debug)]
pub struct SaplingState {
    pub id: Option<i64>,
    pub diff: SaplingDiff,
    pub memo_size: i64,
}
