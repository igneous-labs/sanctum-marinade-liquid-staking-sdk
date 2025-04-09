use borsh::{BorshDeserialize, BorshSerialize};

use crate::Fee;

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct LiqPool {
    pub lp_mint: [u8; 32],
    pub lp_mint_authority_bump_seep: u8,
    pub sol_leg_bump_seed: u8,
    pub msol_leg_authority_bump_seed: u8,
    pub msol_leg: [u8; 32],

    pub lp_liquidity_target: u64,
    pub lp_max_fee: Fee,
    pub lp_min_fee: Fee,
    pub treasury_cut: Fee,

    pub lp_supply: u64,
    pub lent_from_sol_leg: u64,
    pub liquidity_sol_cap: u64,
}

impl LiqPool {
    pub const DEFAULT: Self = Self {
        lp_mint: [0u8; 32],
        lp_mint_authority_bump_seep: 0,
        sol_leg_bump_seed: 0,
        msol_leg_authority_bump_seed: 0,
        msol_leg: [0u8; 32],
        lp_liquidity_target: 0,
        lp_max_fee: Fee::ZERO,
        lp_min_fee: Fee::ZERO,
        treasury_cut: Fee::ZERO,
        lp_supply: 0,
        lent_from_sol_leg: 0,
        liquidity_sol_cap: 0,
    };
}
