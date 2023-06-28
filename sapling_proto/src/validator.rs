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
    tree::CommitmentTree,
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

    if head.commitments_size == 0 {
        if transaction.root != CommitmentTree::empty_root() {
            bail!("Unexpected zero root: {}", transaction.root.to_string())
        }
    } else {
        if !storage.has_root(&transaction.root)? {
            bail!(
                "Transaction is expired (root = {:?})",
                transaction.root.to_string()
            );
        }
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
        if !storage.has_nullifier(&input.nf)? {
            bail!("Input #{} nullifier cannot be found", idx);
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
    const SAPLING_TX_HEX: &'static str = "\
        000001608e57b5c09010468f27dea7390f868d6a2bc475b90351fa44cfc91dd7\
        fdab2caad97a43b97ebda42b4d286c31f867bc58b416af0ddf4b8a33a07d448c\
        be74c97f96d53d578c08d2e0f6f8a264c7248d5c4f796d8eee2fef2f5537fea6\
        27e3e7dab56896734358241671d5368087da32cd308d915621f9933149d730f8\
        5091a4fbbac51abeba1c9c89d878e0b66c8dff2fa9cf7d09dbc0018db4cc77c7\
        500772fb1a64919830ee62c075f8c4684219cfd3e6dc17896427c0c94b71a6e8\
        fd9f1e7c0a0cf2c03c0d4b64791bf016aeda53f39981790310a5e1697e624ea7\
        04ef283bc86eda92646ecbf6f60d97e24285695dadd2af4d5c62f1cba2029f74\
        08c4f3c314ec23e9b0a28fd9b68d83b2462a0612a20a7bbb5f7f6de622b0b68c\
        3d7dcb341f14e5bace9641fae8fe745c639acf75692d39e8cb56f433b5cf1192\
        8b8563b2ad910cacf5641e1e5e4e676f873baabab20707ca230fe036ebc42896\
        22908108000003e6c1766091f7b4283641885f23369caa92155d5d4a51d8f220\
        45fd53c0cb9ddc4da94cd5a796fb19b89113fe319cb603c46a21b4c9875eda4b\
        54a38d85eab0b095a427f679667aef78046c70a3296ebd858d55661bd3d849ab\
        7c2eca178cd37af5034f8d24e3d62d1103e05ff3fc5636938ad32403a62c0ca7\
        12d459f1659f0bb3078680a46944f8acc13351272980d30038b7efd806aac796\
        a15eff75f8458a7de88aec23596145183716f0799929dd878c7c86d154030dc6\
        756c45e67a434148f8aac437e1efc7dd9023fabd528058474d325dc92637868e\
        684cb83b67eeae612f6c3e62ec02005c0a0becc90e4fad0c10eb7ad2ebdad62f\
        a450d51e63134a402fecda1810076aa4b5c6339619b8941e1029d937504fa6a4\
        48f96ba21d36ecac0000004fe5fc9b76ae6f956709c374ffc25e9d7e17dd2d7e\
        1fe0c2a785dc368d334a96ca32d5a8630c91726ae9a4c615d60243e1b10f95d2\
        1ab50906461251354996f8f2c90fe7d988899eddcccbdbdbefcd4499e8f234b9\
        a7b5294011370e068ededf98bf0c5d98024fe5305c1910b9cb3568d13b21116d\
        b6426c63052ca0c085341d3bb5f5a2fa67fbdedf497d2a34b9a7d50afc69176b\
        9c2a010a81c51707a33a72a42c740cf3315cc9593b596f2dd472490aa98e649c\
        1db9efda8a6c74ba9f105a27f25be282d744150d10ef70642f448c7b4473cab7\
        1217ab3d531d62e5d04e813e6cd063167920eedde381534a2bda5086317b3fed\
        9433e17278474a046ae63aeb169cc5e4a71cf37c4079ec2671291681298ffc65\
        82320496736bf4f8639296b09a47834b3dde23a1b35f0fb821e01069f11f7687\
        3ad347ee98ddabf001bf0307c12c62c502fe603b01372248ee589316165912ca\
        c88773c8cfcd8d06c2973e5d08b42971f9bbde9a075256839895e5556e1d057e\
        a8dad424bfa857a9c9d52ca4d03c9a6b5b26c2b8652d4b8a5c3c9c89f41b4fd6\
        917b29f17c414c8ba6386cf2d16d8caecc48c4addd4d2f94b933203f0d39c7e0\
        071712cd21d82d70079ea270c4d2ee26121db6c12f8b922f8af9909bf1a5e50d\
        8364707f657cef0ac568ba9368afb1f7d1b4d77fff07bad9a3f96a0000004f0c\
        4eab5d24f6c3c137a28428f2b4d2b33ab386f1fa1544ea5b0019e7479e362f92\
        6dc2c878ea6c3015aecc01aaa531d1c08f55e8f49cb65c96c3f9089b8a66d121\
        94147e16b95f2340eb5860beae21ed571a724ec5c9ec685b0e22adc2a7294375\
        5fdfe1ebdff616882b096762e8bc996a899452b1132678b879d8f306c3dc3a17\
        925b8c65e14de34023c22dc4bdba5dddee4def4cd25b5c2597582b754d982145\
        ea29cfcb132cf2a35a7b41c17a90f83d675ed156159ea8e7acf58889e83e1b73\
        ed59f5f954af127b0469e5c3216c65005fc98516ce06a75a22e3824c087a3a7e\
        83f47254763c7e7744b764a6d020ebf1509e51af80e39adaaa0e993325a5d4e3\
        b0f5aefcdc43ecfc65a9046b730c000000000000000069a1f12aea9ef4019a05\
        9e69e70d6317c35d936d3ea61181f9fa9fa297fe092f00000000";

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
