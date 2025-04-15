use borsh::{BorshDeserialize, BorshSerialize};

use crate::assert_alignment_is_one;

use super::List;

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct StakeSystem {
    pub stake_list: List,
    pub delayed_unstake_cooling_down: u64,
    pub stake_deposit_bump_seed: u8,
    pub stake_withdraw_bump_seed: u8,

    /// set by admin, how much slots before the end of the epoch, stake-delta can start
    pub slots_for_stake_delta: u64,

    /// Marks the start of stake-delta operations, meaning that if somebody starts a delayed-unstake ticket
    /// after this var is set with epoch_num the ticket will have epoch_created = current_epoch+1
    /// (the user must wait one more epoch, because their unstake-delta will be execute in this epoch)
    pub last_stake_delta_epoch: u64,
    pub min_stake: u64,

    /// can be set by validator-manager-auth to allow a second run of stake-delta to stake late stakers in the last minute of the epoch
    /// so we maximize user's rewards
    pub extra_stake_delta_runs: u32,
}

impl StakeSystem {
    pub const DEFAULT: Self = Self {
        stake_list: List::DEFAULT,
        delayed_unstake_cooling_down: 0,
        stake_deposit_bump_seed: 0,
        stake_withdraw_bump_seed: 0,
        slots_for_stake_delta: 0,
        last_stake_delta_epoch: 0,
        min_stake: 0,
        extra_stake_delta_runs: 0,
    };
}

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq, Default)]
#[repr(C)]
pub struct StakeRecord {
    stake_account: [u8; 32],
    last_update_delegated_lamports: [u8; 8],
    last_update_epoch: [u8; 8],
    is_emergency_unstaking: u8,

    additional_record_space: [u8; 7],
}

assert_alignment_is_one!(StakeRecord);

impl StakeRecord {
    #[inline]
    pub fn stake_account(&self) -> &[u8; 32] {
        &self.stake_account
    }

    #[inline]
    pub fn last_update_delegated_lamports(&self) -> u64 {
        u64::from_le_bytes(self.last_update_delegated_lamports)
    }

    #[inline]
    pub fn last_update_epoch(&self) -> u64 {
        u64::from_le_bytes(self.last_update_epoch)
    }

    #[inline]
    pub fn is_emergency_unstaking(&self) -> bool {
        self.is_emergency_unstaking == 1
    }
}
