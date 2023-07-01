use anyhow::Result;

use crate::{
    storage::{SaplingHead, SaplingStorage},
    tree::CommitmentTree,
};

pub fn run_migrations(storage: &mut impl SaplingStorage, head: &SaplingHead) -> Result<()> {
    storage.check_no_pending_changes()?;

    if head.commitments_size == 0 {
        storage.set_root(CommitmentTree::empty_root(), 0)?;
    }

    storage.commit()
}
