use generic_array_struct::generic_array_struct;

use crate::{
    State, STAKE_DEPOSIT_AUTHORITY_PUBKEY, STAKE_LIST_PUBKEY, STAKE_PROGRAM,
    STAKE_WITHDRAW_AUTHORITY_PUBKEY, STATE_PUBKEY, SYSTEM_PROGRAM, SYSVAR_CLOCK, TOKEN_PROGRAM,
    VALIDATOR_LIST_PUBKEY,
};

pub const INSTRUCTION_DISCRIM_WITHDRAW_STAKE_ACCOUNT: [u8; 8] =
    [211, 85, 184, 65, 183, 177, 233, 217];

#[generic_array_struct(pub)]
#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct WithdrawStakeAccountIxAccs<T> {
    pub state: T,
    pub msol_mint: T,
    pub burn_msol_from: T,
    pub burn_msol_authority: T,
    pub treasury_msol_account: T,
    pub validator_list: T,
    pub stake_list: T,
    pub stake_withdraw_authority: T,
    pub stake_deposit_authority: T,
    pub stake_account: T,
    pub split_stake_account: T,
    pub split_stake_rent_payer: T,
    pub clock: T,
    pub system_program: T,
    pub token_program: T,
    pub stake_program: T,
}

pub type WithdrawStakeAccountIxKeysOwned = WithdrawStakeAccountIxAccs<[u8; 32]>;
pub type WithdrawStakeAccountIxKeys<'a> = WithdrawStakeAccountIxAccs<&'a [u8; 32]>;
pub type WithdrawStakeAccountIxAccsFlag = WithdrawStakeAccountIxAccs<bool>;

pub const WITHDRAW_STAKE_ACCOUNT_IX_PREFIX_IS_WRITER: WithdrawStakeAccountIxAccsFlag =
    WithdrawStakeAccountIxAccs([false; WITHDRAW_STAKE_ACCOUNT_IX_ACCS_LEN])
        .const_with_state(true)
        .const_with_msol_mint(true)
        .const_with_burn_msol_from(true)
        .const_with_burn_msol_authority(true)
        .const_with_treasury_msol_account(true)
        .const_with_validator_list(true)
        .const_with_stake_list(true)
        .const_with_stake_account(true)
        .const_with_split_stake_account(true)
        .const_with_split_stake_rent_payer(true);

pub const WITHDRAW_STAKE_ACCOUNT_IX_PREFIX_IS_SIGNER: WithdrawStakeAccountIxAccsFlag =
    WithdrawStakeAccountIxAccs([false; WITHDRAW_STAKE_ACCOUNT_IX_ACCS_LEN])
        .const_with_burn_msol_authority(true)
        .const_with_split_stake_account(true)
        .const_with_split_stake_rent_payer(true);

impl<T: Clone> WithdrawStakeAccountIxAccs<T> {
    #[inline]
    pub const fn new(arr: [T; WITHDRAW_STAKE_ACCOUNT_IX_ACCS_LEN]) -> Self {
        Self(arr)
    }
}

impl WithdrawStakeAccountIxKeysOwned {
    #[inline]
    pub fn as_borrowed(&self) -> WithdrawStakeAccountIxKeys<'_> {
        WithdrawStakeAccountIxKeys::new(self.0.each_ref())
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

impl<'a> WithdrawStakeAccountIxKeys<'a> {
    #[inline]
    pub fn into_owned(self) -> WithdrawStakeAccountIxKeysOwned {
        WithdrawStakeAccountIxKeysOwned::new(self.0.map(|pk| *pk))
    }

    #[inline]
    pub const fn with_keys_from_stake_pool(
        self,
        State {
            msol_mint,
            validator_system,
            stake_system,
            treasury_msol_account,
            ..
        }: &'a State,
    ) -> Self {
        self.const_with_validator_list(&validator_system.validator_list.account)
            .const_with_stake_list(&stake_system.stake_list.account)
            .const_with_msol_mint(msol_mint)
            .const_with_treasury_msol_account(treasury_msol_account)
    }

    #[inline]
    pub const fn with_consts(self) -> Self {
        self.const_with_clock(&SYSVAR_CLOCK)
            .const_with_system_program(&SYSTEM_PROGRAM)
            .const_with_token_program(&TOKEN_PROGRAM)
            .const_with_stake_program(&STAKE_PROGRAM)
    }

    #[inline]
    pub fn with_mainnet_consts(self) -> Self {
        self.const_with_state(&STATE_PUBKEY)
            .const_with_validator_list(&VALIDATOR_LIST_PUBKEY)
            .const_with_stake_list(&STAKE_LIST_PUBKEY)
            .const_with_stake_withdraw_authority(&STAKE_WITHDRAW_AUTHORITY_PUBKEY)
            .const_with_stake_deposit_authority(&STAKE_DEPOSIT_AUTHORITY_PUBKEY)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WithdrawStakeAccountIxData(
    #[cfg_attr(feature = "serde", serde(with = "serde_bytes"))] [u8; 56],
);

impl WithdrawStakeAccountIxData {
    #[inline]
    pub fn new(
        stake_index: u32,
        validator_index: u32,
        msol_amount: u64,
        beneficiary: [u8; 32],
    ) -> Self {
        let mut buf = [0u8; 56];

        buf[0..8].copy_from_slice(&INSTRUCTION_DISCRIM_WITHDRAW_STAKE_ACCOUNT);
        buf[8..12].copy_from_slice(&stake_index.to_le_bytes());
        buf[12..16].copy_from_slice(&validator_index.to_le_bytes());
        buf[16..24].copy_from_slice(&msol_amount.to_le_bytes());
        buf[24..56].copy_from_slice(&beneficiary);

        Self(buf)
    }

    #[inline]
    pub const fn to_buf(&self) -> [u8; 56] {
        self.0
    }
}

impl Default for WithdrawStakeAccountIxData {
    #[inline]
    fn default() -> Self {
        Self([0u8; 56])
    }
}
