use tezos_michelson::michelson::{
    types::{Parameter, Storage, Type, Code},
    data::Instruction
};
use tezos_michelson::micheline::{
    Micheline,
    sequence::Sequence,
    primitive_application::PrimitiveApplication,
    primitive_application
};

use crate::{
    Result,
    Error,
    stack::Stack,
    types::{StackItem, PairItem, InternalContent, BigMapDiff},
    interpreter::{Interpreter, InterpreterContext, OperationScope, LazyStorage},
    err_mismatch,
    internal_error,
    trace_enter,
    trace_exit, err_unsupported
};

pub struct MichelsonScript {
    parameter_type: Type,
    storage_type: Type,
    code: Instruction
}

pub struct ScriptReturn {
    pub storage: Micheline,
    pub operations: Vec<InternalContent>,
    pub big_map_diff: Vec<BigMapDiff>
}

// Initial mask value: 0b1 <-- terminating 1
// Resulting mask value (example): 0b10001010010
// Least significant bit indicates the entrypoint node
// In order to wrap parameter with "Left"s and "Right"s one need to go in reverse order
fn get_entrypoint_mask(entrypoint: &str, ty: &Type, mask: i32) -> Result<i32> {
    if let Some(annot) = ty.metadata().field_name() {
        if annot.value_without_prefix() == entrypoint {
            return Ok(mask)
        }
    }
    if let Type::Or(or) = ty.clone() {
        if let Ok(res) = get_entrypoint_mask(entrypoint, &or.lhs, mask << 1) {
            return Ok(res)
        }
        if let Ok(res) = get_entrypoint_mask(entrypoint, &or.rhs, (mask << 1) | 1) {
            return Ok(res)
        }
     }
     if mask == 1 && entrypoint == "default" {
         return Ok(mask)
     }
     Err(Error::EntrypointNotFound { name: entrypoint.into() })
}

fn normalize_parameter(parameter: Micheline, entrypoint: &str, param_ty: &Type) -> Result<Micheline> {
    let mut parameter = parameter.normalized();
    let mut mask = get_entrypoint_mask(entrypoint, param_ty, 1)?;
    assert!(mask > 0);
   
    while mask > 1  {
        let prim: String = if mask & 1 == 0 { "Left".into() } else { "Right".into() };
        parameter = PrimitiveApplication::new(prim, Some(vec![parameter]), None).into();
        mask >>= 1;
    }
    
    Ok(parameter)
}

impl MichelsonScript {
    pub fn get_type(&self) -> Type {
        self.parameter_type.clone()
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

        let storage = scope.storage.clone().normalized();
        let storage_item = StackItem::from_micheline(storage, &self.storage_type)?;

        let input = PairItem::new(param_item, storage_item);
        stack.push(input.into())
    }

    pub fn call_end(&self, stack: &mut Stack, scope: &OperationScope, context: &mut impl InterpreterContext) -> Result<ScriptReturn> {
        if stack.len() != 1 {
            return Err(Error::BadReturn);
        }

        let (op_list, mut storage) = match stack.pop()? {
            StackItem::Pair(pair) => match pair.unpair() {
                (StackItem::List(op_list), storage) => (op_list, storage),
                (f, s) => return err_mismatch!("(ListItem * StackItem)", format!("({} * {})", f, s))
            },
            item => return err_mismatch!("PairItem", item)
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
                    operations.push(op.into_content())
                },
                item => return err_mismatch!("OperationItem", item)
            }                        
        }                    

        let ret = ScriptReturn {
            big_map_diff,
            operations,
            storage: storage.into_micheline(&self.storage_type)?,
        };
        Ok(ret)
    }

    pub fn call(&self, scope: &OperationScope, context: &mut impl InterpreterContext) -> Result<ScriptReturn> {
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

    pub fn originate(&self, scope: &OperationScope, context: &mut impl InterpreterContext) -> Result<ScriptReturn> {
        let expr = scope.storage.clone().normalized();
        let mut storage = StackItem::from_micheline(expr, &self.storage_type)?;

        let mut big_map_diff: Vec<BigMapDiff> = Vec::new();
        storage.try_acquire(&scope.self_address, context)?;
        storage.try_aggregate(&mut big_map_diff, &self.storage_type)?;

        let ret = ScriptReturn {
            big_map_diff, 
            storage: storage.into_micheline(&self.storage_type)?,
            operations: vec![]
        };
        Ok(ret)
    }
}

impl Interpreter for MichelsonScript {
    fn execute(&self, stack: &mut Stack, scope: &OperationScope, context: &mut impl InterpreterContext) -> Result<()> {
        self.code.execute(stack, scope, context)
    }
}

impl TryFrom<Micheline> for MichelsonScript {
    type Error = Error;

    fn try_from(src: Micheline) -> std::result::Result<Self, Error> {
        let mut param_ty: Option<Type> = None;
        let mut storage_ty: Option<Type> = None;
        let mut code_ty: Option<Instruction> = None;

        let sections = Sequence::try_from(src.normalized())?;
        for section in sections.into_values() {
            let prim = PrimitiveApplication::try_from(section)?;
            match prim.prim() {
                "parameter" => param_ty = Some(*Parameter::try_from(prim)?.r#type),
                "storage" => storage_ty = Some(*Storage::try_from(prim)?.r#type),
                "code" => code_ty = Some(*Code::try_from(prim)?.code),
                prim => return err_unsupported!(prim),
            }
        }

        // TODO: 
        // - check if all types in storage are storable
        // - check if all types in parameter are passable
        Ok(Self {
            parameter_type: param_ty.ok_or(internal_error!(Parsing, "Missing section:\tparameter"))?,
            storage_type: storage_ty.ok_or(internal_error!(Parsing, "Missing section:\tstorage"))?,
            code: code_ty.ok_or(internal_error!(Parsing, "Missing section:\tcode"))?
        })
    }
}