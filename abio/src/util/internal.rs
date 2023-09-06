use core::mem::MaybeUninit;
use core::ptr;

/// This method converts a `bytes` slice to a fixed-size array without modifying
/// the byte order.
pub(crate) const fn array_assume_init<const LEN: usize>(bytes: &[u8]) -> crate::Result<[u8; LEN]> {
    if bytes.is_empty() {
        Err(crate::Error::null_reference())
    } else if bytes.len() < LEN {
        Err(crate::Error::out_of_bounds(LEN, bytes.len()))
    } else {
        let mut array = maybe_uninit_array::<LEN>();
        let mut pos = 0;
        while pos < LEN {
            unsafe {
                let dst = array.as_mut_ptr().add(pos);
                let src = bytes.as_ptr().add(pos);
                ptr::write(dst, src);
            }
            pos += 1;
        }

        Ok(unsafe { MaybeUninit::array_assume_init(array) })
    }
}

/// This method converts a `bytes` slice to a fixed-size array without modifying
/// the byte order.
pub(crate) const fn array_assume_init_reversed<const LEN: usize>(
    bytes: &[u8],
) -> crate::Result<[u8; LEN]> {
    if bytes.is_empty() {
        Err(crate::Error::null_reference())
    } else if bytes.len() < LEN {
        Err(crate::Error::out_of_bounds(LEN, bytes.len()))
    } else {
        let mut array = maybe_uninit_array::<LEN>();
        let mut pos = 0;
        while pos < LEN {
            unsafe {
                let dst = array.as_mut_ptr().add(pos);
                let src = bytes
                    .as_ptr()
                    .add(LEN - 1 - pos);
                ptr::write(dst, src);
            }
            pos += 1;
        }

        Ok(unsafe { MaybeUninit::array_assume_init(array) })
    }
}

/// Splits a slice of bytes in two at `offset`, returning a pair of byte slices.
///
/// # Hack
///
/// This is a temporary hack to make this operation `const`. This will be removed
/// when the feature is stabilized.
#[inline]
#[must_use]
pub const unsafe fn split_at_unchecked(bytes: &[u8], offset: usize) -> (&[u8], &[u8]) {
    debug_assert!(bytes.len() >= offset);
    let range = bytes.as_ptr()..bytes.as_ptr().add(offset);
    (
        core::slice::from_raw_parts(range.start, offset),
        core::slice::from_raw_parts(range.end, bytes.len() - offset),
    )
}

#[inline(always)]
fn maybe_uninit_array<const N: usize>() -> [MaybeUninit<u8>; N] {
    // SAFETY: An uninitialized `[MaybeUninit<_>; LEN]` is valid.
    unsafe { MaybeUninit::<[MaybeUninit<u8>; N]>::uninit().assume_init() }
}
