use std::fmt::Display;
use tezos_michelson::michelson::{
    types::Type,
    types,
    data::{Data, Instruction},
};

use crate::{
    Result,
    vm::types::{LambdaItem, StackItem},
    err_type,
};

impl LambdaItem {
    pub fn new(param_type: Type, return_type: Type, body: Instruction) -> Self {
        Self { outer_value: body, inner_type: (param_type, return_type) }
    }

    pub fn from_data(data: Data, ty: &Type, parameter_type: &Type, return_type: &Type) -> Result<StackItem> {
        match data {
            Data::Instruction(body) => {
                let lambda = Self::new(parameter_type.clone(), return_type.clone(), body);
                Ok(lambda.into())
            },
            _ => err_type!(ty, data)
        }
    }
        
    pub fn into_data(self, ty: &Type) -> Result<Data> {
        if let Type::Lambda(_) = ty {
            return Ok(Data::Instruction(self.outer_value))
        }
        err_type!(ty, self)
    } 

    pub fn get_type(&self) -> Result<Type> {
        Ok(types::lambda(self.inner_type.0.clone(), self.inner_type.1.clone()))
    }

    pub fn unwrap(self) -> (Instruction, (Type, Type)) {
        (self.outer_value, self.inner_type)
    }
}

impl Display for LambdaItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Lambda")
    }
}