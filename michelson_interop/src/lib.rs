// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

pub mod adt;
pub mod core;
pub mod domain;
pub mod error;
pub mod generic;
pub mod numeric;

use tezos_michelson::michelson::{data::Data, types::Type};

pub use michelson_derive::MichelsonInterop;

pub use crate::{
    core::Bytes,
    domain::Ticket,
    error::{Error, Result},
};

pub trait MichelsonInterop: Sized {
    fn michelson_type(field_name: Option<String>) -> Type;
    fn to_michelson(&self) -> Result<Data>;
    fn from_michelson(data: Data) -> Result<Self>;
}
