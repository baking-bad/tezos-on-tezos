use tezos_michelson::michelson::{
    types::{Parameter, Storage, Code, Type, operation},
    data::{Data, Instruction}
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
    vm::stack::Stack,
    vm::types::{StackItem, PairItem, ListItem, OperationItem}
};

pub struct Script {
    parameter_type: Parameter,
    storage_type: Storage,
    code: Code
}

impl TryFrom<Micheline> for Script {
    type Error = Error;

    fn try_from(src: Micheline) -> Result<Self> {
        let mut param_ty: Option<Parameter> = None;
        let mut storage_ty: Option<Storage> = None;
        let mut code_ty: Option<Code> = None;

        let sections = Sequence::try_from(src.normalized())?;
        for section in sections.into_values() {
            let prim = PrimitiveApplication::try_from(section)?;
            match prim.prim() {
                "parameter" => param_ty = Some(Parameter::try_from(prim)?),
                "storage" => storage_ty = Some(Storage::try_from(prim)?),
                "code" => code_ty = Some(Code::try_from(prim)?),
                "view" => (),  // not supported, ignore
                _ => ()  // invalid, ignore
            }
        }

        Ok(Self {
            parameter_type: param_ty.ok_or(Error::ScriptSectionMissing)?,
            storage_type: storage_ty.ok_or(Error::ScriptSectionMissing)?,
            code: code_ty.ok_or(Error::ScriptSectionMissing)?
        })
    }
}

impl Script {
    pub fn allocate_lazy_storage() -> Result<Vec<LazyStorageDiff>> {
        Ok(())
    }

    pub fn get_lazy_storage_diff() -> Result<Vec<LazyStorageDiff>> {
        Ok(vec![])
    }

    pub fn begin(&self, stack: &mut Stack, parameter: Micheline, storage: Micheline) -> Result<()> {
        if stack.len() != 0 {
            return Err(Error::UnexpectedStackSize);
        }
        let param_item = StackItem::from_micheline(parameter, &Type::Parameter(self.parameter_type))?;
        let storage_item = StackItem::from_micheline(storage, &Type::Storage(self.storage_type))?;
        let input = PairItem::new(param_item, storage_item);
        stack.push(input.into())
    }

    pub fn end(&self, stack: &mut Stack) -> Result<(Micheline, Vec<OperationContent>)> {
        if stack.len() != 1 {
            return Err(Error::UnexpectedStackSize);
        }

        match stack.pop()? {
            StackItem::Pair(pair) => match pair.unpair() {
                (storage, StackItem::List(operations)) => {
                    let mut internal_operations: Vec<OperationContent> = Vec::with_capacity(operations.len());
                    for operation in operations.into_values(&operation()) {
                        match operation {
                            StackItem::Operation(op) => internal_operations.push(op.into_content()),
                            _ => todo!()
                        }                        
                    }
                },
                _ => todo!()
            },
            _ => todo!()
        }

        let res: PairItem = stack.pop()?.try_into();
        let (storage, operations): (StackItem, ListItem) = res.unpair().try_into()?;

        

        Ok((storage.into_micheline(self.storage_type)?, internal_operations))
    }
}