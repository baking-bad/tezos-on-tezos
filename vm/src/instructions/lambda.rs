use tezos_michelson::michelson::data::instructions::{
    Sequence, Lambda, Exec, Apply, Instruction, push, pair
};
use tezos_michelson::michelson::types::Type;

use crate::{
    Result,
    interpreter::{Interpreter, TransactionScope, PureInterpreter, TransactionContext},
    types::{LambdaItem, StackItem},
    stack::Stack,
    pop_cast,
    err_type,
};

impl PureInterpreter for Lambda {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let item = LambdaItem::new(
            self.parameter_type.clone(), 
            self.return_type.clone(),
            self.body.clone().into()
        );
        stack.push(item.into())
    }
}

impl Interpreter for Exec {
    fn execute(&self, stack: &mut Stack, scope: &TransactionScope, context: &mut impl TransactionContext) -> Result<()> {
        let arg = stack.pop()?;
        let (body, (param_type, return_type)) = pop_cast!(stack, Lambda).unwrap();
        arg.type_check(&param_type)?;
        
        let mut inner_stack = Stack::new();
        inner_stack.push(arg)?;
        body.execute(&mut inner_stack, scope, context)?;
        assert_eq!(1, inner_stack.len());

        let ret = inner_stack.pop()?;
        ret.type_check(&return_type)?;
        stack.push(ret)
    }
}

impl PureInterpreter for Apply {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let const_arg = stack.pop()?;
        let (body, (param_type, return_type)) = pop_cast!(stack, Lambda).unwrap();

        let (const_ty, arg_ty) = match param_type {
            Type::Pair(pair_ty) => {
                assert_eq!(2, pair_ty.types.len());
                (pair_ty.types[0].clone(), pair_ty.types[1].clone())
            },
            ty => return err_type!("PairItem", ty)
        };

        let const_arg = const_arg.into_data(&const_ty)?;
        let body = Sequence::form(vec![
            Instruction::Push(push(const_ty, const_arg)),
            Instruction::Pair(pair(None)),
            body
        ]);

        let lambda = LambdaItem::new(arg_ty, return_type, body.into());
        stack.push(lambda.into())
    }
}