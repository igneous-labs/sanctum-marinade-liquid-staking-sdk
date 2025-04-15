use const_crypto::bs58;
use sanctum_marinade_liquid_staking_core::{self as marinade_staking_sdk};
use solana_pubkey::Pubkey;

use crate::common::KeyedUiAccount;

const EMPTY_PUBKEY: [u8; 32] = [0; 32];

#[test]
fn test_state_serde() {
    let state_account = KeyedUiAccount::from_test_fixtures_file("marinade-state");

    let stake_pool =
        marinade_staking_sdk::State::borsh_de(state_account.account_data().as_slice()).unwrap();

    assert_eq!(stake_pool.msol_supply, 3597210656032211);
    assert_eq!(stake_pool.available_reserve_balance, 265139147340070);
    assert_eq!(stake_pool.validator_system.validator_list.item_size, 61);

    println!(
        "stake_item_size: {:?}",
        stake_pool.stake_system.stake_list.item_size
    );

    assert_eq!(
        bs58::encode_pubkey(&stake_pool.pause_authority).str(),
        "AjGjLWx7vbzgPNxPSQUPjLNjeavQCHVS9VoJNWpnyP6n"
    );

    assert_eq!(
        bs58::encode_pubkey(&stake_pool.msol_mint).str(),
        "mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So"
    );

    assert_eq!(
        bs58::encode_pubkey(&stake_pool.admin_authority).str(),
        "42VJbDihcS81YJPbuhHnHgvo1ehu42j8VK9sNwrnAarR"
    );

    assert_eq!(
        bs58::encode_pubkey(&stake_pool.operational_sol_account).str(),
        "opLSF7LdfyWNBby5o6FT8UFsr2A4UGKteECgtLSYrSm"
    );

    assert_eq!(
        bs58::encode_pubkey(&stake_pool.treasury_msol_account).str(),
        "B1aLzaNMeFVAyQ6f3XbbUyKcH2YPHu2fqiEagmiF23VR"
    );
}

#[test]
fn test_validator_list_serde() {
    let validator_list_account = KeyedUiAccount::from_test_fixtures_file("marinade-validator_list");
    let validator_list_data = validator_list_account.account_data();

    let validator_list =
        marinade_staking_sdk::ValidatorList::try_from_acc_data(&validator_list_data).unwrap();

    for (i, validator_record) in validator_list.0.iter().enumerate() {
        match i {
            17 => {
                check_validator_record(
                    validator_record,
                    ExpectedValidatorRecord {
                        score: 11285,
                        validator_pubkey: "DPmsofVJ1UMRZADgwYAHotJnazMwohHzRHSoomL6Qcao",
                    },
                );
            }
            22 => {
                check_validator_record(
                    validator_record,
                    ExpectedValidatorRecord {
                        score: 0,
                        validator_pubkey: "BH7asDZbKkTmT3UWiNfmMVRgQEEpXoVThGPmQfgWwDhg",
                    },
                );
            }
            23 => {
                check_validator_record(
                    validator_record,
                    ExpectedValidatorRecord {
                        score: 107450,
                        validator_pubkey: "yrfQfUfsZechz1zqQyTRRz43czTZQidcrm4SNVWiDPi",
                    },
                );
            }
            44 => {
                check_validator_record(
                    validator_record,
                    ExpectedValidatorRecord {
                        score: 46468,
                        validator_pubkey: "89jnaTMuq5aXUkmpLbykRNaU16i7Du6QywqqPeCPT1Dy",
                    },
                );
            }
            _ => {}
        }
    }
}

#[test]
fn test_stake_list_serde() {
    let stake_list_account: KeyedUiAccount =
        KeyedUiAccount::from_test_fixtures_file("marinade-stake_list");
    let stake_list_data = stake_list_account.account_data();

    let stake_list =
        marinade_staking_sdk::ListAccount::<marinade_staking_sdk::StakeRecord>::try_from_acc_data(
            &stake_list_data,
        )
        .unwrap();

    let state_account = KeyedUiAccount::from_test_fixtures_file("marinade-state");

    let stake_pool =
        marinade_staking_sdk::State::borsh_de(state_account.account_data().as_slice()).unwrap();

    println!(
        "item_size: {:?}",
        bs58::encode_pubkey(&stake_pool.stake_system.stake_list.account).str()
    );

    let mut stake_account_count = 0;

    for stake_record in stake_list.0.iter() {
        if stake_record.stake_account() != &EMPTY_PUBKEY {
            stake_account_count += 1;
        }
    }

    assert!(stake_account_count > 0);
}

struct ExpectedValidatorRecord<'a> {
    score: u32,
    validator_pubkey: &'a str,
}
fn check_validator_record(
    validator_record: &marinade_staking_sdk::ValidatorRecord,
    expected: ExpectedValidatorRecord,
) {
    let validator_pubkey = Pubkey::new_from_array(*validator_record.validator_account());

    assert_eq!(validator_pubkey.to_string(), expected.validator_pubkey);
    assert_eq!(validator_record.score(), expected.score);
}
