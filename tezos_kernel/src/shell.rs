use tezos_core::types::encoded::{ChainId, Encoded, OperationHash};
use tezos_operation::operations::SignedOperation;
use michelson_vm::InterpreterContext;

use tezos_proto::{
    protocol::{
        constants::{Constants, ConstantsAlpha},
        migrations::{Migrations, SandboxSeed}
    },
    batch::{payload::BatchPayload, validator::{validate_implicit_batch, validate_explicit_batch}, executor::execute_batch},
    context::{TezosContext, mempool::MempoolOperation},
    operations::validator::{validate_operation, ValidatedOperation}
};

pub trait Protocol {
    type Constants: Constants;
    type Migrations: Migrations;
    // type Fees: Fees;
    // type Bridge: Bridge;
    // type Contracts: Contracts;
}

pub struct ProtocolAlpha {}

impl Protocol for ProtocolAlpha {
    type Constants = ConstantsAlpha;
    type Migrations = SandboxSeed;
}

pub fn initialize<Proto: Protocol>(context: &mut impl TezosContext, chain_id: ChainId) -> Result<bool> {
    let mut head = context.get_head()?;
    if head.chain_id != chain_id {
        head.chain_id = chain_id;
        context.set_head(head.clone())?;
        context.commit()?;
        context.log(format!("Protocol initialized: {}", head));
        return Ok(true);
    }
    Ok(false)
}

pub fn sync_clock(context: &mut impl TezosContext, timestamp: i64) -> Result<()> {
    
}

pub fn inject_operation<Proto: Protocol>(context: &mut impl TezosContext, operation: SignedOperation) -> Result<()> {
    let hash = operation.hash()?;
    match validate_operation(context, operation, hash.clone(), false)? {
        ValidatedOperation::Valid(opg) => {
            let head = context.get_head()?;
            let mut mempool_state = context.get_mempool_state(head.level)?;
            if mempool_state.add(&opg.hash)? {
                let mempool_operation = MempoolOperation::new(head.level, opg.origin);
                context.set_mempool_operation(opg.hash.value(), Some(mempool_operation))?;
                context.set_mempool_state(head.level, Some(mempool_state))?;
                context.log(format!("Operation added to the mempool ({})", opg.hash.value()));
            } else {
                context.log(format!("Operation already exists in the mempool ({})", opg.hash.value()));
            }
        },
        ValidatedOperation::Invalid(errors) => {
            let description = errors
                .into_iter()
                .map(|e| format!("\t{}", e))
                .collect::<Vec<String>>()
                .join("\n");
            context.log(format!("Operation is invalid ({}):\n{}", hash.value(), description));
        },
    }
    Ok(())
}

pub fn inject_batch<Proto: Protocol>(context: &mut (impl TezosContext + InterpreterContext), batch: BatchPayload) -> Result<()> {
    // validate and execute batch

    let valid_operations = validate_explicit_batch(context, batch.clone(), false)?;
    execute_batch::<Proto>(context, valid_operations, Some(batch.header))?;

    for hash in batch.operations.keys() {
        context.set_mempool_operation(hash.value(), None)?;
    }
    Ok(())
}

pub fn finalize<Proto: Protocol>(context: &mut (impl TezosContext + InterpreterContext), timestamp: i64) -> Result<()> {
    let constants = Proto::Constants::constants();
    let mut head = context.get_head()?;

    let block_delay = constants.minimal_block_delay * constants.blocks_per_cycle;
    let expire_level = head.level - block_delay;
    let is_idle = timestamp - head.timestamp > constants.idle_time;

    if head.timestamp < timestamp {
        head.timestamp = timestamp;
        context.set_head(head.clone())?;
        context.commit()?;
    }

    let mempool_state = context.get_mempool_state(expire_level)?;
    context.set_mempool_state(expire_level, None)?;

    if !mempool_state.is_empty() {
        let mut pending_operations: Vec<(OperationHash, SignedOperation)> = Vec::with_capacity(mempool_state.len());
        for hash in mempool_state.to_vec()? {
            match context.get_mempool_operation(hash.value())? {
                Some(opg) => {
                    context.set_mempool_operation(hash.value(), None)?;
                    pending_operations.push((hash, opg.operation));
                },
                None => {
                    context.log(format!("Operation already included ({})", hash.value()));
                }
            }
        }

        let valid_operations = validate_implicit_batch(context, pending_operations, false)?;
        if !valid_operations.is_empty() {
            return execute_batch::<Proto>(context, valid_operations, None)
        }
    }

    if is_idle {
        return execute_batch::<Proto>(context, vec![], None)
    }

    Ok(())
}
