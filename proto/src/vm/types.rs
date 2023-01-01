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
    data::{Instruction},
    types::Type,
};
use tezos_operation::operations::OperationContent;
use derive_more::{From, TryInto};

#[macro_export]
macro_rules! type_check_fn_comparable {
    ($cmp_ty: ident) => {
        pub fn type_check(&self, ty: &Type) -> Result<()> {
            match ty {
                Type::Comparable(ComparableType::$cmp_ty(_)) => Ok(()),
                _ => err_type!(ty, self)
            }
        }
    };
}

#[macro_export]
macro_rules! partial_traits_unreachable {
    ($item: ty) => {
        impl PartialEq for $item {
            fn eq(&self, _: &Self) -> bool {
                unreachable!("Not a comparable type")
            }
        }

        impl PartialOrd for $item {
            fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
                unreachable!("Not a comparable type")
            }
        }
    };
}

macro_rules! define_item {
    ($name: ident, $impl: ty) => {
        #[derive(Debug, Clone, PartialEq, From)]
        pub struct $name($impl);
    };
}

macro_rules! define_item_ord {
    ($name: ident, $impl: ty) => {
        #[derive(Debug, Clone, PartialEq, PartialOrd, From)]
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

define_item_rec!(OptionItem, Option<Box<StackItem>>, Type);  // algebraic
define_item_rec!(ListItem, Vec<StackItem>, Type);  // collections
define_item_rec!(SetItem, Vec<StackItem>, Type);  // collections
define_item_rec!(MapItem, Vec<(StackItem, StackItem)>, (Type, Type));  // collections
define_item_rec!(LambdaItem, Instruction, (Type, Type));  // domain

partial_traits_unreachable!(ListItem);
partial_traits_unreachable!(SetItem);
partial_traits_unreachable!(MapItem);
partial_traits_unreachable!(BigMapItem);
partial_traits_unreachable!(LambdaItem);
partial_traits_unreachable!(OperationItem);

#[derive(Debug, Clone)]
pub struct OperationItem(OperationContent); // domain

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct PairItem(Box<(StackItem, StackItem)>);  // algebraic

#[derive(Debug, Clone)]
pub struct OrVariant {
    value: Box<StackItem>,
    other_type: Type
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum OrItem {  // algebraic
    Left(OrVariant),
    Right(OrVariant)
}

#[derive(Debug, Clone)]
pub struct BigMapPtr {
    value: i64,
    outer_type: Type
}

#[derive(Debug, Clone)]
pub enum BigMapItem {  // collections
    Ptr(BigMapPtr),
    Map(MapItem)
}

#[derive(Debug, Clone, From, TryInto, PartialEq, PartialOrd)]
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