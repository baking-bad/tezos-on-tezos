use ibig::{IBig, UBig};
use tezos_michelson::michelson::Michelson;
use tezos_michelson::micheline::Micheline;
use tezos_michelson::michelson::data::instructions::{
    Compare, Eq, Ge, Gt, Le, Lt, Neq, Concat, Slice, Size, Pack, Unpack
};
use tezos_michelson::michelson::types::{Type, ComparableType};

use crate::{
    Result,
    interpreter::{PureInterpreter},
    types::{StackItem, OptionItem, IntItem},
    typechecker::{check_type_comparable, check_types_equal},
    formatter::Formatter,
    stack::Stack,
    pop_cast,
    err_mismatch,
    trace_log
};

impl PureInterpreter for Compare {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let a = stack.pop()?;
        let b = stack.pop()?;

        let lty = a.get_type()?;
        check_type_comparable(&lty)?;
        let rty = b.get_type()?;
        check_types_equal(&lty, &rty)?;

        let res = IBig::from(a.cmp(&b) as i8);
        stack.push(StackItem::Int(res.into()))
    }
}

macro_rules! impl_interpreter_for_op {
    ($instr: ty, $op: tt) => {
        impl PureInterpreter for $instr {
            fn execute(&self, stack: &mut Stack) -> Result<()> {
                let a = pop_cast!(stack, Int);        
                let res = a $op IntItem::from(0);
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
            item => return err_mismatch!("StringItem, BytesItem, ListItem, SetItem, or MapItem", item)
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
            item => return err_mismatch!("StringItem or BytesItem", item)
        };
        stack.push(res.into())
    }
}

impl PureInterpreter for Concat {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let res = match stack.pop()? {
            StackItem::List(list) => {
                let (items, inner_ty) = list.into_elements();
                match inner_ty {
                    Type::Comparable(ComparableType::String(_)) => {
                        let mut output: Vec<String> = Vec::with_capacity(items.len());
                        for item in items {
                            match item {
                                StackItem::String(item) => output.push(item.unwrap()),
                                _ => return err_mismatch!("StringItem", item)
                            }
                        }
                        StackItem::String(output.concat().into())
                    },
                    Type::Comparable(ComparableType::Bytes(_)) => {
                        let mut output: Vec<u8> = Vec::new();
                        for item in items {
                            match item {
                                StackItem::Bytes(item) => output.append(item.unwrap().as_mut()),
                                _ => return err_mismatch!("BytesItem", item)
                            }
                        }
                        StackItem::Bytes(output.into())
                    },
                    ty => return err_mismatch!("list(string || bytes)", ty.format())
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
            item => return err_mismatch!("ListItem<StringItem or BytesItem>, SringItem, or BytesItem", item)
        };
        stack.push(res)
    }
}

impl PureInterpreter for Pack {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let item = stack.pop()?;
        let ty = item.get_type()?;
        // TODO: check if packable
        let data = item.into_micheline(&ty)?;
        let schema: Micheline = Michelson::from(ty).into();
        let res = data.pack(Some(&schema))?;
        stack.push(StackItem::Bytes(res.into()))
    }
}

impl PureInterpreter for Unpack {
    fn execute(&self, stack: &mut Stack) -> Result<()> {
        let item = pop_cast!(stack, Bytes);
        let res = match Michelson::unpack(item.unwrap().as_slice(), Some(&self.r#type)) {
            Ok(data) => {
                let item = StackItem::from_data(data.try_into()?, &self.r#type)?;
                OptionItem::some(item)
            },
            Err(_err) => {
                trace_log!(&_err.into());
                OptionItem::none(&self.r#type)
            }
        };
        stack.push(res.into())
    }
}