use crate::common::jcli_wrapper;
use crate::common::process_assert;
use chain_addr::Discrimination;

use chain_crypto::{bech32::Bech32, Ed25519Extended};
use chain_impl_mockchain::testing::address::AddressData;

#[test]
pub fn test_account_address_made_of_incorrect_ed25519_extended_key() {
    let (_, public_key) = AddressData::generate_key_pair::<Ed25519Extended>().into_keys();

    let mut public_key = public_key.to_bech32_str();
    public_key.remove(20);

    // Assertion changed due to issue #306. After fix please change it to correct one
    process_assert::assert_process_failed_and_contains_message(
        jcli_wrapper::jcli_commands::get_address_account_command(&public_key, Discrimination::Test),
        "Failed to parse bech32, invalid data format",
    );
}

#[test]
pub fn test_account_address_made_of_ed25519_extended_key() {
    let (_, public_key) = AddressData::generate_key_pair::<Ed25519Extended>().into_keys();

    let account_address =
        jcli_wrapper::assert_address_account(&public_key.to_bech32_str(), Discrimination::Test);
    assert_ne!(account_address, "", "generated account address is empty");
}
