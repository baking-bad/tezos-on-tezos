use host::runtime::Runtime;
use tezos_operation::{
    operations::Transaction
};
use tezos_core::{
    types::{
        encoded::{Address, Encoded}
    }
};
use tezos_rpc::models::operation::{
    OperationContent as OperationContentWithResult
};
use tezos_rpc::models::operation::{
    operation_contents_and_result::{
        transaction::{Transaction as TransactionWithResult, TransactionMetadata}
    },
    operation_result::operations::{
        transaction::TransactionOperationResult
    },
    operation_result::OperationResultStatus,
    kind::{OperationKind},
};
use tezos_rpc::models::{
    balance_update,
};

use crate::error::Result;
use crate::context::EphemeralContext;

const ALLOCATION_FEE: u32 = 1000u32;

pub fn execute_transaction(host: &mut impl Runtime, context: &mut EphemeralContext, transaction: &Transaction) -> Result<OperationContentWithResult> {
    if transaction.parameters.is_some() {
        todo!("Support smart contract calls");
    }

    let (mut dst_balance, dst_allocate) = match &transaction.destination {
        Address::Implicit(address) => match context.get_balance(host, &address) {
            Some(balance) => (balance, false),
            None => (0u32.into(), true)
        },
        Address::Originated(_addres) => todo!("Support smart contract calls")
    };

    let mut src_balance = context.get_balance(host, &transaction.source).unwrap();  // already checked by validator
    src_balance -= transaction.amount + transaction.fee;
    if dst_allocate {
        src_balance -= ALLOCATION_FEE.into();
    }
    context.set_balance(&transaction.source, &src_balance);
    context.set_counter(&transaction.source, &transaction.counter);

    dst_balance += transaction.amount;
    context.set_contract_balance(&transaction.destination, &dst_balance);

    let res = TransactionWithResult {
        kind: OperationKind::Transaction,
        source: transaction.source.clone(),
        counter: transaction.counter.to_string(),
        fee: transaction.fee,
        gas_limit: transaction.gas_limit.to_string(),
        storage_limit: transaction.storage_limit.to_string(),
        destination: transaction.destination.clone(),
        parameters: None,
        amount: transaction.amount,
        metadata: Some(TransactionMetadata {
            operation_result: TransactionOperationResult { 
                status: OperationResultStatus::Applied, 
                storage: None, 
                big_map_diff: None, 
                balance_updates: Some(vec![
                    balance_update::BalanceUpdate::Contract(balance_update::Contract {
                        kind: balance_update::Kind::Contract,
                        contract: transaction.source.value().into(),
                        change: format!("-{}", transaction.amount + if dst_allocate { ALLOCATION_FEE.into() } else { 0u32.into() }),
                        origin: None
                    }),
                    balance_update::BalanceUpdate::Contract(balance_update::Contract {
                        kind: balance_update::Kind::Contract,
                        contract: transaction.destination.value().into(),
                        change: format!("{}", transaction.amount),
                        origin: None
                    }),
                ]), 
                originated_contracts: None, 
                consumed_gas: None, 
                consumed_milligas: Some("0".into()), 
                storage_size: None, 
                paid_storage_size_diff: None, 
                allocated_destination_contract: Some(dst_allocate), 
                lazy_storage_diff: None, 
                errors: None
            },
            balance_updates: vec![
                balance_update::BalanceUpdate::Contract(balance_update::Contract {
                    kind: balance_update::Kind::Contract,
                    contract: transaction.source.value().into(),
                    change: format!("-{}", transaction.fee),
                    origin: None
                }),
            ],
            internal_operation_results: vec![]
        })
    };
    Ok(OperationContentWithResult::Transaction(res))
}

#[cfg(test)]
mod test {
    use crate::context::EphemeralContext;
    use crate::error::Result;
    use mock_runtime::host::MockHost;
    use tezos_operation::{
        operations::Transaction
    };
    use tezos_core::types::{
        encoded::{ImplicitAddress, Address},
        mutez::Mutez,
        number::Nat
    };

    use super::execute_transaction;

    #[test]
    fn test_transaction_applied() -> Result<()> {
        let mut host = MockHost::default();
        let mut context = EphemeralContext::new();

        let source = ImplicitAddress::try_from("tz1V3dHSCJnWPRdzDmZGCZaTMuiTmbtPakmU").unwrap();
        let destination = ImplicitAddress::try_from("tz1NEgotHhj4fkm8AcwquQqQBrQsAMRUg86c").unwrap();

        context.set_balance(&source, &Mutez::from(1000000000u32));
        context.set_counter(&source, &Nat::try_from("100000").unwrap());

        let transaction = Transaction {
            source: source.clone(),
            counter: 200000u32.into(),
            fee: 1000u32.into(),
            gas_limit: 0u32.into(),
            storage_limit: 0u32.into(),
            amount: 500000000u32.into(),
            destination: Address::Implicit(destination.clone()),
            parameters: None
        };

        let result = execute_transaction(&mut host, &mut context, &transaction);
        assert!(result.is_ok());

        assert_eq!(context.get_balance(&host, &source).unwrap(), Mutez::from(1000000000u32 - 1000u32 - 1000u32 - 500000000u32));
        assert_eq!(context.get_balance(&host, &destination).unwrap(), Mutez::from(500000000u32));
        
        Ok(())
    }
}