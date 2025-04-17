use const_crypto::{bs58, ed25519::derive_program_address};

use crate::{
    liq_pool_msol_leg_authority_seeds, liq_pool_sol_leg_seeds, msol_mint_authority_seeds,
    reserve_seeds, stake_deposit_authority_seeds, stake_withdraw_authority_seeds,
    MARINADE_STAKING_PROGRAM,
};

pub const STATE_PUBKEY: [u8; 32] =
    bs58::decode_pubkey("8szGkuLTAux9XMgZ2vtY39jVSowEcpBfFfD8hXSEqdGC");

pub const VALIDATOR_LIST_PUBKEY: [u8; 32] =
    bs58::decode_pubkey("DwFYJNnhLmw19FBTrVaLWZ8SZJpxdPoSYVSJaio9tjbY");

pub const STAKE_LIST_PUBKEY: [u8; 32] =
    bs58::decode_pubkey("Anv3XE7e5saNdm16MU6bniYS59Mpv7DzQXHAhxJUmAKW");

pub const LIQ_POOL_MSOL_LEG_PUBKEY: [u8; 32] =
    bs58::decode_pubkey("7GgPYjS5Dza89wV6FpZ23kUJRG5vbQ1GM25ezspYFSoE");

// --------- SOL Leg PDA ---------
const LIQ_POOL_SOL_LEG_SEEDS: (&[u8; 32], &[u8; 7]) = liq_pool_sol_leg_seeds(&STATE_PUBKEY);
const LIQ_POOL_SOL_LEG_PDA_TUP: ([u8; 32], u8) = derive_program_address(
    &[LIQ_POOL_SOL_LEG_SEEDS.0, LIQ_POOL_SOL_LEG_SEEDS.1],
    &MARINADE_STAKING_PROGRAM,
);
pub const LIQ_POOL_SOL_LEG_PUBKEY: [u8; 32] = LIQ_POOL_SOL_LEG_PDA_TUP.0;
pub const LIQ_POOL_SOL_LEG_BUMP: u8 = LIQ_POOL_SOL_LEG_PDA_TUP.1;

// --------- MSOL Leg Authority PDA ---------
const LIQ_POOL_MSOL_LEG_AUTHORITY_SEEDS: (&[u8; 32], &[u8; 20]) =
    liq_pool_msol_leg_authority_seeds(&STATE_PUBKEY);
const LIQ_POOL_MSOL_LEG_AUTHORITY_PDA_TUP: ([u8; 32], u8) = derive_program_address(
    &[
        LIQ_POOL_MSOL_LEG_AUTHORITY_SEEDS.0,
        LIQ_POOL_MSOL_LEG_AUTHORITY_SEEDS.1,
    ],
    &MARINADE_STAKING_PROGRAM,
);
pub const LIQ_POOL_MSOL_LEG_AUTHORITY_PUBKEY: [u8; 32] = LIQ_POOL_MSOL_LEG_AUTHORITY_PDA_TUP.0;
pub const LIQ_POOL_MSOL_LEG_AUTHORITY_BUMP: u8 = LIQ_POOL_MSOL_LEG_AUTHORITY_PDA_TUP.1;

// --------- Reserve PDA ---------
const RESERVE_SEEDS: (&[u8; 32], &[u8; 7]) = reserve_seeds(&STATE_PUBKEY);
const RESERVE_PDA_TUP: ([u8; 32], u8) = derive_program_address(
    &[RESERVE_SEEDS.0, RESERVE_SEEDS.1],
    &MARINADE_STAKING_PROGRAM,
);
pub const RESERVE_PUBKEY: [u8; 32] = RESERVE_PDA_TUP.0;
pub const RESERVE_BUMP: u8 = RESERVE_PDA_TUP.1;

// --------- MSOL Mint Authority PDA ---------
const MSOL_MINT_AUTHORITY_SEEDS: (&[u8; 32], &[u8; 7]) = msol_mint_authority_seeds(&STATE_PUBKEY);
const MSOL_MINT_AUTHORITY_PDA_TUP: ([u8; 32], u8) = derive_program_address(
    &[MSOL_MINT_AUTHORITY_SEEDS.0, MSOL_MINT_AUTHORITY_SEEDS.1],
    &MARINADE_STAKING_PROGRAM,
);
pub const MSOL_MINT_AUTHORITY_PUBKEY: [u8; 32] = MSOL_MINT_AUTHORITY_PDA_TUP.0;
pub const MSOL_MINT_AUTHORITY_BUMP: u8 = MSOL_MINT_AUTHORITY_PDA_TUP.1;

// --------- Stake Withdraw Authority PDA ---------
const STAKE_WITHDRAW_AUTHORITY_SEEDS: (&[u8; 32], &[u8; 8]) =
    stake_withdraw_authority_seeds(&STATE_PUBKEY);
const STAKE_WITHDRAW_AUTHORITY_PDA_TUP: ([u8; 32], u8) = derive_program_address(
    &[
        STAKE_WITHDRAW_AUTHORITY_SEEDS.0,
        STAKE_WITHDRAW_AUTHORITY_SEEDS.1,
    ],
    &MARINADE_STAKING_PROGRAM,
);
pub const STAKE_WITHDRAW_AUTHORITY_PUBKEY: [u8; 32] = STAKE_WITHDRAW_AUTHORITY_PDA_TUP.0;
pub const STAKE_WITHDRAW_AUTHORITY_BUMP: u8 = STAKE_WITHDRAW_AUTHORITY_PDA_TUP.1;

// --------- Stake Deposit Authority PDA ---------
const STAKE_DEPOSIT_AUTHORITY_SEEDS: (&[u8; 32], &[u8; 7]) =
    stake_deposit_authority_seeds(&STATE_PUBKEY);
const STAKE_DEPOSIT_AUTHORITY_PDA_TUP: ([u8; 32], u8) = derive_program_address(
    &[
        STAKE_DEPOSIT_AUTHORITY_SEEDS.0,
        STAKE_DEPOSIT_AUTHORITY_SEEDS.1,
    ],
    &MARINADE_STAKING_PROGRAM,
);
pub const STAKE_DEPOSIT_AUTHORITY_PUBKEY: [u8; 32] = STAKE_DEPOSIT_AUTHORITY_PDA_TUP.0;
pub const STAKE_DEPOSIT_AUTHORITY_BUMP: u8 = STAKE_DEPOSIT_AUTHORITY_PDA_TUP.1;
