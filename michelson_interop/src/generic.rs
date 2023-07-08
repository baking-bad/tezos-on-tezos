use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use tezos_michelson::michelson::{
    types::{Type, self},
    data::{Data, self},
};

use crate::{Error, Result, MichelsonInterop};

#[macro_export]
macro_rules! hashset {
    ($($value:expr),*) => {{
        let mut set = HashSet::new();
        $(
            set.insert($value);
        )*
        set
    }};
}

#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => {{
        let mut map = HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

impl<T: MichelsonInterop> MichelsonInterop for Vec<T> {
    fn michelson_type(field_name: Option<String>) -> Type {
        let inner_ty = T::michelson_type(None);
        let ty = types::List::new(inner_ty, None);
        match field_name {
            Some(name) => ty.with_field_annotation(name),
            None => ty.into()
        }   
    }

    fn to_michelson(&self) -> Result<data::Data> {
        let elements: Result<Vec<Data>> = self
            .into_iter()
            .map(|elt| elt.to_michelson())
            .collect();
        let list: data::Sequence = data::sequence(elements?);
        Ok(list.into())
    }

    fn from_michelson(data: Data) -> Result<Self> {
        match data {
            data::Data::Sequence(seq) => {
                seq
                    .into_values()
                    .into_iter()
                    .map(|elt| T::from_michelson(elt))
                    .collect()
            },
            _ => Err(Error::TypeMismatch { message: format!("Expected sequence, got {:?}", data) })
        }
    }
}

impl<T: MichelsonInterop + Hash + Eq> MichelsonInterop for HashSet<T> {
    fn michelson_type(field_name: Option<String>) -> Type {
        let inner_ty = T::michelson_type(None);
        let ty = types::Set::new(inner_ty, None);
        match field_name {
            Some(name) => ty.with_field_annotation(name),
            None => ty.into()
        }   
    }

    fn to_michelson(&self) -> Result<data::Data> {
        let elements: Result<Vec<Data>> = self
            .into_iter()
            .map(|elt| elt.to_michelson())
            .collect();
        let list: data::Sequence = data::sequence(elements?);
        Ok(list.into())
    }

    fn from_michelson(data: Data) -> Result<Self> {
        match data {
            data::Data::Sequence(seq) => {
                seq
                    .into_values()
                    .into_iter()
                    .map(|elt| T::from_michelson(elt))
                    .collect()
            },
            _ => Err(Error::TypeMismatch { message: format!("Expected sequence, got {:?}", data) })
        }
    }
}

impl<K: MichelsonInterop + Hash + Eq, V: MichelsonInterop> MichelsonInterop for HashMap<K, V> {
    fn michelson_type(field_name: Option<String>) -> Type {
        let key_ty = K::michelson_type(None);
        let val_ty = V::michelson_type(None);
        let ty = types::Map::new(key_ty, val_ty, None);
        match field_name {
            Some(name) => ty.with_field_annotation(name),
            None => ty.into()
        }   
    }

    fn to_michelson(&self) -> Result<data::Data> {
        let mut elements: Vec<Data> = Vec::with_capacity(self.len());
        for (k, v) in self.iter() {
            let key = k.to_michelson()?;
            let value = v.to_michelson()?;
            elements.push(data::elt(key, value))
        }
        let list: data::Sequence = data::sequence(elements);
        Ok(list.into())
    }

    fn from_michelson(data: Data) -> Result<Self> {
        match data {
            data::Data::Sequence(seq) => {
                let mut res: HashMap<K, V> = HashMap::with_capacity(seq.values().len());
                for item in seq.into_values() {
                    match item {
                        Data::Elt(elt) => {
                            let k = K::from_michelson(*elt.key)?;
                            let v = V::from_michelson(*elt.value)?;
                            res.insert(k, v);
                        },
                        _ => return Err(Error::TypeMismatch { message: format!("Expected elt, got {:?}", item) })
                    }
                }
                Ok(res)
            },
            _ => Err(Error::TypeMismatch { message: format!("Expected sequence, got {:?}", data) })
        }
    }
}

michelson_derive::michelson_tuple!(A, B);
michelson_derive::michelson_tuple!(A, B, C);
michelson_derive::michelson_tuple!(A, B, C, D);
michelson_derive::michelson_tuple!(A, B, C, D, E);
michelson_derive::michelson_tuple!(A, B, C, D, E, F);
