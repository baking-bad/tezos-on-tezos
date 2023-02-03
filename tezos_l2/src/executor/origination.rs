use tezos_core::{
    internal::crypto::blake2b,
    types::encoded::{ContractAddress, ContractHash, Encoded, OperationHash},
};
use tezos_ctx::ExecutorContext;
use tezos_operation::operations::Origination;
use tezos_rpc::models::operation::{
    operation_result::operations::origination::OriginationOperationResult,
    operation_result::OperationResultStatus,
};
use tezos_vm::interpreter::InterpreterContext;

use crate::{
    executor::balance_updates::BalanceUpdates,
    executor::contract::{deploy_contract, ContractOutput},
    executor::lazy_diff::LazyDiff,
    executor::result::ExecutionResult,
    executor::rpc_errors::RpcErrors,
    Error, Result,
};

pub fn originated_address(opg_hash: &OperationHash, index: i32) -> Result<ContractAddress> {
    let payload = [opg_hash.to_bytes()?, index.to_be_bytes().to_vec()].concat();
    let digest = blake2b(payload.as_slice(), 20)?;
    let hash = ContractHash::from_bytes(digest.as_slice())?;
    Ok(ContractAddress::from_components(&hash, None))
}

pub fn execute_origination(
    context: &mut (impl ExecutorContext + InterpreterContext),
    origination: &Origination,
    hash: &OperationHash,
    origination_index: &mut i32,
    skip: bool,
) -> Result<ExecutionResult> {
    let mut errors = RpcErrors::new();
    let mut balance_updates = BalanceUpdates::new();
    let mut lazy_diff = LazyDiff::new();
    let mut originated_contracts: Option<Vec<ContractAddress>> = None;

    macro_rules! result {
        ($status: ident) => {{
            let applied = OperationResultStatus::$status == OperationResultStatus::Applied;
            Ok(ExecutionResult::Origination {
                content: origination.clone(),
                result: OriginationOperationResult {
                    status: OperationResultStatus::$status,
                    consumed_milligas: if applied { Some("0".into()) } else { None },
                    originated_contracts,
                    lazy_storage_diff: lazy_diff.into(),
                    balance_updates: balance_updates.into(),
                    errors: errors.into(),
                    big_map_diff: None,
                    consumed_gas: None,
                    storage_size: None,
                    paid_storage_size_diff: None,
                },
            })
        }};
    }

    if skip {
        return result!(Skipped);
    }

    let self_address = originated_address(hash, *origination_index)?;
    *origination_index += 1;

    let balance = match balance_updates.transfer(
        context,
        origination.source.value(),
        self_address.value(),
        &origination.balance,
    ) {
        Ok((_, balance)) => balance,
        Err(Error::BalanceTooLow { balance }) => {
            errors.balance_too_low(&origination.balance, &balance, origination.source.value());
            return result!(Failed);
        }
        Err(err) => return Err(err),
    };

    match deploy_contract(context, origination, self_address.clone(), balance) {
        Ok(ContractOutput::Return(ret)) => {
            lazy_diff.update(ret.big_map_diff)?;
            originated_contracts = Some(vec![self_address]);
            result!(Applied)
        }
        Ok(ContractOutput::Error(err)) => {
            errors.runtime_error(self_address.value(), err.to_string());
            result!(Failed)
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod test {
    use tezos_core::types::mutez::Mutez;
    use tezos_ctx::EphemeralContext;
    use tezos_michelson::michelson::{
        data::instructions::failwith,
        data::Unit,
        types::{code, parameter, storage, unit},
    };
    use tezos_operation::operations::Script;

    use super::*;
    use crate::Result;

    #[test]
    fn test_origination_applied() -> Result<()> {
        let mut context = EphemeralContext::new();

        let source = "tz1V3dHSCJnWPRdzDmZGCZaTMuiTmbtPakmU";
        context.set_balance(source, Mutez::from(1000000000u32))?;

        let origination = Origination {
            source: source.try_into()?,
            counter: 200000u32.into(),
            fee: 1000u32.into(),
            gas_limit: 0u32.into(),
            storage_limit: 0u32.into(),
            balance: 500000000u32.into(),
            delegate: None,
            script: Script {
                code: vec![parameter(unit()), storage(unit()), code(failwith())].into(),
                storage: Unit.into(),
            },
        };

        let mut index = 1i32;
        let result = execute_origination(
            &mut context,
            &origination,
            &OperationHash::new("oneDGhZacw99EEFaYDTtWfz5QEhUW3PPVFsHa7GShnLPuDn7gSd".into())?,
            &mut index,
            false,
        )?;
        assert!(result.ok());

        let (_, res): (_, OriginationOperationResult) = result.try_into()?;
        let originated_contract = res.originated_contracts.unwrap().remove(0);
        let dummy_address = ContractAddress::try_from("KT1Mjjcb6tmSsLm7Cb3DSQszePjfchPM4Uxm")?;
        assert_eq!(dummy_address, originated_contract);

        assert_eq!(
            context.get_balance(source)?.unwrap(),
            Mutez::from(1000000000u32 - 500000000u32)
        );
        assert_eq!(
            context.get_balance(originated_contract.value())?.unwrap(),
            Mutez::from(500000000u32)
        );
        Ok(())
    }
}
