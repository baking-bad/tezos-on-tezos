// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use michelson_vm::interpreter::InterpreterContext;
use tezos_core::types::encoded::{ContextHash, OperationHash};
use tezos_rpc::models::operation::Operation as OperationReceipt;

use crate::{
    batch::{header::BatchHeader, receipt::BatchReceipt},
    config::Config,
    context::{head::Head, TezosContext},
    operations::{executor::execute_operation, validator::ValidOperation},
    protocol::{constants::Constants, migrations::Migrations},
    Result,
};

pub fn execute_batch<C: Config>(
    context: &mut (impl TezosContext + InterpreterContext),
    operations: Vec<ValidOperation>,
    header: Option<BatchHeader>,
) -> Result<()> {
    context.check_no_pending_changes()?;

    let head = context.get_head()?;
    let balance_updates = C::Migrations::run(context, &head)?;
    // TODO: implicit deployments C::System
    // TODO: implicit ticket updates C::Bridge

    let mut operation_receipts: Vec<OperationReceipt> = Vec::with_capacity(operations.len());
    let mut operation_hashes: Vec<OperationHash> = Vec::with_capacity(operations.len());
    for opg in operations.iter() {
        operation_receipts.push(execute_operation::<C>(context, opg)?);
        operation_hashes.push(opg.hash.clone());
    }

    let header = match header {
        Some(h) => h, // TODO: verify context hash
        None => {
            // TODO: implement checksum or use durable storage root and store in the context
            let context_hash: ContextHash =
                "CoVhFUFQvxrbdDDemLCSXU4DLMfsVdvrbgCCXcCKBWqPAQ1gK8cL".try_into()?;
            BatchHeader::implicit(&head, operation_hashes.clone(), context_hash)?
        }
    };

    let hash = header.block_hash()?;
    let receipt = BatchReceipt {
        chain_id: head.chain_id.clone(),
        protocol: C::Constants::protocol(),
        hash: hash.clone(),
        header: header.clone(),
        balance_updates: Some(balance_updates),
        batcher: None, // TODO: set
    };
    context.set_batch_receipt(receipt)?;

    for opg_receipt in operation_receipts {
        context.set_operation_receipt(opg_receipt)?;
    }

    // TODO: do not store receipts in kernel storage (to save space), only on the sequencer side

    let head = Head::new(
        head.chain_id,
        header.level,
        hash.clone(),
        header.timestamp,
        operation_hashes,
    );
    context.set_head(head.clone())?;
    context.commit()?;

    Ok(())
}
