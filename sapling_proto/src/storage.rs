use std::collections::HashMap;

use anyhow::Result;
use zcash_primitives::sapling::Node;

use crate::types::{Ciphertext, Hash, Nullifier};

pub const MAX_ROOTS: usize = 120;
pub const MAX_HEIGHT: usize = 32;

#[derive(Clone, Debug)]
pub struct SaplingHead {
    pub roots_pos: usize,
    pub nullifiers_size: usize,
    pub commitments_size: usize,
    pub memo_size: usize,
}

impl SaplingHead {
    pub fn new(memo_size: usize) -> Self {
        Self {
            roots_pos: 0,
            nullifiers_size: 0,
            commitments_size: 0,
            memo_size,
        }
    }
}

pub trait SaplingStorage {
    fn get_head(&self) -> Result<SaplingHead>;
    fn set_head(&mut self, head: SaplingHead) -> Result<()>;

    // Dual representation: as a ring queue and as a hashset
    fn add_root(&mut self, root: Hash, position: usize) -> Result<()>;
    fn get_root(&self, position: usize) -> Result<Hash>;
    fn find_root(&self, root: &Hash) -> Result<bool>;

    // Dual representation: as an array and as a hashset
    fn add_nullifier(&mut self, nullifier: Nullifier, position: usize) -> Result<()>;
    fn get_nullifier(&self, position: usize) -> Result<Nullifier>;
    fn find_nullifier(&self, nullifier: &Nullifier) -> Result<bool>;

    // Flattened incremental Merkle tree
    // The height of the leaves level is 0, height of the root is [MAX_HEIGHT]
    // Path is the sequential number of a node in the tree: root = 1, left = 2, right = 3, etc
    // [Node] is a more generic type for commitment tree structure, can be constructed out of a [Commitment]
    fn add_commitment(&mut self, commitment: Node, path: usize) -> Result<()>;
    fn get_commitment(&self, path: usize) -> Result<Node>;

    // Linked to leaves-level commitments
    // Position is relative number of the leaf, starts from 0, actually it's [Path] - 2 ^ [MAX_HEIGHT]
    fn add_ciphertext(&mut self, ciphertext: Ciphertext, position: usize) -> Result<()>;
    fn get_ciphertext(&self, position: usize) -> Result<Ciphertext>;
}

pub struct NaiveStorage {
    pub head: SaplingHead,
    pub roots: [Hash; MAX_ROOTS],
    pub nullifiers: Vec<Nullifier>,
    pub commitments: HashMap<usize, Node>,
    pub ciphertexts: Vec<Ciphertext>,
}

impl NaiveStorage {
    pub fn new(memo_size: usize) -> Self {
        Self {
            head: SaplingHead::new(memo_size),
            roots: [Hash::default(); MAX_ROOTS],
            nullifiers: Vec::new(),
            commitments: HashMap::new(),
            ciphertexts: Vec::new(),
        }
    }
}

impl SaplingStorage for NaiveStorage {
    fn get_head(&self) -> Result<SaplingHead> {
        Ok(self.head.clone())
    }

    fn set_head(&mut self, head: SaplingHead) -> Result<()> {
        self.head = head;
        Ok(())
    }

    fn add_root(&mut self, root: Hash, position: usize) -> Result<()> {
        assert!(position < MAX_ROOTS);
        self.roots[position] = root;
        Ok(())
    }

    fn get_root(&self, position: usize) -> Result<Hash> {
        assert!(position < MAX_ROOTS);
        Ok(self.roots[position].clone())
    }

    fn find_root(&self, root: &Hash) -> Result<bool> {
        Ok(self.roots.iter().find(|r| *r == root).is_some())
    }

    fn add_nullifier(&mut self, nullifier: Nullifier, position: usize) -> Result<()> {
        assert_eq!(position, self.nullifiers.len());
        self.nullifiers.push(nullifier);
        Ok(())
    }

    fn get_nullifier(&self, position: usize) -> Result<Nullifier> {
        assert!(position < self.nullifiers.len());
        Ok(self.nullifiers.get(position).unwrap().clone())
    }

    fn find_nullifier(&self, nullifier: &Nullifier) -> Result<bool> {
        Ok(self.nullifiers.iter().find(|nf| *nf == nullifier).is_some())
    }

    fn add_commitment(&mut self, commitment: Node, path: usize) -> Result<()> {
        self.commitments.insert(path, commitment);
        Ok(())
    }

    fn get_commitment(&self, path: usize) -> Result<Node> {
        Ok(self.commitments.get(&path).unwrap().clone())
    }

    fn add_ciphertext(&mut self, ciphertext: Ciphertext, position: usize) -> Result<()> {
        assert_eq!(position, self.ciphertexts.len());
        self.ciphertexts.push(ciphertext);
        Ok(())
    }

    fn get_ciphertext(&self, position: usize) -> Result<Ciphertext> {
        assert!(position < self.ciphertexts.len());
        Ok(self.ciphertexts.get(position).unwrap().clone())
    }
}
