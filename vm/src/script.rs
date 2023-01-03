use tezos_michelson::michelson::{
    types::{Parameter, Storage, Type, Code},
    data::Instruction
};
use tezos_operation::operations::OperationContent;
use tezos_rpc::models::operation::operation_result::lazy_storage_diff::LazyStorageDiff;
use tezos_michelson::micheline::{
    Micheline,
    sequence::Sequence,
    primitive_application::PrimitiveApplication
};

use crate::{
    Result,
    Error,
    stack::Stack,
    types::{StackItem, PairItem},
    interpreter::{Interpreter, TransactionResult, TransactionScope, TransactionContext},
    err_type
};

pub struct Script {
    parameter_type: Type,
    storage_type: Type,
    code: Instruction
}

impl TryFrom<Micheline> for Script {
    type Error = Error;

    fn try_from(src: Micheline) -> Result<Self> {
        let mut param_ty: Option<Type> = None;
        let mut storage_ty: Option<Type> = None;
        let mut code_ty: Option<Instruction> = None;

        let sections = Sequence::try_from(src.normalized())?;  // TODO: normalize in-place for better perf
        for section in sections.into_values() {
            let prim = PrimitiveApplication::try_from(section)?;
            match prim.prim() {
                "parameter" => param_ty = Some(*Parameter::try_from(prim)?.r#type),
                "storage" => storage_ty = Some(*Storage::try_from(prim)?.r#type),
                "code" => code_ty = Some(*Code::try_from(prim)?.code),
                "view" => (),  // not supported, ignore
                _ => ()  // invalid, ignore
            }
        }

        Ok(Self {
            parameter_type: param_ty.ok_or(Error::MissingScriptField { prim: "parameter".into() })?,
            storage_type: storage_ty.ok_or(Error::MissingScriptField { prim: "storage".into() })?,
            code: code_ty.ok_or(Error::MissingScriptField { prim: "code".into() })?
        })
    }
}

impl Script {
    pub fn allocate_lazy_storage(&self, storage: Micheline, context: &mut impl TransactionContext) -> Result<Vec<LazyStorageDiff>> {
        Ok(vec![])
    }

    fn aggregate_lazy_storage_diff(&self, context: &mut impl TransactionContext) -> Result<Vec<LazyStorageDiff>> {
        Ok(vec![])
    }

    fn begin(&self, stack: &mut Stack, parameter: Micheline, storage: Micheline) -> Result<()> {
        if stack.len() != 0 {
            return Err(Error::BadReturn.into());
        }
        let param_item = StackItem::from_micheline(parameter, &self.parameter_type)?;
        let storage_item = StackItem::from_micheline(storage, &self.storage_type)?;
        let input = PairItem::new(param_item, storage_item);
        stack.push(input.into())
    }

    fn end(&self, stack: &mut Stack) -> Result<(Micheline, Vec<OperationContent>)> {
        if stack.len() != 1 {
            return Err(Error::BadReturn.into());
        }
        match stack.pop()? {
            StackItem::Pair(pair) => match pair.unpair() {
                (storage, StackItem::List(operations)) => {
                    let mut internal_operations: Vec<OperationContent> = Vec::with_capacity(operations.len());
                    let (items, _) = operations.unwrap();
                    for operation in items {
                        match operation {
                            StackItem::Operation(op) => internal_operations.push(op.into_content()),
                            item => return err_type!("OperationItem", item)
                        }                        
                    }
                    Ok((storage.into_micheline(&self.storage_type)?, internal_operations))
                },
                items => err_type!("(StackItem, ListItem)", items)
            },
            item => err_type!("PairItem", item)
        }
    }

    pub fn execute(&self, scope: &TransactionScope, context: &mut impl TransactionContext) -> Result<TransactionResult> {
        let mut stack = Stack::new();
        self.begin(&mut stack, scope.parameter.clone(), scope.storage.clone())?;
        self.code.execute(&mut stack, scope, context)?;
        let (storage, internal_operations) = self.end(&mut stack)?;
        debug_assert_eq!(0, stack.len());
        let lazy_storage_diff = self.aggregate_lazy_storage_diff(context)?;
        Ok(TransactionResult { storage, internal_operations, lazy_storage_diff })
    }
}