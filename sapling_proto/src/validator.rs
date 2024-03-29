// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use anyhow::{bail, Result};

use blake2b_simd::Params;
use zcash_proofs::sapling::SaplingVerificationContext;

pub const MAX_INPUTS: usize = 5208;
pub const MAX_OUTPUTS: usize = 2019;

use crate::{
    formatter::Formatter,
    params::ZCASH_PARAMS,
    storage::SaplingStorage,
    types::{Hash, Input, Output, SaplingTransaction, HASH_SIZE},
};

pub fn get_sighash(payload: &Vec<u8>, anti_replay: &String) -> Result<Hash> {
    let mut blake2b = Params::new();
    let digest = blake2b
        .key(anti_replay.as_bytes())
        .hash_length(HASH_SIZE)
        .hash(payload.as_slice());

    Ok(digest.as_bytes().try_into().unwrap())
}

pub fn check_spend(
    ctx: &mut SaplingVerificationContext,
    input: &Input,
    anchor: &bls12_381::Scalar,
    anti_replay: &String,
) -> Result<bool> {
    let sighash_value = get_sighash(&input.sig_payload, &anti_replay)?;
    let res = ctx.check_spend(
        &input.cv,
        anchor.clone(),
        &input.nf.0,
        input.rk.clone(),
        &sighash_value,
        input.signature.clone(),
        input.proof_i.clone(),
        &ZCASH_PARAMS.spend_vk,
    );
    Ok(res)
}

pub fn check_output(ctx: &mut SaplingVerificationContext, output: &Output) -> Result<bool> {
    let res = ctx.check_output(
        &output.ciphertext.cv,
        output.cm.clone(),
        output.ciphertext.epk.0.clone(),
        output.proof_o.clone(),
        &ZCASH_PARAMS.output_vk,
    );
    Ok(res)
}

pub fn validate_transaction(
    storage: &mut impl SaplingStorage,
    transaction: &SaplingTransaction,
    anti_replay: &String,
) -> Result<()> {
    let mut ctx = SaplingVerificationContext::new(false);
    let head = storage.get_head()?;

    if transaction.inputs.len() >= MAX_INPUTS {
        bail!("Too many inputs: {}", transaction.inputs.len());
    }

    if transaction.outputs.len() >= MAX_OUTPUTS {
        bail!("Too many outputs: {}", transaction.outputs.len());
    }

    if !storage.has_root(&transaction.root)? {
        bail!(
            "Transaction is expired (root = {:?})",
            transaction.root.to_string()
        );
    }

    // Mind the order (first outputs, then inputs) — it influences the final PK for verifying binding sig
    for (idx, output) in transaction.outputs.iter().enumerate() {
        if output.ciphertext.get_memo_size() != head.memo_size {
            bail!("Output #{} has invalid memo size", idx);
        }

        if !check_output(&mut ctx, output)? {
            bail!("Output #{} is not valid", idx);
        }
    }

    let anchor = bls12_381::Scalar::from_bytes(&transaction.root).unwrap();
    for (idx, input) in transaction.inputs.iter().enumerate() {
        if storage.has_nullifier(&input.nf)? {
            bail!(
                "Input #{} nullifier already in use (nf = {})",
                idx,
                input.nf.to_string()
            );
        }

        if !check_spend(&mut ctx, input, &anchor, anti_replay)? {
            bail!("Input #{} is not valid", idx);
        }
    }

    let sighash_value = get_sighash(&transaction.sig_payload, anti_replay)?;
    if !ctx.final_check(
        transaction.balance.try_into().unwrap(),
        &sighash_value,
        transaction.binding_sig,
    ) {
        bail!("Binding signature is invalid");
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use hex;
    use mockall::predicate::*;
    use mockall::*;
    use std::io;
    use zcash_primitives::{
        constants::SPENDING_KEY_GENERATOR,
        sapling::redjubjub::{PublicKey, Signature},
    };

    use crate::{
        storage::{Ciphertext, SaplingHead, SaplingStorage},
        types::{CommitmentNode, Hash, Nullifier, SaplingTransaction},
        validator::{get_sighash, validate_transaction},
    };

    const CHAIN_ID: &'static str = "NetXnHfVqm9iesp";
    const CONTRACT_ADDRESS: &'static str = "KT1PwYL1B8hagFeCcByAcsN3KTQHmJFfDwnj";
    const SAPLING_TX_HEX: &'static str =
        include_str!("../tests/data/ong6gzsvydC8zgFn1KAM3HFFVymZbroRKqAH4tt1ejcYBcyvroy");

    mock! {
        Storage {}
        impl SaplingStorage for Storage {
            fn set_head(&mut self, head: SaplingHead) -> Result<()>;
            fn get_head(&mut self) -> Result<SaplingHead>;
            fn set_root(&mut self, root: Hash, position: u64) -> Result<()>;
            fn has_root(&self, root: &Hash) -> Result<bool>;
            fn get_root(&mut self, position: u64) -> Result<Option<Hash>>;
            fn set_nullifier(&mut self, nullifier: Nullifier, position: u64) -> Result<()>;
            fn has_nullifier(&self, nullifier: &Nullifier) -> Result<bool>;
            fn get_nullifier(&mut self, position: u64) -> Result<Option<Nullifier>>;
            fn set_commitment(&mut self, commitment: CommitmentNode, path: u64) -> Result<()>;
            fn get_commitment(&mut self, path: u64) -> Result<Option<CommitmentNode>>;
            fn set_ciphertext(&mut self, ciphertext: Ciphertext, position: u64) -> Result<()>;
            fn get_ciphertext(&mut self, position: u64) -> Result<Option<Ciphertext>>;
            fn check_no_pending_changes(&self) -> Result<()>;
            fn commit(&mut self) -> Result<()>;
            fn rollback(&mut self);
        }
    }

    #[test]
    fn test_verify_spend_sig() -> Result<()> {
        // { "cv": "8e57b5c09010468f27dea7390f868d6a2bc475b90351fa44cfc91dd7fdab2caa",
        //   "nf": "d97a43b97ebda42b4d286c31f867bc58b416af0ddf4b8a33a07d448cbe74c97f",
        //   "rk": "96d53d578c08d2e0f6f8a264c7248d5c4f796d8eee2fef2f5537fea627e3e7da",
        //   "proof_i": "b56896734358241671d5368087da32cd308d915621f9933149d730f85091a4fbbac51abeba1c9c89d878e0b66c8dff2fa9cf7d09dbc0018db4cc77c7500772fb1a64919830ee62c075f8c4684219cfd3e6dc17896427c0c94b71a6e8fd9f1e7c0a0cf2c03c0d4b64791bf016aeda53f39981790310a5e1697e624ea704ef283bc86eda92646ecbf6f60d97e24285695dadd2af4d5c62f1cba2029f7408c4f3c314ec23e9b0a28fd9b68d83b2462a0612a20a7bbb5f7f6de622b0b68c3d7dcb34",
        //   "signature": "1f14e5bace9641fae8fe745c639acf75692d39e8cb56f433b5cf11928b8563b2ad910cacf5641e1e5e4e676f873baabab20707ca230fe036ebc4289622908108" }
        let sig_payload: &'static str = "\
            8e57b5c09010468f27dea7390f868d6a2bc475b90351fa44cfc91dd7fdab2caa\
            d97a43b97ebda42b4d286c31f867bc58b416af0ddf4b8a33a07d448cbe74c97f\
            96d53d578c08d2e0f6f8a264c7248d5c4f796d8eee2fef2f5537fea627e3e7da\
            b56896734358241671d5368087da32cd308d915621f9933149d730f85091a4fb\
            bac51abeba1c9c89d878e0b66c8dff2fa9cf7d09dbc0018db4cc77c7500772fb\
            1a64919830ee62c075f8c4684219cfd3e6dc17896427c0c94b71a6e8fd9f1e7c\
            0a0cf2c03c0d4b64791bf016aeda53f39981790310a5e1697e624ea704ef283b\
            c86eda92646ecbf6f60d97e24285695dadd2af4d5c62f1cba2029f7408c4f3c3\
            14ec23e9b0a28fd9b68d83b2462a0612a20a7bbb5f7f6de622b0b68c3d7dcb34";

        let anti_replay = [CONTRACT_ADDRESS, CHAIN_ID].concat();
        let sighash = get_sighash(&hex::decode(sig_payload)?, &anti_replay)?;

        let rk_bytes =
            hex::decode("96d53d578c08d2e0f6f8a264c7248d5c4f796d8eee2fef2f5537fea627e3e7da")?;
        let rk = PublicKey::read(rk_bytes.as_slice())?;

        let mut data_to_be_signed = [0u8; 64];
        data_to_be_signed[0..32].copy_from_slice(&rk_bytes.as_slice());
        data_to_be_signed[32..64].copy_from_slice(&sighash[..]);

        let sig_bytes = hex::decode("1f14e5bace9641fae8fe745c639acf75692d39e8cb56f433b5cf11928b8563b2ad910cacf5641e1e5e4e676f873baabab20707ca230fe036ebc4289622908108")?;
        let spend_auth_sig = Signature::read(sig_bytes.as_slice())?;

        let res = rk.verify_with_zip216(
            &data_to_be_signed,
            &spend_auth_sig,
            SPENDING_KEY_GENERATOR,
            false,
        );
        assert_eq!(res, true);

        Ok(())
    }

    #[test]
    fn test_verify_transaction() -> Result<()> {
        let payload = hex::decode(SAPLING_TX_HEX)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;

        let mut storage = MockStorage::new();
        storage.expect_get_head().returning(|| {
            Ok(SaplingHead {
                commitments_size: 1,
                memo_size: 8,
                nullifiers_size: 0,
                roots_pos: 1,
            })
        });
        storage.expect_has_root().returning(|_| Ok(true));
        storage.expect_has_nullifier().returning(|_| Ok(true));
        storage.expect_commit().returning(|| Ok(()));

        let tx = SaplingTransaction::try_from(payload.as_slice())?;
        validate_transaction(&mut storage, &tx, &[CONTRACT_ADDRESS, CHAIN_ID].concat())
    }
}
