pub mod core;
pub mod numeric;
pub mod domain;
pub mod algebraic;
pub mod collections;

use ibig::{IBig, UBig};
use tezos_core::types::encoded::{
    Address, PublicKey, ImplicitAddress, Signature
};
use tezos_michelson::michelson::{
    data::Instruction,
    types::Type,
};
use tezos_operation::operations::OperationContent;
use derive_more::{From, TryInto};

macro_rules! define_item {
    ($name: ident, $impl: ty) => {
        #[derive(Debug, Clone, PartialEq, From)]
        pub struct $name($impl);
    };
}

macro_rules! define_item_rec {
    ($name: ident, $val: ty, $typ: ty) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            outer_value: $val,
            inner_type: $typ
        }
    };
}

define_item!(UnitItem, ()); // core
define_item!(BoolItem, bool);  // core
define_item!(BytesItem, Vec<u8>);  // core
define_item!(StringItem, String);  // core
define_item!(IntItem, IBig);  // numeric
define_item!(NatItem, UBig);  // numeric
define_item!(MutezItem, i64);  // numeric
define_item!(TimestampItem, i64);  // numeric
define_item!(AddressItem, Address);  // domain
define_item!(KeyItem, PublicKey);  // domain
define_item!(KeyHashItem, ImplicitAddress);  // domain
define_item!(SignatureItem, Signature);  // domain
define_item!(OperationItem, OperationContent);  // operation

// Items where type information might be missing
define_item_rec!(OptionItem, Option<Box<StackItem>>, Type);  // algebraic
define_item_rec!(ListItem, Vec<StackItem>, Type);  // collections
define_item_rec!(SetItem, Vec<StackItem>, Type);  // collections
define_item_rec!(MapItem, Vec<(StackItem, StackItem)>, (Type, Type));  // collections
define_item_rec!(LambdaItem, Vec<Instruction>, (Type, Type));  // domain

#[derive(Debug, Clone)]
pub struct PairItem(Box<(StackItem, StackItem)>);  // algebraic

#[derive(Debug, Clone)]
pub enum OrItem {  // algebraic
    Left { value: Box<StackItem>, right_type: Type },
    Right { value: Box<StackItem>, left_type: Type }
}

#[derive(Debug, Clone)]
pub enum BigMapItem {  // collections
    Ptr { value: i64, outer_type: Type },
    Map(MapItem)
}

#[derive(Debug, Clone, From, TryInto)]
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