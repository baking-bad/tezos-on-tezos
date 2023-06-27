// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use serde_json_wasm;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use tezos_michelson::micheline::{
    literals::Literal, primitive_application::PrimitiveApplication, Micheline,
};

use michelson_vm::Result;

pub fn read_from_file(category: &str, filename: &str) -> Result<Micheline> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/data");
    path.push(category);
    path.push(filename);

    let mut file = File::open(path).expect("Failed to open Micheline file");
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("Failed to read Micheline file");

    let res: Micheline = serde_json_wasm::from_slice(buffer.as_slice())?;
    Ok(res)
}

pub fn parse_literal(outer: PrimitiveApplication) -> String {
    let mut args = outer.into_args().expect("Expected single arg");
    match args.remove(0) {
        Micheline::Literal(Literal::Int(int)) => int.to_string(),
        Micheline::Literal(Literal::String(string)) => string.into_string(),
        Micheline::Literal(Literal::Bytes(bytes)) => bytes.value()[2..].to_string(),
        _ => panic!("Expected literal"),
    }
}
