use std::sync::OnceLock;

use alloy_primitives::keccak256;
use alloy_rlp::{RlpDecodable, RlpEncodable};

use crate::{
    access_list::AccessList,
    signature::{Signature, SignatureError},
    transaction::{kind::TransactionKind, request::Eip2930TransactionRequest},
    utils::envelop_bytes,
    Address, Bytes, B256, U256,
};

#[derive(Clone, Debug, Eq, RlpDecodable, RlpEncodable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Eip2930SignedTransaction {
    // The order of these fields determines de-/encoding order.
    #[cfg_attr(feature = "serde", serde(with = "crate::serde::u64"))]
    pub chain_id: u64,
    #[cfg_attr(feature = "serde", serde(with = "crate::serde::u64"))]
    pub nonce: u64,
    pub gas_price: U256,
    #[cfg_attr(feature = "serde", serde(with = "crate::serde::u64"))]
    pub gas_limit: u64,
    pub kind: TransactionKind,
    pub value: U256,
    pub input: Bytes,
    pub access_list: AccessList,
    pub odd_y_parity: bool,
    pub r: U256,
    pub s: U256,
    /// Cached transaction hash
    #[rlp(default)]
    #[rlp(skip)]
    #[cfg_attr(feature = "serde", serde(skip))]
    pub hash: OnceLock<B256>,
}

impl Eip2930SignedTransaction {
    pub fn hash(&self) -> &B256 {
        self.hash.get_or_init(|| {
            let encoded = alloy_rlp::encode(self);
            let enveloped = envelop_bytes(1, &encoded);

            keccak256(enveloped)
        })
    }

    /// Recovers the Ethereum address which was used to sign the transaction.
    pub fn recover(&self) -> Result<Address, SignatureError> {
        let signature = Signature {
            r: self.r,
            s: self.s,
            v: u64::from(self.odd_y_parity),
        };

        signature.recover(Eip2930TransactionRequest::from(self).hash())
    }
}

impl PartialEq for Eip2930SignedTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.chain_id == other.chain_id
            && self.nonce == other.nonce
            && self.gas_price == other.gas_price
            && self.gas_limit == other.gas_limit
            && self.kind == other.kind
            && self.value == other.value
            && self.input == other.input
            && self.access_list == other.access_list
            && self.odd_y_parity == other.odd_y_parity
            && self.r == other.r
            && self.s == other.s
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use alloy_rlp::Decodable;
    use k256::SecretKey;

    use super::*;
    use crate::{access_list::AccessListItem, signature::secret_key_from_str};

    fn dummy_request() -> Eip2930TransactionRequest {
        let to = Address::from_str("0xc014ba5ec014ba5ec014ba5ec014ba5ec014ba5e").unwrap();
        let input = hex::decode("1234").unwrap();
        Eip2930TransactionRequest {
            chain_id: 1,
            nonce: 1,
            gas_price: U256::from(2),
            gas_limit: 3,
            kind: TransactionKind::Call(to),
            value: U256::from(4),
            input: Bytes::from(input),
            access_list: vec![AccessListItem {
                address: Address::ZERO,
                storage_keys: vec![B256::ZERO, B256::from(U256::from(1))],
            }],
        }
    }

    fn dummy_secret_key() -> SecretKey {
        secret_key_from_str("e331b6d69882b4cb4ea581d88e0b604039a3de5967688d3dcffdd2270c0fd109")
            .unwrap()
    }

    #[test]
    fn test_eip2930_signed_transaction_encoding() {
        // Generated by Hardhat
        let expected =
            hex::decode("f8bd0101020394c014ba5ec014ba5ec014ba5ec014ba5ec014ba5e04821234f85bf859940000000000000000000000000000000000000000f842a00000000000000000000000000000000000000000000000000000000000000000a0000000000000000000000000000000000000000000000000000000000000000101a0a9f9f0c845cc2d257838df2679a59af6f19055012ce1de11ba25b4ca9df503cfa02c70c54cf6c49b4a641b269c93308fa07de541aa3bcd3fce0fc722aaabe3a8d8")
                .unwrap();

        let request = dummy_request();
        let signed = request.sign(&dummy_secret_key()).unwrap();

        let encoded = alloy_rlp::encode(&signed);
        assert_eq!(expected, encoded);
    }

    #[test]
    fn test_eip2930_signed_transaction_hash() {
        // Generated by hardhat
        let expected = B256::from_slice(
            &hex::decode("1d4f5ef5c7b4b0bd61d4dd622615ec280ae5b9a57136ce6b7686025999220611")
                .unwrap(),
        );

        let request = dummy_request();
        let signed = request.sign(&dummy_secret_key()).unwrap();

        assert_eq!(expected, *signed.hash());
    }

    #[test]
    fn test_eip2930_signed_transaction_rlp() {
        let request = dummy_request();
        let signed = request.sign(&dummy_secret_key()).unwrap();

        let encoded = alloy_rlp::encode(&signed);
        assert_eq!(
            signed,
            Eip2930SignedTransaction::decode(&mut encoded.as_slice()).unwrap()
        );
    }
}
