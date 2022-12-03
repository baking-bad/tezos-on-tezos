use host::runtime::Runtime;
use tezos_operation::operations::Reveal;
use tezos_rpc::models::operation::{
    operation_result::{
        operations::reveal::RevealOperationResult,
        OperationResultStatus
    },
    operation_contents_and_result::reveal::{
        Reveal as RevealReceipt, 
        RevealMetadata
    }
};

use crate::executor::{
    runtime_error::RuntimeErrors,
    balance_update::BalanceUpdates
};
use crate::error::Result;
use crate::context::EphemeralContext;

pub fn skip_reveal(reveal: Reveal) -> RevealReceipt {
    RevealReceipt {
        metadata: Some(RevealMetadata {
            operation_result: RevealOperationResult {
                status: OperationResultStatus::Skipped,
                consumed_gas: None,
                consumed_milligas: None,
                errors: None
            },
            balance_updates: vec![]
        }),
        ..reveal.into()
    }
}

pub fn execute_reveal(host: &mut impl Runtime, context: &mut EphemeralContext, reveal: &Reveal) -> Result<RevealReceipt> {
    let mut errors = RuntimeErrors::new();
    let charges =  BalanceUpdates::fee(&reveal.source, &reveal.fee);

    macro_rules! make_receipt {
        ($a: expr) => {
            RevealReceipt {
                metadata: Some(RevealMetadata {
                    operation_result: RevealOperationResult {
                        status: $a,
                        consumed_gas: None,
                        consumed_milligas: Some("0".into()),
                        errors: errors.into()
                    },
                    balance_updates: charges.unwrap()
                }),
                ..reveal.to_owned().into()
            }
        }
    }

    if context.has_public_key(host, &reveal.source)? {
        errors.previously_revealed_key(&reveal.source);
        return Ok(make_receipt!(OperationResultStatus::Failed))
    }

    context.set_public_key(&reveal.source, &reveal.public_key)?;
    Ok(make_receipt!(OperationResultStatus::Applied))
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
        encoded::{ImplicitAddress, PublicKey, Encoded},
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

        context.set_balance(&address.value(), &Mutez::from(1000000000u32))?;
        context.set_counter(&address, &Nat::try_from("100000").unwrap())?;

        let reveal = Reveal {
            source: address.clone(),
            counter: 200000u32.into(),
            fee: 1000u32.into(),
            gas_limit: 0u32.into(),
            storage_limit: 0u32.into(),
            public_key: public_key.clone()
        };

        let receipt = execute_reveal(&mut host, &mut context, &reveal);
        assert!(receipt.is_ok());
        assert!(receipt.unwrap().metadata.is_some());

        assert_eq!(context.get_public_key(&host, &address)?.expect("Public key expected"), public_key);
        assert_eq!(context.get_balance(&host, &address.value())?.expect("Balance expected"), Mutez::from(1000000000u32 - 1000u32));
        
        Ok(())
    }
}