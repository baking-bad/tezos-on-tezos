pub mod types;

use tezos_core::types::encoded::{
        OperationHash,
        OperationListListHash,
        BlockPayloadHash,
        BlockHash,
        Encoded
    };
use tezos_operation::operations::SignedOperation;
use tezos_rpc::models::balance_update::BalanceUpdate;

use crate::{
    error::Result,
    context::{Context, head::Head, migrations::run_migrations},
    validator::{ManagerOperation, validate_operation},
    executor::execute_operation,
    producer::types::{BatchHeader, BatchReceipt},
    constants::*,
    execution_error,
    validation_error
};

pub fn execute_batch_unchecked(
    context: &mut impl Context,
    header: BatchHeader,
    operations: Vec<ManagerOperation>,
    balance_updates: Option<Vec<BalanceUpdate>>
) -> Result<Head> {
    if context.has_pending_changes() {
        return execution_error!("Cannot proceed with uncommited state changes");
    }

    for (i, opg) in operations.iter().enumerate() {
        let receipt = execute_operation(context, opg)?;
        context.store_operation_receipt(&header.level, &(i as i32), receipt)?;
    }

    let hash = BlockHash::new("".into()).unwrap();  // TODO: forge header and blake2b
    let head = Head::new(header.level, hash.clone(), header.timestamp);
    let receipt = BatchReceipt {
        chain_id: CHAIN_ID.try_into().unwrap(),
        protocol: PROTOCOL.try_into().unwrap(),
        hash: hash.clone(),
        header,
        balance_updates
    };

    context.store_batch_receipt(receipt.header.level, receipt)?;
    context.commit()?;

    context.persist("/kernel/head".into(), head.clone())?;
    
    Ok(head)
}

pub fn execute_batch(
    context: &mut impl Context,
    head: Head,
    batch_payload: Vec<(OperationHash, SignedOperation)>
) -> Result<Head> {
    if context.has_pending_changes() {
        return validation_error!("Cannot proceed with uncommitted state changes");
    }

    let balance_updates = run_migrations(context, &head)?;

    let mut operations: Vec<ManagerOperation> = Vec::with_capacity(batch_payload.len());
    let mut operation_hashes: Vec<OperationHash> = Vec::with_capacity(batch_payload.len());
    
    for (hash, opg) in batch_payload.into_iter() {
        match validate_operation(context, opg, hash.clone()) {
            Ok(op) => {
                let balance = context.get_balance(&op.source)?.expect("Missing balance");
                context.set_balance(&op.source, &(balance - op.total_fees))?;
                context.set_counter(&op.source, &op.last_counter)?;
                operation_hashes.push(hash);
                operations.push(op);
            },
            Err(_) => ()  // TODO: context.error_log(err)
        }
    }
    context.rollback();  // clear validator changes

    let header = BatchHeader {
        level: head.level + 1,
        predecessor: head.hash.to_owned(),
        payload_hash: BlockPayloadHash::from_parts(
            head.hash, 
            0, 
            operation_hashes.to_owned()
        )?,
        operations_hash: OperationListListHash::try_from(
            vec![vec![], vec![], vec![], operation_hashes]
        )?,
        context: context.get_checksum()?.hash()?,
        timestamp: head.timestamp + BLOCK_TIME,
    };

    execute_batch_unchecked(context, header, operations, balance_updates)
}