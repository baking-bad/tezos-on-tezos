// use tezos_rpc::models::constants::Constants;

pub struct Config {
    pub blocks_per_cycle: i32,
    pub max_operations_time_to_live: i32,
    pub max_block_header_length: i32,
    pub max_operation_data_length: i32,
    pub max_operations_list_length: i32,
    pub pow_nonce: &'static str,
}

impl Config {
    pub fn default() -> Self {
        Self {
            blocks_per_cycle: 8096,
            max_operations_time_to_live: 240,
            max_block_header_length: 2048,
            max_operation_data_length: 86400,
            max_operations_list_length: 1024,
            pow_nonce: "deadbeef"
        }
    }
}