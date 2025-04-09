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
    pub in_amount: u64,
    pub out_amount: u64,
    /// In terms of newly minted LSTs
    pub referral_fee: u64,
    /// In terms of newly minted LSTs
    pub manager_fee: u64,
}

impl DepositSolQuote {
    pub fn total_fees(&self) -> u64 {
        self.referral_fee + self.manager_fee
    }
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
    /// Staked and unstaked lamports, before subtracting fees
    pub stake_account_lamports_in: StakeAccountLamports,

    /// Output tokens, after subtracting fees
    pub tokens_out: u64,

    /// In terms of output tokens
    pub manager_fee: u64,

    /// In terms of output tokens
    pub referral_fee: u64,
}

impl DepositStakeQuote {
    pub fn total_fees(&self) -> u64 {
        self.referral_fee + self.manager_fee
    }
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
