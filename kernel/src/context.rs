use std::collections::HashMap;
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

use crate::error::Result;

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
            _ => panic!("Type mismatch (expected mutez)")
        }
    }

    fn wrap(value: &Self) -> ContextNode {
        return ContextNode::Mutez(value.clone());
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
            _ => panic!("Type mismatch (expected nat)")
        }
    }

    fn wrap(value: &Self) -> ContextNode {
        return ContextNode::Nat(value.clone());
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
            _ => panic!("Type mismatch (expected public key)")
        }
    }

    fn wrap(value: &Self) -> ContextNode {
        return ContextNode::PublicKey(value.clone());
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
            _ => panic!("Type mismatch (expected operation)")
        }
    }

    fn wrap(value: &Self) -> ContextNode {
        return ContextNode::OperationReceipt(value.clone());
    }
}

pub struct EphemeralContext {
    state: HashMap<String, ContextNode>,
    modified_keys: Vec<String>
}

impl EphemeralContext {
    pub fn new() -> Self {
        return EphemeralContext { state: HashMap::new(), modified_keys: Vec::new() };
    }

    fn has(&self, host: &impl Runtime, key: String) -> bool {
        match self.state.contains_key(&key) {
            true => true,
            false => {
                let path = RefPath::assert_from(key.as_bytes());
                match host.store_has(&path) {
                    Ok(Some(ValueType::Value)) => true,
                    Err(_) => panic!("Could not read store: {}", key),
                    _ => false
                }
            }
        }
    }

    fn get<V: ContextType>(&mut self, host: &impl Runtime, key: String) -> Option<V> {
        match self.state.get(&key) {
            Some(cached_value) => return Some(V::unwrap(cached_value)),
            None => {
                let path = RefPath::assert_from(key.as_bytes());
                if let Ok(Some(ValueType::Value)) = host.store_has(&path) {
                    if let Ok(stored_value) = host.store_read(&path, 0, 1024) {
                        if let Ok(value) = V::parse(stored_value.as_slice()) {
                            let inner_value = V::unwrap(&value);
                            self.state.insert(key, value);
                            return Some(inner_value);
                        } else {
                            panic!("Could not decode stored value: {}", key)
                        }
                    } else {
                        panic!("Could not read store: {}", key)
                    }
                }
                return None;
            }
        }
    }

    fn set<V: ContextType>(&mut self, key: String, val: &V) {
        let value = V::wrap(val);
        if let Some(cached_value) = self.state.get_mut(&key) {
            *cached_value = value  // FIXME: possible trailing garbage
        } else {
            self.state.insert(key.clone(), value);
        }
        self.modified_keys.push(key);
    }

    pub fn has_pending_changes(&self) -> bool {
        return !self.modified_keys.is_empty();
    }

    pub fn commit(&mut self, host: &mut impl Runtime) {
        for key in self.modified_keys.iter() {
            let path = RefPath::assert_from(key.as_bytes());
            let cached_value = self.state.get(key).unwrap();
            let raw_value = cached_value.to_vec().unwrap();
            match host.store_write(&path, raw_value.as_slice(), 0) {
                Ok(_) => continue,
                Err(err) => panic!("Failed to write store at {}: {:?}", key, err)
            }
        }
        self.modified_keys.clear();
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

    pub fn get_balance(&mut self, host: &impl Runtime, address: &ImplicitAddress) -> Option<Mutez> {
        return self.get(host, format!("/context/contracts/{}/balance", address.value()));
    }

    pub fn set_balance(&mut self, address: &ImplicitAddress, balance: &Mutez) {
        return self.set(format!("/context/contracts/{}/balance", address.value()), balance);
    }

    pub fn set_contract_balance(&mut self, address: &Address, balance: &Mutez) {
        return self.set(format!("/context/contracts/{}/balance", address.value()), balance);
    }

    pub fn get_counter(&mut self, host: &impl Runtime, address: &ImplicitAddress) -> Option<Nat> {
        return self.get(host, format!("/context/contracts/{}/counter", address.value()));
    }

    pub fn set_counter(&mut self, address: &ImplicitAddress, counter: &Nat) {
        return self.set(format!("/context/contracts/{}/counter", address.value()), counter);
    }

    pub fn get_public_key(&mut self, host: &impl Runtime, address: &ImplicitAddress) -> Option<PublicKey> {
        return self.get(host, format!("/context/contracts/{}/pubkey", address.value()));
    }

    pub fn set_public_key(&mut self, address: &ImplicitAddress, public_key: &PublicKey) {
        return self.set(format!("/context/contracts/{}/pubkey", address.value()), public_key);
    }

    pub fn has_public_key(&self, host: &impl Runtime, address: &ImplicitAddress) -> bool {
        return self.has(host, format!("/context/contracts/{}/pubkey", address.value()));
    }

    pub fn store_operation_receipt(&mut self, level: &i32, index: &i32, receipt: &OperationReceipt) {
        return self.set(format!("/context/blocks/{}/operations/{}", level, index), receipt);
    }

    pub fn get_operation_receipt(&mut self, host: &impl Runtime, level: &i32, index: &i32) -> Option<OperationReceipt> {
        return self.get(host, format!("/context/blocks/{}/operations/{}", level, index));
    }
}


#[cfg(test)]
mod test {
    use crate::context::EphemeralContext;
    use mock_runtime::host::MockHost;
    use tezos_core::types::{
        encoded::ImplicitAddress,
        mutez::Mutez
    };

    #[test]
    fn store_balance() {
        let mut host = MockHost::default();
        let mut context = EphemeralContext::new();

        let address = ImplicitAddress::try_from("tz1Mj7RzPmMAqDUNFBn5t5VbXmWW4cSUAdtT").unwrap();
        let balance = Mutez::try_from(1000u32).unwrap();

        assert!(context.get_balance(&host, &address).is_none());  // both host and cache accessed

        context.set_balance(&address, &balance);  // cached
        context.commit(&mut host);  // sent to the host
        context.clear();  // cache cleared

        assert!(context.get_balance(&host, &address).is_some());  // cached
        assert_eq!(context.get_balance(&host, &address).unwrap(), balance);  // served from the cache
    }
}