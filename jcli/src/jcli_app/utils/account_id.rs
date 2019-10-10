use bech32::{Bech32, FromBase32};
use chain_addr::{Address, Kind};
use chain_crypto::{Ed25519, PublicKey};
use chain_impl_mockchain::account;
use std::str::FromStr;

#[derive(Debug)]
pub struct AccountId {
    // arg: String,
    bytes: Vec<u8>,
    // account: account::Identifier,
}

fn id_from_pub(pk: PublicKey<Ed25519>) -> account::Identifier {
    account::Identifier::from(pk)
}

impl AccountId {
    // accept either an address with the account kind
    // or a ed25519 publickey
    pub fn try_from_str(src: &str) -> Result<Self, Error> {
        if let Ok(b) = Bech32::from_str(src) {
            let bytes = Vec::from_base32(b.data()).unwrap();
            Ok(Self { bytes })
        } else {
            Err(Error::NotRecognized {
                addr: src.to_string(),
            })
        }
    }

    // account id is encoded in hexadecimal in url argument
    pub fn to_url_arg(&self) -> String {
        // use std::str::FromStr;
        // ;        hex::encode(Vec::from_base32(&Bech32::from_str(&self.arg).unwrap().data()).unwrap())
        hex::encode(&self.bytes)
        // ;hex::encode(self.account.as_ref().as_ref())
    }
}

custom_error! { pub Error
    NotRecognized { addr: String } = "account parameter '{addr}' isn't a valid address or publickey",
    AddressNotAccount { addr: String, kind: String } = "account parameter '{addr}' isn't an account address, found: '{kind}'",
}
