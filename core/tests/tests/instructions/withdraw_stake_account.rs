use crate::common::{
    marinade_mainnet_accounts, metas_from_keys_signer_writer, mollusk_marinade_prog,
    msol_token_acc, payer_account, token_acc_balance, KeyedUiAccount,
};
use borsh::BorshDeserialize;
use const_crypto::bs58;
use mollusk_svm::{
    program::{create_keyed_account_for_builtin_program, keyed_account_for_system_program},
    result::InstructionResult,
};
use sanctum_marinade_liquid_staking_core::{self as marinade_staking_sdk};
use solana_account::Account;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use solana_sdk::stake::state::StakeStateV2;

#[test]
fn withdraw_stake_account_ix() {
    let state_account = KeyedUiAccount::from_test_fixtures_file("marinade-state");
    let state =
        marinade_staking_sdk::State::borsh_de(state_account.account_data().as_slice()).unwrap();

    // This is the validator index for BLADE1...
    let validator_index = 589;
    let stake_index = 10;

    let pool_tokens_to_withdraw = 2_000_000_000;
    let quote = state.quote_withdraw_stake(pool_tokens_to_withdraw).unwrap();

    let user = Pubkey::new_unique();
    let burn_msol_from = Pubkey::new_unique();
    let split_stake_account = Pubkey::new_unique();

    let stake_account = KeyedUiAccount::from_test_fixtures_file("withdraw_stake_account");
    let stake_account_pubkey = bs58::decode_pubkey(&stake_account.pubkey);

    let treasury_msol_account =
        KeyedUiAccount::from_test_fixtures_file("marinade-treasury_msol_account");
    let before_treasury_msol_balance = u64::from_le_bytes(
        treasury_msol_account.account_data()[64..72]
            .try_into()
            .unwrap(),
    );

    let keys = marinade_staking_sdk::WithdrawStakeAccountIxKeysOwned::default()
        .with_consts()
        .with_mainnet_consts()
        .with_keys_from_stake_pool(&state)
        .with_burn_msol_from(burn_msol_from.to_bytes())
        .with_burn_msol_authority(user.to_bytes())
        .with_stake_account(stake_account_pubkey)
        .with_split_stake_account(split_stake_account.to_bytes())
        .with_split_stake_rent_payer(user.to_bytes());

    let metas = metas_from_keys_signer_writer(
        keys.0,
        marinade_staking_sdk::WITHDRAW_STAKE_ACCOUNT_IX_PREFIX_IS_SIGNER.0,
        marinade_staking_sdk::WITHDRAW_STAKE_ACCOUNT_IX_PREFIX_IS_WRITER.0,
    );

    let data = marinade_staking_sdk::WithdrawStakeAccountIxData::new(
        stake_index,
        validator_index,
        pool_tokens_to_withdraw,
        user.to_bytes(),
    );

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
            mollusk_svm_programs_token::token::keyed_account(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            (user, payer_account(5_000_000_000_000)),
            (burn_msol_from, msol_token_acc(5_000_000_000, user)),
            (split_stake_account, Account::default()),
        ])
        .collect();

    let InstructionResult {
        raw_result,
        resulting_accounts,
        ..
    } = mollusk.process_instruction_chain(&[ix], &accounts);

    raw_result.unwrap();

    let treasury_msol_account_index = 4;
    let burn_msol_from_index = 21;
    let split_stake_account_index = 22;

    let burn_msol_from_account = &resulting_accounts[burn_msol_from_index];
    let split_stake_account = &resulting_accounts[split_stake_account_index];
    let treasury_msol_account = &resulting_accounts[treasury_msol_account_index];

    assert_eq!(
        token_acc_balance(&burn_msol_from_account.1),
        5_000_000_000 - 2_000_000_000
    );

    let split_stake =
        StakeStateV2::deserialize(&mut split_stake_account.1.data.as_slice()).unwrap();

    assert_eq!(
        split_stake.delegation().unwrap().stake,
        quote.lamports_staked
    );

    let fee_amount = token_acc_balance(&treasury_msol_account.1) - before_treasury_msol_balance;
    assert_eq!(fee_amount, quote.fee_amount);
}
