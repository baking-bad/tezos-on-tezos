pub mod batch;
pub mod head;
pub mod interpreter;
pub mod migrations;
pub mod tezos;
pub mod types;

use core::ops::{Deref, DerefMut};
use layered_store::{EphemeralStore, LayeredStore};

pub use tezos::TezosContext;
pub use types::TezosStoreType;

pub struct CtxRef<T>(pub T);

pub type TezosEphemeralContext = CtxRef<EphemeralStore<TezosStoreType>>;

impl TezosEphemeralContext {
    pub fn new() -> Self {
        Self(EphemeralStore::<TezosStoreType>::new())
    }

    pub fn spawn(&self) -> Self {
        Self(self.0.spawn())
    }
}

impl<T: LayeredStore<TezosStoreType>> Deref for CtxRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: LayeredStore<TezosStoreType>> DerefMut for CtxRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}