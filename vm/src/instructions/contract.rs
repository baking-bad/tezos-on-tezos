use tezos_michelson::michelson::data::instructions::{
    Address, Contract, ImplicitAccount, TransferTokens
};
use tezos_michelson::michelson::{
    types::Type,
    types,
    annotations::{Kind, Annotation}
};
use tezos_core::types::{encoded, encoded::Encoded};

use crate::interpreter::LazyStorage;
use crate::{
    Result,
    Error,
    interpreter::{PureInterpreter, InterpreterContext, ContextInterpreter, Interpreter, OperationScope},
    types::{AddressItem, StackItem, OptionItem, ContractItem, InternalContent, OperationItem},
    stack::Stack,
    typechecker::check_types_equal,
    trace_log,
    pop_cast,
    err_mismatch
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

fn search_entrypoint(ty: Type, entrypoint: &str, depth: usize) -> Result<Type> {
    if let Some(annot) = ty.metadata().field_name() {
       if annot.value() == entrypoint {
            return Ok(ty)
        }
    }
    if let Type::Or(or) = ty.clone() {
        for inner_ty in [or.lhs, or.rhs] {
            if let Ok(ty) = search_entrypoint(*inner_ty, entrypoint, depth + 1) {
                return Ok(ty)
            }
        }        
    }
    if depth == 0 && entrypoint == "default" {
        return Ok(ty)
    }
    Err(Error::EntrypointNotFound { name: entrypoint.into() })
}

fn get_contract_type(
    address: &encoded::Address,
    annots: Vec<&Annotation>,
    context: &mut impl InterpreterContext
) -> Result<Type> {
    match address {
        encoded::Address::Implicit(_) => Ok(types::unit()),
        encoded::Address::Originated(kt) => {
            let field_annot = annots
                .into_iter()
                .filter(|a| a.kind() == Kind::Field)
                .last();

            let entrypoint = match (field_annot, kt.entrypoint()) {
                (None, None) => "default",
                (Some(annot), None) => annot.value(),
                (None, Some(entrypoint)) => entrypoint,
                (Some(_), Some(_)) => return Err(Error::ConflictingEntrypoints)
            };

            match context.get_contract_type(&kt)? {
                Some(expr) => search_entrypoint(expr.try_into()?, entrypoint, 0),
                None => Err(Error::ContractNotFound { address: kt.clone().into_string() })
            }
        }
    }
}

impl ContextInterpreter for Contract {
    fn execute(&self, stack: &mut Stack, context: &mut impl InterpreterContext) -> Result<()> {
        let address = pop_cast!(stack, Address);
        let address = address.unwrap();

        let res = match get_contract_type(&address, self.annotations(), context) {
            Ok(ty) => check_types_equal(&ty, &self.r#type),
            Err(err) => Err(err)
        };

        let item = match res {
            Ok(()) => {
                let item = ContractItem::new(address, self.r#type.clone());
                OptionItem::some(item.into())
            },
            Err(_err) => {
                trace_log!(&_err);
                OptionItem::None(types::contract(self.r#type.clone()))
            }
        };

        stack.push(item.into())
    }
}

impl Interpreter for TransferTokens {
    fn execute(&self, stack: &mut Stack, _scope: &OperationScope, context: &mut impl InterpreterContext) -> Result<()> {
        let mut param = stack.pop()?;
        let amount = pop_cast!(stack, Mutez);
        let destination = pop_cast!(stack, Contract);

        let (destination, param_type) = destination.into_components();

        if let encoded::Address::Originated(kt) = &destination {
            param.try_acquire(kt, context)?;
            // TODO: support big_map ownership transfer
        }

        let content = InternalContent::Transaction {
            destination,
            parameter: param.into_micheline(&param_type)?,
            amount: amount.try_into()?
        };

        let res = OperationItem::new(content);
        stack.push(res.into())
    }
}