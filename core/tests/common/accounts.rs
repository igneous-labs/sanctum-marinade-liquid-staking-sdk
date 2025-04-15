use std::{fs::File, path::Path};

use sanctum_marinade_liquid_staking_core::{
    LIQ_POOL_MSOL_LEG_AUTHORITY_PUBKEY, LIQ_POOL_SOL_LEG_PUBKEY, MSOL_MINT_ADDR,
    MSOL_MINT_AUTHORITY_PUBKEY, RESERVE_PUBKEY, SYSTEM_PROGRAM, TOKEN_PROGRAM,
};
use serde::{Deserialize, Serialize};
use solana_account::Account;
use solana_account_decoder_client_types::UiAccount;
use solana_pubkey::Pubkey;

use super::test_fixtures_dir;

/// This is the json format of
/// `solana account -o <FILENAME>.json --output json <ACCOUNT-PUBKEY>`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyedUiAccount {
    pub pubkey: String,
    pub account: UiAccount,
}

impl KeyedUiAccount {
    fn from_file<P: AsRef<Path>>(json_file_path: P) -> Self {
        let mut file = File::open(json_file_path).unwrap();
        serde_json::from_reader(&mut file).unwrap()
    }

    /// Loads a KeyedUiAccount from `<test_fixtures_dir()>/relative_json_file_path.json`
    pub fn from_test_fixtures_file<P: AsRef<Path>>(relative_json_file_path: P) -> Self {
        Self::from_file(
            test_fixtures_dir()
                .join(relative_json_file_path)
                .with_extension("json"),
        )
    }

    /// Assumes data is not `UiAccountData::Json`
    pub fn account_data(&self) -> Vec<u8> {
        self.account.data.decode().unwrap()
    }
}

pub fn payer_account(lamports: u64) -> Account {
    Account::new(lamports, 0, &Pubkey::new_from_array(SYSTEM_PROGRAM))
}

pub fn msol_token_acc(amt: u64, owner: Pubkey) -> Account {
    let mut data = vec![0; 165];
    data[0..32].copy_from_slice(&MSOL_MINT_ADDR);
    data[32..64].copy_from_slice(owner.as_array());
    data[64..72].copy_from_slice(&amt.to_le_bytes());
    data[108] = 1; // AccountState
    Account {
        lamports: 2_039_280,
        data,
        owner: Pubkey::new_from_array(TOKEN_PROGRAM),
        executable: false,
        rent_epoch: u64::MAX,
    }
}

pub fn token_acc_balance(account: &Account) -> u64 {
    u64::from_le_bytes(account.data[64..72].try_into().unwrap())
}

fn test_fixtures_accounts<'a>(
    fnames: &'a [&'a str],
) -> impl Iterator<Item = (Pubkey, Account)> + 'a {
    fnames.iter().map(|fname| {
        let KeyedUiAccount { pubkey, account } = KeyedUiAccount::from_test_fixtures_file(fname);
        (pubkey.parse().unwrap(), account.decode().unwrap())
    })
}

pub fn marinade_mainnet_accounts() -> impl Iterator<Item = (Pubkey, Account)> {
    test_fixtures_accounts(
        [
            "marinade-state",
            "marinade-validator_list",
            "marinade-stake_list",
            "marinade-msol_mint",
            "marinade-liq_pool_msol_leg",
            "stake_account",
        ]
        .as_slice(),
    )
    .chain([
        (
            Pubkey::new_from_array(LIQ_POOL_SOL_LEG_PUBKEY),
            payer_account(1_000_000),
        ),
        (
            Pubkey::new_from_array(LIQ_POOL_MSOL_LEG_AUTHORITY_PUBKEY),
            Account::default(),
        ),
        (
            Pubkey::new_from_array(RESERVE_PUBKEY),
            payer_account(1_000_000),
        ),
        (
            Pubkey::new_from_array(MSOL_MINT_AUTHORITY_PUBKEY),
            Account::default(),
        ),
    ])
}
