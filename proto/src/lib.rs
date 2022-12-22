pub mod error;
pub mod context;
pub mod validator;
pub mod executor;
pub mod producer;

pub mod constants {
    pub const CHAIN_ID: &str = "NetXP2FfcNxFANL";
    pub const PROTOCOL: &str = "ProtoALphaALphaALphaALphaALphaALphaALphaALphaDdp3zK";
    pub const CYCLE_BLOCKS: i32 = 8192;
    pub const MAX_OPERATIONS_TTL: i32 = 120;
    pub const MAX_OPERATION_DATA_LENGTH: i32 = 32768;
    pub const MAX_BLOCK_HEADER_LENGTH: i32 = 289;
    pub const MAX_OPERATION_LIST_LENGTH: i32 = 100;
    pub const MAX_TOTAL_OPERATION_DATA_LENGTH: i32 = 524288;
    pub const ZERO_SIGNATURE: &str = "sigMzJ4GVAvXEd2RjsKGfG2H9QvqTSKCZsuB2KiHbZRGFz72XgF6KaKADznh674fQgBatxw3xdHqTtMHUZAGRprxy64wg1aq";
    pub const ZERO_BLOCK_HASH: &str = "BKiHLREqU3JkXfzEDYAkmmfX48gBDtYhMrpA98s7Aq4SzbUAB6M";
    pub const POW_NONCE: &str = "0xdeadbeef";
    pub const BLOCK_TIME: i64 = 15;
    pub const ALLOCATION_FEE: u32 = 1000u32;
}