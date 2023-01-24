use tezos_michelson::micheline::Micheline;
use tezos_rpc::models::operation::operation_result::{
    lazy_storage_diff::{
        big_map::{BigMap, Diff, Update},
        Kind, LazyStorageDiff,
    },
    DiffAction,
};
use tezos_vm::types::BigMapDiff;

use crate::Result;

const DEFAULT_DIFF: Diff = Diff {
    action: DiffAction::Update,
    key_type: None,
    value_type: None,
    updates: vec![],
    key_hash: None,
    key: None,
    value: None,
    source: None,
};

#[derive(Clone, Debug)]
pub struct LazyDiff {
    lazy_diff: Vec<LazyStorageDiff>,
}

impl LazyDiff {
    pub fn new() -> Self {
        Self {
            lazy_diff: Vec::new(),
        }
    }

    pub fn update(&mut self, big_map_diff: Vec<BigMapDiff>) -> Result<()> {
        for diff in big_map_diff {
            let diff = Self::make_diff(diff)?;
            self.lazy_diff.push(diff);
        }
        Ok(())
    }

    pub fn make_update(update: (String, (Micheline, Option<Micheline>))) -> Result<Update> {
        Ok(Update {
            key_hash: update.0.try_into()?,
            key: update.1 .0,
            value: update.1 .1,
        })
    }

    pub fn make_diff(diff: BigMapDiff) -> Result<LazyStorageDiff> {
        let updates: Result<Vec<Update>> = diff
            .updates
            .into_iter()
            .map(LazyDiff::make_update)
            .collect();

        Ok(LazyStorageDiff::BigMap(BigMap {
            kind: Kind::BigMap,
            id: diff.id.to_string(),
            diff: match diff.alloc {
                true => Diff {
                    action: DiffAction::Alloc,
                    key_type: Some(diff.inner_type.0.into()),
                    value_type: Some(diff.inner_type.1.into()),
                    updates: updates?,
                    ..DEFAULT_DIFF
                },
                false => Diff {
                    action: DiffAction::Update,
                    updates: updates?,
                    ..DEFAULT_DIFF
                },
            },
        }))
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
