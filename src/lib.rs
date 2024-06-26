// Copyright (C) 2024      Whittier Digital Technologies LLC
//
// This file is part of silent-payment-indexer.
//
// silent-payment-indexer is free software: you can redistribute it and/or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// silent-payment-indexer is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Foobar. If not, see
// <https://www.gnu.org/licenses/>.

mod indexer;
mod pubkey_extraction;
mod receiver;
pub mod sender;
mod tagged_hashes;
mod test_data;

use bitcoin::secp256k1::PublicKey;
use bitcoin::{Script, ScriptBuf, Witness, XOnlyPublicKey};
use hex_conservative::DisplayHex;
use once_cell::sync::Lazy;
use std::fmt::Display;

/// "Nothing Up My Sleeves" number from BIP 341: 0x50929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0
static NUMS: [u8; 32] = [
    80, 146, 155, 116, 193, 160, 73, 84, 183, 139, 75, 96, 53, 233, 122, 94, 7, 138, 90, 15, 40,
    236, 150, 213, 71, 191, 238, 154, 206, 128, 58, 192,
];
static NUMS_PUBKEY: Lazy<XOnlyPublicKey> = Lazy::new(|| XOnlyPublicKey::from_slice(&NUMS).unwrap());

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct PublicKeySummation {
    inner: PublicKey,
}
impl PublicKeySummation {
    fn new(keys: &[&PublicKey]) -> Option<Self> {
        PublicKey::combine_keys(keys)
            .ok()
            .map(|pubkey| Self { inner: pubkey })
    }
    fn public_key(&self) -> PublicKey {
        self.inner
    }
}
impl Display for PublicKeySummation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.serialize().as_hex())
    }
}

/// The data required to derive a pubkey from an input.
///
/// This differs from bip-0352's VinInfo because VinInfo contains data not strictly necessary for
/// retrieving the pubkey. VinInfo includes: "outpoint", "scriptSig", "txinwitness", "prevout", and
/// "private_key"
#[derive(Hash, Debug)]
struct InputData<'a> {
    /// The _scriptPubKey_hex of the prevout
    pub prevout: &'a Script,
    pub script_sig: Option<&'a Script>,
    pub txinwitness: Option<&'a Witness>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrive_at_nums() {
        let (nums, _, _) = "50929b74c1a04954b78b4b6035e97a5e078a5a0f28ec96d547bfee9ace803ac0"
            .chars()
            .enumerate()
            .fold(
                (vec![], ' ', ' '),
                |(mut v, mut left, mut right), (i, ch)| {
                    if i % 2 == 0 {
                        left = ch;
                    } else {
                        right = ch;
                        let src = format!("{}{}", left, right);
                        v.push(u8::from_str_radix(&src, 16).unwrap());
                    }
                    (v, left, right)
                },
            );
        assert_eq!(NUMS, nums.as_slice());
    }
}
