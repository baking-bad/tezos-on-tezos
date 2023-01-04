use tezos_michelson::michelson::data::instructions::{
    Balance, Address, Contract, Self_, ImplicitAccount
};
use tezos_michelson::michelson::{
    types::Type,
    types,
    annotations::{Kind, Annotation}
};
use tezos_core::types::encoded;

use crate::{
    Result,
    Error,
    interpreter::{TransactionScope, PureInterpreter, Interpreter, TransactionContext, ContextIntepreter},
    types::{MutezItem, AddressItem, StackItem, OptionItem, ContractItem},
    stack::Stack,
    typechecker::check_types_equal,
    trace_stack,
    pop_cast,
    err_type
};

impl Interpreter for Balance {
    fn execute(&self, stack: &mut Stack, scope: &TransactionScope, context: &mut impl TransactionContext) -> Result<()> {
        let balance = context
            .get_balance(&encoded::Address::Originated(scope.self_address.clone()))?
            .ok_or(Error::ContractNotFound)?;
        let res = MutezItem::new(balance.try_into()?)?;
        stack.push(res.into())
    }
}

impl PureInterpreter for Address {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let contract = pop_cast!(stack, Contract);
        let (address, _) = contract.unwrap();
        stack.push(AddressItem::new(address).into())
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
    context: &mut impl TransactionContext
) -> Result<Type> {
    match address {
        encoded::Address::Implicit(_) => {
            match context.get_balance(&address)? {
                Some(_) => Ok(types::unit()),
                None => Err(Error::ContractNotFound)
            }
        },
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
                None => Err(Error::ContractNotFound)
            }
        }
    }
}

impl ContextIntepreter for Contract {
    fn execute(&self, stack: &mut Stack, context: &mut impl TransactionContext) -> Result<()> {
        let address = pop_cast!(stack, Address);
        let address = address.unwrap();

        let res = match get_contract_type(&address, self.annotations(), context) {
            Ok(ty) => check_types_equal(&ty, &self.r#type),
            Err(err) => Err(err)
        };

        let item = match res {
            Ok(()) => {
                let item = ContractItem::new(address, self.r#type.clone());
                OptionItem::some(item.into())?
            },
            Err(_err) => {
                trace_stack!(&_err);
                OptionItem::none(&types::contract(self.r#type.clone()))
            }
        };

        stack.push(item.into())
    }
}

impl Interpreter for Self_ {
    fn execute(&self, stack: &mut Stack, scope: &TransactionScope, context: &mut impl TransactionContext) -> Result<()> {
        let address = encoded::Address::Originated(scope.self_address.clone());
        let self_type = get_contract_type(&address, self.annotations(), context)?;
        let item = ContractItem::new(address, self_type);
        stack.push(item.into())
    }
}

impl ContextIntepreter for ImplicitAccount {
    fn execute(&self, stack: &mut Stack, context: &mut impl TransactionContext) -> Result<()> {
        let key_hash = pop_cast!(stack, KeyHash);
        let address = encoded::Address::Implicit(key_hash.unwrap());
        let ty = get_contract_type(&address, vec![], context)?;
        check_types_equal(&ty, &types::unit())?;
        let item = ContractItem::new(address, ty);
        stack.push(item.into())
    }
}