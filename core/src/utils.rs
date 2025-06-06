use borsh::{BorshDeserialize, BorshSerialize};

use crate::StakeRecord;

#[derive(Clone, Copy, Debug)]
pub struct DepositSolQuoteArgs {
    pub msol_leg_balance: u64,
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct DepositSolQuote {
    /// Input SOL amount in lamports
    pub in_amount: u64,
    /// Output mSOL amount in lamports (Marinade does not charge fees on deposits)
    pub out_amount: u64,
}

#[derive(Clone, Copy, Debug)]
pub struct DepositStakeQuoteArgs {
    pub msol_leg_balance: u64,
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct DepositStakeQuote {
    /// Staked and unstaked lamports in the stake account
    pub stake_account_lamports_in: StakeAccountLamports,

    /// Output mSOL tokens (Marinade does not charge fees on deposits)
    pub tokens_out: u64,
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct StakeAccountLamports {
    pub staked: u64,
    pub unstaked: u64,
}

impl StakeAccountLamports {
    pub fn total(&self) -> u64 {
        self.staked + self.unstaked
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WithdrawStakeQuoteArgs {
    pub stake_record: StakeRecord,
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct WithdrawStakeQuote {
    pub tokens_in: u64,
    pub lamports_staked: u64,
    /// fee is levied in mSOL and transferred to the
    /// pool's manager fee destination
    pub fee_amount: u64,
}
