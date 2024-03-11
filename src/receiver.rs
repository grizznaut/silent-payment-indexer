use bech32::{
    primitives::decode::{CheckedHrpstring, UncheckedHrpstring},
    Bech32, Bech32m, ByteIterExt, Fe32, Fe32IterExt, Hrp,
};
use bitcoin::secp256k1::PublicKey;
use hex_conservative::DisplayHex;

pub struct SilentPaymentAddress {
    pub b_scan: PublicKey,
    pub b_spend: PublicKey,
    pub label: Option<u32>,
}
impl SilentPaymentAddress {
    pub fn new(b_scan: &PublicKey, b_spend: &PublicKey, label: Option<u32>) -> Self {
        Self {
            b_scan: *b_scan,
            b_spend: *b_spend,
            label,
        }
    }
    pub fn from_bech32(bech32: &str) -> Self {
        let unchecked =
            UncheckedHrpstring::new(bech32).expect("valid bech32 character encoded string");
        if unchecked.has_valid_checksum::<Bech32>() {
            panic!("expected a Bech32m string, not a Bech32 string")
        } else if unchecked.has_valid_checksum::<Bech32m>() {
            let mut checked = unchecked.remove_checksum::<Bech32m>();
            if checked.hrp() != Hrp::parse_unchecked("sp") {
                panic!("Expected an 'sp' human readable part (HRP)")
            }
            match checked.remove_witness_version() {
                Some(Fe32::Q) => {
                    let address_bytes = checked.byte_iter().collect::<Vec<u8>>();
                    let (b_scan, b_spend) = address_bytes.split_at(33);
                    let b_scan = PublicKey::from_slice(b_scan).expect("b_scan slice");
                    let b_spend = PublicKey::from_slice(b_spend).expect("b_spend slice");
                    SilentPaymentAddress::new(&b_scan, &b_spend, None)
                }
                _ => panic!("Wrong witness version"),
            }
        } else {
            panic!("Not a valid bech32m Silent Payment address")
        }
    }
    pub fn to_bech32(&self) -> String {
        self.b_scan
            .serialize()
            .iter()
            .chain(self.b_spend.serialize().iter())
            .copied()
            .bytes_to_fes()
            .with_checksum::<Bech32m>(&Hrp::parse_unchecked("sp"))
            .with_witness_version(Fe32::Q)
            .chars()
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use bitcoin::{key::Secp256k1, OutPoint, ScriptBuf, XOnlyPublicKey};

    use crate::{
        pubkey_extraction::get_input_for_ssd,
        test_data::{BIP352TestVectors, ReceivingObject},
        InputData,
    };
    use std::{fs::File, io::Read};

    fn get_bip352_test_vectors() -> BIP352TestVectors {
        let path = format!(
            "{}/test/send_and_receive_test_vectors.json",
            env!("CARGO_MANIFEST_DIR")
        );
        let mut file = File::open(path).unwrap();
        let mut json = String::new();
        file.read_to_string(&mut json).unwrap();
        serde_json::from_str(&json).unwrap()
    }
    fn test_a() {
        let secp = Secp256k1::new();
        let test_vectors = get_bip352_test_vectors();
        let receiving_objects: Vec<&ReceivingObject> = test_vectors
            .test_vectors
            .iter()
            .flat_map(|test_case| {
                println!("{}", test_case.comment);
                test_case.receiving.iter()
            })
            .collect();
        for receiving in receiving_objects.iter() {
            let given = &receiving.given;
            let expected = &receiving.expected;
            let outputs_to_check: Vec<XOnlyPublicKey> =
                given.outputs.iter().map(|output| output.output).collect();
            let vins = given.vin.iter().flat_map(|vin| {
                let prevout = ScriptBuf::from_hex(&vin.prevout.script_pubkey.hex).unwrap();
                let input_data = InputData {
                    prevout: &prevout,
                    script_sig: vin.script_sig.as_deref(),
                    txinwitness: vin.txinwitness.as_ref(),
                };
                let out_point = OutPoint::new(vin.txid, vin.vout);
                get_input_for_ssd(&input_data).map(|input_for_ssd| (out_point, input_for_ssd))
            });
            let b_scan = &given.key_material.scan_priv_key;
            let b_spend = &given.key_material.spend_priv_key;
            let b_scan_pubkey = b_scan.public_key(&secp);
            let b_spend_pubkey = b_spend.public_key(&secp);
            let mut receiving_addresses = Vec::<String>::new();

            for (outpoint, input_for_ssd_pubkey) in vins {}
        }
    }
}