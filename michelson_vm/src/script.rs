// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use tezos_core::types::encoded::Address;
use tezos_michelson::micheline::{primitive_application, sequence, sequence::Sequence, Micheline};
use tezos_michelson::michelson::{
    data::Instruction,
    types,
    types::{Code, Parameter, Storage, Type},
};

use crate::interpreter::TicketStorage;
use crate::types::ticket::TicketBalanceDiff;
use crate::{
    entrypoints::normalize_parameter,
    err_mismatch, err_unsupported, internal_error,
    interpreter::{Interpreter, InterpreterContext, LazyStorage, OperationScope},
    stack::Stack,
    trace_enter, trace_exit,
    types::{BigMapDiff, InternalContent, PairItem, StackItem},
    Error, Result,
};

#[derive(Clone, Debug)]
pub struct MichelsonScript {
    parameter_type: Type,
    storage_type: Type,
    code: Instruction,
}

#[derive(Clone, Debug)]
pub struct ScriptReturn {
    pub storage: Micheline,
    pub operations: Vec<InternalContent>,
    pub big_map_diff: Vec<BigMapDiff>,
    pub ticket_balance_diff: Vec<TicketBalanceDiff>,
}

impl MichelsonScript {
    pub fn get_type(&self) -> Micheline {
        self.parameter_type.clone().into()
    }

    pub fn get_code(&self) -> Micheline {
        sequence(vec![
            types::parameter(self.parameter_type.clone()),
            types::storage(self.storage_type.clone()),
            types::code(self.code.clone()),
        ])
    }

    pub fn call_begin(&self, stack: &mut Stack, scope: &OperationScope) -> Result<()> {
        trace_enter!();
        if stack.len() != 0 {
            return Err(Error::BadStack { location: 0 });
        }

        let (entrypoint, parameter) = match &scope.parameters {
            Some((e, p)) => (e.as_str(), p.clone()),
            None => ("default", primitive_application("Unit").into()),
        };

        let param = normalize_parameter(parameter, entrypoint, &self.parameter_type)?;
        let param_item = StackItem::from_micheline(param, &self.parameter_type)?;

        // check tickets
        let mut has_tickets = false;

        param_item.iter_tickets(&mut |t| -> Result<()> {
            has_tickets = true;
            if t.amount.is_zero() {
                return Err(Error::ForbiddenZeroAmountTicket);
            }
            Ok(())
        })?;

        if has_tickets {
            if let Address::Implicit(_) = scope.sender {
                return Err(Error::UnexpectedTicketOwner);
            }
        }

        let storage = scope.storage.clone().normalized();
        let storage_item = StackItem::from_micheline(storage, &self.storage_type)?;

        let input = PairItem::new(param_item, storage_item);
        stack.push(input.into())
    }

    pub fn call_end(
        &self,
        stack: &mut Stack,
        scope: &OperationScope,
        context: &mut impl InterpreterContext,
    ) -> Result<ScriptReturn> {
        if stack.len() != 1 {
            return Err(Error::BadReturn);
        }

        let (op_list, mut storage) = match stack.pop()? {
            StackItem::Pair(pair) => match pair.unpair() {
                (StackItem::List(op_list), storage) => (op_list, storage),
                (f, s) => {
                    return err_mismatch!("(ListItem * StackItem)", format!("({} * {})", f, s))
                }
            },
            item => return err_mismatch!("PairItem", item),
        };

        let mut big_map_diff: Vec<BigMapDiff> = Vec::new();
        storage.try_acquire(&scope.self_address, context)?;
        storage.try_aggregate(&mut big_map_diff, &self.storage_type)?;

        let mut operations: Vec<InternalContent> = Vec::with_capacity(op_list.len());
        let (items, _) = op_list.into_elements();
        for item in items {
            match item {
                StackItem::Operation(mut op) => {
                    op.aggregate_diff(&mut big_map_diff);
                    operations.push(op.into_content()?)
                }
                item => return err_mismatch!("OperationItem", item),
            }
        }

        let ticket_balance_updates = context.aggregate_ticket_updates();

        let ret = ScriptReturn {
            big_map_diff,
            operations,
            storage: storage.into_micheline(&self.storage_type)?,
            ticket_balance_diff: ticket_balance_updates,
        };
        Ok(ret)
    }

    pub fn call(
        &self,
        scope: &OperationScope,
        context: &mut impl InterpreterContext,
    ) -> Result<ScriptReturn> {
        let mut stack = Stack::new();

        if let Err(err) = self.call_begin(&mut stack, scope) {
            trace_exit!(Some(&err));
            return Err(err);
        }

        if let Err(err) = self.execute(&mut stack, scope, context) {
            trace_exit!(Some(&err));
            return Err(err);
        }

        match self.call_end(&mut stack, scope, context) {
            Ok(ret) => Ok(ret),
            Err(err) => {
                trace_exit!(Some(&err));
                Err(err)
            }
        }
    }

    pub fn originate(
        &self,
        scope: &OperationScope,
        context: &mut impl InterpreterContext,
    ) -> Result<ScriptReturn> {
        let expr = scope.storage.clone().normalized();
        let mut storage = StackItem::from_micheline(expr, &self.storage_type)?;

        let mut big_map_diff: Vec<BigMapDiff> = Vec::new();
        storage.try_acquire(&scope.self_address, context)?;
        storage.try_aggregate(&mut big_map_diff, &self.storage_type)?;

        context.set_contract_type(scope.self_address.clone(), scope.self_type.clone())?;

        let ret = ScriptReturn {
            big_map_diff,
            storage: storage.into_micheline(&self.storage_type)?,
            operations: vec![],
            ticket_balance_diff: vec![],
        };
        Ok(ret)
    }
}

impl Interpreter for MichelsonScript {
    fn execute(
        &self,
        stack: &mut Stack,
        scope: &OperationScope,
        context: &mut impl InterpreterContext,
    ) -> Result<()> {
        self.code.execute(stack, scope, context)
    }
}

impl TryFrom<Sequence> for MichelsonScript {
    type Error = Error;

    fn try_from(sections: Sequence) -> Result<Self> {
        let mut param_ty: Option<Type> = None;
        let mut storage_ty: Option<Type> = None;
        let mut code_ty: Option<Instruction> = None;

        for section in sections.into_values() {
            match section {
                Micheline::Sequence(inner) => return MichelsonScript::try_from(inner),
                Micheline::PrimitiveApplication(prim) => match prim.prim() {
                    "parameter" => param_ty = Some(*Parameter::try_from(prim)?.r#type),
                    "storage" => storage_ty = Some(*Storage::try_from(prim)?.r#type),
                    "code" => code_ty = Some(*Code::try_from(prim)?.code),
                    prim => return err_unsupported!(prim),
                },
                Micheline::Literal(_) => return err_unsupported!("literal"),
            }
        }

        // TODO:
        // - check if all types in storage are storable
        // - check if all types in parameter are passable
        Ok(Self {
            parameter_type: param_ty.ok_or(internal_error!("Missing section:\tparameter"))?,
            storage_type: storage_ty.ok_or(internal_error!("Missing section:\tstorage"))?,
            code: code_ty.ok_or(internal_error!("Missing section:\tcode"))?,
        })
    }
}

impl TryFrom<Micheline> for MichelsonScript {
    type Error = Error;

    fn try_from(value: Micheline) -> Result<Self> {
        let sections = Sequence::try_from(value.normalized())?;
        Self::try_from(sections)
    }
}
