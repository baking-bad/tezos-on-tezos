use context::ExecutorContext;
use tezos_core::types::{
    encoded::{Encoded, ImplicitAddress, OperationHash},
    mutez::Mutez,
    number::Nat,
};
use tezos_operation::operations::{OperationContent, SignedOperation};

use crate::{Error, Result};

pub struct ManagerOperation {
    pub hash: OperationHash,
    pub origin: SignedOperation,
    pub source: ImplicitAddress,
    pub total_fees: Mutez,
    pub total_spent: Mutez,
    pub last_counter: Nat,
}

pub fn validate_operation(
    context: &mut impl ExecutorContext,
    opg: SignedOperation,
    hash: OperationHash,
    dry_run: bool,
) -> Result<ManagerOperation> {
    let mut source = None;
    let mut total_fees: Mutez = 0u32.into();
    let mut total_spent: Mutez = 0u32.into();

    // TODO: validate branch?
    // On the one hand there should be something like TTL
    // On the other hand if even with L2 we hit issues with throughput, why bother?
    // In order to validate branch one need to keep the entire (or rolling) history of heads [Head]

    for content in opg.contents.iter() {
        let (address, amount) = match content {
            OperationContent::Reveal(reveal) => (&reveal.source, None),
            OperationContent::Transaction(transaction) => {
                (&transaction.source, Some(transaction.amount))
            }
            OperationContent::Origination(origination) => (&origination.source, None),
            _ => return Err(Error::OperationKindUnsupported),
        };

        if source.is_none() {
            source = Some(address);
        } else if source.unwrap() != address {
            return Err(Error::InconsistentSources);
        }

        // TODO: check against constant fee values (per operation kind)

        total_fees += content.fee();
        total_spent += content.fee() + amount.unwrap_or(0u32.into());
    }

    if source.is_none() {
        return Err(Error::ContentsListError);
    }

    let source = source.unwrap().clone();
    let public_key = match context.get_public_key(&source.value())? {
        Some(value) => value,
        None => {
            let revealed_key = opg
                .contents
                .iter()
                .filter_map(|content| match content {
                    OperationContent::Reveal(reveal) => Some(reveal.public_key.clone()),
                    _ => None,
                })
                .next();

            if revealed_key.is_some() {
                revealed_key.unwrap()
            } else {
                return Err(Error::UnrevealedPublicKey);
            }
        }
    };

    if !dry_run {
        match opg.verify(&public_key) {
            Ok(true) => (),
            Ok(false) => return Err(Error::InvalidSignature),
            Err(err) => return Err(err.into()),
        };
    }

    let balance = match context.get_balance(&source.value())? {
        Some(value) => value,
        None => return Err(Error::EmptyImplicitContract),
    };

    if balance < total_spent {
        return Err(Error::BalanceTooLow { balance });
    }

    let mut counter = match context.get_counter(&source.value())? {
        Some(value) => value.to_integer()?,
        None => 0u64,
    };

    for content in opg.contents.iter() {
        let next_counter: u64 = match content {
            OperationContent::Reveal(reveal) => &reveal.counter,
            OperationContent::Transaction(transaction) => &transaction.counter,
            OperationContent::Origination(origination) => &origination.counter,
            _ => return Err(Error::OperationKindUnsupported),
        }
        .to_integer()?;
        if next_counter <= counter {
            return Err(Error::CounterInThePast {
                expected: counter + 1,
                found: next_counter,
            });
        }
        counter = next_counter;
    }

    Ok(ManagerOperation {
        hash,
        origin: opg,
        source: source.clone(),
        total_fees,
        total_spent,
        last_counter: counter.into(),
    })
}

#[cfg(test)]
mod test {
    use context::{EphemeralContext, ExecutorContext, GenericContext};
    use tezos_core::types::{
        encoded::{Encoded, ImplicitAddress, PublicKey},
        mutez::Mutez,
        number::Nat,
    };
    use tezos_operation::operations::{Reveal, SignedOperation, Transaction};

    use super::*;
    use crate::Result;

    #[test]
    fn test_valid_tx() -> Result<()> {
        let mut context = EphemeralContext::new();

        let address = "tz1V3dHSCJnWPRdzDmZGCZaTMuiTmbtPakmU";
        context.set_balance(address, Mutez::from(1000000000u32))?;
        context.set_counter(address, Nat::try_from("100000").unwrap())?;
        context.set_public_key(
            address,
            PublicKey::try_from("edpktipCJ3SkjvtdcrwELhvupnyYJSmqoXu3kdzK1vL6fT5cY8FTEa").unwrap(),
        )?;
        context.commit()?;

        let opg = SignedOperation::new(
            "BMNvSHmWUkdonkG2oFwwQKxHUdrYQhUXqxLaSRX9wjMGfLddURC"
                .try_into()
                .unwrap(),
            vec![Transaction::new(
                address.clone().try_into()?,
                417u32.into(),
                2336132u32.into(),
                1527u32.into(),
                357u32.into(),
                498719u32.into(),
                "tz1d5Dr3gjsxQo5XNbjAj558mLy3nGGQgMFA".try_into().unwrap(),
                None,
            )
            .into()],
            "sigw1WNdYweqz1c7zKcvZFHQ18swSv4HBWje5quRmixxitPk7z8jtY63qXgKLPVfTM6XGxExPatBWJP44Bknyu3hDHDKJZgY"
                .try_into()
                .unwrap(),
        );

        let hash = opg.hash()?;
        let op = validate_operation(&mut context, opg, hash, false)?;
        assert_eq!(op.total_fees, 417u32.into());
        assert_eq!(op.last_counter, 2336132u32.into());

        Ok(())
    }

    #[test]
    fn test_reveal_and_tx_batch() -> Result<()> {
        let mut context = EphemeralContext::new();

        let address = ImplicitAddress::try_from("tz1Ng3bkhPwf6byrSWzBeBRTuaiKCQXzyRUK").unwrap();
        context.set_balance(&address.value(), Mutez::from(1000000000u32))?;
        context.commit()?;

        let opg = SignedOperation::new(
            "BMY9L2Nq2wTiHbS3UD8zncaKrbjpD3JdUvyf28ViJYadwpDKLBz"
                .try_into()
                .unwrap(),
            vec![
                Reveal::new(
                    address.clone(),
                    374u32.into(),
                    85938846u32.into(),
                    1100u32.into(),
                    0u32.into(),
                    "edpktvzfDT9BVRGxGmd4XR5qNELdvQD25iviUbKaj1U8ZdNj1GwJRV"
                        .try_into()
                        .unwrap(),
                )
                .into(),
                Transaction::new(
                    address.clone(),
                    665u32.into(),
                    85938847u32.into(),
                    1601u32.into(),
                    257u32.into(),
                    264282311u32.into(),
                    "tz1i8Z9QpQyejB66futrjwdyaEZMND7kMtTy".try_into().unwrap(),
                    None,
                )
                .into(),
            ],
            "sigchjxVdGxuHRb4aqhvYBufFz3t1kpfVQJdKEVvM685D5SuXAfu4h7dpCtkF8yNN1emcWF4vyNMxbEK4DFKFxvYtmxC24uo"
                .try_into()
                .unwrap(),
        );

        let hash = opg.hash()?;
        let op = validate_operation(&mut context, opg, hash, false)?;
        assert_eq!(op.total_fees, 1039u32.into());
        assert_eq!(op.last_counter, 85938847u32.into());

        Ok(())
    }
}
