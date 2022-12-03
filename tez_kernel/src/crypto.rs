use tezos_core::{CryptoProvider, CryptoConfig, Result, Error};
use tezos_core::{
    internal::crypto::Crypto,
    types::encoded::{OperationHash, Encoded}
};
use ed25519_dalek::{Signature, PublicKey, Verifier};

pub struct WasmCryptoConfig;

impl CryptoConfig for WasmCryptoConfig {
    fn get_ed25519_crypto_provider(&self) -> Option<Box<dyn CryptoProvider>> {
        Some(Box::from(Ed25519CryptoProvider))
    }

    fn get_secp256_k1_crypto_provider(&self) -> Option<Box<dyn CryptoProvider>> {
        None
    }

    fn get_p256_crypto_provider(&self) -> Option<Box<dyn CryptoProvider>> {
        None
    }
}

#[derive(Debug)]
pub struct Ed25519CryptoProvider;
impl CryptoProvider for Ed25519CryptoProvider {
    fn sign(&self, _message: &[u8], _secret: &[u8]) -> Result<Vec<u8>> {
        todo!("Not required in the scope of the kernel")
    }

    fn verify(&self, message: &[u8], signature: &[u8], public_key: &[u8]) -> Result<bool> {
        let public_key = PublicKey::from_bytes(public_key).map_err(|_| Error::InvalidPublicKeyBytes)?;
        let signature = Signature::from_bytes(signature).map_err(|_| Error::InvalidSignatureBytes)?;
        Ok(public_key.verify(message, &signature).is_ok())
    }
}

pub fn operation_hash<'a>(payload: &'a [u8]) -> Result<OperationHash> {
    let crypto = Crypto::new(None, None, None);
    let hash = crypto.blake2b(payload, 32)?;
    OperationHash::from_bytes(&hash).map_err(|e| e.into())
}

#[cfg(test)]
mod test {
    use super::{Ed25519CryptoProvider, Result};
    use tezos_core::types::{
        encoded::{Signature, Ed25519PublicKey, Encoded}
    };
    use tezos_core::internal::crypto::Crypto;

    #[test]
    fn test_ed25519_verify() -> Result<()> {
        let cp = Crypto::new(
            Some(Box::from(Ed25519CryptoProvider)), 
            None, 
            None
        );

        let values: Vec<(&'static str, &'static str, &'static [u8])> = vec![
            ("edpku976gpuAD2bXyx1XGraeKuCo1gUZ3LAJcHM12W1ecxZwoiu22R",
            "edsigtzLBGCyadERX1QsYHKpwnxSxEYQeGLnJGsSkHEsyY8vB5GcNdnvzUZDdFevJK7YZQ2ujwVjvQZn62ahCEcy74AwtbA8HuN",
            b"test")
        ];

        for (pk, sig, msg) in values {
            let public_key = Ed25519PublicKey::try_from(pk).unwrap().to_bytes()?;
            let signature = Signature::try_from(sig).unwrap().to_bytes()?;
            let message = cp.blake2b(msg, 32)?;  // this is Tezos-specific step, be careful
            let result = cp.verify_ed25519(&message, &signature, &public_key)?;
            assert!(result);
        }

        Ok(())
    }
}