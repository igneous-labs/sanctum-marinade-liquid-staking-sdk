use const_crypto::bs58;
use mollusk_svm::{
    program::{create_keyed_account_for_builtin_program, keyed_account_for_system_program},
    result::InstructionResult,
};
use sanctum_marinade_liquid_staking_core::{self as marinade_staking_sdk};
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;

use crate::common::{
    marinade_mainnet_accounts, metas_from_keys_signer_writer, mollusk_marinade_prog,
    msol_token_acc, payer_account, KeyedUiAccount,
};

#[test]
fn deposit_ix() {
    let state_account = KeyedUiAccount::from_test_fixtures_file("marinade-state");
    let state_pubkey = bs58::decode_pubkey(&state_account.pubkey);
    let state =
        marinade_staking_sdk::State::borsh_de(state_account.account_data().as_slice()).unwrap();

    let deposit_amount: u64 = 1_000;
    let quote = state.quote_deposit_sol(deposit_amount).unwrap();

    let transfer_from = Pubkey::new_unique();
    let mint_to = Pubkey::new_unique();

    let mollusk = mollusk_marinade_prog();

    let keys = marinade_staking_sdk::DepositIxKeysOwned::default()
        .with_consts()
        .with_mainnet_const_pdas()
        .with_keys_from_stake_pool(&state)
        .with_state(state_pubkey)
        .with_transfer_from(transfer_from.to_bytes())
        .with_mint_to(mint_to.to_bytes());

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

    assert_eq!(transfer_from_account.1.lamports, 999_999_000);

    let mint_to_account = resulting_accounts
        .iter()
        .find(|(pubkey, _)| pubkey == &mint_to)
        .expect("mint_to account should exist");

    let msol_amount = u64::from_le_bytes(mint_to_account.1.data[64..72].try_into().unwrap());
    assert_eq!(msol_amount, 1_000_000 + quote.out_amount);
}
