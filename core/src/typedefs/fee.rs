use borsh::{BorshDeserialize, BorshSerialize};
use sanctum_u64_ratio::{Floor, Ratio};

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fee {
    pub basis_points: u32,
}

type F = sanctum_fee_ratio::Fee<Floor<Ratio<u32, u16>>>;

impl Fee {
    pub const ZERO: Self = Self { basis_points: 0 };

    #[inline]
    pub const fn to_fee_floor(&self) -> Option<F> {
        F::new(Ratio {
            n: self.basis_points,
            d: 10_000,
        })
    }
}

impl Default for Fee {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Fee {
    inherent_borsh_serde!();
}
