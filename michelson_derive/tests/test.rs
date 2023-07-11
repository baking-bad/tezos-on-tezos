// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use michelson_interop::{hashmap, hashset, MichelsonInterop, Ticket};
use std::collections::{HashMap, HashSet};
use tezos_core::types::{
    encoded::{Address, ChainId},
    mutez::Mutez,
    number::Nat,
};
use tezos_vm::formatter::Formatter;

#[derive(Debug, PartialEq, Eq, Hash, MichelsonInterop)]
struct InnerType {
    pub data: Vec<u8>,
    pub none: (),
}

#[derive(Debug, PartialEq, MichelsonInterop)]
struct RecordType {
    pub name: String,
    pub flag: bool,
    pub inner: InnerType,
    pub list: Vec<InnerType>,
    pub set: HashSet<InnerType>,
    pub map: HashMap<String, bool>,
}

#[test]
fn test_michelson_record() {
    println!(
        "Michelson type: {}",
        RecordType::michelson_type(None).format()
    );

    let src = RecordType {
        name: "test".into(),
        flag: true,
        inner: InnerType {
            data: vec![0u8, 1u8, 2u8],
            none: (),
        },
        list: vec![InnerType {
            data: vec![],
            none: (),
        }],
        set: hashset![InnerType {
            data: vec![],
            none: ()
        }],
        map: hashmap! { "hello".into() => true, "world".into() => false },
    };

    let res = src.to_michelson().expect("Failed to serialize");
    println!("Serialized: {}", res.format());

    let dst = RecordType::from_michelson(res).expect("Failed to deserialize");
    println!("Deserialized: {:#?}", dst);

    assert_eq!(src, dst);
}

#[derive(Debug, PartialEq, MichelsonInterop)]
struct TupleType(String, bool, (Vec<u8>, ()));

#[test]
fn test_michelson_tuple() {
    println!(
        "Michelson type: {}",
        TupleType::michelson_type(None).format()
    );

    let src = TupleType("Hello".into(), false, (vec![42u8], ()));

    let res = src.to_michelson().expect("Failed to serialize");
    println!("Serialized: {}", res.format());

    let dst = TupleType::from_michelson(res).expect("Failed to deserialize");
    println!("Deserialized: {:#?}", dst);

    assert_eq!(src, dst);
}

#[derive(Debug, PartialEq, MichelsonInterop)]
enum EnumType {
    Default,
    Entry1(Vec<u8>, String),
    Entry2 { flag: bool, list: Vec<bool> },
}

#[test]
fn test_michelson_or() {
    println!(
        "Michelson type: {}",
        EnumType::michelson_type(None).format()
    );

    let src = EnumType::Entry2 {
        flag: true,
        list: vec![false],
    };

    let res = src.to_michelson().expect("Failed to serialize");
    println!("Serialized: {}", res.format());

    let dst = EnumType::from_michelson(res).expect("Failed to deserialize");
    println!("Deserialized: {:#?}", dst);

    assert_eq!(src, dst);
}

#[derive(Debug, PartialEq, MichelsonInterop)]
struct DomainType {
    inline: (ChainId, Address, Mutez),
    ticket: Option<Ticket<()>>,
}

#[test]
fn test_michelson_domain() {
    println!(
        "Michelson type: {}",
        DomainType::michelson_type(None).format()
    );

    let src = DomainType {
        inline: (
            "NetXnHfVqm9iesp".try_into().unwrap(),
            "tz1V16tR1LMKRernkmXzngkfznmEcTGXwDuk".try_into().unwrap(),
            100u32.try_into().unwrap(),
        ),
        ticket: Some((
            "tz1V16tR1LMKRernkmXzngkfznmEcTGXwDuk".try_into().unwrap(),
            (),
            42u64.into(),
        )),
    };

    let res = src.to_michelson().expect("Failed to serialize");
    println!("Serialized: {}", res.format());

    let dst = DomainType::from_michelson(res).expect("Failed to deserialize");
    println!("Deserialized: {:#?}", dst);

    assert_eq!(src, dst);
}

#[derive(Debug, PartialEq, MichelsonInterop)]
enum TokenKind {
    FA1(Nat),
    FA2(Nat),
}

#[derive(Debug, PartialEq, MichelsonInterop)]
struct TokenType {
    token_metadata: HashMap<Nat, (Nat, HashMap<String, Vec<u8>>)>,
    token_kind: TokenKind,
    time: i64,
}

#[test]
fn test_michelson_token() {
    println!(
        "Michelson type: {}",
        TokenType::michelson_type(None).format()
    );

    let src = TokenType {
        token_metadata: hashmap! {
            0u64.into() => (
                0u64.into(), hashmap! {
                    "".into() => vec![0u8]
                }
            )
        },
        token_kind: TokenKind::FA1(2u32.into()),
        time: 12345678,
    };

    let res = src.to_michelson().expect("Failed to serialize");
    println!("Serialized: {}", res.format());

    let dst = TokenType::from_michelson(res).expect("Failed to deserialize");
    println!("Deserialized: {:#?}", dst);

    assert_eq!(src, dst);
}
