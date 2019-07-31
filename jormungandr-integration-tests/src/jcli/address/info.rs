use crate::common::jcli_wrapper;

use chain_addr::Discrimination;
use chain_crypto::{bech32::Bech32, Ed25519Extended};
use chain_impl_mockchain::testing::address::AddressData;

#[test]
pub fn test_info_unknown_address_public_key() {
    let account_address = "48mDfYyQn21iyEPzCfkATEHTwZBcZJqXhRJezmswfvc6Ne89u1axXsiazmgd7SwT8VbafbVnCvyXhBSMhSkPiCezMkqHC4dmxRahRC86SknFu6JF6hwSg8";
    jcli_wrapper::assert_get_address_info_fails(&account_address, "invalid internal encoding");
}

#[test]
pub fn test_info_account_address() {
    let account_address = AddressData::account(Discrimination::Test);

    let info = jcli_wrapper::assert_get_address_info(&account_address.to_bech32_str());
    assert_eq!(
        info.get("discrimination").unwrap(),
        "testing",
        "wrong discrimination"
    );
    assert_eq!(
        info.get("account").unwrap(),
        &account_address.public_key().to_bech32_str(),
        "wrong address"
    );
}

#[test]
pub fn test_info_account_address_for_prod() {
    let account_address = AddressData::account(Discrimination::Production);

    let info = jcli_wrapper::assert_get_address_info(&account_address.to_bech32_str());
    assert_eq!(
        info.get("discrimination").unwrap(),
        "production",
        "wrong discrimination"
    );
    assert_eq!(
        info.get("account").unwrap(),
        &account_address.public_key().to_bech32_str(),
        "wrong address"
    );
}

#[test]
pub fn test_info_delegation_address() {
    let delegation_address = AddressData::delegation(Discrimination::Test);
    let info = jcli_wrapper::assert_get_address_info(&delegation_address.to_bech32_str());
    assert_eq!(
        info.get("discrimination").unwrap(),
        "testing",
        "wrong discrimination"
    );
    assert_eq!(
        info.get("public key").unwrap(),
        &delegation_address.public_key().to_bech32_str(),
        "wrong public key"
    );
    assert_eq!(
        info.get("group key").unwrap(),
        &delegation_address.delegation_key().to_bech32_str(),
        "wrong group key"
    );
}
