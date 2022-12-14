use tezos_core::{
    internal::crypto::Crypto,
    types::encoded::{OperationHash, Encoded},
    Result
};

pub fn operation_hash<'a>(payload: &'a [u8]) -> Result<OperationHash> {
    let crypto = Crypto::new(None, None, None);
    let hash = crypto.blake2b(payload, 32)?;
    OperationHash::from_bytes(&hash).map_err(|e| e.into())
}
