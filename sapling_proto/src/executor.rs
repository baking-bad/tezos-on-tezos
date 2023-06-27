// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use anyhow::Result;

use crate::{
    storage::{SaplingStorage, MAX_HEIGHT, MAX_ROOTS},
    tree::CommitmentTree,
    types::{Commitment, SaplingTransaction},
};

pub fn execute_transaction(
    storage: &mut impl SaplingStorage,
    transaction: &SaplingTransaction,
) -> Result<()> {
    let mut head = storage.get_head()?;

    for input in transaction.inputs.iter() {
        storage.set_nullifier(input.nf.clone(), head.nullifiers_size)?;
        head.nullifiers_size += 1;
    }

    let mut tree = CommitmentTree::new(head.commitments_size, MAX_HEIGHT);
    let mut commitments: Vec<Commitment> = Vec::with_capacity(transaction.outputs.len());

    for output in transaction.outputs.iter() {
        commitments.push(output.cm);
        storage.set_ciphertext(output.ciphertext.clone(), head.commitments_size)?;
        head.commitments_size += 1;
    }

    let root = tree.add_commitments(storage, &commitments)?;

    // Starting from position 1, because at the very beginning there's empty root at position 0
    head.roots_pos = (head.roots_pos + 1) % MAX_ROOTS;
    storage.set_root(root, head.roots_pos)?;

    storage.set_head(head)?;
    storage.commit()
}
