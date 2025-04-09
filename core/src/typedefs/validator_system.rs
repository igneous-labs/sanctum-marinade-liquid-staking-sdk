use borsh::{BorshDeserialize, BorshSerialize};

use super::List;

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct ValidatorSystem {
    pub validator_list: List,
    pub manager_authority: [u8; 32],
    pub total_validator_score: u32,

    /// sum of all active lamports staked
    pub total_active_balance: u64,

    /// DEPRECATED, no longer used
    pub auto_add_validator_enabled: u8,
}
