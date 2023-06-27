// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use anyhow::{bail, Result};
use incrementalmerkletree::Hashable;
use zcash_primitives::merkle_tree::HashSer;

use crate::{
    storage::{SaplingStorage, MAX_HEIGHT},
    types::{Commitment, CommitmentNode, Hash},
};

pub struct CommitmentTree {
    pub max_height: usize,
    pub dissect_path: usize,
    pub commitments_size: usize,
}

impl CommitmentTree {
    pub fn new(commitments_size: usize, max_height: usize) -> Self {
        Self {
            dissect_path: if commitments_size > 0 {
                (1 << max_height) + commitments_size
            } else {
                0
            },
            max_height,
            commitments_size,
        }
    }

    pub fn get_root_at(
        &self,
        storage: &mut impl SaplingStorage,
        height: usize,
        path: usize,
    ) -> Result<CommitmentNode> {
        if path <= self.dissect_path >> height {
            let cm = storage
                .get_commitment(path)?
                .unwrap_or_else(|| CommitmentNode::empty_root(height.try_into().unwrap()));
            Ok(cm)
        } else {
            Ok(CommitmentNode::empty_root(height.try_into().unwrap()))
        }
    }

    fn split_at(commitments: &[Commitment], mid: usize) -> (&[Commitment], &[Commitment]) {
        if mid < commitments.len() {
            commitments.split_at(mid)
        } else {
            (commitments, &[])
        }
    }

    fn add_commitments_at(
        &self,
        storage: &mut impl SaplingStorage,
        commitments: &[Commitment],
        position: usize,
        height: usize,
        path: usize,
    ) -> Result<CommitmentNode> {
        if height > self.max_height {
            bail!(
                "Height {} is greater than expected maximum {}",
                height,
                self.max_height
            );
        }

        // If no pending commitments, return commitment for this path (if exists)
        if commitments.is_empty() {
            return self.get_root_at(storage, height, path);
        }

        if height == 0 {
            if commitments.len() != 1 {
                bail!("Unexpected number of commitments {}", commitments.len());
            }

            let node = CommitmentNode::from_cmu(&commitments[0]);
            storage.set_commitment(node.clone(), path)?;

            Ok(node)
        } else {
            // We know that the tree is append only
            // In case of a single commitment it's safe to assume all right-side branches are uncommitted
            // Everything on the left side we need to fetch from the storage
            // If we insert more than one commitment there might be cases when both branches are affected
            let height = height - 1;
            let level_pos = 1 << height;

            // Recall that position is the index of first commitment related to the left-most leaf of the given subtree
            let (hl, hr) = if position < level_pos {
                let (cml, cmr) = Self::split_at(commitments, level_pos - position);
                (
                    self.add_commitments_at(storage, cml, position, height, path << 1)?,
                    self.add_commitments_at(storage, cmr, 0, height, (path << 1) + 1)?,
                )
            } else {
                (
                    self.get_root_at(storage, height, path << 1)?,
                    self.add_commitments_at(
                        storage,
                        commitments,
                        position - level_pos,
                        height,
                        (path << 1) + 1,
                    )?,
                )
            };

            Ok(CommitmentNode::combine(
                height.try_into().unwrap(),
                &hl,
                &hr,
            ))
        }
    }

    pub fn add_commitments(
        &mut self,
        storage: &mut impl SaplingStorage,
        commitments: &Vec<Commitment>,
    ) -> Result<Hash> {
        let res = self.add_commitments_at(
            storage,
            commitments.as_slice(),
            self.commitments_size,
            MAX_HEIGHT,
            1,
        )?;
        self.commitments_size += commitments.len();

        let mut root: Hash = Hash::default();
        res.write(root.as_mut_slice())?;
        Ok(root)
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;
    use hex;
    use incrementalmerkletree::Hashable;

    use crate::{
        storage::{SaplingEphemeralStorage, MAX_HEIGHT},
        tree::CommitmentTree,
        types::{Commitment, CommitmentNode},
    };

    // https://rpc.tzkt.io/ghostnet/chains/main/blocks/head/context/raw/json/sapling/index/6055

    #[test]
    fn test_empty_tree() -> Result<()> {
        let mut storage = SaplingEphemeralStorage::default();
        let tree = CommitmentTree::new(0, MAX_HEIGHT);
        let root = tree.get_root_at(&mut storage, MAX_HEIGHT, 1)?;

        assert_eq!(
            CommitmentNode::empty_root(MAX_HEIGHT.try_into().unwrap()),
            root
        );
        Ok(())
    }

    #[test]
    fn test_single_commitment() -> Result<()> {
        let mut storage = SaplingEphemeralStorage::default();
        let mut tree = CommitmentTree::new(0, MAX_HEIGHT);

        let cm = hex::decode("f1de6f589f17cda6e8811dd2fb5b2b78875d440de07f6964a2f06e4e26f99b25")?;
        let commitment = Commitment::from_bytes(cm.as_slice().try_into().unwrap()).unwrap();

        let root = hex::decode("69a1f12aea9ef4019a059e69e70d6317c35d936d3ea61181f9fa9fa297fe092f")?;
        let root_hash: [u8; 32] = root.as_slice().try_into().unwrap();

        let res = tree.add_commitments(&mut storage, &vec![commitment])?;
        assert_eq!(root_hash, res);
        Ok(())
    }
}
