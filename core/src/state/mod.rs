use borsh::{BorshDeserialize, BorshSerialize};

use crate::{Fee, FeeCents, LiqPool, StakeSystem, ValidatorSystem};

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct State {
    discriminator: [u8; 8],

    pub msol_mint: [u8; 32],

    pub admin_authority: [u8; 32],

    // Target for withdrawing rent reserve SOLs. Save bot wallet account here
    pub operational_sol_account: [u8; 32],

    // treasury - external accounts managed by marinade DAO
    pub treasury_msol_account: [u8; 32],

    pub reserve_bump_seed: u8,

    pub msol_mint_authority_bump_seed: u8,

    pub rent_exempt_for_token_acc: u64, // Token-Account For rent exempt

    // fee applied on rewards
    pub reward_fee: Fee,

    pub stake_system: StakeSystem,

    pub validator_system: ValidatorSystem,

    // sum of all the orders received in this epoch
    // must not be used for stake-unstake amount calculation
    // only for reference
    // epoch_stake_orders: u64,
    // epoch_unstake_orders: u64,
    pub liq_pool: LiqPool,

    pub available_reserve_balance: u64, // reserve_pda.lamports() - self.rent_exempt_for_token_acc. Virtual value (real may be > because of transfers into reserve). Use Update* to align

    pub msol_supply: u64, // Virtual value (may be < because of token burn). Use Update* to align
    // For FE. Don't use it for token amount calculation
    pub msol_price: u64,

    ///count tickets for delayed-unstake
    pub circulating_ticket_count: u64,
    ///total lamports amount of generated and not claimed yet tickets
    pub circulating_ticket_balance: u64,
    pub lent_from_reserve: u64,
    pub min_deposit: u64,
    pub min_withdraw: u64,
    pub staking_sol_cap: u64,

    pub emergency_cooling_down: u64,

    pub pause_authority: [u8; 32],
    pub paused: bool,

    // delayed unstake account fee
    // to avoid economic attacks this value should not be zero
    // (this is required because tickets are ready at the end of the epoch)
    // preferred value is one epoch rewards
    pub delayed_unstake_fee: FeeCents,

    // withdraw stake account fee
    // to avoid economic attacks this value should not be zero
    // (this is required because stake accounts are delivered immediately)
    // preferred value is one epoch rewards
    pub withdraw_stake_account_fee: FeeCents,
    pub withdraw_stake_account_enabled: bool,

    // Limit moving stakes from one validator to another
    // by calling redelegate, emergency_unstake and partial_unstake
    // in case of stolen validator manager key or broken delegation strategy bot
    pub last_stake_move_epoch: u64, // epoch of the last stake move action
    pub stake_moved: u64,           // total amount of moved SOL during the epoch #stake_move_epoch
    pub max_stake_moved_per_epoch: Fee, // % of total_lamports_under_control
}

impl State {
    inherent_borsh_serde!();
}
