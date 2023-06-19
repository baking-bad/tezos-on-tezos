use bellman::groth16::{prepare_verifying_key, PreparedVerifyingKey, VerifyingKey};
use bls12_381::Bls12;
use lazy_static::lazy_static;
use std::io;

pub fn read_verifying_key(data: &[u8]) -> io::Result<PreparedVerifyingKey<Bls12>> {
    let vk = VerifyingKey::<Bls12>::read(data)?;
    Ok(prepare_verifying_key(&vk))
}

pub struct SaplingParams {
    pub spend_vk: PreparedVerifyingKey<Bls12>,
    pub output_vk: PreparedVerifyingKey<Bls12>,
}

impl SaplingParams {
    pub fn zcash() -> Self {
        let spend_vk = read_verifying_key(include_bytes!("./vk_spend.bin"))
            .expect("Failed to read spend verifying key");
        let output_vk = read_verifying_key(include_bytes!("./vk_output.bin"))
            .expect("Failed to read spend output key");
        Self {
            spend_vk,
            output_vk,
        }
    }
}

lazy_static! {
    pub static ref ZCASH_PARAMS: SaplingParams = SaplingParams::zcash();
}
