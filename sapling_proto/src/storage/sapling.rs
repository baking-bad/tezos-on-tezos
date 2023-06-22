use anyhow::{anyhow, Result};
use hex;
use layered_store::LayeredStore;

use crate::{
    storage::{SaplingHead, SaplingStoreType},
    types::{Ciphertext, CommitmentNode, Hash, Nullifier},
};

pub trait SaplingStorage {
    fn set_head(&mut self, head: SaplingHead) -> Result<()>;
    fn get_head(&mut self) -> Result<SaplingHead>;

    // Dual representation: as a ring queue and as a hashset
    fn set_root(&mut self, root: Hash, position: usize) -> Result<()>;
    fn has_root(&self, root: &Hash) -> Result<bool>;
    fn get_root(&mut self, position: usize) -> Result<Option<Hash>>;

    // Dual representation: as an array and as a hashset
    fn set_nullifier(&mut self, nullifier: Nullifier, position: usize) -> Result<()>;
    fn has_nullifier(&self, nullifier: &Nullifier) -> Result<bool>;
    fn get_nullifier(&mut self, position: usize) -> Result<Option<Nullifier>>;

    // Flattened incremental Merkle tree
    // The height of the leaves level is 0, height of the root is [MAX_HEIGHT]
    // Path is the sequential number of a node in the tree: root = 1, left = 2, right = 3, etc
    // [CommitmentNode] is a more generic type for commitment tree structure, can be constructed out of a [Commitment]
    fn set_commitment(&mut self, commitment: CommitmentNode, path: usize) -> Result<()>;
    fn get_commitment(&mut self, path: usize) -> Result<Option<CommitmentNode>>;

    // Linked to leaves-level commitments
    // Position is relative number of the leaf, starts from 0, actually it's [Path] - 2 ^ [MAX_HEIGHT]
    fn set_ciphertext(&mut self, ciphertext: Ciphertext, position: usize) -> Result<()>;
    fn get_ciphertext(&mut self, position: usize) -> Result<Option<Ciphertext>>;
}

impl<T: LayeredStore<SaplingStoreType>> SaplingStorage for T {
    fn get_head(&mut self) -> Result<SaplingHead> {
        let value = self
            .get("/head".into())?
            .unwrap_or_else(|| SaplingHead::default().into());
        value.try_into().map_err(|_| anyhow!("Unexpected variant"))
    }

    fn set_head(&mut self, head: SaplingHead) -> Result<()> {
        self.set("/head".into(), Some(head.into()))?;
        Ok(())
    }

    fn set_root(&mut self, root: Hash, position: usize) -> Result<()> {
        if let Some(expired_root) = self.get_root(position)? {
            self.set(format!("/roots/index/{}", hex::encode(expired_root)), None)?;
        }
        let value: SaplingStoreType = root.into();
        self.set(format!("/roots/list/{}", position), Some(value.clone()))?;
        self.set(format!("/roots/index/{}", hex::encode(root)), Some(value))?;
        Ok(())
    }

    fn has_root(&self, root: &Hash) -> Result<bool> {
        let res = self.has(format!("/roots/index/{}", hex::encode(root)))?;
        Ok(res)
    }

    fn get_root(&mut self, position: usize) -> Result<Option<Hash>> {
        match self.get(format!("/roots/list/{}", position))? {
            Some(val) => val
                .try_into()
                .map_err(|_| anyhow!("Unexpected variant"))
                .map(|v| Some(v)),
            None => Ok(None),
        }
    }

    fn set_nullifier(&mut self, nullifier: Nullifier, position: usize) -> Result<()> {
        let value: SaplingStoreType = nullifier.into();
        self.set(
            format!("/nullifiers/list/{}", position),
            Some(value.clone()),
        )?;
        self.set(
            format!("/nullifiers/index/{}", hex::encode(nullifier.0)),
            Some(value),
        )?;
        Ok(())
    }

    fn has_nullifier(&self, nullifier: &Nullifier) -> Result<bool> {
        Ok(self.has(format!("/nullifiers/index/{}", hex::encode(nullifier.0)))?)
    }

    fn get_nullifier(&mut self, position: usize) -> Result<Option<Nullifier>> {
        match self.get(format!("/nullifiers/list/{}", position))? {
            Some(val) => val
                .try_into()
                .map_err(|_| anyhow!("Unexpected variant"))
                .map(|v| Some(v)),
            None => Ok(None),
        }
    }

    fn set_commitment(&mut self, commitment: CommitmentNode, path: usize) -> Result<()> {
        self.set(format!("/commitments/{}", path), Some(commitment.into()))?;
        Ok(())
    }

    fn get_commitment(&mut self, path: usize) -> Result<Option<CommitmentNode>> {
        match self.get(format!("/commitments/{}", path))? {
            Some(val) => val
                .try_into()
                .map_err(|_| anyhow!("Unexpected variant"))
                .map(|v| Some(v)),
            None => Ok(None),
        }
    }

    fn set_ciphertext(&mut self, ciphertext: Ciphertext, position: usize) -> Result<()> {
        self.set(
            format!("/ciphertexts/{}", position),
            Some(ciphertext.into()),
        )?;
        Ok(())
    }

    fn get_ciphertext(&mut self, position: usize) -> Result<Option<Ciphertext>> {
        match self.get(format!("/ciphertexts/{}", position))? {
            Some(val) => val
                .try_into()
                .map_err(|_| anyhow!("Unexpected variant"))
                .map(|v| Some(v)),
            None => Ok(None),
        }
    }
}
