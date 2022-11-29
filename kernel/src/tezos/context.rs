use std::collections::HashMap;
use tezos_core::{
    types::{
        encoded::{Encoded, PublicKey, Address, ImplicitAddress},
        mutez::Mutez,
        number::Nat,
    },
    Result
};
use tezos_rpc::models::{
    operation::Operation,
    block::Header
};
use host::{
    runtime::{Runtime, ValueType},
    path::RefPath
};

#[derive(Debug)]
enum ContextNode {
    Mutez(Mutez),
    Nat(Nat),
    PublicKey(PublicKey),
    Operation(Operation),
    BlockHeader(Header),
}

impl ContextNode {
    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            Self::Mutez(val) => val.to_bytes().unwrap(),
            Self::Nat(val) => val.to_bytes().unwrap(),
            Self::PublicKey(val) => val.to_bytes().unwrap(),
            Self::Operation(_) => todo!("serialize operation with metadata"),
            Self::BlockHeader(_) => todo!("serialize block header")
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
            Err(error) => Err(error)
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
            Err(error) => Err(error)
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
            Err(error) => Err(error)
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

impl ContextType for Operation {
    fn parse(bytes: &[u8]) -> Result<ContextNode> {
        todo!()
    }

    fn unwrap(node: &ContextNode) -> Self {
        match node {
            ContextNode::Operation(value) => value.clone(),
            _ => panic!("Type mismatch (expected operation)")
        }
    }

    fn wrap(value: &Self) -> ContextNode {
        return ContextNode::Operation(value.clone());
    }
}

impl ContextType for Header {
    fn parse(bytes: &[u8]) -> Result<ContextNode> {
        todo!()
    }

    fn unwrap(node: &ContextNode) -> Self {
        match node {
            ContextNode::BlockHeader(value) => value.clone(),
            _ => panic!("Type mismatch (expected block header)")
        }
    }

    fn wrap(value: &Self) -> ContextNode {
        return ContextNode::BlockHeader(value.clone());
    }
}

struct EphemeralContext {
    state: HashMap<String, ContextNode>,
    modified_keys: Vec<String>
}

impl EphemeralContext {
    pub fn new() -> Self {
        return EphemeralContext { state: HashMap::new(), modified_keys: Vec::new() };
    }

    fn get<V: ContextType>(&mut self, host: &impl Runtime, key: String) -> Option<V> {
        let path = RefPath::assert_from(key.as_bytes());
        match self.state.get(&key) {
            Some(cached_value) => return Some(V::unwrap(cached_value)),
            None => {
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

    pub fn commit(&mut self, host: &mut impl Runtime) {
        for key in self.modified_keys.iter() {
            let path = RefPath::assert_from(key.as_bytes());
            let cached_value = self.state.get(key).unwrap();
            match host.store_write(&path, cached_value.to_vec().as_slice(), 0) {
                Ok(_) => continue,
                Err(err) => panic!("Failed to write store at {}: {:?}", key, err)
            }
        }
        self.modified_keys.clear();
    }

    pub fn get_balance(&mut self, host: &impl Runtime, address: &Address) -> Option<Mutez> {
        return self.get(host, format!("/context/contracts/{}/balance", address.value()));
    }

    pub fn set_balance(&mut self, address: &Address, balance: &Mutez) {
        return self.set(format!("/context/contracts/{}/balance", address.value()), balance);
    }

    pub fn get_counter(&mut self, host: &impl Runtime, address: &ImplicitAddress) -> Option<Nat> {
        return self.get(host, format!("/context/contracts/{}/counter", address.value()));
    }

    pub fn set_counter(&mut self, address: &ImplicitAddress, counter: &Nat) {
        return self.set(format!("/context/contracts/{}/counter", address.value()), counter);
    }

    pub fn get_public_key(&mut self, host: &impl Runtime, address: &ImplicitAddress) -> Option<PublicKey> {
        return self.get(host, format!("/context/contracts/{}/manager_key", address.value()));
    }

    pub fn set_public_key(&mut self, address: &ImplicitAddress, public_key: &PublicKey) {
        return self.set(format!("/context/contracts/{}/manager_key", address.value()), public_key);
    }

    pub fn set_operation(&mut self, level: &u64, index: &u32, operation: &Operation) {
        return self.set(format!("/context/blocks/{}/operations/{}", level, index), operation);
    }

    pub fn set_block_header(&mut self, level: &u64, block_header: &Header) {
        return self.set(format!("/context/blocks/{}/header", level), block_header);
    }
}