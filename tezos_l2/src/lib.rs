pub mod batcher;
pub mod error;
pub mod executor;
pub mod validator;

pub use error::{Error, Result};

pub mod constants {
    pub const CHAIN_ID: &str = "NetXP2FfcNxFANL";
    pub const PROTOCOL: &str = "ProtoALphaALphaALphaALphaALphaALphaALphaALphaDdp3zK";
    pub const CYCLE_BLOCKS: i32 = 8192;
    pub const MAX_OPERATIONS_TTL: i32 = 120;
    pub const MAX_OPERATION_DATA_LENGTH: i32 = 32768;
    pub const MAX_BLOCK_HEADER_LENGTH: i32 = 289;
    pub const MAX_OPERATION_LIST_LENGTH: i32 = 100;
    pub const MAX_TOTAL_OPERATION_DATA_LENGTH: i32 = 524288;
    pub const ZERO_SIGNATURE: &str =
        "sigMzJ4GVAvXEd2RjsKGfG2H9QvqTSKCZsuB2KiHbZRGFz72XgF6KaKADznh674fQgBatxw3xdHqTtMHUZAGRprxy64wg1aq";
    pub const ZERO_BLOCK_HASH: &str = "BKiHLREqU3JkXfzEDYAkmmfX48gBDtYhMrpA98s7Aq4SzbUAB6M";
    pub const DEFAULT_ORIGINATED_ADDRESS: &str = "KT1BEqzn5Wx8uJrZNvuS9DVHmLvG9td3fDLi";
    pub const DEFAULT_IMPLICIT_ADDRESS: &str = "tz1Ke2h7sDdakHJQh8WX4Z372du1KChsksyU";
    pub const POW_NONCE: &str = "deadbeef";
    pub const BLOCK_TIME: i64 = 8;
}
