use borsh::{BorshDeserialize, BorshSerialize};
use sanctum_u64_ratio::{Floor, Ratio};

use crate::{
    DepositSolQuote, DepositStakeQuote, Fee, FeeCents, LiqPool, StakeAccountLamports, StakeSystem,
    ValidatorSystem, WithdrawStakeQuote,
};

pub mod list;

pub use list::*;

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
    pub const DEFAULT: Self = Self {
        discriminator: [0; 8],
        msol_mint: [0u8; 32],
        admin_authority: [0u8; 32],
        operational_sol_account: [0u8; 32],
        treasury_msol_account: [0u8; 32],
        reserve_bump_seed: 0,
        msol_mint_authority_bump_seed: 0,
        rent_exempt_for_token_acc: 0,
        reward_fee: Fee::ZERO,
        stake_system: StakeSystem::DEFAULT,
        validator_system: ValidatorSystem::DEFAULT,
        liq_pool: LiqPool::DEFAULT,
        available_reserve_balance: 0,
        msol_supply: 0,
        msol_price: 0,
        circulating_ticket_count: 0,
        circulating_ticket_balance: 0,
        lent_from_reserve: 0,
        min_deposit: 0,
        min_withdraw: 0,
        staking_sol_cap: 0,
        emergency_cooling_down: 0,
        pause_authority: [0u8; 32],
        paused: false,
        delayed_unstake_fee: FeeCents::ZERO,
        withdraw_stake_account_fee: FeeCents::ZERO,
        withdraw_stake_account_enabled: false,
        last_stake_move_epoch: 0,
        stake_moved: 0,
        max_stake_moved_per_epoch: Fee::ZERO,
    };

    #[inline]
    pub fn quote_deposit_sol(&self, lamports: u64) -> Option<DepositSolQuote> {
        let out_amount = if self.msol_supply == 0 {
            lamports
        } else {
            self.lamports_to_pool_tokens(lamports)?
        };

        Some(DepositSolQuote {
            in_amount: lamports,
            out_amount,
        })
    }

    #[inline]
    pub fn quote_deposit_stake(
        &self,
        stake_account_lamports: StakeAccountLamports,
    ) -> Option<DepositStakeQuote> {
        let new_pool_tokens = self.lamports_to_pool_tokens(stake_account_lamports.total())?;
        let new_pool_tokens_from_stake =
            self.lamports_to_pool_tokens(stake_account_lamports.staked)?;

        if new_pool_tokens_from_stake > new_pool_tokens {
            return None;
        }

        Some(DepositStakeQuote {
            stake_account_lamports_in: stake_account_lamports,
            // TODO: confirm it's _from_stake and not just total
            tokens_out: new_pool_tokens_from_stake,
        })
    }

    #[inline]
    pub fn quote_withdraw_stake(&self, pool_tokens: u64) -> Option<WithdrawStakeQuote> {
        let lamports = self.pool_tokens_to_lamports(pool_tokens)?;

        // https://github.com/marinade-finance/liquid-staking-program/blob/main/programs/marinade-finance/src/instructions/user/withdraw_stake_account.rs#L176
        let withdraw_fee_lamports = self.withdraw_stake_account_fee.apply(lamports)?;

        Some(WithdrawStakeQuote {
            tokens_in: pool_tokens,
            lamports_staked: lamports,
            fee_amount: withdraw_fee_lamports.fee(),
        })
    }
}

impl State {
    #[inline]
    pub const fn supply_over_lamports(&self) -> Floor<Ratio<u64, u64>> {
        Floor(Ratio {
            n: self.msol_supply,
            d: self.total_virtual_staked_lamports(),
        })
    }

    #[inline]
    pub const fn lamports_over_supply(&self) -> Floor<Ratio<u64, u64>> {
        Floor(Ratio {
            n: self.total_virtual_staked_lamports(),
            d: self.msol_supply,
        })
    }

    #[inline]
    pub const fn lamports_to_pool_tokens(&self, lamports: u64) -> Option<u64> {
        let ratio = self.supply_over_lamports();
        if ratio.0.is_zero() {
            return Some(lamports);
        }
        ratio.apply(lamports)
    }

    #[inline]
    pub const fn pool_tokens_to_lamports(&self, pool_tokens: u64) -> Option<u64> {
        let ratio = self.lamports_over_supply();
        if ratio.0.is_zero() {
            return Some(pool_tokens);
        }
        ratio.apply(pool_tokens)
    }

    #[inline]
    const fn total_cooling_down(&self) -> u64 {
        self.stake_system
            .delayed_unstake_cooling_down
            .checked_add(self.emergency_cooling_down)
            .expect("Total cooling down overflow")
    }

    #[inline]
    const fn total_lamports_under_control(&self) -> u64 {
        self.validator_system
            .total_active_balance
            .checked_add(self.total_cooling_down())
            .expect("Stake balance overflow")
            .checked_add(self.available_reserve_balance)
            .expect("Total SOLs under control overflow")
    }

    #[inline]
    const fn total_virtual_staked_lamports(&self) -> u64 {
        self.total_lamports_under_control()
            .saturating_sub(self.circulating_ticket_balance)
    }
}

impl State {
    inherent_borsh_serde!();
}
