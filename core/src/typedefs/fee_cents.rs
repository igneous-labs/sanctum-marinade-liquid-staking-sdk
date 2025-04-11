use borsh::{BorshDeserialize, BorshSerialize};
use sanctum_fee_ratio::AftFee;
use sanctum_u64_ratio::{Floor, Ratio};

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
/// FeeCents, same as Fee but / 1_000_000 instead of 10_000
/// 1 FeeCent = 0.0001%, 10_000 FeeCent = 1%, 1_000_000 FeeCent = 100%
pub struct FeeCents {
    pub bp_cents: u32,
}

impl FeeCents {
    pub const ZERO: Self = Self { bp_cents: 0 };

    #[inline]
    pub const fn apply(&self, amt: u64) -> Option<AftFee> {
        type F = sanctum_fee_ratio::Fee<Floor<Ratio<u32, u32>>>;

        let f = match F::new(Ratio {
            n: self.bp_cents,
            d: 1_000_000,
        }) {
            None => return None,
            Some(f) => f,
        };

        f.apply(amt)
    }
}

impl Default for FeeCents {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl FeeCents {
    inherent_borsh_serde!();
}
