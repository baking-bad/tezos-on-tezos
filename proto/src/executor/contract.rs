use tezos_core::types::{encoded::{Encoded, ContractAddress}, mutez::Mutez};
use tezos_operation::operations::{Transaction, Origination, OperationContent};
use vm::{
    script::{MichelsonScript, ScriptReturn},
    interpreter::{OperationScope, InterpreterContext},
    types::InternalContent
};

use crate::{
    Result,
    Error,
    context::proto::ProtoContext,
    constants
};

pub fn deploy_contract(
    context: &mut (impl ProtoContext + InterpreterContext),
    origination: &Origination,
    self_address: ContractAddress,
    balance: Mutez
) -> Result<ScriptReturn> {
    let head = context.get_head()?;
    let script = MichelsonScript::try_from(origination.script.code.clone())?;

    let scope = OperationScope {
        amount: 0u32.into(),
        balance,
        chain_id: constants::CHAIN_ID.try_into()?,
        level: head.level + 1,
        now: head.timestamp + constants::BLOCK_TIME,
        parameters: None,
        self_address,
        self_type: script.get_type(),
        sender: origination.source.into(),
        source: origination.source.into(),
        storage: origination.script.storage
    };

    match script.originate(&scope, context) {
        Ok(ret) => {
            context.set_contract_code(
                scope.self_address.value(),
                script.get_code(),
            )?;
            context.set_contract_storage(
                scope.self_address.value(), 
                ret.storage
            )?;
            Ok(ret)
        },
        Err(err) => Err(err.into())
    }
}


pub fn execute_contract(
    context: &mut (impl ProtoContext + InterpreterContext),
    transaction: &Transaction,
    balance: Mutez,
) -> Result<ScriptReturn> {
    let self_address = transaction.destination.value();
    let code = context.get_contract_code(self_address)?
        .ok_or(Error::ContractCodeMissing { address: self_address.into() })?;
    let initial_storage = context.get_contract_storage(self_address)?
        .ok_or(Error::ContractStorageMissing { address: self_address.into() })?;

    let head = context.get_head()?;
    let script = MichelsonScript::try_from(code)?;

    let scope = OperationScope {
        amount: transaction.amount.clone(),
        balance,
        chain_id: constants::CHAIN_ID.try_into()?,
        level: head.level + 1,
        now: head.timestamp + constants::BLOCK_TIME,
        parameters: match transaction.parameters {
            Some(params) => Some((params.entrypoint.to_str().into(), params.value)),
            None => None
        },
        self_address: self_address.try_into()?,
        self_type: script.get_type(),
        sender: transaction.source.clone().into(),
        source: transaction.source.clone(),
        storage: initial_storage
    };

    Ok(script.call(&scope, context)?)
}

pub fn expand_content(internal: InternalContent) -> OperationContent {
    todo!()
}