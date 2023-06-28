// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use anyhow::Result;
use hex;
use layered_store::{LayeredStore, StoreBackend};
use zcash_primitives::merkle_tree::HashSer;

use crate::{
    storage::{Ciphertext, SaplingHead},
    types::{CommitmentNode, Hash, Nullifier},
};

pub trait SaplingStorage {
    fn set_head(&mut self, head: SaplingHead) -> Result<()>;
    fn get_head(&mut self) -> Result<SaplingHead>;

    // Dual representation: as a ring queue and as a hashset
    fn set_root(&mut self, root: Hash, position: u64) -> Result<()>;
    fn has_root(&self, root: &Hash) -> Result<bool>;
    fn get_root(&mut self, position: u64) -> Result<Option<Hash>>;

    // Dual representation: as an array and as a hashset
    fn set_nullifier(&mut self, nullifier: Nullifier, position: u64) -> Result<()>;
    fn has_nullifier(&self, nullifier: &Nullifier) -> Result<bool>;
    fn get_nullifier(&mut self, position: u64) -> Result<Option<Nullifier>>;

    // Flattened incremental Merkle tree
    // The height of the leaves level is 0, height of the root is [MAX_HEIGHT]
    // Path is the sequential number of a node in the tree: root = 1, left = 2, right = 3, etc
    // [CommitmentNode] is a more generic type for commitment tree structure, can be constructed out of a [Commitment]
    fn set_commitment(&mut self, commitment: CommitmentNode, path: u64) -> Result<()>;
    fn get_commitment(&mut self, path: u64) -> Result<Option<CommitmentNode>>;

    // Linked to leaves-level commitments
    // Position is relative number of the leaf, starts from 0, actually it's [Path] - 2 ^ [MAX_HEIGHT]
    fn set_ciphertext(&mut self, ciphertext: Ciphertext, position: u64) -> Result<()>;
    fn get_ciphertext(&mut self, position: u64) -> Result<Option<Ciphertext>>;

    fn commit(&mut self) -> Result<()>;
    fn rollback(&mut self);
}

impl<Backend: StoreBackend> SaplingStorage for LayeredStore<Backend> {
    fn get_head(&mut self) -> Result<SaplingHead> {
        Ok(self
            .get("/head".into())?
            .unwrap_or_else(|| SaplingHead::default()))
    }

    fn set_head(&mut self, head: SaplingHead) -> Result<()> {
        Ok(self.set("/head".into(), Some(head))?)
    }

    fn set_root(&mut self, root: Hash, position: u64) -> Result<()> {
        if let Some(expired_root) = self.get_root(position)? {
            self.set::<Hash>(format!("/roots_hashed/{}", hex::encode(expired_root)), None)?;
        }
        self.set(format!("/roots/{}", position), Some(root.clone()))?;
        self.set(format!("/roots_hashed/{}", hex::encode(root)), Some(root))?;
        Ok(())
    }

    fn has_root(&self, root: &Hash) -> Result<bool> {
        Ok(self.has(format!("/roots_hashed/{}", hex::encode(root)))?)
    }

    fn get_root(&mut self, position: u64) -> Result<Option<Hash>> {
        Ok(self.get(format!("/roots/{}", position))?)
    }

    fn set_nullifier(&mut self, nullifier: Nullifier, position: u64) -> Result<()> {
        self.set(
            format!("/nullifiers_ordered/{}", position),
            Some(nullifier.0),
        )?;
        self.set(
            format!("/nullifiers_hashed/{}", hex::encode(nullifier.0)),
            Some(nullifier.0),
        )?;
        Ok(())
    }

    fn has_nullifier(&self, nullifier: &Nullifier) -> Result<bool> {
        Ok(self.has(format!("/nullifiers_hashed/{}", hex::encode(nullifier.0)))?)
    }

    fn get_nullifier(&mut self, position: u64) -> Result<Option<Nullifier>> {
        Ok(self
            .get(format!("/nullifiers_ordered/{}", position))?
            .map(|nf| Nullifier(nf)))
    }

    fn set_commitment(&mut self, commitment: CommitmentNode, path: u64) -> Result<()> {
        let mut cm = [0u8; 32];
        commitment.write(cm.as_mut_slice())?;
        Ok(self.set(format!("/commitments/{}", path), Some(cm))?)
    }

    fn get_commitment(&mut self, path: u64) -> Result<Option<CommitmentNode>> {
        if let Some(cm) = self.get::<[u8; 32]>(format!("/commitments/{}", path))? {
            Ok(Some(CommitmentNode::read(cm.as_slice())?))
        } else {
            Ok(None)
        }
    }

    fn set_ciphertext(&mut self, ciphertext: Ciphertext, position: u64) -> Result<()> {
        Ok(self.set(format!("/ciphertexts/{}", position), Some(ciphertext))?)
    }

    fn get_ciphertext(&mut self, position: u64) -> Result<Option<Ciphertext>> {
        Ok(self.get(format!("/ciphertexts/{}", position))?)
    }

    fn commit(&mut self) -> Result<()> {
        Ok(LayeredStore::commit(self)?)
    }

    fn rollback(&mut self) {
        LayeredStore::rollback(self);
    }
}
