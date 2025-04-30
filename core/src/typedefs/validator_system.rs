use borsh::{BorshDeserialize, BorshSerialize};

use super::List;

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct ValidatorRecord {
    validator_account: [u8; 32],
    active_balance: [u8; 8],
    score: [u8; 4],
    last_stake_delta_epoch: [u8; 8],
    duplication_flag_bump_seed: u8,

    additional_record_space: [u8; 8],
}

impl ValidatorRecord {
    #[inline]
    pub fn validator_account(&self) -> &[u8; 32] {
        &self.validator_account
    }

    #[inline]
    pub fn active_balance(&self) -> u64 {
        u64::from_le_bytes(self.active_balance)
    }

    #[inline]
    pub fn score(&self) -> u32 {
        u32::from_le_bytes(self.score)
    }

    #[inline]
    pub fn last_stake_delta_epoch(&self) -> u64 {
        u64::from_le_bytes(self.last_stake_delta_epoch)
    }

    #[inline]
    pub fn duplication_flag_bump_seed(&self) -> u8 {
        self.duplication_flag_bump_seed
    }
}
