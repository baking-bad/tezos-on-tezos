use tezos_vm::interpreter::InterpreterContext;
use context::{Head, ExecutorContext, migrations::run_migrations};

use crate::{
    constants::*,
    executor::operation::execute_operation,
    producer::types::{
        BatchHeader, BatchReceipt, BlockPayloadHash, OperationHash, OperationListListHash,
        OperationReceipt, SignedOperation,
    },
    validator::{batch::validate_batch, operation::ManagerOperation},
    Result,
};

fn naive_header(
    head: Head,
    operations: &Vec<ManagerOperation>,
) -> Result<BatchHeader> {
    let operation_hashes: Vec<OperationHash> = operations.iter().map(|o| o.hash.clone()).collect();
    Ok(BatchHeader {
        level: head.level + 1,
        predecessor: head.hash.to_owned(),
        payload_hash: BlockPayloadHash::from_parts(head.hash, 0, operation_hashes.to_owned())?,
        operations_hash: OperationListListHash::try_from(vec![
            vec![],
            vec![],
            vec![],
            operation_hashes,
        ])?,
        context: "CoVhFUFQvxrbdDDemLCSXU4DLMfsVdvrbgCCXcCKBWqPAQ1gK8cL".try_into()?, // TODO
        timestamp: head.timestamp + BLOCK_TIME,
    })
}

pub fn apply_batch(
    context: &mut (impl ExecutorContext + InterpreterContext),
    head: Head,
    batch_payload: Vec<(OperationHash, SignedOperation)>,
) -> Result<Head> {
    context.check_no_pending_changes()?;

    let balance_updates = run_migrations(context, &head)?;
    let operations = validate_batch(context, batch_payload, false)?;

    let mut operation_receipts: Vec<OperationReceipt> = Vec::with_capacity(operations.len());
    for opg in operations.iter() {
        operation_receipts.push(execute_operation(context, opg)?);
    }

    // TODO: fees to batch producer (balance updates + update balance)

    let header = naive_header(head, &operations)?;
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
        context.commit_operation(head.level, index as i32, hash, opg_receipt)?;
    }

    context.commit_batch(receipt.header.level, receipt.hash.clone(), receipt)?;
    context.commit_head(head.clone())?;

    Ok(head)
}
