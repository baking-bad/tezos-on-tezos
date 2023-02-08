pub mod batcher;
pub mod error;
pub mod executor;
pub mod validator;

pub use error::{Error, Result};

pub mod constants {
    pub const PROTOCOL: &str = "ProtoALphaALphaALphaALphaALphaALphaALphaALphaDdp3zK";
    pub const BLOCK_TIME: i64 = 8;
}
