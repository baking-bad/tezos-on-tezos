pub mod core;
pub mod mutez;
pub mod timestamp;
pub mod nat;
pub mod int;
pub mod encoded;
pub mod pair;
pub mod list;
pub mod set;
pub mod map;
pub mod big_map;
pub mod option;
pub mod or;
pub mod lambda;
pub mod operation;
pub mod contract;

use std::collections::BTreeMap;
use ibig::{IBig, UBig};
use tezos_core::types::{
    encoded::{Address, PublicKey, ImplicitAddress, Signature, ChainId},
    mutez::Mutez
};
use tezos_michelson::michelson::{
    data::Instruction,
    types::Type,
};
use tezos_michelson::micheline::Micheline;
use derive_more::{From, TryInto, Display};

#[macro_export]
macro_rules! not_comparable {
    ($item: ty) => {
        impl PartialOrd for $item {
            fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
                unreachable!("Not a comparable type")
            }
        }

        impl Ord for $item {
            fn cmp(&self, _: &Self) -> std::cmp::Ordering {
                unreachable!("Not a comparable type")
            }
        }

        impl Eq for $item {}
    };
}

macro_rules! define_item {
    ($name: ident, $impl: ty) => {
        #[derive(Debug, Clone, PartialEq, Eq, From)]
        pub struct $name($impl);
    };
}

macro_rules! define_item_ord {
    ($name: ident, $impl: ty) => {
        #[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, From)]
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

define_item_ord!(UnitItem, ()); // algebraic
define_item_ord!(BoolItem, bool);  // core
define_item_ord!(BytesItem, Vec<u8>);  // core
define_item_ord!(StringItem, String);  // core
define_item_ord!(IntItem, IBig);  // numeric
define_item_ord!(NatItem, UBig);  // numeric
define_item_ord!(MutezItem, i64);  // numeric
define_item_ord!(TimestampItem, i64);  // numeric

define_item!(AddressItem, Address);  // domain
define_item!(KeyItem, PublicKey);  // domain
define_item!(KeyHashItem, ImplicitAddress);  // domain
define_item!(SignatureItem, Signature);  // domain
define_item!(ChainIdItem, ChainId);  // domain

define_item_rec!(ListItem, Vec<StackItem>, Type);  // collections
define_item_rec!(SetItem, Vec<StackItem>, Type);  // collections
define_item_rec!(MapItem, Vec<(StackItem, StackItem)>, (Type, Type));  // collections
define_item_rec!(LambdaItem, Instruction, (Type, Type));  // domain
define_item_rec!(ContractItem, Address, Type); // domain

not_comparable!(ListItem);
not_comparable!(SetItem);
not_comparable!(MapItem);
not_comparable!(LambdaItem);
not_comparable!(ContractItem);
not_comparable!(BigMapItem);
not_comparable!(OperationItem);

#[derive(Debug, Clone, PartialEq)]
pub enum InternalContent {
    Transaction {
        destination: Address,
        parameter: Micheline,
        amount: Mutez,
    }
}

#[derive(Debug, Clone)]
pub struct OperationItem { // domain
    content: InternalContent,
    big_map_diff: Vec<BigMapDiff>
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct PairItem(Box<(StackItem, StackItem)>);  // algebraic

#[derive(Debug, Clone)]
pub enum OptionItem {  // algebraic
    None(Type),
    Some(Box<StackItem>)
}

#[derive(Debug, Clone)]
pub struct OrVariant {
    value: Box<StackItem>,
    other_type: Type
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum OrItem {  // algebraic
    Left(OrVariant),
    Right(OrVariant)
}

#[derive(Debug, Clone)]
pub struct BigMapDiff {
    pub id: i64,
    pub inner_type: (Type, Type),
    pub updates: BTreeMap<String, (Micheline, Option<Micheline>)>,
    pub alloc: bool
}

#[derive(Debug, Clone, PartialEq)]
pub enum BigMapItem {  // collections
    Diff(BigMapDiff),
    Map(MapItem),
    Ptr(i64)
}

#[derive(Debug, Display, Clone, From, TryInto, PartialEq, PartialOrd, Eq, Ord)]
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
    ChainId(ChainIdItem),
    Contract(ContractItem),
    Operation(OperationItem),
    Option(OptionItem),
    Or(OrItem),
    Pair(PairItem),
    List(ListItem),
    Set(SetItem),
    Map(MapItem),
    BigMap(BigMapItem),
    Lambda(LambdaItem),
}