use context::{
    migrations::run_migrations, ExecutorContext, GenericContext, Head, InterpreterContext,
};
use tezos_core::types::encoded::{BlockPayloadHash, OperationHash, OperationListListHash};
use tezos_operation::operations::SignedOperation;
use tezos_rpc::models::operation::Operation as OperationReceipt;

use crate::{
    constants::*,
    executor::operation::execute_operation,
    producer::types::{BatchHeader, BatchReceipt},
    validator::{batch::validate_batch, operation::ManagerOperation},
    Result,
};

fn naive_header(prev_head: Head, operations: &Vec<ManagerOperation>) -> Result<BatchHeader> {
    let operation_hashes: Vec<OperationHash> = operations.iter().map(|o| o.hash.clone()).collect();
    Ok(BatchHeader {
        level: prev_head.level + 1,
        predecessor: prev_head.hash.to_owned(),
        payload_hash: BlockPayloadHash::from_parts(prev_head.hash, 0, operation_hashes.to_owned())?,
        operations_hash: OperationListListHash::try_from(vec![
            vec![],
            vec![],
            vec![],
            operation_hashes,
        ])?,
        context: "CoVhFUFQvxrbdDDemLCSXU4DLMfsVdvrbgCCXcCKBWqPAQ1gK8cL".try_into()?, // TODO
        timestamp: prev_head.timestamp + BLOCK_TIME,
    })
}

pub fn apply_batch(
    context: &mut (impl GenericContext + ExecutorContext + InterpreterContext),
    prev_head: Head,
    batch_payload: Vec<(OperationHash, SignedOperation)>,
    atomic: bool,
) -> Result<Head> {
    context.check_no_pending_changes()?;

    let balance_updates = run_migrations(context, &prev_head)?;
    let operations = validate_batch(context, batch_payload, atomic)?;

    let mut operation_receipts: Vec<OperationReceipt> = Vec::with_capacity(operations.len());
    for opg in operations.iter() {
        operation_receipts.push(execute_operation(context, opg)?);
    }

    // TODO: fees to batch producer (balance updates + update balance)

    let header = naive_header(prev_head, &operations)?;
    let hash = header.hash()?;
    let head = Head::new(header.level, hash.clone(), header.timestamp);

    let receipt = BatchReceipt {
        chain_id: CHAIN_ID.try_into().unwrap(),
        protocol: PROTOCOL.try_into().unwrap(),
        hash,
        header,
        balance_updates,
    };

    for (index, opg_receipt) in operation_receipts.into_iter().enumerate() {
        let hash = opg_receipt.hash.clone().unwrap();
        context.set_operation(head.level, index as i32, hash, opg_receipt)?;
    }

    context.set_batch(receipt.header.level, receipt.hash.clone(), receipt)?;
    context.set_head(head.clone())?;
    context.commit()?;

    Ok(head)
}
