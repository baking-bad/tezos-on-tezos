// SPDX-FileCopyrightText: 2023 Baking Bad <hello@bakingbad.dev>
//
// SPDX-License-Identifier: MIT

use std::io::{self, Read, Write};

use crate::{
    storage::ciphertext::{Ciphertext, NONCE_SIZE, PAYLOAD_OUT_SIZE},
    types::{
        Commitment, Input, Nullifier, Output, Proof, PublicKey, SaplingTransaction, Signature,
        ValueCommitment,
    },
};

pub fn read_size<R: Read>(mut reader: R) -> io::Result<usize> {
    let mut buf: [u8; 4] = [0u8; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_be_bytes(buf).try_into().unwrap())
}

pub fn read_bytes32<R: Read>(mut reader: R) -> io::Result<[u8; 32]> {
    let mut bytes = [0u8; 32];
    reader.read_exact(&mut bytes)?;
    Ok(bytes)
}

pub fn read_input<R: Read>(mut reader: R) -> io::Result<Input> {
    let mut buf = [0u8; 32 * 3 + 192];
    reader.read_exact(&mut buf)?;

    let sig_payload = buf.to_vec();
    let mut sub_reader = buf.as_slice();

    let cv = read_value_commitment(&mut sub_reader)?;
    let nf = Nullifier(read_bytes32(&mut sub_reader)?);
    let rk = PublicKey::read(&mut sub_reader)?;
    let proof_i = Proof::read(&mut sub_reader)?;

    let signature = Signature::read(&mut reader)?;
    Ok(Input {
        cv,
        nf,
        rk,
        proof_i,
        signature,
        sig_payload,
    })
}

pub fn read_list<R: Read>(mut reader: R) -> io::Result<Vec<u8>> {
    let total_size = read_size(&mut reader)?;
    let mut bytes = Vec::with_capacity(total_size);

    reader
        .take(total_size.try_into().unwrap())
        .read_to_end(&mut bytes)?;

    Ok(bytes)
}

pub fn read_inputs<R: Read>(mut reader: R) -> io::Result<Vec<Input>> {
    let mut inputs: Vec<Input> = Vec::with_capacity(1);
    while let Ok(input) = read_input(&mut reader) {
        inputs.push(input);
    }
    Ok(inputs)
}

pub fn read_cmu<R: Read>(mut reader: R) -> io::Result<Commitment> {
    let bytes = read_bytes32(&mut reader)?;
    Option::from(Commitment::from_bytes(&bytes))
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "cmu not in field"))
}

pub fn read_value_commitment<R: Read>(mut reader: R) -> io::Result<ValueCommitment> {
    let bytes = read_bytes32(&mut reader)?;
    let cv = ValueCommitment::from_bytes_not_small_order(&bytes);
    if cv.is_none().into() {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid value commitment",
        ))
    } else {
        Ok(cv.unwrap())
    }
}

pub fn read_payload_enc<R: Read>(mut reader: R) -> io::Result<Vec<u8>> {
    let payload_enc_size = read_size(&mut reader)?;
    let mut payload_enc = Vec::with_capacity(payload_enc_size);
    reader
        .take(payload_enc_size.try_into().unwrap())
        .read_to_end(&mut payload_enc)?;

    Ok(payload_enc)
}

pub fn read_ciphertext<R: Read>(mut reader: R) -> io::Result<Ciphertext> {
    let cv = read_value_commitment(&mut reader)?;
    let epk = PublicKey::read(&mut reader)?;
    let payload_enc = read_payload_enc(&mut reader)?;

    let mut nonce_enc = [0u8; NONCE_SIZE];
    let mut payload_out = [0u8; PAYLOAD_OUT_SIZE];
    let mut nonce_out = [0u8; NONCE_SIZE];
    reader.read_exact(&mut nonce_enc)?;
    reader.read_exact(&mut payload_out)?;
    reader.read_exact(&mut nonce_out)?;

    Ok(Ciphertext {
        cv,
        epk,
        payload_enc,
        nonce_enc,
        payload_out,
        nonce_out,
    })
}

pub fn write_ciphertext<W: Write>(ciphertext: &Ciphertext, mut writer: W) -> io::Result<()> {
    writer.write(ciphertext.cv.to_bytes().as_slice())?;
    ciphertext.epk.write(&mut writer)?;

    writer.write(&ciphertext.payload_enc.len().to_be_bytes()[4..])?;
    writer.write(ciphertext.payload_enc.as_slice())?;

    writer.write(ciphertext.nonce_enc.as_slice())?;
    writer.write(ciphertext.payload_out.as_slice())?;
    writer.write(ciphertext.nonce_out.as_slice())?;

    Ok(())
}

pub fn read_output<R: Read>(mut reader: R) -> io::Result<Output> {
    let cm = read_cmu(&mut reader)?;
    let proof_o = Proof::read(&mut reader)?;
    let ciphertext = read_ciphertext(&mut reader)?;

    Ok(Output {
        cm,
        proof_o,
        ciphertext,
    })
}

pub fn read_outputs<R: Read>(mut reader: R) -> io::Result<Vec<Output>> {
    let mut outputs: Vec<Output> = Vec::with_capacity(2);
    while let Ok(output) = read_output(&mut reader) {
        outputs.push(output);
    }
    Ok(outputs)
}

pub fn read_i64<R: Read>(mut reader: R) -> io::Result<i64> {
    let mut buf = [0u8; 8];
    reader.read_exact(&mut buf)?;
    Ok(i64::from_be_bytes(buf))
}

pub fn read_vec<R: Read>(mut reader: R) -> io::Result<Vec<u8>> {
    let vec_size = read_size(&mut reader)?;
    let mut bytes = Vec::with_capacity(vec_size);
    reader
        .take(vec_size.try_into().unwrap())
        .read_to_end(&mut bytes)?;
    Ok(bytes)
}

pub fn read_transaction<R: Read>(mut reader: R) -> io::Result<SaplingTransaction> {
    let input_bytes = read_list(&mut reader)?;
    let output_bytes = read_list(&mut reader)?;
    let inputs = read_inputs(input_bytes.as_slice())?;
    let outputs = read_outputs(output_bytes.as_slice())?;
    let binding_sig = Signature::read(&mut reader)?;
    let balance = read_i64(&mut reader)?;
    let root = read_bytes32(&mut reader)?;
    let bound_data = read_vec(&mut reader)?;
    let sig_payload = [input_bytes, output_bytes, bound_data.clone()].concat();
    Ok(SaplingTransaction {
        inputs,
        outputs,
        binding_sig,
        balance,
        root,
        bound_data,
        sig_payload,
    })
}

impl TryFrom<&[u8]> for SaplingTransaction {
    type Error = io::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut reader = io::BufReader::new(value);
        read_transaction(&mut reader)
    }
}

impl TryFrom<&[u8]> for Ciphertext {
    type Error = io::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        read_ciphertext(value)
    }
}

impl TryInto<Vec<u8>> for &Ciphertext {
    type Error = io::Error;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut writer: Vec<u8> = Vec::new();
        write_ciphertext(self, &mut writer)?;
        Ok(writer)
    }
}

#[cfg(test)]
mod test {
    use std::io;

    use crate::{storage::Ciphertext, types::SaplingTransaction};

    const CIPHERTEXT_HEX: &'static str = "\
        3f0d39c7e0\
        071712cd21d82d70079ea270c4d2ee26121db6c12f8b922f8af9909bf1a5e50d\
        8364707f657cef0ac568ba9368afb1f7d1b4d77fff07bad9a3f96a0000004f0c\
        4eab5d24f6c3c137a28428f2b4d2b33ab386f1fa1544ea5b0019e7479e362f92\
        6dc2c878ea6c3015aecc01aaa531d1c08f55e8f49cb65c96c3f9089b8a66d121\
        94147e16b95f2340eb5860beae21ed571a724ec5c9ec685b0e22adc2a7294375\
        5fdfe1ebdff616882b096762e8bc996a899452b1132678b879d8f306c3dc3a17\
        925b8c65e14de34023c22dc4bdba5dddee4def4cd25b5c2597582b754d982145\
        ea29cfcb132cf2a35a7b41c17a90f83d675ed156159ea8e7acf58889e83e1b73\
        ed59f5f954af127b0469e5c3216c";

    #[test]
    fn test_ciphertext_encode() -> std::io::Result<()> {
        let payload = hex::decode(CIPHERTEXT_HEX)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;

        let ciphertext = Ciphertext::try_from(payload.as_slice())?;
        let res: Vec<u8> = (&ciphertext).try_into()?;

        assert_eq!(payload, res);
        Ok(())
    }

    // https://ghostnet.tzkt.io/KT1PwYL1B8hagFeCcByAcsN3KTQHmJFfDwnj
    const SHIELDING_TX_HEX: &'static str =
        include_str!("../tests/data/op4u8djpMUU5n2q1vuoSa4CApTa2cim3jjyXJD7pJeMQ9mH6Vxc");

    #[test]
    fn test_shielding_tx_decode() -> std::io::Result<()> {
        let payload = hex::decode(SHIELDING_TX_HEX)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;

        let tx = SaplingTransaction::try_from(payload.as_slice())?;
        assert_eq!(0, tx.bound_data.len(), "Expected empty bound data");
        assert_eq!(-10000000, tx.balance, "Expected negative balance");
        assert_eq!(0, tx.inputs.len(), "Expected no inputs");
        assert_eq!(1, tx.outputs.len(), "Expected one output");
        Ok(())
    }

    // https://ghostnet.tzkt.io/KT1PwYL1B8hagFeCcByAcsN3KTQHmJFfDwnj
    const SAPLING_TX_HEX: &'static str =
        include_str!("../tests/data/opDWbJCeqTFayGipzgtczTxdvwVjVFYpC2qAbkvFJLLVcqF6rEx");

    #[test]
    fn test_sapling_tx_decode() -> std::io::Result<()> {
        let payload = hex::decode(SAPLING_TX_HEX)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;

        let tx = SaplingTransaction::try_from(payload.as_slice())?;
        assert_eq!(0, tx.bound_data.len(), "Expected empty bound data");
        assert_eq!(0, tx.balance, "Expected zero balance");
        assert_eq!(1, tx.inputs.len(), "Expected single input");
        assert_eq!(2, tx.outputs.len(), "Expected two outputs");
        Ok(())
    }

    // https://ghostnet.tzkt.io/KT1PwYL1B8hagFeCcByAcsN3KTQHmJFfDwnj
    const UNSHIELDING_TX_HEX: &'static str =
        include_str!("../tests/data/ooCfUWEY9785vAiGxvCqS74qDyCd8vWPXikuLycHjexwe17ALAb");

    #[test]
    fn test_unshielding_tx_decode() -> std::io::Result<()> {
        let payload = hex::decode(UNSHIELDING_TX_HEX)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;

        let tx = SaplingTransaction::try_from(payload.as_slice())?;
        assert_ne!(0, tx.bound_data.len(), "Expected non-empty bound data");
        assert_eq!(10000000, tx.balance, "Expected positibe balance");
        assert_eq!(1, tx.inputs.len(), "Expected single input");
        assert_eq!(1, tx.outputs.len(), "Expected single output");
        Ok(())
    }
}
