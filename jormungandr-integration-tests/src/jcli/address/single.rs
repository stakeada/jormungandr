use crate::common::jcli_wrapper;
use crate::common::process_assert;
use chain_addr::Discrimination;

use chain_crypto::{bech32::Bech32, Ed25519Extended};
use chain_impl_mockchain::testing::address::AddressData;

#[test]
pub fn test_utxo_address_made_of_ed25519_extended_key() {
    let (_, public_key) = AddressData::generate_key_pair::<Ed25519Extended>().into_keys();
    let utxo_address =
        jcli_wrapper::assert_address_single(&public_key.to_bech32_str(), Discrimination::Test);
    assert_ne!(utxo_address, "", "generated utxo address is empty");
}

#[test]
pub fn test_delegation_address_made_of_ed25519_extended_seed_key() {
    let (_, public_key) = AddressData::generate_key_pair::<Ed25519Extended>().into_keys();
    let (_, delegation_key) = AddressData::generate_key_pair::<Ed25519Extended>().into_keys();

    let delegation_address = jcli_wrapper::assert_address_delegation(
        &public_key.to_bech32_str(),
        &delegation_key.to_bech32_str(),
        Discrimination::Test,
    );
    assert_ne!(
        delegation_address, "",
        "generated delegation adress is empty"
    );
}

#[test]
pub fn test_delegation_address_is_the_same_as_public() {
    let (_, public_key) = AddressData::generate_key_pair::<Ed25519Extended>().into_keys();

    let delegation_address = jcli_wrapper::assert_address_delegation(
        &public_key.to_bech32_str(),
        &public_key.to_bech32_str(),
        Discrimination::Test,
    );
    assert_ne!(
        delegation_address, "",
        "generated delegation address is empty"
    );
}

#[test]
pub fn test_delegation_address_for_prod_discrimination() {
    let (_, public_key) = AddressData::generate_key_pair::<Ed25519Extended>().into_keys();

    let delegation_address = jcli_wrapper::assert_address_delegation(
        &public_key.to_bech32_str(),
        &public_key.to_bech32_str(),
        Discrimination::Production,
    );
    assert_ne!(
        delegation_address, "",
        "generated delegation address is empty"
    );
}

#[test]
pub fn test_single_address_for_prod_discrimination() {
    let (_, public_key) = AddressData::generate_key_pair::<Ed25519Extended>().into_keys();

    let delegation_address = jcli_wrapper::assert_address_delegation(
        &public_key.to_bech32_str(),
        &public_key.to_bech32_str(),
        Discrimination::Production,
    );
    assert_ne!(delegation_address, "", "generated single address is empty");
}

#[test]
pub fn test_account_address_for_prod_discrimination() {
    let (_, public_key) = AddressData::generate_key_pair::<Ed25519Extended>().into_keys();

    let delegation_address = jcli_wrapper::assert_address_delegation(
        &public_key.to_bech32_str(),
        &public_key.to_bech32_str(),
        Discrimination::Production,
    );
    assert_ne!(delegation_address, "", "generated account address is empty");
}
#[test]
pub fn test_utxo_address_made_of_incorrect_ed25519_extended_key() {
    let (_, public_key) = AddressData::generate_key_pair::<Ed25519Extended>().into_keys();
    let mut public_key = public_key.to_bech32_str();
    public_key.push('A');

    // Assertion changed due to issue #306. After fix please change it to correct one
    process_assert::assert_process_failed_and_contains_message(
        jcli_wrapper::jcli_commands::get_address_single_command(&public_key, Discrimination::Test),
        "Failed to parse bech32, invalid data format",
    );
}

#[test]
pub fn test_delegation_address_made_of_random_string() {
    let (_, public_key) = AddressData::generate_key_pair::<Ed25519Extended>().into_keys();

    let delegation_key = "adfasdfasdfdasfasdfadfasdf";

    // Assertion changed due to issue #306. After fix please change it to correct one
    process_assert::assert_process_failed_and_contains_message(
        jcli_wrapper::jcli_commands::get_address_delegation_command(
            &public_key.to_bech32_str(),
            &delegation_key,
            Discrimination::Test,
        ),
        "Failed to parse bech32, invalid data format",
    );
}

#[test]
pub fn test_delegation_address_made_of_incorrect_public_ed25519_extended_key() {
    let (_, public_key) = AddressData::generate_key_pair::<Ed25519Extended>().into_keys();
    let (_, delegation_key) = AddressData::generate_key_pair::<Ed25519Extended>().into_keys();

    let mut public_key = public_key.to_bech32_str();
    let delegation_key = delegation_key.to_bech32_str();

    public_key.push('A');

    // Assertion changed due to issue #306. After fix please change it to correct one
    process_assert::assert_process_failed_and_contains_message(
        jcli_wrapper::jcli_commands::get_address_delegation_command(
            &public_key,
            &delegation_key,
            Discrimination::Test,
        ),
        "Failed to parse bech32, invalid data format",
    );
}
