pub mod ephemeral;
pub mod executor;
pub mod interpreter;
pub mod viewer;

use tezos_core::types::{
    encoded::{ContractAddress, PublicKey, ScriptExprHash},
    mutez::Mutez,
    number::Nat,
};
use tezos_michelson::micheline::Micheline;
use tezos_rpc::models::operation::Operation;

use crate::{BatchReceipt, ContextNode, Head, Result};

pub trait GenericContext {
    fn log(&self, msg: String);
    fn has(&self, key: String) -> Result<bool>;
    fn get(&mut self, key: String) -> Result<Option<ContextNode>>;
    fn set(&mut self, key: String, val: Option<ContextNode>) -> Result<()>;
    fn has_pending_changes(&self) -> bool;
    fn commit(&mut self) -> Result<()>;
    fn rollback(&mut self);
    fn clear(&mut self);
}

pub trait ExecutorContext {
    fn get_head(&mut self) -> Result<Head>;
    fn set_head(&mut self, head: Head) -> Result<()>;
    fn get_balance(&mut self, address: &str) -> Result<Option<Mutez>>;
    fn set_balance(&mut self, address: &str, balance: Mutez) -> Result<()>;
    fn get_counter(&mut self, address: &str) -> Result<Option<Nat>>;
    fn set_counter(&mut self, address: &str, counter: Nat) -> Result<()>;
    fn has_public_key(&self, address: &str) -> Result<bool>;
    fn get_public_key(&mut self, address: &str) -> Result<Option<PublicKey>>;
    fn set_public_key(&mut self, address: &str, public_key: PublicKey) -> Result<()>;
    fn set_contract_code(&mut self, address: &str, code: Micheline) -> Result<()>;
    fn get_contract_code(&mut self, address: &str) -> Result<Option<Micheline>>;
    fn get_contract_storage(&mut self, address: &str) -> Result<Option<Micheline>>;
    fn set_contract_storage(&mut self, address: &str, storage: Micheline) -> Result<()>;
    fn set_batch_receipt(&mut self, receipt: BatchReceipt) -> Result<()>;
    fn get_batch_receipt(&mut self, hash: &str) -> Result<BatchReceipt>;
    fn set_operation_receipt(&mut self, receipt: Operation) -> Result<()>;
    fn get_operation_receipt(&mut self, hash: &str) -> Result<Operation>;
    fn check_no_pending_changes(&self) -> Result<()>;
}

pub trait InterpreterContext {
    fn get_contract_type(&mut self, address: &ContractAddress) -> Result<Option<Micheline>>;
    fn set_contract_type(&mut self, address: ContractAddress, value: Micheline) -> Result<()>;
    fn allocate_big_map(&mut self, owner: ContractAddress) -> Result<i64>;
    // TODO: transfer_big_map
    fn get_big_map_owner(&mut self, ptr: i64) -> Result<Option<ContractAddress>>;
    fn has_big_map_value(&mut self, ptr: i64, key_hash: &ScriptExprHash) -> Result<bool>;
    fn get_big_map_value(
        &mut self,
        ptr: i64,
        key_hash: &ScriptExprHash,
    ) -> Result<Option<Micheline>>;
    fn set_big_map_value(
        &mut self,
        ptr: i64,
        key_hash: ScriptExprHash,
        value: Option<Micheline>,
    ) -> Result<()>;
}

pub trait ViewerContext {}

#[macro_export]
macro_rules! context_get_opt {
    ($context: expr, $($arg:tt)*) => {
        match $context.get(format!($($arg)*)) {
            Ok(Some(value)) => Ok(Some(value.try_into()?)),
            Ok(None) => Ok(None),
            Err(err) => Err(err)
        }
    };
}

#[macro_export]
macro_rules! context_get {
    ($context: expr, $default: expr, $($arg:tt)*) => {
        match $context.get(format!($($arg)*)) {
            Ok(Some(value)) => Ok(value.try_into()?),
            Ok(None) => Ok($default),
            Err(err) => Err(err)
        }
    };
}

#[macro_export]
macro_rules! context_unwrap {
    ($context: expr, $($arg:tt)*) => {
        match $context.get(format!($($arg)*)) {
            Ok(Some(value)) => Ok(value.try_into()?),
            Ok(None) => Err($crate::internal_error!(Store, $($arg)*)),
            Err(err) => Err(err)
        }
    };
}
