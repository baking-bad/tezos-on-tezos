use ibig::{IBig, UBig};
use tezos_core::types::encoded::{
    Address, PublicKey, ImplicitAddress, Signature
};
use tezos_michelson::michelson::{
    data::{Data, Instruction},
    types::Type,
};
use tezos_operation::operations::OperationContent;
use derive_more::From;

use crate::{
    error::{Result, Error}
};

macro_rules! define_item {
    ($name: ident, $impl: ty) => {
        #[derive(Debug, Clone, PartialEq, From)]
        pub struct $name(pub $impl);
    };
}

define_item!(UnitItem, ());
define_item!(BytesItem, Vec<u8>);
define_item!(StringItem, String);
define_item!(IntItem, IBig);
define_item!(NatItem, UBig);
define_item!(BoolItem, bool);
define_item!(TimestampItem, i64);
define_item!(MutezItem, i64);
define_item!(AddressItem, Address);
define_item!(KeyItem, PublicKey);
define_item!(KeyHashItem, ImplicitAddress);
define_item!(SignatureItem, Signature);
define_item!(OperationItem, OperationContent);
define_item!(OptionItem, Option<Box<StackItem>>);
define_item!(PairItem, (Box<StackItem>, Box<StackItem>));
define_item!(ListItem, Vec<StackItem>);
define_item!(SetItem, Vec<StackItem>);
define_item!(MapItem, Vec<(StackItem, StackItem)>);

#[derive(Debug, Clone, PartialEq)]
pub enum OrItem {
    Left(Box<StackItem>),
    Right(Box<StackItem>)
}

#[derive(Debug, Clone, PartialEq)]
pub enum BigMapItem {
    Ptr(i64),
    Map(MapItem)
}

#[derive(Debug, Clone, PartialEq)]
pub struct LambdaItem {
    arg_type: Type,
    ret_type: Type,
    body: Vec<Instruction>
}

#[derive(Debug, Clone, PartialEq, From)]
pub enum StackItem {
    Unit(UnitItem),
    Bytes(BytesItem),
    String(StringItem),
    Int(IntItem),
    Nat(NatItem),
    Bool(BoolItem),
    Timestamp(TimestampItem),
    Mutez(MutezItem),
    Address(AddressItem),
    Key(KeyItem),
    KeyHash(KeyHashItem),
    Signature(SignatureItem),
    Option(OptionItem),
    Or(OrItem),
    Pair(PairItem),
    List(ListItem),
    Set(SetItem),
    Map(MapItem),
    BigMap(BigMapItem),
    Lambda(LambdaItem),
    Operation(OperationItem),
}

pub struct Stack {
    items: Vec<StackItem>,
    protected: usize
}

impl Stack {
    pub fn new() -> Self {
        Self { items: Vec::new(), protected: 0 }
    }

    pub fn protect(&mut self, count: usize) -> Result<()> {
        if self.items.len() < count {
            return Err(Error::BadStack { message: format!("Attempt to protect more items than on stack"), position: count })
        }
        self.protected += count;
        Ok(())
    }

    pub fn restore(&mut self, count: usize) -> Result<()> {
        if self.protected < count {
            return Err(Error::BadStack { message: format!("Attempt to restore more items than protected"), position: count })
        }
        self.protected -= count;
        Ok(())
    }

    pub fn push(&mut self, item: StackItem) {
        self.items.insert(self.protected, item);
    }
}
