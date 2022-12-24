use tezos_core::{
    types::encoded::{
        OperationHash,
        ContractAddress,
        ContractHash,
        Encoded
    },
    internal::crypto::blake2b
};
use tezos_michelson::micheline::Micheline;
use tezos_operation::operations::Origination;
use tezos_rpc::models::operation::{
    operation_result::operations::origination::OriginationOperationResult,
    operation_result::OperationResultStatus,
    operation_contents_and_result::origination::{
        Origination as OriginationReceipt,
        OriginationMetadata
    }
};

use crate::{
    error::{Result, RpcErrors}, 
    context::Context, 
    executor::balance_update::BalanceUpdates,
};

macro_rules! if_applied {
    ($status: expr, $val: expr) => {
        if $status == OperationResultStatus::Applied {
            Some($val)
        } else {
            None
        }
    };
}

const DEFAULT_RESULT: OriginationOperationResult = OriginationOperationResult {
    status: OperationResultStatus::Skipped,
    big_map_diff: None, 
    balance_updates: None, 
    originated_contracts: None, 
    consumed_gas: None, 
    consumed_milligas: None, 
    storage_size: None, 
    paid_storage_size_diff: None,
    lazy_storage_diff: None,
    errors: None
};

pub fn skip_origination(origination: Origination) -> OriginationReceipt {
    OriginationReceipt {
        metadata: Some(OriginationMetadata {
            operation_result: DEFAULT_RESULT,
            balance_updates: vec![]
        }),
        ..origination.into()
    }
}

pub fn calculate_address(opg_hash: &OperationHash, index: &i32) -> Result<ContractAddress> {
    let payload = [opg_hash.to_bytes()?, index.to_be_bytes().to_vec()].concat();
    let digest = blake2b(payload.as_slice(), 20)?;
    let hash = ContractHash::from_bytes(digest.as_slice())?;
    Ok(ContractAddress::from_components(&hash, None))
}

pub fn execute_origination(
    context: &mut impl Context,
    origination: &Origination,
    hash: &OperationHash,
    origination_index: &mut i32
) -> Result<OriginationReceipt> {
    let originated_address = calculate_address(hash, origination_index)?;
    *origination_index += 1;

    let mut src_balance = context
        .get_balance(&origination.source)?
        .expect("Source balance has to be checked by validator");

    let mut errors = RpcErrors::new();
    let mut balance_updates = BalanceUpdates::new();
    let charges =  BalanceUpdates::fee(&origination.source, &origination.fee);

    macro_rules! make_receipt {
        ($a: expr) => {
            OriginationReceipt {
                metadata: Some(OriginationMetadata {
                    operation_result: OriginationOperationResult {
                        status: $a, 
                        originated_contracts: if_applied!($a, vec![originated_address]),
                        balance_updates: balance_updates.into(),
                        consumed_milligas: Some("0".into()),
                        errors: errors.into(),
                        ..DEFAULT_RESULT
                    },
                    balance_updates: charges.unwrap()
                }),
                ..origination.clone().into()
            }
        }
    }

    if src_balance < origination.balance {
        errors.balance_too_low(&origination.balance, &src_balance, &origination.source);
        return Ok(make_receipt!(OperationResultStatus::Failed));
    } else {
        src_balance -= origination.balance;
        balance_updates.spend(&origination.source, &origination.balance);
        balance_updates.topup(&originated_address, &origination.balance);
    }

    // TODO: check that contract code is valid (all sections present)
    // TODO: check that contract does not use unsupported primitives
    // TODO: check storage against its type

    context.set_contract_code(&originated_address, Micheline::Sequence(origination.script.code.clone()))?;
    context.set_contract_storage(&originated_address, origination.script.storage.clone())?;

    context.set_balance(&origination.source, &src_balance)?;
    context.set_balance(&originated_address, &origination.balance)?;

    Ok(make_receipt!(OperationResultStatus::Applied))
}

#[cfg(test)]
mod test {
    use crate::context::{Context, ephemeral::EphemeralContext};
    use crate::Result;
    use tezos_operation::{
        operations::Script
    };
    use tezos_core::types::{
        encoded::ImplicitAddress,
        mutez::Mutez
    };
    use tezos_michelson::michelson::{
        types::{code, storage, parameter, unit},
        data::instructions::{fail_with},
        data::Unit
    };

    use super::*;

    #[test]
    fn test_origination_applied() -> Result<()> {
        let mut context = EphemeralContext::new();

        let source = ImplicitAddress::try_from("tz1V3dHSCJnWPRdzDmZGCZaTMuiTmbtPakmU")?;
        context.set_balance(&source, &Mutez::from(1000000000u32))?;

        let origination = Origination {
            source: source.clone(),
            counter: 200000u32.into(),
            fee: 1000u32.into(),
            gas_limit: 0u32.into(),
            storage_limit: 0u32.into(),
            balance: 500000000u32.into(),
            delegate: None,
            script: Script {
                code: vec![
                    parameter(unit()),
                    storage(unit()),
                    code(fail_with())
                ].into(),
                storage: Unit.into()
            }
        };

        let mut index = 1i32;
        let receipt = execute_origination(
            &mut context, 
            &origination,
            &OperationHash::new("oneDGhZacw99EEFaYDTtWfz5QEhUW3PPVFsHa7GShnLPuDn7gSd".into())?,
            &mut index
        )?;
        let metadata = receipt.metadata.unwrap();
        let originated_contracts = metadata.operation_result.originated_contracts.unwrap();
        let address = originated_contracts.first().unwrap();
        let dummy_address = ContractAddress::try_from("KT1Mjjcb6tmSsLm7Cb3DSQszePjfchPM4Uxm")?;
        assert_eq!(dummy_address, *address);

        assert_eq!(context.get_balance(&source)?.unwrap(), Mutez::from(1000000000u32 - 500000000u32));
        assert_eq!(context.get_balance(address)?.unwrap(), Mutez::from(500000000u32));
        Ok(())
    }
}