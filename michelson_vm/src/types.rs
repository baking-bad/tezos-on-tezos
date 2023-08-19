// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

pub mod big_map;
pub mod contract;
pub mod core;
pub mod encoded;
pub mod int;
pub mod lambda;
pub mod list;
pub mod map;
pub mod mutez;
pub mod nat;
pub mod operation;
pub mod option;
pub mod or;
pub mod pair;
pub mod set;
pub mod ticket;
pub mod timestamp;

use derive_more::{Display, From, TryInto};
use ibig::{IBig, UBig};
use std::collections::BTreeMap;
use tezos_core::types::{
    encoded::{Address, ChainId, ImplicitAddress, PublicKey, Signature},
    mutez::Mutez,
};
use tezos_michelson::micheline::Micheline;
use tezos_michelson::michelson::{data::Instruction, types::Type};

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
            inner_type: $typ,
        }
    };
}

define_item_ord!(UnitItem, ()); // algebraic
define_item_ord!(BoolItem, bool); // core
define_item_ord!(BytesItem, Vec<u8>); // core
define_item_ord!(StringItem, String); // core
define_item_ord!(IntItem, IBig); // numeric
define_item_ord!(NatItem, UBig); // numeric
define_item_ord!(MutezItem, i64); // numeric
define_item_ord!(TimestampItem, i64); // numeric

define_item!(AddressItem, Address); // domain
define_item!(KeyItem, PublicKey); // domain
define_item!(KeyHashItem, ImplicitAddress); // domain
define_item!(SignatureItem, Signature); // domain
define_item!(ChainIdItem, ChainId); // domain

define_item_rec!(ListItem, Vec<StackItem>, Type); // collections
define_item_rec!(SetItem, Vec<StackItem>, Type); // collections
define_item_rec!(MapItem, Vec<(StackItem, StackItem)>, (Type, Type)); // collections
define_item_rec!(LambdaItem, Instruction, (Type, Type)); // domain
define_item_rec!(ContractItem, Address, Type); // domain

not_comparable!(ListItem);
not_comparable!(SetItem);
not_comparable!(MapItem);
not_comparable!(LambdaItem);
not_comparable!(ContractItem);
not_comparable!(BigMapItem);
not_comparable!(OperationItem);
not_comparable!(TicketItem);

#[derive(Debug, Clone, PartialEq)]
pub enum InternalContent {
    Transaction {
        destination: Address,
        parameter: Micheline,
        amount: Mutez,
        source: ImplicitAddress,
    },
}

#[derive(Debug, Clone)]
pub struct OperationItem {
    // domain
    //content: InternalContent,
    destination: Address,
    param: Box<StackItem>,
    param_type: Type,
    amount: MutezItem,
    source: ImplicitAddress,
    big_map_diff: Vec<BigMapDiff>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct PairItem(Box<(StackItem, StackItem)>); // algebraic

#[derive(Debug, Clone)]
pub enum OptionItem {
    // algebraic
    None(Type),
    Some(Box<StackItem>),
}

#[derive(Debug, Clone)]
pub struct OrVariant {
    value: Box<StackItem>,
    other_type: Type,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum OrItem {
    // algebraic
    Left(OrVariant),
    Right(OrVariant),
}

#[derive(Debug, Clone)]
pub struct BigMapDiff {
    pub id: i64,
    pub inner_type: (Type, Type),
    pub updates: BTreeMap<String, (Micheline, Option<Micheline>)>,
    pub alloc: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BigMapItem {
    // collections
    Diff(BigMapDiff),
    Map(MapItem),
    Ptr(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TicketItem {
    pub source: AddressItem,
    pub identifier: Box<StackItem>,
    pub amount: NatItem,
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
    Ticket(TicketItem),
}

impl AsMut<StackItem> for StackItem {
    fn as_mut(&mut self) -> &mut StackItem {
        self
    }
}
