pub mod head;
pub mod sapling;
pub mod store;

pub const MAX_ROOTS: usize = 120;
pub const MAX_HEIGHT: usize = 32;

pub use head::SaplingHead;
pub use sapling::SaplingStorage;
pub use store::SaplingStoreType;

pub type SaplingEphemeralStorage = layered_store::EphemeralStore<SaplingStoreType>;
