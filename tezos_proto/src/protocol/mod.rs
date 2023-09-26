pub mod constants;
pub mod migrations;

use crate::protocol::{
    constants::{Constants, ConstantsAlpha},
    migrations::{Migrations, SandboxSeed}
};

pub trait Protocol {
    type Constants: Constants;
    type Migrations: Migrations;
    // type Fees: Fees;
    // type Bridge: Bridge;
    // type Contracts: Contracts;
}

pub struct ProtocolAlpha {}

impl Protocol for ProtocolAlpha {
    type Constants = ConstantsAlpha;
    type Migrations = SandboxSeed;
}
