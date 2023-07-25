use core::marker::PhantomData;
use core::{mem, slice};

use super::*;

/// Read and write data to and from the underlying byte buffer.
///
/// # Operations
///
/// Each set of operations may support a try, panicking and unchecked variations, see
/// below for more information.
///
/// * `read(offset)`
///
///   Reads a (potentially unaligned) value out of the view.
///
/// * `read_into(offset, dest)`
///
///   Reads a (potentially unaligned) value out of the view into the dest argument.
///
/// * `get(offset)`
///
///   Gets a reference to the data given the offset.
///   Errors if the final pointer is misaligned for the given type.
///
/// * `get_mut(offset)`
///
///   Gets a mutable reference to the data given the offset.
///   Errors if the final pointer is misaligned for the given type.
///
/// * `slice(offset, len)`
///
///   Gets a slice to the data given the offset and len.
///   Errors if the final pointer is misaligned for the given type.
///
/// * `slice_mut(offset, len)`
///
///   Gets a mutable slice to the data given the offset.
///   Errors if the final pointer is misaligned for the given type.
///
/// * `write(offset, value)`
///
///   Writes a value to the view at the given offset.
///
/// # Panics
///
/// *Panicking* methods have no prefix or suffix. They invoke the *Try* methods and
/// panic if they return `None`.
///
/// When calling *Panicking* variation with an offset that ends up out of bounds or
/// if the final pointer is misaligned for the given type the method panics with the
/// message `"invalid offset"`.
///
/// The relevant methods are annotated with `#[track_caller]` providing a useful
/// location where the error happened.
///
/// # Safety
///
/// The *Unchecked* methods have the `_unchecked` suffix and simply assume the offset
/// is correct. This is *Undefined Behavior* when it results in an out of bounds read
/// or write or if a misaligned reference is produced.
///
/// If the *Try* variation returns `None` then the *Unchecked* variation invokes
/// *Undefined Behavior*.
#[repr(transparent)]
pub struct Slice {
    bytes: [u8],
}

pub trait SliceUnsized = ?Sized + BytesOf + Decode;

impl Slice {
    /// Returns a data view into the object's memory.
    #[inline]
    pub fn from_ref<T: ?Sized + BytesOf>(v: &T) -> &Slice {
        // SAFETY: Slice and [u8] have the same exact memory representation and `Slice` is
        // `#[repr(transparent]]`.
        unsafe { mem::transmute(v.bytes_of()) }
    }

    /// Returns a mutable data view into the object's memory.
    #[inline]
    pub fn from_mut<T: BytesOf>(bytes: &mut T) -> &mut Slice {
        // SAFETY: Slice and [u8] have the same exact memory representation and `Slice` is
        // `#[repr(transparent]]`.
        unsafe { mem::transmute(bytes.bytes_of_mut()) }
    }
}

/// Returns the object's memory as a byte slice.
///
/// ```
/// let v = 0xcdcdcdcd_u32;
/// assert_eq!(dataview::bytes(&v), &[0xcd, 0xcd, 0xcd, 0xcd]);
/// ```
#[inline]
pub fn bytes<T: ?Sized + Pod>(src: &T) -> &[u8] {
    unsafe {
        slice::from_raw_parts(
            core::array::from_ref(src).as_ptr().cast::<u8>(),
            mem::size_of_val(src),
        )
    }
}

/// Returns the object's memory as a mutable byte slice.
#[inline]
pub fn bytes_mut<T: ?Sized + Pod>(src: &mut T) -> &mut [u8] {
    unsafe { slice::from_raw_parts_mut(src as *mut _ as *mut u8, mem::size_of_val(src)) }
}

/// Types whose values can be safely transmuted between byte arrays of the same size.
///
/// # Safety
///
/// It must be safe to transmute between any byte array (with length equal to the
/// size of the type) and `Self`.
///
/// This is true for these primitive types: `i8`, `i16`, `i32`, `i64`, `i128`, `u8`,
/// `u16`, `u32`, `u64`, `u128`, `f32`, `f64`. The raw pointer types are not pod
/// under strict provenance rules but can be through the 'int2ptr' feature.
/// Primitives such as `str` and `bool` are not pod because not every valid byte
/// pattern is a valid instance of these types. References or types with lifetimes
/// are _never_ pod.
///
/// Arrays and slices of pod types are also pod themselves.
///
/// Note that it is legal for pod types to be a [ZST](https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts).
///
/// When `Pod` is implemented for a user defined type it must meet the following
/// requirements:
///
/// * Must be annotated with [`#[repr(C)]`](https://doc.rust-lang.org/nomicon/other-reprs.html#reprc)
///   or [`#[repr(transparent)]`](https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent).
/// * Must have every field's type implement `Pod` itself.
/// * Must not have any padding between its fields, define dummy fields to cover the
///   padding.
///
/// # Derive macro
///
/// To help with safely implementing this trait for user defined types, a [derive
/// macro](derive@Pod) is provided to implement the `Pod` trait if the requirements
/// are satisfied.
pub unsafe trait Pod: 'static {}

/// Returns a zero-initialized instance of the type.
///
/// ```
/// let v: i32 = dataview::zeroed();
/// assert_eq!(v, 0);
/// ```
#[inline]
pub fn zeroed<T: Pod>() -> T {
    unsafe { mem::MaybeUninit::zeroed().assume_init() }
}

/// Helper trait to provide methods directly on the pod types.
///
/// Do not use this trait in any signatures, use [`Pod`] directly instead.
/// There's a blanket impl that provides these methods for all pod types.
pub trait PodMethods {
    /// Returns a zero-initialized instance of the type.
    fn zeroed() -> Self
    where
        Self: Sized;
    /// Returns the object's memory as a byte slice.
    fn as_bytes(&self) -> &[u8];
    /// Returns the object's memory as a mutable byte slice.
    fn as_bytes_mut(&mut self) -> &mut [u8];
    /// Returns a data view into the object's memory.
    fn as_data_view(&self) -> &Slice;
    /// Returns a mutable data view into the object's memory.
    fn as_data_view_mut(&mut self) -> &mut Slice;
}

impl<T: ?Sized + Pod> PodMethods for T {
    #[inline]
    fn zeroed() -> T
    where
        T: Sized,
    {
        zeroed()
    }

    #[inline]
    fn as_bytes(&self) -> &[u8] {
        bytes(self)
    }

    #[inline]
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        bytes_mut(self)
    }

    #[inline]
    fn as_data_view(&self) -> &Slice {
        Slice::from_ref(self)
    }

    #[inline]
    fn as_data_view_mut(&mut self) -> &mut Slice {
        Slice::from_mut(self)
    }
}

unsafe impl Pod for () {}

unsafe impl Pod for i8 {}
unsafe impl Pod for i16 {}
unsafe impl Pod for i32 {}
unsafe impl Pod for i64 {}
unsafe impl Pod for i128 {}
unsafe impl Pod for isize {}

unsafe impl Pod for u8 {}
unsafe impl Pod for u16 {}
unsafe impl Pod for u32 {}
unsafe impl Pod for u64 {}
unsafe impl Pod for u128 {}
unsafe impl Pod for usize {}

unsafe impl Pod for f32 {}
unsafe impl Pod for f64 {}

unsafe impl<T: 'static> Pod for PhantomData<T> {}

unsafe impl<T: Pod> Pod for [T] {}
unsafe impl<T: Pod, const N: usize> Pod for [T; N] {}

// Strict provenance approved way of checking raw pointer alignment without exposing
// the pointer
const fn is_aligned<T: Abi>(ptr: *const T) -> bool {
    ptr.addr() & (T::ALIGN - 1) == 0 && ptr.is_access_aligned()
    // let addr: usize = unsafe { mem::transmute(ptr) };
    // addr % mem::align_of::<T>() == 0
}
