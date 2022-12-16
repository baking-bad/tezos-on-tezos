use tezos_operation::{
    operations::{UnsignedOperation, SignedOperation, OperationContent}
};
use tezos_core::{
    types::{
        encoded::{Signature, Encoded, ImplicitAddress}, mutez::Mutez, number::Nat
    }
};

use crate::{context::Context, validation_error};
use crate::error::Result;

const SIGNATURE_SIZE: usize = 64;

pub fn parse_operation<'a>(payload: &'a [u8]) -> Result<SignedOperation> {
    if payload.len() <= SIGNATURE_SIZE {
        return validation_error!("Payload too short");
    }

    let opg = UnsignedOperation::from_forged_bytes(&payload[..payload.len() - SIGNATURE_SIZE])?;  
    let signature = Signature::from_bytes(&payload[payload.len() - SIGNATURE_SIZE..])?;
    Ok(SignedOperation::from(opg, signature))
}

pub struct ManagerOperation {
    pub origin: SignedOperation,
    pub source: ImplicitAddress,
    pub total_fees: Mutez,
    pub last_counter: Nat
}

pub fn validate_operation(context: &mut impl Context, opg: SignedOperation) -> Result<ManagerOperation> {
    if context.has_pending_changes() {
        return validation_error!("Cannot proceed with uncommited state changes");
    }
    
    let mut source = None;
    let mut total_fees: Mutez = 0u32.into();

    for content in opg.contents.iter() {
        let address = match content {
            OperationContent::Reveal(reveal) => &reveal.source,
            OperationContent::Transaction(transaction) => &transaction.source,
            OperationContent::Origination(origination) => &origination.source,
            _ => return validation_error!("Unsupported operation kind: {:?}", content)
        };

        if source.is_none() {
            source = Some(address);
        } else if source.unwrap() != address {
            return validation_error!("Multiple sources in a group (expected {:?}, found {:?})", source.unwrap(), address);
        }

        total_fees += content.fee();
    }

    if source.is_none() {
        return validation_error!("Empty operation group");
    }

    let source = source.unwrap().clone();
    let public_key = match context.get_public_key(&source)? {
        Some(value) => value,
        None => {
            let revealed_key = opg.contents.iter().filter_map(|content| {
                match content {
                    OperationContent::Reveal(reveal) => Some(reveal.public_key.clone()),
                    _ => None
                }
            }).next();

            if revealed_key.is_some() {
                revealed_key.unwrap()
            } else {
                return validation_error!("Account {} has not revealed public key", source.value())
            }
        }
    };

    match opg.verify(&public_key) {
        Ok(true) => (),
        Ok(false) => return validation_error!("Signature is invalid"),
        Err(error) => return validation_error!("{}", error.to_string())
    };


    let balance = match context.get_balance(&source.value())? {
        Some(value) => value,
        None => return validation_error!("Balance not initialized for {}", source.value())
    };

    if balance < total_fees {
        return validation_error!("Account {} tries to spent more than it has", source.value());
    }

    let mut counter = match context.get_counter(&source)? {
        Some(value) => value.to_integer()?,
        None => 0u64
    };

    for content in opg.contents.iter() {
        let next_counter: u64 = match content {
            OperationContent::Reveal(reveal) => &reveal.counter,
            OperationContent::Transaction(transaction) => &transaction.counter,
            OperationContent::Origination(origination) => &origination.counter,
            _ => return validation_error!("Unsupported operation kind: {:?}", content)
        }.to_integer()?;
        if next_counter <= counter {
            return validation_error!("Account {} tries to reuse counter {}", source.value(), next_counter);
        }
        counter = next_counter;
    }

    Ok(ManagerOperation {
        origin: opg,
        source: source.clone(),
        total_fees,
        last_counter: counter.into()
    })
}

#[cfg(test)]
mod test {
    use crate::context::{Context, ephemeral::EphemeralContext};
    use crate::error::Result;
    use tezos_operation::{
        operations::{SignedOperation, Transaction}
    };
    use tezos_core::types::{
        encoded::{ImplicitAddress, PublicKey, Encoded},
        mutez::Mutez,
        number::Nat
    };

    use super::validate_operation;

    #[test]
    fn test_valid_tx() -> Result<()> {
        let mut context = EphemeralContext::new();

        let address = ImplicitAddress::try_from("tz1V3dHSCJnWPRdzDmZGCZaTMuiTmbtPakmU").unwrap();
        context.set_balance(&address.value(), &Mutez::from(1000000000u32))?;
        context.set_counter(&address, &Nat::try_from("100000").unwrap())?;
        context.set_public_key(&address, &PublicKey::try_from("edpktipCJ3SkjvtdcrwELhvupnyYJSmqoXu3kdzK1vL6fT5cY8FTEa").unwrap())?;
        context.commit()?;

        let opg = SignedOperation::new(
            "BMNvSHmWUkdonkG2oFwwQKxHUdrYQhUXqxLaSRX9wjMGfLddURC".try_into().unwrap(),
            vec![
                Transaction::new(
                    address.clone(),
                    417u32.into(),
                    2336132u32.into(),
                    1527u32.into(),
                    357u32.into(),
                    498719u32.into(),
                    "tz1d5Dr3gjsxQo5XNbjAj558mLy3nGGQgMFA".try_into().unwrap(),
                    None
                ).into()
            ],
            "sigw1WNdYweqz1c7zKcvZFHQ18swSv4HBWje5quRmixxitPk7z8jtY63qXgKLPVfTM6XGxExPatBWJP44Bknyu3hDHDKJZgY".try_into().unwrap()
        );

        let op = validate_operation(&mut context, opg)?;
        assert_eq!(op.total_fees, 417u32.into());
        assert_eq!(op.last_counter, 2336132u32.into());

        Ok(())    
    }
}