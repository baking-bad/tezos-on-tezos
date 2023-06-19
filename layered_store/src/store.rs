use crate::Result;

pub trait StoreType: Clone + Sized {
    fn to_vec(&self) -> Result<Vec<u8>>;
    fn from_vec(value: Vec<u8>) -> Result<Self>;
}

pub trait LayeredStore<T: StoreType> {
    fn log(&self, msg: String);
    fn has(&self, key: String) -> Result<bool>;
    fn get(&mut self, key: String) -> Result<Option<T>>;
    fn set(&mut self, key: String, val: Option<T>) -> Result<()>;
    fn has_pending_changes(&self) -> bool;
    fn commit(&mut self) -> Result<()>;
    fn rollback(&mut self);
    fn clear(&mut self);
}

#[macro_export]
macro_rules! store_get_opt {
    ($context: expr, $($arg:tt)*) => {
        match $context.get(format!($($arg)*)) {
            Ok(Some(value)) => Ok(Some(value.try_into()?)),
            Ok(None) => Ok(None),
            Err(err) => Err(err.into())
        }
    };
}

#[macro_export]
macro_rules! store_get {
    ($context: expr, $default: expr, $($arg:tt)*) => {
        match $context.get(format!($($arg)*)) {
            Ok(Some(value)) => Ok(value.try_into()?),
            Ok(None) => Ok($default),
            Err(err) => Err(err.into())
        }
    };
}

#[macro_export]
macro_rules! store_unwrap {
    ($context: expr, $($arg:tt)*) => {
        match $context.get(format!($($arg)*)) {
            Ok(Some(value)) => Ok(value.try_into()?),
            Ok(None) => Err($crate::internal_error!($($arg)*).into()),
            Err(err) => Err(err.into())
        }
    };
}
