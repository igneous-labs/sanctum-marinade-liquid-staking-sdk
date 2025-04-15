use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct DepositSolQuote {
    /// Input SOL amount in lamports
    pub in_amount: u64,
    /// Output mSOL amount in lamports (Marinade does not charge fees on deposits)
    pub out_amount: u64,
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
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
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
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
