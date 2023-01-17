pub mod types;

use crate::{
    constants::*,
    context::{head::Head, migrations::run_migrations, Context},
    error::{Error, Result},
    executor::execute_operation,
    producer::types::{
        BatchHeader, BatchReceipt, BlockPayloadHash, OperationHash, OperationListListHash,
        OperationReceipt, SignedOperation,
    },
    validator::{validate_batch, ManagerOperation},
};

fn naive_header(
    context: &mut impl Context,
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
        context: context.get_checksum()?.hash()?, // IMPORTANT: after all operations are executed and state is committed
        timestamp: head.timestamp + BLOCK_TIME,
    })
}

pub fn apply_batch(
    context: &mut impl Context,
    head: Head,
    batch_payload: Vec<(OperationHash, SignedOperation)>,
) -> Result<Head> {
    if context.has_pending_changes() {
        return Err(Error::ContextUnstagedError);
    }

    let balance_updates = run_migrations(context, &head)?;
    let operations = validate_batch(context, batch_payload, false)?;

    let mut operation_receipts: Vec<OperationReceipt> = Vec::with_capacity(operations.len());
    for opg in operations.iter() {
        operation_receipts.push(execute_operation(context, opg)?);
    }

    // TODO: fees to batch producer (balance updates + update balance)

    let header = naive_header(context, head, &operations)?;
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
        context.commit_operation_receipt(head.level, index as i32, opg_receipt)?;
    }

    context.commit_batch_receipt(receipt.header.level, receipt)?;
    context.commit_head(head.clone())?;

    Ok(head)
}
