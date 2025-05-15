use borsh::{BorshDeserialize, BorshSerialize};
use sanctum_u64_ratio::{Floor, Ratio};

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// FeeCents, same as Fee but / 1_000_000 instead of 10_000
/// 1 FeeCent = 0.0001%, 10_000 FeeCent = 1%, 1_000_000 FeeCent = 100%
pub struct FeeCents {
    pub bp_cents: u32,
}

type F = sanctum_fee_ratio::Fee<Floor<Ratio<u32, u32>>>;

impl FeeCents {
    pub const ZERO: Self = Self { bp_cents: 0 };

    #[inline]
    pub const fn to_fee_floor(&self) -> Option<F> {
        F::new(Ratio {
            n: self.bp_cents,
            d: 1_000_000,
        })
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
