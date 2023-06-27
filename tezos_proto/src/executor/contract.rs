// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use derive_more::From;
use michelson_vm::{
    interpreter::{InterpreterContext, OperationScope},
    script::{MichelsonScript, ScriptReturn},
    types::InternalContent,
};
use tezos_core::types::{
    encoded::{Address, ContractAddress, Encoded},
    mutez::Mutez,
};
use tezos_operation::operations::{
    Entrypoint, OperationContent, Origination, Parameters, Transaction,
};

use crate::{config, context::TezosContext, Error, Result};

#[derive(Debug, From)]
pub enum ContractOutput {
    Error(michelson_vm::Error),
    Return(ScriptReturn),
}

pub fn deploy_contract(
    context: &mut (impl TezosContext + InterpreterContext),
    origination: &Origination,
    self_address: ContractAddress,
    balance: Mutez,
) -> Result<ContractOutput> {
    let head = context.get_head()?;
    let script = MichelsonScript::try_from(origination.script.code.clone())?;

    let scope = OperationScope {
        amount: 0u32.into(),
        balance,
        chain_id: head.chain_id,
        level: head.level + 1,
        now: head.timestamp + config::BLOCK_TIME,
        parameters: None,
        self_address,
        self_type: script.get_type(),
        sender: origination.source.clone().into(),
        source: origination.source.clone().into(),
        storage: origination.script.storage.clone(),
    };

    match script.originate(&scope, context) {
        Ok(ret) => {
            context.set_contract_code(scope.self_address.value(), script.get_code())?;
            context.set_contract_storage(scope.self_address.value(), ret.storage.clone())?;
            Ok(ret.into())
        }
        Err(err) => Err(err.into()),
    }
}

pub fn execute_contract(
    context: &mut (impl TezosContext + InterpreterContext),
    transaction: &Transaction,
    sender: Option<Address>,
    balance: Mutez,
) -> Result<ContractOutput> {
    let self_address = transaction.destination.value();
    let code = context
        .get_contract_code(self_address)?
        .ok_or(Error::ContractCodeMissing {
            address: self_address.into(),
        })?;
    let initial_storage =
        context
            .get_contract_storage(self_address)?
            .ok_or(Error::ContractStorageMissing {
                address: self_address.into(),
            })?;

    let head = context.get_head()?;
    let script = MichelsonScript::try_from(code)?;

    let scope = OperationScope {
        amount: transaction.amount.clone(),
        balance,
        chain_id: head.chain_id,
        level: head.level + 1,
        now: head.timestamp + config::BLOCK_TIME,
        parameters: match &transaction.parameters {
            Some(params) => Some((params.entrypoint.to_str().into(), params.value.clone())),
            None => None,
        },
        self_address: self_address.try_into()?,
        self_type: script.get_type(),
        sender: sender.unwrap_or(transaction.source.clone().into()),
        source: transaction.source.clone(),
        storage: initial_storage,
    };

    match script.call(&scope, context) {
        Ok(ret) => {
            context.set_contract_storage(self_address, ret.storage.clone())?;
            Ok(ret.into())
        }
        Err(err) => Ok(err.into()),
    }
}

pub fn expand_content(internal: InternalContent) -> OperationContent {
    match internal {
        InternalContent::Transaction {
            destination,
            parameter,
            amount,
            source,
        } => {
            let (destination, parameters) = match destination {
                Address::Implicit(_) => (destination, None),
                Address::Originated(kt) => {
                    let params = Parameters {
                        entrypoint: Entrypoint::from_str(kt.entrypoint().unwrap_or("default")),
                        value: parameter,
                    };
                    (kt.into(), Some(params))
                }
            };
            OperationContent::Transaction(Transaction {
                source,
                amount,
                destination,
                parameters,
                fee: 0u32.into(),
                counter: 0u32.into(),
                gas_limit: 0u32.into(),
                storage_limit: 0u32.into(),
            })
        }
    }
}
