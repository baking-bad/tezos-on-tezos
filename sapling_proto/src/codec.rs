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

pub fn read_string<R: Read>(mut reader: R) -> io::Result<String> {
    let string_size = read_size(&mut reader)?;
    let mut string = String::with_capacity(string_size);
    reader
        .take(string_size.try_into().unwrap())
        .read_to_string(&mut string)?;
    Ok(string)
}

pub fn read_transaction<R: Read>(mut reader: R) -> io::Result<SaplingTransaction> {
    let input_bytes = read_list(&mut reader)?;
    let output_bytes = read_list(&mut reader)?;
    let inputs = read_inputs(input_bytes.as_slice())?;
    let outputs = read_outputs(output_bytes.as_slice())?;
    let binding_sig = Signature::read(&mut reader)?;
    let balance = read_i64(&mut reader)?;
    let root = read_bytes32(&mut reader)?;
    let bound_data = read_string(&mut reader)?;
    let sig_payload = [input_bytes, output_bytes, bound_data.clone().into_bytes()].concat();
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
    use std::{borrow::Borrow, io};

    use crate::{storage::Ciphertext, types::SaplingTransaction};

    // Generated using octez, as in https://tezos.gitlab.io/alpha/sapling.html#sandbox-tutorial
    // Decoded using `octez-codec decode sapling.transaction from sapling_transaction`
    // { "inputs":
    //     [ { "cv":
    //           "8e57b5c09010468f27dea7390f868d6a2bc475b90351fa44cfc91dd7fdab2caa",
    //         "nf":
    //           "d97a43b97ebda42b4d286c31f867bc58b416af0ddf4b8a33a07d448cbe74c97f",
    //         "rk":
    //           "96d53d578c08d2e0f6f8a264c7248d5c4f796d8eee2fef2f5537fea627e3e7da",
    //         "proof_i":
    //           "b56896734358241671d5368087da32cd308d915621f9933149d730f85091a4fbbac51abeba1c9c89d878e0b66c8dff2fa9cf7d09dbc0018db4cc77c7500772fb1a64919830ee62c075f8c4684219cfd3e6dc17896427c0c94b71a6e8fd9f1e7c0a0cf2c03c0d4b64791bf016aeda53f39981790310a5e1697e624ea704ef283bc86eda92646ecbf6f60d97e24285695dadd2af4d5c62f1cba2029f7408c4f3c314ec23e9b0a28fd9b68d83b2462a0612a20a7bbb5f7f6de622b0b68c3d7dcb34",
    //         "signature":
    //           "1f14e5bace9641fae8fe745c639acf75692d39e8cb56f433b5cf11928b8563b2ad910cacf5641e1e5e4e676f873baabab20707ca230fe036ebc4289622908108" } ],
    //   "outputs":
    //     [ { "cm":
    //           "c1766091f7b4283641885f23369caa92155d5d4a51d8f22045fd53c0cb9ddc4d",
    //         "proof_o":
    //           "a94cd5a796fb19b89113fe319cb603c46a21b4c9875eda4b54a38d85eab0b095a427f679667aef78046c70a3296ebd858d55661bd3d849ab7c2eca178cd37af5034f8d24e3d62d1103e05ff3fc5636938ad32403a62c0ca712d459f1659f0bb3078680a46944f8acc13351272980d30038b7efd806aac796a15eff75f8458a7de88aec23596145183716f0799929dd878c7c86d154030dc6756c45e67a434148f8aac437e1efc7dd9023fabd528058474d325dc92637868e684cb83b67eeae61",
    //         "ciphertext":
    //           { "cv":
    //               "2f6c3e62ec02005c0a0becc90e4fad0c10eb7ad2ebdad62fa450d51e63134a40",
    //             "epk":
    //               "2fecda1810076aa4b5c6339619b8941e1029d937504fa6a448f96ba21d36ecac",
    //             "payload_enc":
    //               "e5fc9b76ae6f956709c374ffc25e9d7e17dd2d7e1fe0c2a785dc368d334a96ca32d5a8630c91726ae9a4c615d60243e1b10f95d21ab50906461251354996f8f2c90fe7d988899eddcccbdbdbefcd44",
    //             "nonce_enc": "99e8f234b9a7b5294011370e068ededf98bf0c5d98024fe5",
    //             "payload_out":
    //               "305c1910b9cb3568d13b21116db6426c63052ca0c085341d3bb5f5a2fa67fbdedf497d2a34b9a7d50afc69176b9c2a010a81c51707a33a72a42c740cf3315cc9593b596f2dd472490aa98e649c1db9ef",
    //             "nonce_out": "da8a6c74ba9f105a27f25be282d744150d10ef70642f448c" } },
    //       { "cm":
    //           "7b4473cab71217ab3d531d62e5d04e813e6cd063167920eedde381534a2bda50",
    //         "proof_o":
    //           "86317b3fed9433e17278474a046ae63aeb169cc5e4a71cf37c4079ec2671291681298ffc6582320496736bf4f8639296b09a47834b3dde23a1b35f0fb821e01069f11f76873ad347ee98ddabf001bf0307c12c62c502fe603b01372248ee589316165912cac88773c8cfcd8d06c2973e5d08b42971f9bbde9a075256839895e5556e1d057ea8dad424bfa857a9c9d52ca4d03c9a6b5b26c2b8652d4b8a5c3c9c89f41b4fd6917b29f17c414c8ba6386cf2d16d8caecc48c4addd4d2f94b93320",
    //         "ciphertext":
    //           { "cv":
    //               "3f0d39c7e0071712cd21d82d70079ea270c4d2ee26121db6c12f8b922f8af990",
    //             "epk":
    //               "9bf1a5e50d8364707f657cef0ac568ba9368afb1f7d1b4d77fff07bad9a3f96a",
    //             "payload_enc":
    //               "0c4eab5d24f6c3c137a28428f2b4d2b33ab386f1fa1544ea5b0019e7479e362f926dc2c878ea6c3015aecc01aaa531d1c08f55e8f49cb65c96c3f9089b8a66d12194147e16b95f2340eb5860beae21",
    //             "nonce_enc": "ed571a724ec5c9ec685b0e22adc2a72943755fdfe1ebdff6",
    //             "payload_out":
    //               "16882b096762e8bc996a899452b1132678b879d8f306c3dc3a17925b8c65e14de34023c22dc4bdba5dddee4def4cd25b5c2597582b754d982145ea29cfcb132cf2a35a7b41c17a90f83d675ed156159e",
    //             "nonce_out": "a8e7acf58889e83e1b73ed59f5f954af127b0469e5c3216c" } } ],
    //   "binding_sig":
    //     "65005fc98516ce06a75a22e3824c087a3a7e83f47254763c7e7744b764a6d020ebf1509e51af80e39adaaa0e993325a5d4e3b0f5aefcdc43ecfc65a9046b730c",
    //   "balance": "0",
    //   "root": "69a1f12aea9ef4019a059e69e70d6317c35d936d3ea61181f9fa9fa297fe092f",
    //   "bound_data": ""
    // }
    const SAPLING_TX_HEX: &'static str = "\
        000001608e57b5c09010468f27dea7390f868d6a2bc475b90351fa44cfc91dd7\
        fdab2caad97a43b97ebda42b4d286c31f867bc58b416af0ddf4b8a33a07d448c\
        be74c97f96d53d578c08d2e0f6f8a264c7248d5c4f796d8eee2fef2f5537fea6\
        27e3e7dab56896734358241671d5368087da32cd308d915621f9933149d730f8\
        5091a4fbbac51abeba1c9c89d878e0b66c8dff2fa9cf7d09dbc0018db4cc77c7\
        500772fb1a64919830ee62c075f8c4684219cfd3e6dc17896427c0c94b71a6e8\
        fd9f1e7c0a0cf2c03c0d4b64791bf016aeda53f39981790310a5e1697e624ea7\
        04ef283bc86eda92646ecbf6f60d97e24285695dadd2af4d5c62f1cba2029f74\
        08c4f3c314ec23e9b0a28fd9b68d83b2462a0612a20a7bbb5f7f6de622b0b68c\
        3d7dcb341f14e5bace9641fae8fe745c639acf75692d39e8cb56f433b5cf1192\
        8b8563b2ad910cacf5641e1e5e4e676f873baabab20707ca230fe036ebc42896\
        22908108000003e6c1766091f7b4283641885f23369caa92155d5d4a51d8f220\
        45fd53c0cb9ddc4da94cd5a796fb19b89113fe319cb603c46a21b4c9875eda4b\
        54a38d85eab0b095a427f679667aef78046c70a3296ebd858d55661bd3d849ab\
        7c2eca178cd37af5034f8d24e3d62d1103e05ff3fc5636938ad32403a62c0ca7\
        12d459f1659f0bb3078680a46944f8acc13351272980d30038b7efd806aac796\
        a15eff75f8458a7de88aec23596145183716f0799929dd878c7c86d154030dc6\
        756c45e67a434148f8aac437e1efc7dd9023fabd528058474d325dc92637868e\
        684cb83b67eeae612f6c3e62ec02005c0a0becc90e4fad0c10eb7ad2ebdad62f\
        a450d51e63134a402fecda1810076aa4b5c6339619b8941e1029d937504fa6a4\
        48f96ba21d36ecac0000004fe5fc9b76ae6f956709c374ffc25e9d7e17dd2d7e\
        1fe0c2a785dc368d334a96ca32d5a8630c91726ae9a4c615d60243e1b10f95d2\
        1ab50906461251354996f8f2c90fe7d988899eddcccbdbdbefcd4499e8f234b9\
        a7b5294011370e068ededf98bf0c5d98024fe5305c1910b9cb3568d13b21116d\
        b6426c63052ca0c085341d3bb5f5a2fa67fbdedf497d2a34b9a7d50afc69176b\
        9c2a010a81c51707a33a72a42c740cf3315cc9593b596f2dd472490aa98e649c\
        1db9efda8a6c74ba9f105a27f25be282d744150d10ef70642f448c7b4473cab7\
        1217ab3d531d62e5d04e813e6cd063167920eedde381534a2bda5086317b3fed\
        9433e17278474a046ae63aeb169cc5e4a71cf37c4079ec2671291681298ffc65\
        82320496736bf4f8639296b09a47834b3dde23a1b35f0fb821e01069f11f7687\
        3ad347ee98ddabf001bf0307c12c62c502fe603b01372248ee589316165912ca\
        c88773c8cfcd8d06c2973e5d08b42971f9bbde9a075256839895e5556e1d057e\
        a8dad424bfa857a9c9d52ca4d03c9a6b5b26c2b8652d4b8a5c3c9c89f41b4fd6\
        917b29f17c414c8ba6386cf2d16d8caecc48c4addd4d2f94b933203f0d39c7e0\
        071712cd21d82d70079ea270c4d2ee26121db6c12f8b922f8af9909bf1a5e50d\
        8364707f657cef0ac568ba9368afb1f7d1b4d77fff07bad9a3f96a0000004f0c\
        4eab5d24f6c3c137a28428f2b4d2b33ab386f1fa1544ea5b0019e7479e362f92\
        6dc2c878ea6c3015aecc01aaa531d1c08f55e8f49cb65c96c3f9089b8a66d121\
        94147e16b95f2340eb5860beae21ed571a724ec5c9ec685b0e22adc2a7294375\
        5fdfe1ebdff616882b096762e8bc996a899452b1132678b879d8f306c3dc3a17\
        925b8c65e14de34023c22dc4bdba5dddee4def4cd25b5c2597582b754d982145\
        ea29cfcb132cf2a35a7b41c17a90f83d675ed156159ea8e7acf58889e83e1b73\
        ed59f5f954af127b0469e5c3216c65005fc98516ce06a75a22e3824c087a3a7e\
        83f47254763c7e7744b764a6d020ebf1509e51af80e39adaaa0e993325a5d4e3\
        b0f5aefcdc43ecfc65a9046b730c000000000000000069a1f12aea9ef4019a05\
        9e69e70d6317c35d936d3ea61181f9fa9fa297fe092f00000000";

    #[test]
    fn test_sapling_tx_decode() -> std::io::Result<()> {
        let payload = hex::decode(SAPLING_TX_HEX)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;

        let tx = SaplingTransaction::try_from(payload.as_slice())?;
        assert_eq!("", tx.bound_data.as_str(), "Expected empty bound data");
        assert_eq!(0, tx.balance, "Expected zero balance");
        assert_eq!(1, tx.inputs.len(), "Expected single input");
        assert_eq!(2, tx.outputs.len(), "Expected two outputs");
        Ok(())
    }

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
        let res: Vec<u8> = ciphertext.borrow().try_into()?;

        assert_eq!(payload, res);
        Ok(())
    }
}
