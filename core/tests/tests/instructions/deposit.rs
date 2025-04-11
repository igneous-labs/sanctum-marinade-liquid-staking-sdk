use const_crypto::bs58;
use mollusk_svm::{
    program::{create_keyed_account_for_builtin_program, keyed_account_for_system_program},
    result::InstructionResult,
};
use sanctum_marinade_liquid_staking_core::{self as marinade_staking_sdk};
use solana_account::Account;
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

use crate::common::{
    marinade_mainnet_accounts, metas_from_keys_signer_writer, mollusk_marinade_prog,
    msol_token_acc, payer_account, KeyedUiAccount,
};

#[test]
fn deposit_keys() {
    let state_account = KeyedUiAccount::from_test_fixtures_file("marinade-state");
    let state_pubkey = bs58::decode_pubkey(&state_account.pubkey);
    let state =
        marinade_staking_sdk::State::borsh_de(&mut &state_account.account_data()[..]).unwrap();

    let keys = marinade_staking_sdk::DepositIxKeysOwned::default()
        .with_consts()
        .with_keys_from_stake_pool(&state)
        .with_state(state_pubkey);

    assert_eq!(keys.0[0], state_pubkey);
    assert_eq!(keys.0[1], state.msol_mint);
    assert_eq!(keys.0[3], state.liq_pool.msol_leg);
}

#[test]
fn deposit_fixture() {
    let state_account = KeyedUiAccount::from_test_fixtures_file("marinade-state");
    let state_pubkey = bs58::decode_pubkey(&state_account.pubkey);
    let state =
        marinade_staking_sdk::State::borsh_de(&mut &state_account.account_data()[..]).unwrap();

    let sol_leg_seeds = marinade_staking_sdk::liq_pool_sol_leg_seeds(&state_pubkey);
    let (liq_pool_sol_leg_pda, _bump) = Pubkey::find_program_address(
        &[sol_leg_seeds.0, sol_leg_seeds.1],
        &Pubkey::from(marinade_staking_sdk::MARINADE_STAKING_PROGRAM),
    );

    let msol_leg_authority_seeds =
        marinade_staking_sdk::liq_pool_msol_leg_authority_seeds(&state_pubkey);
    let (liq_pool_msol_leg_authority_pda, _bump) = Pubkey::find_program_address(
        &[msol_leg_authority_seeds.0, msol_leg_authority_seeds.1],
        &Pubkey::from(marinade_staking_sdk::MARINADE_STAKING_PROGRAM),
    );

    let reserve_seeds = marinade_staking_sdk::reserve_seeds(&state_pubkey);
    let (reserve_pda, _bump) = Pubkey::find_program_address(
        &[reserve_seeds.0, reserve_seeds.1],
        &Pubkey::from(marinade_staking_sdk::MARINADE_STAKING_PROGRAM),
    );

    let msol_mint_authority_seeds = marinade_staking_sdk::msol_mint_authority_seeds(&state_pubkey);
    let (msol_mint_authority_pda, _bump) = Pubkey::find_program_address(
        &[msol_mint_authority_seeds.0, msol_mint_authority_seeds.1],
        &Pubkey::from(marinade_staking_sdk::MARINADE_STAKING_PROGRAM),
    );

    let transfer_from = Pubkey::new_unique();
    let mint_to = Pubkey::new_unique();

    let mollusk = mollusk_marinade_prog();

    let keys = marinade_staking_sdk::DepositIxKeysOwned::default()
        .with_liq_pool_sol_leg_pda(liq_pool_sol_leg_pda.to_bytes())
        .with_liq_pool_msol_leg_authority(liq_pool_msol_leg_authority_pda.to_bytes())
        .with_reserve_pda(reserve_pda.to_bytes())
        .with_msol_mint_authority(msol_mint_authority_pda.to_bytes())
        .with_transfer_from(transfer_from.to_bytes())
        .with_mint_to(mint_to.to_bytes())
        .with_consts()
        .with_keys_from_stake_pool(&state)
        .with_state(state_pubkey);

    assert_eq!(keys.0[0], state_pubkey);
    assert_eq!(keys.0[1], state.msol_mint);
    assert_eq!(keys.0[2], liq_pool_sol_leg_pda.to_bytes());
    assert_eq!(keys.0[3], state.liq_pool.msol_leg);
    assert_eq!(keys.0[4], liq_pool_msol_leg_authority_pda.to_bytes());
    assert_eq!(keys.0[5], reserve_pda.to_bytes());
    assert_eq!(keys.0[6], transfer_from.to_bytes());
    assert_eq!(keys.0[7], mint_to.to_bytes());
    assert_eq!(keys.0[8], msol_mint_authority_pda.to_bytes());
    assert_eq!(keys.0[9], marinade_staking_sdk::SYSTEM_PROGRAM);
    assert_eq!(keys.0[10], marinade_staking_sdk::TOKEN_PROGRAM);

    let metas = metas_from_keys_signer_writer(
        keys.0,
        marinade_staking_sdk::DEPOSIT_IX_PREFIX_IS_SIGNER.0,
        marinade_staking_sdk::DEPOSIT_IX_PREFIX_IS_WRITER.0,
    );

    let data = marinade_staking_sdk::DepositIxData::new(1_000);

    let ix = Instruction {
        program_id: Pubkey::new_from_array(marinade_staking_sdk::MARINADE_STAKING_PROGRAM),
        accounts: metas,
        data: data.to_buf().into(),
    };

    let accounts: Vec<_> = marinade_mainnet_accounts()
        .chain([
            keyed_account_for_system_program(),
            create_keyed_account_for_builtin_program(
                &Pubkey::new_from_array(marinade_staking_sdk::STAKE_PROGRAM),
                "solana_stake_program",
            ),
            mollusk_svm_programs_token::token::keyed_account(),
            mollusk.sysvars.keyed_account_for_clock_sysvar(),
            (transfer_from, payer_account(1_000_000_000)),
            (mint_to, msol_token_acc(1_000_000, transfer_from)),
            // TODO: Should we export these keys, like how its done here, https://github.com/igneous-labs/solido-legacy-sdk/blob/master/core/src/keys.rs#L19
            // And then we can use them in `marinade_mainnet_accounts` directly?
            (liq_pool_sol_leg_pda, payer_account(1_000_000)),
            (liq_pool_msol_leg_authority_pda, Account::default()),
            (reserve_pda, payer_account(1_000_000)),
            (msol_mint_authority_pda, Account::default()),
        ])
        .collect();

    let InstructionResult {
        raw_result,
        resulting_accounts,
        ..
    } = mollusk.process_instruction_chain(&[ix], &accounts);

    raw_result.unwrap();

    let transfer_from_account = resulting_accounts
        .iter()
        .find(|(pubkey, _)| pubkey == &transfer_from)
        .expect("transfer_from account should exist");

    println!("transfer_from: {}", transfer_from_account.1.lamports);
}
