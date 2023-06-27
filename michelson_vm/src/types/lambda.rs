// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use std::fmt::Display;
use tezos_michelson::michelson::{
    data::{Data, Instruction},
    types,
    types::Type,
};

use crate::{
    err_mismatch,
    formatter::Formatter,
    type_cast,
    types::{LambdaItem, StackItem},
    Result,
};

fn parse_instruction(data: Data) -> Result<Instruction> {
    match data {
        Data::Instruction(instr) => Ok(instr),
        Data::Sequence(seq) => {
            let values = seq.into_values();
            let mut instructions: Vec<Instruction> = Vec::with_capacity(values.len());
            for value in values {
                instructions.push(parse_instruction(value)?);
            }
            Ok(Instruction::Sequence(instructions.into()))
        }
        _ => err_mismatch!("Instruction or Sequence", data.format()),
    }
}

impl LambdaItem {
    pub fn new(param_type: Type, return_type: Type, body: Instruction) -> Self {
        Self {
            outer_value: body,
            inner_type: (param_type, return_type),
        }
    }

    pub fn from_data(data: Data, parameter_type: &Type, return_type: &Type) -> Result<StackItem> {
        let body = parse_instruction(data)?;
        let lambda = Self::new(parameter_type.clone(), return_type.clone(), body);
        Ok(lambda.into())
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        type_cast!(ty, Lambda);
        Ok(Data::Instruction(self.outer_value))
    }

    pub fn get_type(&self) -> Result<Type> {
        Ok(types::lambda(
            self.inner_type.0.clone(),
            self.inner_type.1.clone(),
        ))
    }

    pub fn unwrap(self) -> (Instruction, (Type, Type)) {
        (self.outer_value, self.inner_type)
    }
}

impl PartialEq for LambdaItem {
    fn eq(&self, other: &Self) -> bool {
        // For testing purposes
        self.outer_value == other.outer_value
    }
}

impl Display for LambdaItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Lambda")
    }
}
