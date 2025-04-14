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

impl ValidatorSystem {
    pub const DEFAULT: Self = Self {
        validator_list: List::DEFAULT,
        manager_authority: [0u8; 32],
        total_validator_score: 0,
        total_active_balance: 0,
        auto_add_validator_enabled: 0,
    };

    pub fn get_validator_record(
        &self,
        validator_list_data: &[u8],
        index: u32,
    ) -> Result<ValidatorRecord, borsh::io::Error> {
        self.validator_list.get(validator_list_data, index)
    }
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq, Default)]
pub struct ValidatorRecord {
    pub validator_account: [u8; 32],
    pub active_balance: u64,
    pub score: u32,
    pub last_stake_delta_epoch: u64,
    pub duplication_flag_bump_seed: u8,
}
