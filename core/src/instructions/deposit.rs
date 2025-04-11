use generic_array_struct::generic_array_struct;

use crate::{State, SYSTEM_PROGRAM, TOKEN_PROGRAM};

pub const INSTRUCTION_DISCRIM_DEPOSIT: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];

#[generic_array_struct(pub)]
#[repr(transparent)]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi)
)]
pub struct DepositIxAccs<T> {
    pub state: T,
    pub msol_mint: T,
    pub liq_pool_sol_leg_pda: T,
    pub liq_pool_msol_leg: T,
    pub liq_pool_msol_leg_authority: T,
    pub reserve_pda: T,
    pub transfer_from: T,
    pub mint_to: T,
    pub msol_mint_authority: T,
    pub system_program: T,
    pub token_program: T,
}

pub type DepositIxKeysOwned = DepositIxAccs<[u8; 32]>;
pub type DepositIxKeys<'a> = DepositIxAccs<&'a [u8; 32]>;
pub type DepositIxAccsFlag = DepositIxAccs<bool>;

pub const DEPOSIT_IX_PREFIX_IS_WRITER: DepositIxAccsFlag =
    DepositIxAccs([false; DEPOSIT_IX_ACCS_LEN])
        .const_with_state(true)
        .const_with_msol_mint(true)
        .const_with_liq_pool_sol_leg_pda(true)
        .const_with_liq_pool_msol_leg(true)
        .const_with_reserve_pda(true)
        .const_with_transfer_from(true)
        .const_with_mint_to(true);

pub const DEPOSIT_IX_PREFIX_IS_SIGNER: DepositIxAccsFlag =
    DepositIxAccs([false; DEPOSIT_IX_ACCS_LEN]).const_with_transfer_from(true);

impl<T: Clone> DepositIxAccs<T> {
    #[inline]
    pub const fn new(arr: [T; DEPOSIT_IX_ACCS_LEN]) -> Self {
        Self(arr)
    }
}

impl DepositIxKeysOwned {
    #[inline]
    pub fn as_borrowed(&self) -> DepositIxKeys<'_> {
        DepositIxKeys::new(self.0.each_ref())
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
}

impl<'a> DepositIxKeys<'a> {
    #[inline]
    pub fn into_owned(self) -> DepositIxKeysOwned {
        DepositIxKeysOwned::new(self.0.map(|pk| *pk))
    }

    #[inline]
    pub const fn with_keys_from_stake_pool(
        self,
        State {
            msol_mint,
            liq_pool,
            ..
        }: &'a State,
    ) -> Self {
        self.const_with_msol_mint(msol_mint)
            .const_with_liq_pool_msol_leg(&liq_pool.msol_leg)
    }

    #[inline]
    pub const fn with_consts(self) -> Self {
        // TODO: in spl-sdk, we don't do `const_with_token_program`, why??
        self.const_with_system_program(&SYSTEM_PROGRAM)
            .const_with_token_program(&TOKEN_PROGRAM)
    }
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DepositIxData([u8; 16]);

impl DepositIxData {
    #[inline]
    pub fn new(deposit_lamports: u64) -> Self {
        let mut buf = [0u8; 16];

        buf[0..8].copy_from_slice(&INSTRUCTION_DISCRIM_DEPOSIT);
        buf[8..16].copy_from_slice(&deposit_lamports.to_le_bytes());

        Self(buf)
    }

    #[inline]
    pub const fn to_buf(&self) -> [u8; 16] {
        self.0
    }
}
