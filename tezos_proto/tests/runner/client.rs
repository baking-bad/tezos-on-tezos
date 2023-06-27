// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use michelson_vm::interpreter::InterpreterContext;
use std::collections::HashMap;
use tezos_core::internal::crypto::blake2b;
use tezos_core::types::{
    encoded::{
        Ed25519PublicKey, Ed25519PublicKeyHash, Ed25519SecretKey, Ed25519Seed, Encoded,
        ImplicitAddress, OperationHash, PublicKey, SecretKey,
    },
    number::Nat,
};
use tezos_michelson::micheline::Micheline;
use tezos_operation::operations::{
    Entrypoint, OperationContent, Origination, Parameters, Reveal, Script, SignedOperation,
    Transaction, UnsignedOperation,
};
use tezos_rpc::models::operation::Operation;

use tezos_proto::{
    batcher::apply_batch,
    context::{head::Head, migrations::run_migrations, TezosContext},
};

pub struct Wallet {
    pub counter: u32,
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
    pub address: ImplicitAddress,
}

impl Wallet {
    fn from_keypair(keypair: ed25519_dalek::Keypair) -> Self {
        let secret_key = Ed25519SecretKey::from_bytes(&keypair.to_bytes())
            .expect("Failed to decode ed25519 secret key");

        let public_key = Ed25519PublicKey::from_bytes(&keypair.public.to_bytes())
            .expect("Failed to encode public key");

        let digest = blake2b(&keypair.public.to_bytes(), 20).unwrap();
        let key_hash =
            Ed25519PublicKeyHash::from_bytes(digest.as_slice()).expect("Failed to encode address");

        Self {
            counter: 0,
            secret_key: secret_key.into(),
            public_key: public_key.into(),
            address: key_hash.into(),
        }
    }

    pub fn from(seed: Ed25519Seed) -> Self {
        let sk = ed25519_dalek::SecretKey::from_bytes(seed.to_bytes().unwrap().as_slice())
            .expect("Failed to decode seed");
        let pk = ed25519_dalek::PublicKey::from(&sk);

        let bytes = [sk.to_bytes(), pk.to_bytes()].concat();
        let keypair = ed25519_dalek::Keypair::from_bytes(bytes.as_slice())
            .expect("Failed to reconstruct key pair");

        Self::from_keypair(keypair)
    }

    pub fn get_counter(&mut self) -> Nat {
        self.counter += 1;
        self.counter.into()
    }
}

pub struct Client<Context> {
    context: Context,
    wallets: HashMap<String, Wallet>,
    group: Vec<OperationContent>,
    batch: Vec<SignedOperation>,
    alias: Option<&'static str>,
}

impl<Context: TezosContext + InterpreterContext> Client<Context> {
    pub fn new(context: Context) -> Self {
        Self {
            context,
            wallets: HashMap::new(),
            group: Vec::new(),
            batch: Vec::new(),
            alias: None,
        }
    }

    pub fn migrate(&mut self) -> &mut Self {
        let head = self.context.get_head().expect("Failed to get head");

        run_migrations(&mut self.context, &head).expect("Failed to run context migrations");

        self
    }

    pub fn use_wallet(&mut self, alias: &'static str) -> &mut Self {
        self.alias = Some(alias);
        self
    }

    pub fn import_wallet(&mut self, alias: &'static str, sk: &'static str) {
        self.wallets.insert(
            alias.into(),
            Wallet::from(Ed25519Seed::new(sk.into()).expect("Failed to decode secret key")),
        );
    }

    pub fn get_contract_balance(&mut self, address: &str) -> u32 {
        self.context
            .get_balance(address)
            .unwrap()
            .expect("Contract not found")
            .try_into()
            .unwrap()
    }

    pub fn get_balance(&mut self) -> u32 {
        let wallet = self
            .wallets
            .get(self.alias.expect("Active wallet not selected"))
            .expect("Wallet not defined");

        self.context
            .get_balance(wallet.address.value())
            .unwrap()
            .expect("Account not found")
            .try_into()
            .unwrap()
    }

    pub fn get_contract_storage(&mut self, address: &str) -> String {
        let storage = self
            .context
            .get_contract_storage(address)
            .unwrap()
            .expect("Contract storage not found");

        serde_json::to_string(&storage).unwrap()
    }

    pub fn get_operation(&mut self, hash: &str) -> String {
        let receipt: Operation = self
            .context
            .get_operation_receipt(hash)
            .expect("Failed to get operation receipt");

        serde_json::to_string(&receipt).unwrap()
    }

    pub fn reveal(&mut self) -> &mut Self {
        let wallet = self
            .wallets
            .get_mut(self.alias.expect("Active wallet not selected"))
            .expect("Wallet not defined");

        self.group.push(OperationContent::Reveal(Reveal {
            source: wallet.address.clone(),
            counter: wallet.get_counter(),
            public_key: wallet.public_key.clone(),
            fee: 0u32.into(),
            gas_limit: 0u32.into(),
            storage_limit: 0u32.into(),
        }));
        self
    }

    pub fn originate(
        &mut self,
        script: Micheline,
        storage: serde_json::Value,
        balance: u32,
    ) -> &mut Self {
        let wallet = self
            .wallets
            .get_mut(self.alias.expect("Active wallet not selected"))
            .expect("Wallet not defined");

        self.group.push(OperationContent::Origination(Origination {
            source: wallet.address.clone(),
            counter: wallet.get_counter(),
            balance: balance.into(),
            delegate: None,
            script: Script {
                code: script.into_sequence().expect("Expected sequence"),
                storage: serde_json::from_value(storage).expect("Failed to decode initial storage"),
            },
            fee: 0u32.into(),
            gas_limit: 0u32.into(),
            storage_limit: 0u32.into(),
        }));
        self
    }

    pub fn transfer(&mut self, destination: &str, amount: u32) -> &mut Self {
        let wallet = self
            .wallets
            .get_mut(self.alias.expect("Active wallet not selected"))
            .expect("Wallet not defined");

        self.group.push(OperationContent::Transaction(Transaction {
            source: wallet.address.clone(),
            counter: wallet.get_counter(),
            amount: amount.into(),
            destination: destination.try_into().expect("Invalid destination address"),
            parameters: None,
            fee: 0u32.into(),
            gas_limit: 0u32.into(),
            storage_limit: 0u32.into(),
        }));
        self
    }

    pub fn call(
        &mut self,
        contract: &str,
        entrypoint: &str,
        parameter: serde_json::Value,
        amount: u32,
    ) -> &mut Self {
        let wallet = self
            .wallets
            .get_mut(self.alias.expect("Active wallet not selected"))
            .expect("Wallet not defined");

        self.group.push(OperationContent::Transaction(Transaction {
            source: wallet.address.clone(),
            counter: wallet.get_counter(),
            amount: amount.into(),
            destination: contract.try_into().expect("Invalid destination address"),
            parameters: Some(Parameters {
                entrypoint: Entrypoint::from_str(entrypoint),
                value: serde_json::from_value(parameter).expect("Failed to decode parameter"),
            }),
            fee: 0u32.into(),
            gas_limit: 0u32.into(),
            storage_limit: 0u32.into(),
        }));
        self
    }

    pub fn inject(&mut self) -> OperationHash {
        let wallet = self
            .wallets
            .get_mut(self.alias.expect("Active wallet not selected"))
            .expect("Wallet not defined");

        let head = self.context.get_head().expect("Failed to get head");

        let operation = UnsignedOperation {
            branch: head.hash.clone(),
            contents: self.group.drain(..).collect(),
        };
        // println!("{:#?}", operation);
        let signature = operation
            .sign(&wallet.secret_key)
            .expect("Failed to sign operation");

        let opg = SignedOperation::from(operation, signature);
        let hash = opg.hash().unwrap();

        self.batch.push(opg);
        hash
    }

    pub fn bake(&mut self) -> Head {
        let head = self.context.get_head().expect("Failed to get head");

        let payload = self
            .batch
            .drain(..)
            .map(|o| (o.hash().expect("Failed to calculate opg hash"), o))
            .collect();

        apply_batch(&mut self.context, head, payload, true).expect("Failed to apply batch")
    }

    pub fn clear(&mut self) {
        self.group.clear();
        self.batch.clear();
        self.alias = None;
    }
}
