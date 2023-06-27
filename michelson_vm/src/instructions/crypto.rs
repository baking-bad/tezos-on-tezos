// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use tezos_core::{
    internal::crypto::blake2b,
    types::encoded::{Encoded, ImplicitAddress, PublicKey},
    Tezos,
};
use tezos_michelson::michelson::data::instructions::{
    Blake2B,
    CheckSignature,
    HashKey, // TODO: Sha256, Sha512, Sha3, Keccak,
};

use crate::{
    err_mismatch,
    interpreter::PureInterpreter,
    pop_cast,
    stack::Stack,
    types::{KeyHashItem, StackItem},
    Result,
};

impl PureInterpreter for Blake2B {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let payload = pop_cast!(stack, Bytes);
        let digest = blake2b(payload.unwrap().as_slice(), 32)?;
        stack.push(StackItem::Bytes(digest.into()))
    }
}

impl PureInterpreter for HashKey {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let key = pop_cast!(stack, Key);

        let public_key = key.unwrap().to_bytes()?; // first byte represents the curve
        let mut digest = blake2b(&public_key.as_slice()[1..], 20)?;
        digest.insert(0, public_key[0]);

        let key_hash = ImplicitAddress::from_bytes(digest.as_slice())?;
        stack.push(KeyHashItem::new(key_hash).into())
    }
}

impl PureInterpreter for CheckSignature {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let key = pop_cast!(stack, Key);
        let sig = pop_cast!(stack, Signature);
        let msg = pop_cast!(stack, Bytes);

        let crypto = Tezos::default().get_crypto();
        let public_key = key.unwrap();
        let signature = sig.unwrap().to_bytes()?;
        let message = crypto.blake2b(msg.unwrap().as_slice(), 32)?;

        let res = match public_key {
            PublicKey::Ed25519(public_key) => crypto.verify_ed25519(
                message.as_slice(),
                signature.as_slice(),
                public_key.to_bytes()?.as_slice(),
            )?,
            PublicKey::Secp256K1(public_key) => crypto.verify_secp256_k1(
                message.as_slice(),
                signature.as_slice(),
                public_key.to_bytes()?.as_slice(),
            )?,
            PublicKey::P256(public_key) => crypto.verify_p256(
                message.as_slice(),
                signature.as_slice(),
                public_key.to_bytes()?.as_slice(),
            )?,
        };
        stack.push(StackItem::Bool(res.into()))
    }
}
