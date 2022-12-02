use host::runtime::Runtime;
use tezos_operation::operations::Reveal;
use tezos_core::types::{
        encoded::Encoded
};
use tezos_rpc::models::operation::{
    OperationContent as OperationContentWithResult
};
use tezos_rpc::models::operation::{
    operation_contents_and_result::{
        reveal::{Reveal as RevealWithResult, RevealMetadata}
    },
    operation_result::operations::{
        reveal::RevealOperationResult
    },
    operation_result::OperationResultStatus,
    kind::{OperationKind},
};
use tezos_rpc::models::{
    balance_update,
    error::RpcError
};

use crate::error::Result;
use crate::context::EphemeralContext;

pub fn execute_reveal(host: &mut impl Runtime, context: &mut EphemeralContext, reveal: &Reveal) -> Result<OperationContentWithResult> {
    let mut balance = context.get_balance(host, &reveal.source).unwrap();  // already checked by validator
    balance -= reveal.fee;  // TODO: need additional check?
    context.set_balance(&reveal.source, &balance);
    context.set_counter(&reveal.source, &reveal.counter);

    let status;
    let errors;

    if context.has_public_key(host, &reveal.source) {
        status = OperationResultStatus::Failed;
        errors = Some(vec![
            RpcError {
                kind: "permanent".into(),
                id: "contract.previously_revealed_key".into(),
                amount: None,
                balance: None,
                contract: None,
                message: None,
                msg: None
            }
        ]);
    } else {
        context.set_public_key(&reveal.source, &reveal.public_key);
        status = OperationResultStatus::Applied;
        errors = None;
    }

    let res = RevealWithResult {
        kind: OperationKind::Reveal,
        source: reveal.source.clone(),
        counter: reveal.counter.to_string(),
        fee: reveal.fee,
        gas_limit: reveal.gas_limit.to_string(),
        storage_limit: reveal.storage_limit.to_string(),
        public_key: reveal.public_key.clone(),
        metadata: Some(RevealMetadata { 
            operation_result: RevealOperationResult { 
                status,
                consumed_gas: None, 
                consumed_milligas: Some("0".into()), 
                errors
            }, 
            balance_updates: vec![
                balance_update::BalanceUpdate::Contract(balance_update::Contract {
                    kind: balance_update::Kind::Contract,
                    contract: reveal.source.value().into(),
                    change: format!("-{}", reveal.fee),
                    origin: None  // TODO: increase sequencer's balance
                })
            ]
        })
    };
    Ok(OperationContentWithResult::Reveal(res))
}

#[cfg(test)]
mod test {
    use crate::context::EphemeralContext;
    use crate::error::Result;
    use mock_runtime::host::MockHost;
    use tezos_operation::{
        operations::Reveal
    };
    use tezos_core::types::{
        encoded::{ImplicitAddress, PublicKey},
        mutez::Mutez,
        number::Nat
    };

    use super::execute_reveal;

    #[test]
    fn test_reveal_applied() -> Result<()> {
        let mut host = MockHost::default();
        let mut context = EphemeralContext::new();

        let address = ImplicitAddress::try_from("tz1V3dHSCJnWPRdzDmZGCZaTMuiTmbtPakmU").unwrap();
        let public_key = PublicKey::try_from("edpktipCJ3SkjvtdcrwELhvupnyYJSmqoXu3kdzK1vL6fT5cY8FTEa").unwrap();

        context.set_balance(&address, &Mutez::from(1000000000u32));
        context.set_counter(&address, &Nat::try_from("100000").unwrap());

        let reveal = Reveal {
            source: address.clone(),
            counter: 200000u32.into(),
            fee: 1000u32.into(),
            gas_limit: 0u32.into(),
            storage_limit: 0u32.into(),
            public_key: public_key.clone()
        };

        let result = execute_reveal(&mut host, &mut context, &reveal);
        assert!(result.is_ok());

        assert_eq!(context.get_public_key(&host, &address).unwrap(), public_key);
        assert_eq!(context.get_balance(&host, &address).unwrap(), Mutez::from(1000000000u32 - 1000u32));
        
        Ok(())
    }
}