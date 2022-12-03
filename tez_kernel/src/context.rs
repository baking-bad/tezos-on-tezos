use std::{collections::HashMap, ops::Deref};
use tezos_core::{
    types::{
        encoded::{Encoded, PublicKey, Address, ImplicitAddress},
        mutez::Mutez,
        number::Nat
    }
};
use tezos_rpc::models::operation::{
    Operation as OperationReceipt
};
use host::{
    runtime::{Runtime, ValueType},
    path::RefPath
};
use serde_json;

use crate::error::{Result, Error};
use crate::storage_error;

#[derive(Debug)]
pub enum ContextNode {
    Mutez(Mutez),
    Nat(Nat),
    PublicKey(PublicKey),
    OperationReceipt(OperationReceipt)
}

impl ContextNode {
    pub fn to_vec(&self) -> Result<Vec<u8>> {
        match self {
            Self::Mutez(value) => value.to_bytes().map_err(|e| e.into()),
            Self::Nat(value) => value.to_bytes().map_err(|e| e.into()),
            Self::PublicKey(value) => value.to_bytes().map_err(|e| e.into()),
            Self::OperationReceipt(value) => serde_json::to_vec(value).map_err(|e| e.into())
        }
    }
}

trait ContextType {
    fn parse(bytes: &[u8]) -> Result<ContextNode>;
    fn unwrap(node: &ContextNode) -> Self;
    fn wrap(value: &Self) -> ContextNode;
}

impl ContextType for Mutez {
    fn parse(bytes: &[u8]) -> Result<ContextNode> {
        match Mutez::from_bytes(bytes) {
            Ok(value) => Ok(ContextNode::Mutez(value)),
            Err(error) => Err(error.into())
        }
    }

    fn unwrap(node: &ContextNode) -> Self {
        match node {
            ContextNode::Mutez(value) => value.clone(),
            node => panic!("Type mismatch (expected Mutez, got {:?})", node)
        }
    }

    fn wrap(value: &Self) -> ContextNode {  
        ContextNode::Mutez(value.clone())
    }
}

impl ContextType for Nat {
    fn parse(bytes: &[u8]) -> Result<ContextNode> {
        match Nat::from_bytes(bytes) {
            Ok(value) => Ok(ContextNode::Nat(value)),
            Err(error) => Err(error.into())
        }
    }

    fn unwrap(node: &ContextNode) -> Self {
        match node {
            ContextNode::Nat(value) => value.clone(),
            node => panic!("Type mismatch (expected Nat, got {:?})", node)
        }
    }

    fn wrap(value: &Self) -> ContextNode {
        ContextNode::Nat(value.clone())
    }
}

impl ContextType for PublicKey {
    fn parse(bytes: &[u8]) -> Result<ContextNode> {
        match PublicKey::from_bytes(bytes) {
            Ok(value) => Ok(ContextNode::PublicKey(value)),
            Err(error) => Err(error.into())
        }
    }

    fn unwrap(node: &ContextNode) -> Self {
        match node {
            ContextNode::PublicKey(value) => value.clone(),
            node => panic!("Type mismatch (expected PublicKey, got {:?})", node)
        }
    }

    fn wrap(value: &Self) -> ContextNode {
        ContextNode::PublicKey(value.clone())
    }
}

impl ContextType for OperationReceipt {
    fn parse(bytes: &[u8]) -> Result<ContextNode> {
        match serde_json::from_slice(bytes) {
            Ok(value) => Ok(ContextNode::OperationReceipt(value)),
            Err(error) => Err(error.into())
        }
    }

    fn unwrap(node: &ContextNode) -> Self {
        match node {
            ContextNode::OperationReceipt(value) => value.clone(),
            node => panic!("Type mismatch (expected OperationReceipt, got {:?})", node)
        }
    }

    fn wrap(value: &Self) -> ContextNode {
        ContextNode::OperationReceipt(value.clone())
    }
}

pub trait TezosAddress {
    fn to_string(&self) -> &str;
}

impl TezosAddress for ImplicitAddress {
    fn to_string(&self) -> &str {
        self.to_string()
    }
}

impl TezosAddress for Address {
    fn to_string(&self) -> &str {
        self.to_string()
    }
}

impl TezosAddress for &str {
    fn to_string(&self) -> &str {
        self.deref()
    }
}

pub struct EphemeralContext {
    state: HashMap<String, ContextNode>,
    modified_keys: Vec<String>
}

impl EphemeralContext {
    pub fn new() -> Self {
        EphemeralContext {
            state: HashMap::new(),
            modified_keys: Vec::new()
        }
    }

    fn has(&self, host: &impl Runtime, key: String) -> Result<bool> {
        match self.state.contains_key(&key) {
            true => Ok(true),
            false => {
                let path = RefPath::assert_from(key.as_bytes());
                match host.store_has(&path) {
                    Ok(Some(ValueType::Value)) => Ok(true),
                    Err(error) => Err(error.into()),
                    _ => Ok(false)
                }
            }
        }
    }

    fn get<V: ContextType>(&mut self, host: &impl Runtime, key: String) -> Result<Option<V>> {
        match self.state.get(&key) {
            Some(cached_value) => Ok(Some(V::unwrap(cached_value))),
            None => {
                let path = RefPath::assert_from(key.as_bytes());
                match host.store_has(&path) {
                    Ok(Some(ValueType::Value)) => {
                        let stored_value = host
                            .store_read(&path, 0, 1024)?;

                        let value = V::parse(&stored_value)?;
                        let inner_value = V::unwrap(&value);
                        self.state.insert(key, value);
                        Ok(Some(inner_value))
                    },
                    Ok(Some(node_type)) => storage_error!("Unexpected node type {:?}", node_type),
                    Ok(None) => Ok(None),
                    Err(error) => Err(error.into())
                }
            }
        }
    }

    fn set<V: ContextType>(&mut self, key: String, val: &V) -> Result<()> {
        self.state.insert(key.clone(), V::wrap(val));
        self.modified_keys.push(key);
        Ok(())
    }

    pub fn has_pending_changes(&self) -> bool {
        !self.modified_keys.is_empty()
    }

    pub fn commit(&mut self, host: &mut impl Runtime) -> Result<()> {
        for key in self.modified_keys.iter() {
            let path = RefPath::assert_from(key.as_bytes());
            let cached_value = self.state.get(key).expect("Modified value has to be cached");
            let raw_value = cached_value.to_vec()?;
            host.store_write(&path, raw_value.as_slice(), 0).map_err(|e| Error::from(e))?;
        }
        self.modified_keys.clear();
        Ok(())
    }

    pub fn clear(&mut self) {
        self.state.clear();
    }

    pub fn rollback(&mut self) {
        for key in self.modified_keys.iter() {
            self.state.remove(key);
        }
        self.modified_keys.clear();
    }

    pub fn get_balance(&mut self, host: &impl Runtime, address: &impl TezosAddress) -> Result<Option<Mutez>> {
        return self.get(host, format!("/context/contracts/{}/balance", address.to_string()));
    }

    pub fn set_balance(&mut self, address: &impl TezosAddress, balance: &Mutez) -> Result<()> {
        return self.set(format!("/context/contracts/{}/balance", address.to_string()), balance);
    }

    pub fn get_counter(&mut self, host: &impl Runtime, address: &impl TezosAddress) -> Result<Option<Nat>> {
        return self.get(host, format!("/context/contracts/{}/counter", address.to_string()));
    }

    pub fn set_counter(&mut self, address: &impl TezosAddress, counter: &Nat) -> Result<()> {
        return self.set(format!("/context/contracts/{}/counter", address.to_string()), counter);
    }

    pub fn get_public_key(&mut self, host: &impl Runtime, address: &impl TezosAddress) -> Result<Option<PublicKey>> {
        return self.get(host, format!("/context/contracts/{}/pubkey", address.to_string()));
    }

    pub fn set_public_key(&mut self, address: &impl TezosAddress, public_key: &PublicKey) -> Result<()> {
        return self.set(format!("/context/contracts/{}/pubkey", address.to_string()), public_key);
    }

    pub fn has_public_key(&self, host: &impl Runtime, address: &impl TezosAddress) -> Result<bool> {
        return self.has(host, format!("/context/contracts/{}/pubkey", address.to_string()));
    }

    pub fn store_operation_receipt(&mut self, level: &i32, index: &i32, receipt: &OperationReceipt) -> Result<()> {
        return self.set(format!("/context/blocks/{}/operations/{}", level, index), receipt);
    }

    pub fn get_operation_receipt(&mut self, host: &impl Runtime, level: &i32, index: &i32) -> Result<Option<OperationReceipt>> {
        return self.get(host, format!("/context/blocks/{}/operations/{}", level, index));
    }
}


#[cfg(test)]
mod test {
    use crate::context::EphemeralContext;
    use mock_runtime::host::MockHost;
    use crate::error::Result;

    #[test]
    fn store_balance() -> Result<()> {
        let mut host = MockHost::default();
        let mut context = EphemeralContext::new();

        let address = "tz1Mj7RzPmMAqDUNFBn5t5VbXmWW4cSUAdtT";
        let balance = 1000u32.into();

        assert!(context.get_balance(&host, &address)?.is_none());  // both host and cache accessed

        context.set_balance(&address, &balance)?;  // cached
        context.commit(&mut host);  // sent to the host
        context.clear();  // cache cleared

        assert!(context.get_balance(&host, &address)?.is_some());  // cached
        assert_eq!(context.get_balance(&host, &address)?.expect("Balance must not be null"), balance);  // served from the cache

        Ok(())
    }
}