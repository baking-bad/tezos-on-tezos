use tezos_rpc::models::block::{
    FullHeader as BlockHeader,
    Metadata as BlockMetadata
};
use host::runtime::Runtime;

use crate::error::Result;
use crate::context::EphemeralContext;

pub fn wrap_block(host: &mut impl Runtime, context: &mut EphemeralContext) -> Result<()> {
    Ok(())
}
