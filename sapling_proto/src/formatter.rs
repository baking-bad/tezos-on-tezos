use hex;
use zcash_primitives::merkle_tree::HashSer;

use crate::{
    storage::{Ciphertext, SaplingHead},
    types::{
        Commitment, CommitmentNode, Hash, Input, Nullifier, Output, Proof, PublicKey,
        SaplingTransaction, Signature, ValueCommitment,
    },
};

pub trait Formatter {
    fn sprintf(&self, indent: usize, output: &mut String);

    fn to_string(&self) -> String {
        let mut output = String::with_capacity(256);
        self.sprintf(0, &mut output);
        output
    }
}

impl Formatter for &[u8] {
    fn sprintf(&self, _indent: usize, output: &mut String) {
        output.push_str(&hex::encode(self));
    }
}

impl Formatter for Vec<u8> {
    fn sprintf(&self, indent: usize, output: &mut String) {
        self.as_slice().sprintf(indent, output);
    }
}

impl Formatter for Hash {
    fn sprintf(&self, indent: usize, output: &mut String) {
        self.as_slice().sprintf(indent, output);
    }
}

impl Formatter for Commitment {
    fn sprintf(&self, indent: usize, output: &mut String) {
        self.to_bytes().sprintf(indent, output);
    }
}

impl Formatter for ValueCommitment {
    fn sprintf(&self, indent: usize, output: &mut String) {
        self.to_bytes().sprintf(indent, output);
    }
}

impl Formatter for CommitmentNode {
    fn sprintf(&self, indent: usize, output: &mut String) {
        let mut bytes = [0u8; 32];
        self.write(bytes.as_mut_slice())
            .expect("Failed to serialize commitment node");
        bytes.sprintf(indent, output)
    }
}

impl Formatter for Nullifier {
    fn sprintf(&self, indent: usize, output: &mut String) {
        self.0.sprintf(indent, output);
    }
}

impl Formatter for PublicKey {
    fn sprintf(&self, indent: usize, output: &mut String) {
        let mut bytes = [0u8; 32];
        self.write(bytes.as_mut_slice())
            .expect("Failed to serialize public key");
        bytes.as_slice().sprintf(indent, output)
    }
}

impl Formatter for Signature {
    fn sprintf(&self, indent: usize, output: &mut String) {
        let mut bytes = [0u8; 64];
        self.write(bytes.as_mut_slice())
            .expect("Failed to serialize signature");
        bytes.as_slice().sprintf(indent, output)
    }
}

impl Formatter for Proof {
    fn sprintf(&self, indent: usize, output: &mut String) {
        let mut bytes = [0u8; 192];
        self.write(bytes.as_mut_slice())
            .expect("Failed to serialize groth16 proof");
        bytes.as_slice().sprintf(indent, output)
    }
}

impl Formatter for Ciphertext {
    fn sprintf(&self, indent: usize, output: &mut String) {
        output.push_str("{");
        output.push_str(&format!("\n{}cv: ", " ".repeat(indent + 2)));
        self.cv.sprintf(0, output);
        output.push_str(&format!("\n{}epk: ", " ".repeat(indent + 2)));
        self.epk.sprintf(0, output);
        output.push_str(&format!("\n{}payload_enc: ", " ".repeat(indent + 2)));
        self.payload_enc.as_slice().sprintf(0, output);
        output.push_str(&format!("\n{}nonce_enc: ", " ".repeat(indent + 2)));
        self.nonce_enc.as_slice().sprintf(0, output);
        output.push_str(&format!("\n{}payload_out: ", " ".repeat(indent + 2)));
        self.payload_out.as_slice().sprintf(0, output);
        output.push_str(&format!("\n{}nonce_out: ", " ".repeat(indent + 2)));
        self.nonce_out.as_slice().sprintf(0, output);
        output.push_str(&format!("\n{}}}", " ".repeat(indent)));
    }
}

impl Formatter for Input {
    fn sprintf(&self, indent: usize, output: &mut String) {
        output.push_str("{");
        output.push_str(&format!("\n{}cv: ", " ".repeat(indent + 2)));
        self.cv.sprintf(0, output);
        output.push_str(&format!("\n{}nf: ", " ".repeat(indent + 2)));
        self.nf.sprintf(0, output);
        output.push_str(&format!("\n{}rk: ", " ".repeat(indent + 2)));
        self.rk.sprintf(0, output);
        output.push_str(&format!("\n{}proof_i: ", " ".repeat(indent + 2)));
        self.proof_i.sprintf(0, output);
        output.push_str(&format!("\n{}signature: ", " ".repeat(indent + 2)));
        self.signature.sprintf(0, output);
        output.push_str(&format!("\n{}}}", " ".repeat(indent)));
    }
}

impl Formatter for Output {
    fn sprintf(&self, indent: usize, output: &mut String) {
        output.push_str("{");
        output.push_str(&format!("\n{}cm: ", " ".repeat(indent + 2)));
        self.cm.sprintf(0, output);
        output.push_str(&format!("\n{}proof_o: ", " ".repeat(indent + 2)));
        self.proof_o.sprintf(0, output);
        output.push_str(&format!("\n{}ciphertext: ", " ".repeat(indent + 2)));
        self.ciphertext.sprintf(indent + 2, output);
        output.push_str(&format!("\n{}}}", " ".repeat(indent)));
    }
}

impl<T: Formatter> Formatter for Vec<T> {
    fn sprintf(&self, indent: usize, output: &mut String) {
        output.push_str("[");
        for (i, item) in self.iter().enumerate() {
            item.sprintf(indent + 2, output);
            if i < self.len() - 1 {
                output.push_str(", ");
            }
        }
        output.push_str("]");
    }
}

impl Formatter for SaplingTransaction {
    fn sprintf(&self, indent: usize, output: &mut String) {
        output.push_str("{");
        output.push_str(&format!("\n{}inputs: ", " ".repeat(indent + 2)));
        self.inputs.sprintf(indent + 2, output);
        output.push_str(&format!("\n{}outputs: ", " ".repeat(indent + 2)));
        self.outputs.sprintf(indent + 2, output);
        output.push_str(&format!("\n{}binding_sig: ", " ".repeat(indent + 2)));
        self.binding_sig.sprintf(0, output);
        output.push_str(&format!(
            "\n{}balance: {}",
            " ".repeat(indent + 2),
            self.balance
        ));
        output.push_str(&format!("\n{}root: ", " ".repeat(indent + 2)));
        self.root.sprintf(0, output);
        output.push_str(&format!("\n{}bound_data: ", " ".repeat(indent + 2)));
        self.bound_data.sprintf(0, output);
        output.push_str(&format!("\n{}}}", " ".repeat(indent)));
    }
}

impl Formatter for SaplingHead {
    fn sprintf(&self, indent: usize, output: &mut String) {
        output.push_str("{");
        output.push_str(&format!(
            "\n{}roots_pos: {}",
            " ".repeat(indent + 2),
            self.roots_pos
        ));
        output.push_str(&format!(
            "\n{}nullifiers_size: {}",
            " ".repeat(indent + 2),
            self.nullifiers_size
        ));
        output.push_str(&format!(
            "\n{}commitments_size: {}",
            " ".repeat(indent + 2),
            self.commitments_size
        ));
        output.push_str(&format!(
            "\n{}memo_size: {}",
            " ".repeat(indent + 2),
            self.memo_size
        ));
        output.push_str(&format!("\n{}}}", " ".repeat(indent)));
    }
}
