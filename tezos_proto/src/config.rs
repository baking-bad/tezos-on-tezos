use crate::protocol::{constants::{Constants, ProtocolAlpha}, migrations::{Migrations, SandboxSeed}};

pub trait Config {
    type Constants: Constants;
    type Migrations: Migrations;
    // type Fees: Fees;
    // type Bridge: Bridge;
    // type Mempool: Mempool;
    // type Producer: Producer;
    // type Contracts: Contracts;
}

pub struct DefaultConfig {}

impl Config for DefaultConfig {
    type Constants = ProtocolAlpha;
    type Migrations = SandboxSeed;
}
