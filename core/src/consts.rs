use const_crypto::bs58;

pub const SYSVAR_RENT: [u8; 32] =
    bs58::decode_pubkey("SysvarRent111111111111111111111111111111111");

pub const SYSVAR_STAKE_HISTORY: [u8; 32] =
    bs58::decode_pubkey("SysvarStakeHistory1111111111111111111111111");

pub const SYSVAR_CLOCK: [u8; 32] =
    bs58::decode_pubkey("SysvarC1ock11111111111111111111111111111111");

pub const STAKE_PROGRAM: [u8; 32] =
    bs58::decode_pubkey("Stake11111111111111111111111111111111111111");

pub const SYSVAR_STAKE_CONFIG: [u8; 32] =
    bs58::decode_pubkey("StakeConfig11111111111111111111111111111111");

pub const SYSTEM_PROGRAM: [u8; 32] = bs58::decode_pubkey("11111111111111111111111111111111");

pub const TOKEN_PROGRAM: [u8; 32] =
    bs58::decode_pubkey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

pub const ASSOCIATED_TOKEN_PROGRAM: [u8; 32] =
    bs58::decode_pubkey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

// TODO: Confirm if we want `liquid-staking` or `liquid-staking-referral`
pub const MARINADE_STAKING_PROGRAM: [u8; 32] =
    bs58::decode_pubkey("MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD");

pub const MSOL_MINT_ADDR: [u8; 32] =
    bs58::decode_pubkey("mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So");
