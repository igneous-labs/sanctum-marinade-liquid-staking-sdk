use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct List {
    pub account: [u8; 32],
    pub item_size: u32,
    pub count: u32,

    pub _reserved1: [u8; 32],
    pub _reserved2: u32,
}

impl List {
    pub const DEFAULT: Self = Self {
        account: [0u8; 32],
        item_size: 0,
        count: 0,
        _reserved1: [0u8; 32],
        _reserved2: 0,
    };

    #[inline]
    pub fn item_size(&self) -> u32 {
        self.item_size
    }

    #[inline]
    pub fn len(&self) -> u32 {
        self.count
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}
