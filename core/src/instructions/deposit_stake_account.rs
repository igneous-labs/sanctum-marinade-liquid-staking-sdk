use generic_array_struct::generic_array_struct;

use crate::{
    State, MSOL_MINT_AUTHORITY_PUBKEY, STAKE_PROGRAM, STATE_PUBKEY, SYSTEM_PROGRAM, SYSVAR_CLOCK,
    SYSVAR_RENT, TOKEN_PROGRAM,
};

pub const INSTRUCTION_DISCRIM_DEPOSIT_STAKE_ACCOUNT: [u8; 8] = [110, 130, 115, 41, 164, 102, 2, 59];

#[generic_array_struct(pub)]
#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct DepositStakeAccountIxAccs<T> {
    pub state: T,
    pub validator_list: T,
    pub stake_list: T,
    pub stake_account: T,
    pub stake_authority: T,
    pub duplication_flag: T,
    pub rent_payer: T,
    pub msol_mint: T,
    pub mint_to: T,
    pub msol_mint_authority: T,
    pub clock: T,
    pub rent: T,
    pub system_program: T,
    pub token_program: T,
    pub stake_program: T,
}

pub type DepositStakeAccountIxKeysOwned = DepositStakeAccountIxAccs<[u8; 32]>;
pub type DepositStakeAccountIxKeys<'a> = DepositStakeAccountIxAccs<&'a [u8; 32]>;
pub type DepositStakeAccountIxAccsFlag = DepositStakeAccountIxAccs<bool>;

pub const DEPOSIT_STAKE_ACCOUNT_IX_PREFIX_IS_WRITER: DepositStakeAccountIxAccsFlag =
    DepositStakeAccountIxAccs([false; DEPOSIT_STAKE_ACCOUNT_IX_ACCS_LEN])
        .const_with_state(true)
        .const_with_validator_list(true)
        .const_with_stake_list(true)
        .const_with_stake_account(true)
        .const_with_duplication_flag(true)
        .const_with_rent_payer(true)
        .const_with_msol_mint(true)
        .const_with_mint_to(true);

pub const DEPOSIT_STAKE_ACCOUNT_IX_PREFIX_IS_SIGNER: DepositStakeAccountIxAccsFlag =
    DepositStakeAccountIxAccs([false; DEPOSIT_STAKE_ACCOUNT_IX_ACCS_LEN])
        .const_with_stake_authority(true)
        .const_with_rent_payer(true);

impl<T: Clone> DepositStakeAccountIxAccs<T> {
    #[inline]
    pub const fn new(arr: [T; DEPOSIT_STAKE_ACCOUNT_IX_ACCS_LEN]) -> Self {
        Self(arr)
    }
}

impl DepositStakeAccountIxKeysOwned {
    #[inline]
    pub fn as_borrowed(&self) -> DepositStakeAccountIxKeys<'_> {
        DepositStakeAccountIxKeys::new(self.0.each_ref())
    }

    #[inline]
    pub fn with_keys_from_stake_pool(self, pool: &State) -> Self {
        self.as_borrowed()
            .with_keys_from_stake_pool(pool)
            .into_owned()
    }

    #[inline]
    pub fn with_consts(self) -> Self {
        self.as_borrowed().with_consts().into_owned()
    }

    #[inline]
    pub fn with_mainnet_consts(self) -> Self {
        self.as_borrowed().with_mainnet_consts().into_owned()
    }
}

impl<'a> DepositStakeAccountIxKeys<'a> {
    #[inline]
    pub fn into_owned(self) -> DepositStakeAccountIxKeysOwned {
        DepositStakeAccountIxKeysOwned::new(self.0.map(|pk| *pk))
    }

    #[inline]
    pub const fn with_keys_from_stake_pool(
        self,
        State {
            msol_mint,
            validator_system,
            stake_system,
            ..
        }: &'a State,
    ) -> Self {
        self.const_with_validator_list(&validator_system.validator_list.account)
            .const_with_stake_list(&stake_system.stake_list.account)
            .const_with_msol_mint(msol_mint)
    }

    #[inline]
    pub const fn with_consts(self) -> Self {
        self.const_with_clock(&SYSVAR_CLOCK)
            .const_with_rent(&SYSVAR_RENT)
            .const_with_system_program(&SYSTEM_PROGRAM)
            .const_with_token_program(&TOKEN_PROGRAM)
            .const_with_stake_program(&STAKE_PROGRAM)
    }

    #[inline]
    pub fn with_mainnet_consts(self) -> Self {
        self.const_with_msol_mint_authority(&MSOL_MINT_AUTHORITY_PUBKEY)
            .const_with_state(&STATE_PUBKEY)
    }
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositStakeAccountIxData([u8; 12]);

impl DepositStakeAccountIxData {
    #[inline]
    pub fn new(validator_index: u32) -> Self {
        let mut buf = [0u8; 12];

        buf[0..8].copy_from_slice(&INSTRUCTION_DISCRIM_DEPOSIT_STAKE_ACCOUNT);
        buf[8..12].copy_from_slice(&validator_index.to_le_bytes());

        Self(buf)
    }

    #[inline]
    pub const fn to_buf(&self) -> [u8; 12] {
        self.0
    }
}
