use std::collections::HashMap;

use tezos_core::types::encoded::{ImplicitAddress, ChainId, OperationHash};
use tezos_operation::operations::{SignedOperation};

use crate::{context::{batch::BatchHeader, head::Head, TezosContext}, Result};

pub struct Batch {
    pub header: BatchHeader,
    pub source: ImplicitAddress,
    pub operations: Vec<SignedOperation>    
}

pub struct TezosChain {
    pub head: Head,
    pub mempool: HashMap<OperationHash, SignedOperation>
}

impl TezosChain {
    pub fn load(context: &mut impl TezosContext, chain_id: ChainId) -> Result<Self> {
        let mut head = context.get_head()?;
        head.chain_id = chain_id;
        Ok(Self { head, mempool: HashMap::new() })
    }

    pub fn add_pending_operation(&mut self, hash: OperationHash, operation: SignedOperation) -> Result<()> {
        self.mempool.insert(hash, operation);
        Ok(())
    }

    pub fn has_pending_operation(&self, hash: &OperationHash) -> bool {
        self.mempool.contains_key(hash)
    }

    pub fn remove_pending_operation(&mut self, hash: &OperationHash) -> Result<SignedOperation> {
        self.mempool.remove(hash).ok_or(Error::)
    }

    pub fn apply_batch(&mut self, context: &mut impl TezosContext, batch: Batch) -> Result<()> {
        Ok(())
    }

    pub fn aggregate_batch(&mut self) -> Result<Batch> {
        todo!()
    }
}