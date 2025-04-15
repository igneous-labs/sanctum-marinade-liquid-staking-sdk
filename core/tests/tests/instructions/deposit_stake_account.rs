use borsh::BorshDeserialize;
use const_crypto::bs58;
use mollusk_svm::{
    program::{create_keyed_account_for_builtin_program, keyed_account_for_system_program},
    result::InstructionResult,
};
use sanctum_marinade_liquid_staking_core::{
    self as marinade_staking_sdk, StakeAccountLamports, MARINADE_STAKING_PROGRAM,
};
use solana_account::Account;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use solana_sdk::stake::state::StakeStateV2;

use crate::common::{
    marinade_mainnet_accounts, metas_from_keys_signer_writer, mollusk_marinade_prog,
    msol_token_acc, payer_account, token_acc_balance, KeyedUiAccount,
};

#[test]
fn deposit_stake_account_ix() {
    let state_account = KeyedUiAccount::from_test_fixtures_file("marinade-state");
    let state =
        marinade_staking_sdk::State::borsh_de(state_account.account_data().as_slice()).unwrap();

    // This is the validator index for BLADE1...
    let validator_index = 0;

    // This is the pubkey of the stake account in the fixture
    let staker_pubkey = bs58::decode_pubkey("73mx3pb9AccyrfoY6Agx3baocyqPUNmLFuTM2nbfv1T8");
    let staker = Pubkey::new_from_array(staker_pubkey);

    let stake_account = KeyedUiAccount::from_test_fixtures_file("stake_account");

    let stake_account_pubkey = bs58::decode_pubkey(&stake_account.pubkey);
    let stake_state =
        StakeStateV2::deserialize(&mut stake_account.account_data().as_slice()).unwrap();

    let duplication_flag_pubkey = Pubkey::new_unique();

    let quote = state
        .quote_deposit_stake(StakeAccountLamports {
            staked: stake_state.delegation().unwrap().stake,
            unstaked: stake_account
                .account
                .lamports
                .saturating_sub(stake_state.delegation().unwrap().stake),
        })
        .unwrap();

    let mint_to = Pubkey::new_unique();

    let keys = marinade_staking_sdk::DepositStakeAccountIxKeysOwned::default()
        .with_consts()
        .with_mainnet_consts()
        .with_keys_from_stake_pool(&state)
        .with_stake_account(stake_account_pubkey)
        .with_stake_authority(staker_pubkey)
        .with_duplication_flag(duplication_flag_pubkey.to_bytes())
        .with_rent_payer(staker_pubkey)
        .with_mint_to(mint_to.to_bytes());

    let metas = metas_from_keys_signer_writer(
        keys.0,
        marinade_staking_sdk::DEPOSIT_STAKE_ACCOUNT_IX_PREFIX_IS_SIGNER.0,
        marinade_staking_sdk::DEPOSIT_STAKE_ACCOUNT_IX_PREFIX_IS_WRITER.0,
    );

    let data = marinade_staking_sdk::DepositStakeAccountIxData::new(validator_index);

    let ix = Instruction {
        program_id: Pubkey::new_from_array(marinade_staking_sdk::MARINADE_STAKING_PROGRAM),
        accounts: metas,
        data: data.to_buf().into(),
    };

    let mollusk = mollusk_marinade_prog();

    let accounts: Vec<_> = marinade_mainnet_accounts()
        .chain([
            keyed_account_for_system_program(),
            create_keyed_account_for_builtin_program(
                &Pubkey::new_from_array(marinade_staking_sdk::STAKE_PROGRAM),
                "solana_stake_program",
            ),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            mollusk.sysvars.keyed_account_for_rent_sysvar(),
            mollusk_svm_programs_token::token::keyed_account(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            (staker, payer_account(1_000_000_000)),
            (mint_to, msol_token_acc(1_000_000, staker)),
            (
                duplication_flag_pubkey,
                Account::new(
                    1_000_000_000,
                    0,
                    &Pubkey::new_from_array(MARINADE_STAKING_PROGRAM),
                ),
            ),
        ])
        .collect();

    let InstructionResult {
        raw_result,
        resulting_accounts,
        ..
    } = mollusk.process_instruction_chain(&[ix], &accounts);

    raw_result.unwrap();

    let mint_to_index = 17;
    let mint_to_account = &resulting_accounts[mint_to_index];

    let msol_amount = token_acc_balance(&mint_to_account.1);
    assert_eq!(msol_amount, 1_000_000 + quote.tokens_out);
}
