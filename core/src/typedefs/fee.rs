use borsh::{BorshDeserialize, BorshSerialize};
use sanctum_fee_ratio::AftFee;
use sanctum_u64_ratio::{Floor, Ratio};

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Fee {
    pub basis_points: u32,
}

impl Fee {
    pub const ZERO: Self = Self { basis_points: 0 };

    #[inline]
    pub const fn apply(&self, amt: u64) -> Option<AftFee> {
        type F = sanctum_fee_ratio::Fee<Floor<Ratio<u32, u16>>>;

        let f = match F::new(Ratio {
            n: self.basis_points,
            d: 10_000,
        }) {
            None => return None,
            Some(f) => f,
        };

        f.apply(amt)
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
