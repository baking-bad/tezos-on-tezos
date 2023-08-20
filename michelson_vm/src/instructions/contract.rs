// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use ibig::IBig;
use tezos_core::types::{encoded, encoded::Encoded};
use tezos_michelson::michelson::data::instructions::{
    Address, Contract, ImplicitAccount, Self_, TransferTokens,
};
use tezos_michelson::michelson::{annotations::Annotation, types, types::Type};

use crate::interpreter::TicketStorage;
use crate::{
    entrypoints::search_entrypoint,
    err_mismatch,
    interpreter::{
        ContextInterpreter, Interpreter, InterpreterContext, LazyStorage, OperationScope,
        PureInterpreter, ScopedInterpreter,
    },
    pop_cast,
    stack::Stack,
    trace_log,
    typechecker::check_types_equal,
    types::{AddressItem, ContractItem, OperationItem, OptionItem, StackItem},
    Error, Result,
};

impl PureInterpreter for Address {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let contract = pop_cast!(stack, Contract);
        let (address, _) = contract.into_components();
        stack.push(AddressItem::new(address).into())
    }
}

impl PureInterpreter for ImplicitAccount {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let key_hash = pop_cast!(stack, KeyHash);
        let address = encoded::Address::Implicit(key_hash.unwrap());
        let item = ContractItem::new(address, types::unit());
        stack.push(item.into())
    }
}

fn make_contract(address: &str, entrypoint: Option<&str>) -> Result<encoded::Address> {
    let contract_hash: encoded::ContractHash = address.try_into()?;
    let entrypoint = match entrypoint {
        Some("default") => None,
        Some(val) => Some(val),
        None => None,
    };
    let contract = encoded::ContractAddress::from_components(&contract_hash, entrypoint);
    Ok(contract.into())
}

fn get_contract(
    address: encoded::Address,
    field_name: &Option<Annotation>,
    expected_type: &Type,
    context: &mut impl InterpreterContext,
) -> Result<encoded::Address> {
    match address {
        encoded::Address::Implicit(_) => {
            check_types_equal(expected_type, &types::unit())?;
            Ok(address)
        }
        encoded::Address::Originated(kt) => {
            let entrypoint = match (field_name, kt.entrypoint()) {
                (None, None) => None,
                (Some(annot), None) => Some(annot.value_without_prefix()),
                (None, Some(entrypoint)) => Some(entrypoint),
                (Some(_), Some(_)) => {
                    return Err(Error::ConflictingEntrypoints {
                        address: kt.clone().into_string(),
                    })
                }
            };

            match context.get_contract_type(&kt)? {
                Some(expr) => {
                    let ty = search_entrypoint(expr.try_into()?, entrypoint, 0)?;
                    check_types_equal(expected_type, &ty)?;
                    make_contract(kt.contract_hash(), entrypoint)
                }
                None => Err(Error::ContractNotFound {
                    address: kt.clone().into_string(),
                }),
            }
        }
    }
}

impl ContextInterpreter for Contract {
    fn execute(&self, stack: &mut Stack, context: &mut impl InterpreterContext) -> Result<()> {
        let address = pop_cast!(stack, Address);

        let item = match get_contract(
            address.unwrap(),
            self.metadata().field_name(),
            &self.r#type,
            context,
        ) {
            Ok(contract) => {
                let item = ContractItem::new(contract, self.r#type.clone());
                OptionItem::some(item.into())
            }
            Err(_err) => {
                trace_log!(&_err);
                OptionItem::None(types::contract(self.r#type.clone()))
            }
        };

        stack.push(item.into())
    }
}

impl ScopedInterpreter for Self_ {
    fn execute(&self, stack: &mut Stack, scope: &OperationScope) -> Result<()> {
        let contract_hash = scope.self_address.contract_hash();
        let entrypoint = self
            .metadata()
            .field_name()
            .as_ref()
            .map(|annot| annot.value_without_prefix());
        let contract_type: Type = scope.self_type.clone().try_into()?;
        let self_ = ContractItem::new(
            make_contract(contract_hash, entrypoint)?,
            search_entrypoint(contract_type, entrypoint, 0)?,
        );
        stack.push(self_.into())
    }
}

impl Interpreter for TransferTokens {
    fn execute(
        &self,
        stack: &mut Stack,
        scope: &OperationScope,
        context: &mut impl InterpreterContext,
    ) -> Result<()> {
        let mut param = stack.pop()?;
        let amount = pop_cast!(stack, Mutez);
        let destination = pop_cast!(stack, Contract);

        let (destination, param_type) = destination.into_components();

        if let encoded::Address::Originated(kt) = &destination {
            param.try_acquire(kt, context)?;
            // TODO: support big_map ownership transfer
        }

        param.iter_tickets(&mut |t| {
            let amount: IBig = t.amount.value().into();
            context.update_ticket_balance(
                &scope.self_address.clone().into(),
                &t.identifier
                    .clone()
                    .into_micheline(&t.identifier.get_type()?)
                    .unwrap(),
                &t.identifier.get_type()?,
                &scope.self_address.clone().into(),
                -amount,
            )
        })?;

        let res = OperationItem::new(destination, param, param_type, amount, scope.source.clone());
        stack.push(res.into())
    }
}
