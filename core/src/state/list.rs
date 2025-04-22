/// A generic list account that can be used to interpret account data as a slice of items
///
/// Type T must have an alignment of 1 (typically ensured by using #[repr(C)] and
/// containing only types that are byte arrays or have alignment 1)
#[derive(Clone, Copy, Debug)]
pub struct ListAccount<'a, T>(pub &'a [T]);

impl<'a, T> ListAccount<'a, T> {
    /// Tries to interpret the account data as a list of items of type T
    ///
    /// - Skips the 8-byte discriminator prefix
    /// - Verifies that the remaining data has enough bytes for the requested count
    /// - Returns None if the data doesn't match the expected structure
    ///
    /// # Safety
    ///
    /// This uses unsafe code to reinterpret account data bytes as a typed slice.
    /// The type T should:
    /// - Have alignment requirement of 1 (use #[repr(C)] and only byte array members)
    /// - Be suitable for reading from raw memory (typically Copy and 'static)
    pub fn try_from_acc_data(data: &'a [u8], count: usize) -> Option<Self> {
        const {
            assert!(
                core::mem::align_of::<T>() == 1,
                "Type T must have alignment 1 for ListAccount"
            );
            assert!(
                core::mem::size_of::<T>() > 0,
                "Type T must have non-zero size for ListAccount"
            );
        }

        // Skip the 8-byte discriminator
        if data.len() <= 8 {
            return None;
        }

        let remaining = &data[8..];

        // Get the size of type T
        let record_size = core::mem::size_of::<T>();

        // Calculate bytes needed for the requested count
        let bytes_needed = count * record_size;

        // Ensure we have enough data for the requested count
        if remaining.len() < bytes_needed {
            return None;
        }

        // SAFETY:
        // - We've verified that T has alignment of 1
        // - We've verified the data contains at least 'count' T elements
        // - We're treating the data as a read-only slice
        // - The lifetime of the resulting slice is tied to the input data lifetime
        let items = unsafe { core::slice::from_raw_parts(remaining.as_ptr().cast(), count) };

        Some(Self(items))
    }

    /// Get a reference to the underlying slice
    pub fn as_slice(&self) -> &'a [T] {
        self.0
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if self.0.len() <= index {
            return None;
        }

        Some(&self.0[index])
    }
}
