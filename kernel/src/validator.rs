use host::runtime::Runtime;
use num_traits::cast::ToPrimitive;
use tezos_operation::{
    operations::{UnsignedOperation, SignedOperation, OperationContent}
};
use tezos_core::{
    types::{
        encoded::{Signature, Encoded, ImplicitAddress}
    },
    Tezos
};

use crate::{context::{EphemeralContext}, validation_error};
use crate::error::{Error, Result};
use crate::crypto::WasmCryptoConfig;

const SIGNATURE_SIZE: usize = 64;

pub fn parse_operation<'a>(payload: &'a [u8]) -> Result<SignedOperation> {
    if payload.len() <= SIGNATURE_SIZE {
        return validation_error!("Payload too short");
    }

    let signature = match Signature::from_bytes(&payload[payload.len() - SIGNATURE_SIZE..]) {
        Ok(value) => value,
        Err(error) => return Err(Error::from(error))
    };

    let opg = match UnsignedOperation::from_forged_bytes(&payload[..payload.len() - SIGNATURE_SIZE]) {
        Ok(value) => value,
        Err(error) => return Err(Error::from(error))
    };
    
    Ok(SignedOperation::from(opg, signature))
}

trait SupportedManagerContent {
    fn is_supported(&self) -> bool;
    fn source(&self) -> &ImplicitAddress;
    fn spent(&self) -> u64;
    fn counter(&self) -> u64;
}

impl SupportedManagerContent for OperationContent {
    fn is_supported(&self) -> bool {
        match self {
            OperationContent::Reveal(_) => true,
            OperationContent::Transaction(_) => true,
            OperationContent::Origination(_) => true,
            _ => false
        }
    }

    fn source(&self) -> &ImplicitAddress {
        match self {
            OperationContent::Reveal(reveal) => &reveal.source,
            OperationContent::Transaction(transaction) => &transaction.source,
            OperationContent::Origination(origination) => &origination.source,
            _ => panic!("Not supported")
        }
    }

    fn spent(&self) -> u64 {
        let spent = match self {
            OperationContent::Reveal(reveal) => reveal.fee,
            OperationContent::Transaction(transaction) => transaction.amount + transaction.fee,
            OperationContent::Origination(origination) => origination.balance + origination.fee,
            _ => panic!("Not supported")
        };
        spent.to_u64().unwrap()
    }

    fn counter(&self) -> u64 {
        let counter = match self {
            OperationContent::Reveal(reveal) => &reveal.counter,
            OperationContent::Transaction(transaction) => &transaction.counter,
            OperationContent::Origination(origination) => &origination.counter,
            _ => panic!("Not supported")
        };
        counter.to_integer().unwrap()
    }
}

pub fn validate_operation<'a>(host: &mut impl Runtime, context: &mut EphemeralContext, opg: &SignedOperation) -> Result<()> {
    if opg.contents.iter().any(|x| !x.is_supported()) {
        return validation_error!("Unsupported operation content");
    }
    
    let source = match opg.contents.iter().last() {
        Some(content) => content.source(),
        None => return validation_error!("At least one operation content required")
    };

    let public_key = match context.get_public_key(host, source) {
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

    let tezos = Tezos::new(Box::from(WasmCryptoConfig));
    match opg.verify_with(&public_key, &tezos) {
        Ok(true) => (),
        Ok(false) => return validation_error!("Signature is invalid"),
        Err(error) => return validation_error!("{}", error.to_string())
    };

    let mut counter: u64 = match context.get_counter(host, source) {
        Some(value) => value.to_integer().unwrap(),
        None => return validation_error!("Counter not initialized for {}", source.value())
    };

    let mut balance = match context.get_balance(host, &source) {
        Some(value) => value.to_u64().unwrap(),
        None => return validation_error!("Balance not initialized for {}", source.value())
    };

    for content in opg.contents.iter() {
        let next_counter = content.counter();
        if next_counter <= counter {
            return validation_error!("Account {} tries to reuse counter {}", source.value(), next_counter);
        }
        counter = next_counter;

        let spent = content.spent();
        if spent > balance {
            return validation_error!("Account {} tries to spent more than it has", source.value());
        }
        balance -= spent;
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::context::EphemeralContext;
    use crate::error::Result;
    use mock_runtime::host::MockHost;
    use tezos_operation::{
        operations::{SignedOperation, Transaction}
    };
    use tezos_core::types::{
        encoded::{ImplicitAddress, PublicKey},
        mutez::Mutez,
        number::Nat
    };

    use super::validate_operation;

    #[test]
    fn test_valid_tx() -> Result<()> {
        let mut host = MockHost::default();
        let mut context = EphemeralContext::new();

        let address = ImplicitAddress::try_from("tz1V3dHSCJnWPRdzDmZGCZaTMuiTmbtPakmU").unwrap();
        context.set_balance(&address, &Mutez::from(1000000000u32));
        context.set_counter(&address, &Nat::try_from("100000").unwrap());
        context.set_public_key(&address, &PublicKey::try_from("edpktipCJ3SkjvtdcrwELhvupnyYJSmqoXu3kdzK1vL6fT5cY8FTEa").unwrap());

        let opg = SignedOperation::new(
            "BMNvSHmWUkdonkG2oFwwQKxHUdrYQhUXqxLaSRX9wjMGfLddURC".try_into().unwrap(),
            vec![
                Transaction::new(
                    "tz1V3dHSCJnWPRdzDmZGCZaTMuiTmbtPakmU".try_into().unwrap(),
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

        validate_operation(&mut host, &mut context, &opg)        
    }
}