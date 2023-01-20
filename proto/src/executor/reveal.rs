use tezos_core::types::encoded::Encoded;
use tezos_operation::operations::Reveal;
use tezos_rpc::models::operation::operation_result::{
    operations::reveal::RevealOperationResult, OperationResultStatus,
};

use crate::{
    context::proto::ProtoContext,
    error::{Result, RpcErrors},
    executor::result::ExecutionResult,
};

pub fn execute_reveal(
    context: &mut impl ProtoContext,
    reveal: &Reveal,
    skip: bool,
) -> Result<ExecutionResult> {
    let mut errors = RpcErrors::new();

    macro_rules! result {
        ($status: ident) => {
            Ok(ExecutionResult::Reveal {
                content: reveal.clone(),
                result: RevealOperationResult {
                    status: OperationResultStatus::$status,
                    consumed_gas: None,
                    consumed_milligas: Some("0".into()),
                    errors: errors.into(),
                },
            })
        };
    }

    if skip {
        return result!(Skipped);
    }

    if context.has_public_key(reveal.source.value())? {
        errors.previously_revealed_key(reveal.source.value());
        return result!(Failed);
    }

    // TODO: check that public key actually matches address {
    //     errors.inconsistent_hash(&reveal.source);
    //     return Ok(make_receipt!(OperationResultStatus::Failed))
    // }

    context.set_public_key(reveal.source.value(), &reveal.public_key)?;
    result!(Applied)
}

#[cfg(test)]
mod test {
    use crate::context::{ephemeral::EphemeralContext, proto::ProtoContext};
    use crate::Result;
    use tezos_core::types::{encoded::PublicKey, mutez::Mutez, number::Nat};
    use tezos_operation::operations::Reveal;

    use super::execute_reveal;

    #[test]
    fn test_reveal_applied() -> Result<()> {
        let mut context = EphemeralContext::new();

        let address = "tz1V3dHSCJnWPRdzDmZGCZaTMuiTmbtPakmU";
        let public_key =
            PublicKey::try_from("edpktipCJ3SkjvtdcrwELhvupnyYJSmqoXu3kdzK1vL6fT5cY8FTEa").unwrap();

        context.set_balance(address, &Mutez::from(1000000000u32))?;
        context.set_counter(address, &Nat::try_from("100000").unwrap())?;

        let reveal = Reveal {
            source: address.try_into()?,
            counter: 200000u32.into(),
            fee: 1000u32.into(),
            gas_limit: 0u32.into(),
            storage_limit: 0u32.into(),
            public_key: public_key.clone(),
        };

        let res = execute_reveal(&mut context, &reveal, false)?;

        assert_eq!(
            context
                .get_public_key(address)?
                .expect("Public key expected"),
            public_key
        );
        assert_eq!(
            context.get_balance(address)?.expect("Balance expected"),
            Mutez::from(1000000000u32)
        );

        Ok(())
    }
}
