use ibig::{IBig, UBig};
use tezos_michelson::michelson::Michelson;
use tezos_michelson::micheline::Micheline;
use tezos_michelson::michelson::data::instructions::{
    Compare, Eq, Ge, Gt, Le, Lt, Neq, Concat, Slice, Size, Pack, Unpack
};
use tezos_michelson::michelson::types::{Type, ComparableType};

use crate::{
    Result,
    Error,
    vm::interpreter::{PureInterpreter},
    vm::types::{StackItem},
    vm::typechecker::{check_type_comparable, check_types_equal},
    vm::stack::Stack,
    pop_cast,
    err_type
};

impl PureInterpreter for Compare {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let a = stack.pop()?;
        let b = stack.pop()?;

        let lty = a.get_type()?;
        check_type_comparable(&lty)?;
        let rty = b.get_type()?;
        check_types_equal(&lty, &rty)?;

        let res = match a.partial_cmp(&b) {
            Some(ord) => IBig::from(ord as i8),
            None => return Err(Error::ComparisonError)
        };
        stack.push(StackItem::Int(res.into()))
    }
}

macro_rules! impl_interpreter_for_op {
    ($instr: ty, $op: tt) => {
        impl PureInterpreter for $instr {
            fn execute(&self, stack: &mut Stack) -> Result<()> {
                let a = stack.pop()?;
                let b = stack.pop()?;
        
                let lty = a.get_type()?;
                check_type_comparable(&lty)?;
                let rty = b.get_type()?;
                check_types_equal(&lty, &rty)?;
        
                let res = a $op b;
                stack.push(StackItem::Bool(res.into()))
            }
        }
    };
}

impl_interpreter_for_op!(Eq, ==);
impl_interpreter_for_op!(Neq, !=);
impl_interpreter_for_op!(Ge, >=);
impl_interpreter_for_op!(Gt, >);
impl_interpreter_for_op!(Le, <=);
impl_interpreter_for_op!(Lt, <);

impl PureInterpreter for Size {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let size = match stack.pop()? {
            StackItem::String(item) => item.len(),
            StackItem::Bytes(item) => item.len(),
            StackItem::List(item) => item.len(),
            StackItem::Set(item) => item.len(),
            StackItem::Map(item) => item.len(),
            item => return err_type!("Sizeable", item)
        };
        stack.push(StackItem::Nat(UBig::from(size).into()))
    }
}

impl PureInterpreter for Slice {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let offset = pop_cast!(stack, Nat);
        let length = pop_cast!(stack, Nat);

        let offset: usize = offset.try_into()?;
        let length: usize = length.try_into()?;

        let res = match stack.pop()? {
            StackItem::String(item) => item.slice(offset, offset + length),
            StackItem::Bytes(item) => item.slice(offset, offset + length),
            item => return err_type!("string or bytes", item)
        };
        stack.push(res.into())
    }
}

impl PureInterpreter for Concat {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let res = match stack.pop()? {
            StackItem::List(list) => {
                let (items, inner_ty) = list.unwrap();
                match inner_ty {
                    Type::Comparable(ComparableType::String(_)) => {
                        let mut output: Vec<String> = Vec::with_capacity(items.len());
                        for item in items {
                            match item {
                                StackItem::String(item) => output.push(item.unwrap()),
                                _ => return err_type!("string", item)
                            }
                        }
                        StackItem::String(output.concat().into())
                    },
                    Type::Comparable(ComparableType::Bytes(_)) => {
                        let mut output: Vec<u8> = Vec::new();
                        for item in items {
                            match item {
                                StackItem::Bytes(item) => output.append(item.unwrap().as_mut()),
                                _ => return err_type!("bytes", item)
                            }
                        }
                        StackItem::Bytes(output.into())
                    },
                    ty => return err_type!("string or bytes", ty)
                }
            },
            StackItem::String(a) => {
                let b = pop_cast!(stack, String);
                (a + b).into()
            },
            StackItem::Bytes(a) => {
                let b = pop_cast!(stack, Bytes);
                (a + b).into()
            },
            item => return err_type!("list(str | bytes) or (str, str) or (bytes, bytes)", item)
        };
        stack.push(res)
    }
}

impl PureInterpreter for Pack {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let item = stack.pop()?;
        let ty = item.get_type()?;
        let data = item.into_micheline(&ty)?;
        let schema: Micheline = Michelson::from(ty).into();
        let res = data.pack(Some(&schema))?;
        stack.push(StackItem::Bytes(res.into()))
    }
}

impl PureInterpreter for Unpack {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let item = pop_cast!(stack, Bytes);
        todo!("Add Unpack type to the tezos_michelson crate")
    }
}