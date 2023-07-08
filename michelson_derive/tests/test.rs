use std::collections::{HashSet, HashMap};
use michelson_interop::{MichelsonInterop, hashset, hashmap};
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
        list: vec![InnerType { data: vec![], none: () }],
        set: hashset![InnerType { data: vec![], none: () }],
        map: hashmap!{ "hello".into() => true, "world".into() => false },
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

    let src = TupleType(
        "Hello".into(),
        false,
        (vec![42u8], ())
    );

    let res = src.to_michelson().expect("Failed to serialize");
    println!("Serialized: {}", res.format());

    let dst = TupleType::from_michelson(res).expect("Failed to deserialize");
    println!("Deserialized: {:#?}", dst);

    assert_eq!(src, dst);
}
