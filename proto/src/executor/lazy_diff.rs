use tezos_rpc::models::operation::operation_result::lazy_storage_diff::LazyStorageDiff;

use vm::types::BigMapDiff;

#[derive(Clone, Debug)]
pub struct LazyDiff {
    lazy_diff: Vec<LazyStorageDiff>,
}

impl LazyDiff {
    pub fn new() -> Self {
        Self { lazy_diff: Vec::new() }
    }

    pub fn update(&mut self, big_map_diff: Vec<BigMapDiff>) {
        todo!()
    }
}

impl Into<Option<Vec<LazyStorageDiff>>> for LazyDiff {
    fn into(self) -> Option<Vec<LazyStorageDiff>> {
        if !self.lazy_diff.is_empty() {
            return Some(self.lazy_diff);
        }
        None
    }
}