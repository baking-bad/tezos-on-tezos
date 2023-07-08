pub mod core;
pub mod error;
pub mod generic;
pub mod domain;
pub mod adt;

use tezos_michelson::michelson::{data::Data, types::Type};

pub use michelson_derive::MichelsonInterop;

pub use crate::{
    error::{Error, Result},
    core::Bytes,
    domain::Ticket
};

pub trait MichelsonInterop: Sized {
    fn michelson_type(field_name: Option<String>) -> Type;
    fn to_michelson(&self) -> Result<Data>;
    fn from_michelson(data: Data) -> Result<Self>;
}
