/// A generic list account that can be used to interpret account data as a slice of items
///
/// Type T must have an alignment of 1 (typically ensured by using #[repr(C)] and
/// containing only types that are byte arrays or have alignment 1)
///
/// # Example
///
/// ```rust
/// use crate::{ListAccount, assert_alignment_is_one};
///
/// #[repr(C)]
/// struct MyRecord {
///     data: [u8; 32],
///     value: [u8; 8],
/// }
///
/// // Verify at compile time that MyRecord has alignment 1
/// assert_alignment_is_one!(MyRecord);
///
/// // Later use it with ListAccount
/// fn process_my_records(account_data: &[u8]) -> Option<Vec<MyRecord>> {
///     let list = ListAccount::<MyRecord>::try_from_acc_data(account_data)?;
///     Some(list.as_slice().to_vec())
/// }
/// ```
pub struct ListAccount<'a, T>(pub &'a [T]);

impl<'a, T> ListAccount<'a, T> {
    /// Tries to interpret the account data as a list of items of type T
    ///
    /// - Skips the 8-byte discriminator prefix
    /// - Verifies that the remaining data is a multiple of the size of T
    /// - Returns None if the data doesn't match the expected structure
    ///
    /// # Safety
    ///
    /// This uses unsafe code to reinterpret account data bytes as a typed slice.
    /// The type T should:
    /// - Have alignment requirement of 1 (use #[repr(C)] and only byte array members)
    /// - Be suitable for reading from raw memory (typically Copy and 'static)
    pub fn try_from_acc_data(data: &'a [u8]) -> Option<Self> {
        // Skip the 8-byte discriminator
        if data.len() <= 8 {
            return None;
        }

        let remaining = &data[8..];

        // Size of T
        let record_size = core::mem::size_of::<T>();

        // Make sure record_size is non-zero
        if record_size == 0 {
            return None;
        }

        // Verify that T has alignment 1, which is required for the unsafe operation below
        if core::mem::align_of::<T>() != 1 {
            return None;
        }

        // Ensure data is not empty
        if remaining.len() < record_size {
            return None;
        }

        // Ensure data length is divisible by the size of T
        if remaining.len() % record_size != 0 {
            return None;
        }

        // Calculate count
        let count = remaining.len() / record_size;

        // SAFETY:
        // - We've verified that T has alignment of 1
        // - We've verified the slice contains a whole number of T elements
        // - We're treating the data as a read-only slice
        // - The lifetime of the resulting slice is tied to the input data lifetime
        let items = unsafe { core::slice::from_raw_parts(remaining.as_ptr() as *const T, count) };

        Some(Self(items))
    }

    /// Get a reference to the underlying slice
    pub fn as_slice(&self) -> &'a [T] {
        self.0
    }
}

// Add a compile-time assertion that T has alignment 1 when it's known at compile time
#[macro_export]
macro_rules! assert_alignment_is_one {
    ($type:ty) => {
        const _: () = assert!(
            core::mem::align_of::<$type>() == 1,
            concat!("Type ", stringify!($type), " must have alignment 1")
        );
    };
}
