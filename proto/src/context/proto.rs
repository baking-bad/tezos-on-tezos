use tezos_core::types::{encoded::PublicKey, mutez::Mutez, number::Nat};
use tezos_michelson::micheline::Micheline;
use tezos_rpc::models::operation::Operation as OperationReceipt;

use crate::{
    context::{head::Head, Context},
    producer::types::BatchReceipt,
    Error, Result,
};

pub trait ProtoContext {
    fn get_head(&mut self) -> Result<Head>;
    fn commit_head(&mut self, head: Head) -> Result<()>;
    fn get_balance(&mut self, address: &str) -> Result<Option<Mutez>>;
    fn set_balance(&mut self, address: &str, balance: &Mutez) -> Result<()>;
    fn get_counter(&mut self, address: &str) -> Result<Option<Nat>>;
    fn set_counter(&mut self, address: &str, counter: &Nat) -> Result<()>;
    fn has_public_key(&self, address: &str) -> Result<bool>;
    fn get_public_key(&mut self, address: &str) -> Result<Option<PublicKey>>;
    fn set_public_key(&mut self, address: &str, public_key: &PublicKey) -> Result<()>;
    fn get_operation_receipt(&mut self, level: i32, index: i32)
        -> Result<Option<OperationReceipt>>;
    fn commit_operation_receipt(
        &mut self,
        level: i32,
        index: i32,
        receipt: OperationReceipt,
    ) -> Result<()>;
    fn get_batch_receipt(&mut self, level: i32) -> Result<Option<BatchReceipt>>;
    fn commit_batch_receipt(&mut self, level: i32, receipt: BatchReceipt) -> Result<()>;
    fn set_contract_code(&mut self, address: &str, code: Micheline) -> Result<()>;
    fn get_contract_code(&mut self, address: &str) -> Result<Option<Micheline>>;
    fn get_contract_storage(&mut self, address: &str) -> Result<Option<Micheline>>;
    fn set_contract_storage(&mut self, address: &str, storage: Micheline) -> Result<()>;
    fn commit(&mut self) -> Result<()>;
    fn rollback(&mut self);
    fn check_no_pending_changes(&self) -> Result<()>;
    fn debug_log(&self, message: String);
}

macro_rules! context_get_opt {
    ($context: expr, $($arg:tt)*) => {
        match $context.get(format!($($arg)*)) {
            Ok(Some(value)) => Ok(Some(value.try_into()?)),
            Ok(None) => Ok(None),
            Err(err) => Err(err)
        }
    };
}

macro_rules! context_get {
    ($context: expr, $default: expr, $($arg:tt)*) => {
        match $context.get(format!($($arg)*)) {
            Ok(Some(value)) => Ok(value.try_into()?),
            Ok(None) => Ok($default),
            Err(err) => Err(err)
        }
    };
}

impl<T: Context> ProtoContext for T {
    fn get_head(&mut self) -> Result<Head> {
        context_get!(self, Head::default(), "/head")
    }

    fn commit_head(&mut self, head: Head) -> Result<()> {
        self.save("/head".into(), Some(head.into()))?;
        Ok(())
    }

    fn get_balance(&mut self, address: &str) -> Result<Option<Mutez>> {
        context_get_opt!(self, "/context/contracts/{}/balance", address)
    }

    fn set_balance(&mut self, address: &str, balance: &Mutez) -> Result<()> {
        return self.set(
            format!("/context/contracts/{}/balance", address),
            balance.to_owned().into(),
        );
    }

    fn get_counter(&mut self, address: &str) -> Result<Option<Nat>> {
        // TODO: use u64 or UBig instead of Nat, because it is String under the hood, not good for math
        context_get_opt!(self, "/context/contracts/{}/counter", address)
    }

    fn set_counter(&mut self, address: &str, counter: &Nat) -> Result<()> {
        return self.set(
            format!("/context/contracts/{}/counter", address),
            counter.to_owned().into(),
        );
    }

    fn get_public_key(&mut self, address: &str) -> Result<Option<PublicKey>> {
        context_get_opt!(self, "/context/contracts/{}/pubkey", address)
    }

    fn set_public_key(&mut self, address: &str, public_key: &PublicKey) -> Result<()> {
        // NOTE: Underscores are not allowed in path (host restriction)
        return self.set(
            format!("/context/contracts/{}/pubkey", address),
            public_key.to_owned().into(),
        );
    }

    fn has_public_key(&self, address: &str) -> Result<bool> {
        return self.has(format!("/context/contracts/{}/pubkey", address));
    }

    fn commit_operation_receipt(
        &mut self,
        level: i32,
        index: i32,
        receipt: OperationReceipt,
    ) -> Result<()> {
        if let Some(hash) = &receipt.hash {
            self.save(
                format!("/blocks/{}/ophashes/{}", level, index),
                Some(hash.clone().into()),
            )?;
        }
        self.save(
            format!("/blocks/{}/operations/{}", level, index),
            Some(receipt.into()),
        )?;
        Ok(())
    }

    fn get_operation_receipt(
        &mut self,
        level: i32,
        index: i32,
    ) -> Result<Option<OperationReceipt>> {
        context_get_opt!(self, "/blocks/{}/operations/{}", level, index)
    }

    fn commit_batch_receipt(&mut self, level: i32, receipt: BatchReceipt) -> Result<()> {
        self.save(
            format!("/blocks/{}/hash", level),
            Some(receipt.hash.clone().into()),
        )?;
        self.save(format!("/blocks/{}/header", level), Some(receipt.into()))?;
        Ok(())
    }

    fn get_batch_receipt(&mut self, level: i32) -> Result<Option<BatchReceipt>> {
        return context_get_opt!(self, "/blocks/{}/header", level);
    }

    fn get_contract_code(&mut self, address: &str) -> Result<Option<Micheline>> {
        context_get_opt!(self, "/context/contracts/{}/code", address)
    }

    fn set_contract_code(&mut self, address: &str, code: Micheline) -> Result<()> {
        // TODO: support splitting into chunks (generic read/write loop)
        self.set(format!("/context/contracts/{}/code", address), code.into())
    }

    fn get_contract_storage(&mut self, address: &str) -> Result<Option<Micheline>> {
        context_get_opt!(self, "/context/contracts/{}/storage", address)
    }

    fn set_contract_storage(&mut self, address: &str, storage: Micheline) -> Result<()> {
        self.set(
            format!("/context/contracts/{}/storage", address),
            storage.into(),
        )
    }

    fn commit(&mut self) -> Result<()> {
        for (key, val) in self.agg_pending_changes() {
            self.save(key, val)?;
        }
        Ok(())
    }

    fn rollback(&mut self) {
        self.agg_pending_changes();
    }

    fn check_no_pending_changes(&self) -> Result<()> {
        if self.has_pending_changes() {
            Err(Error::ContextUnstagedError)
        } else {
            Ok(())
        }
    }

    fn debug_log(&self, message: String) {
        self.log(message)
    }
}
