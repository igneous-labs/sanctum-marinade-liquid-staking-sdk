pub const RESERVE_SEED: [u8; 7] = *b"reserve";
pub const SOL_LEG_SEED: [u8; 7] = *b"liq_sol";
pub const MSOL_LEG_AUTHORITY_SEED: [u8; 20] = *b"liq_st_sol_authority";
pub const MSOL_MINT_AUTHORITY_SEED: [u8; 7] = *b"st_mint";

#[inline]
pub const fn liq_pool_sol_leg_seeds(state: &[u8; 32]) -> (&[u8; 32], &[u8; 7]) {
    (state, &SOL_LEG_SEED)
}

#[inline]
pub const fn liq_pool_msol_leg_authority_seeds(state: &[u8; 32]) -> (&[u8; 32], &[u8; 20]) {
    (state, &MSOL_LEG_AUTHORITY_SEED)
}

#[inline]
pub const fn reserve_seeds(state: &[u8; 32]) -> (&[u8; 32], &[u8; 7]) {
    (state, &RESERVE_SEED)
}

#[inline]
pub const fn msol_mint_authority_seeds(state: &[u8; 32]) -> (&[u8; 32], &[u8; 7]) {
    (state, &MSOL_MINT_AUTHORITY_SEED)
}
