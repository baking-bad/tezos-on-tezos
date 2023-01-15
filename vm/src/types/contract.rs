use std::fmt::Display;
use tezos_core::types::encoded::{Address, Encoded};
use tezos_michelson::michelson::{
    types::Type,
    types,
    data::Data,
    data,
};

use crate::{
    Result,
    types::{ContractItem, StackItem},
    typechecker::check_types_equal,
    formatter::Formatter,
    err_mismatch,
    type_cast,
};

impl ContractItem {
    pub fn new(address: Address, inner_type: Type) -> Self {
        Self { outer_value: address, inner_type: inner_type }
    }

    pub fn from_data(data: Data, inner_type: &Type) -> Result<StackItem> {
        let address = match data {
            Data::String(str) => {
                Address::new(str.into_string())?
            },
            Data::Bytes(val) => {
                let bytes: Vec<u8> = (&val).into();
                Address::from_bytes(bytes.as_slice())?
            },
            _ => return err_mismatch!("String or Bytes", data.format())
        };
        Ok(StackItem::Contract(Self::new(address, inner_type.clone())))
    }

    pub fn into_data(self, ty: &Type) -> Result<Data> {
        let ty = type_cast!(ty, Contract);
        check_types_equal(&ty.r#type, &self.inner_type)?;
        Ok(Data::String(data::String::from_string(self.outer_value.into_string())?))
    }

    pub fn get_type(&self) -> Type {
        types::contract(self.inner_type.clone())
    }

    pub fn into_components(self) -> (Address, Type) {
        (self.outer_value, self.inner_type)
    }
}

impl PartialEq for ContractItem {
    fn eq(&self, other: &Self) -> bool {
        // for testing purposes
        self.outer_value == other.outer_value
    }
}

impl Display for ContractItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.outer_value.value())
    }
}
