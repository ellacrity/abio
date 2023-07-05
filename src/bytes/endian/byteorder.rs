//! Endian-aware primitives based on the `byteorder` crate.

// TODO: Fill out this module and complete `Endianness` primitives and trait(s)

use core::fmt::Debug;
use core::hash::Hash;
use core::slice;

use crate::integral::{I16, I32, U128, U16, U32, U64};

#[inline]
fn extend_sign(val: u64, nbytes: usize) -> i64 {
    let shift = (8 - nbytes) * 8;
    (val << shift) as i64 >> shift
}

#[inline]
fn extend_sign128(val: u128, nbytes: usize) -> i128 {
    let shift = (16 - nbytes) * 8;
    (val << shift) as i128 >> shift
}

#[inline]
fn unextend_sign(val: i64, nbytes: usize) -> u64 {
    let shift = (8 - nbytes) * 8;
    (val << shift) as u64 >> shift
}

#[inline]
fn unextend_sign128(val: i128, nbytes: usize) -> u128 {
    let shift = (16 - nbytes) * 8;
    (val << shift) as u128 >> shift
}

#[inline]
fn pack_size(n: u64) -> usize {
    if n < 1 << 8 {
        1
    } else if n < 1 << 16 {
        2
    } else if n < 1 << 24 {
        3
    } else if n < 1 << 32 {
        4
    } else if n < 1 << 40 {
        5
    } else if n < 1 << 48 {
        6
    } else if n < 1 << 56 {
        7
    } else {
        8
    }
}

#[inline]
fn pack_size128(n: u128) -> usize {
    if n < 1 << 8 {
        1
    } else if n < 1 << 16 {
        2
    } else if n < 1 << 24 {
        3
    } else if n < 1 << 32 {
        4
    } else if n < 1 << 40 {
        5
    } else if n < 1 << 48 {
        6
    } else if n < 1 << 56 {
        7
    } else if n < 1 << 64 {
        8
    } else if n < 1 << 72 {
        9
    } else if n < 1 << 80 {
        10
    } else if n < 1 << 88 {
        11
    } else if n < 1 << 96 {
        12
    } else if n < 1 << 104 {
        13
    } else if n < 1 << 112 {
        14
    } else if n < 1 << 120 {
        15
    } else {
        16
    }
}

/// `ByteOrder` describes types that can serialize integers as bytes.
///
/// Note that `Self` does not appear anywhere in this trait's definition!
/// Therefore, in order to use it, you'll need to use syntax like
/// `T::read_u16(&[0, 1])` where `T` implements `ByteOrder`.
///
/// This crate provides two types that implement `ByteOrder`: [`BigEndian`]
/// and [`LittleEndian`].
/// This trait is sealed and cannot be implemented for callers to avoid
/// breaking backwards compatibility when adding new derived traits.
///
/// # Examples
///
/// Write and read `u32` numbers in little endian order:
///
/// ```rust
/// use byteorder::{ByteOrder, LittleEndian};
///
/// let mut buf = [0; 4];
/// LittleEndian::write_u32(&mut buf, 1_000_000);
/// assert_eq!(1_000_000, LittleEndian::read_u32(&buf));
/// ```
///
/// Write and read `i16` numbers in big endian order:
///
/// ```rust
/// use byteorder::{ByteOrder, BigEndian};
///
/// let mut buf = [0; 2];
/// BigEndian::write_i16(&mut buf, -5_000);
/// assert_eq!(-5_000, BigEndian::read_i16(&buf));
/// ```
///
/// [`BigEndian`]: enum.BigEndian.html
/// [`LittleEndian`]: enum.LittleEndian.html
pub trait ByteOrder {
    /// Reads an unsigned 16 bit integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 2`.
    fn read_u16(buf: &[u8]) -> U16;

    /// Reads an unsigned 32 bit integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 4`.
    ///
    /// # Examples
    ///
    /// Write and read [`U32`] numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 4];
    /// LittleEndian::write_u32(&mut buf, 1_000_000);
    /// assert_eq!(1_000_000, LittleEndian::read_u32(&buf));
    /// ```
    fn read_u32(buf: &[u8]) -> U32;

    /// Reads an unsigned 64 bit integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 8`.
    ///
    /// # Examples
    ///
    /// Write and read `u64` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 8];
    /// LittleEndian::write_u64(&mut buf, 1_000_000);
    /// assert_eq!(1_000_000, LittleEndian::read_u64(&buf));
    /// ```
    fn read_u64(buf: &[u8]) -> U64;

    /// Reads an unsigned 128 bit integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 16`.
    ///
    /// # Examples
    ///
    /// Write and read `u128` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 16];
    /// LittleEndian::write_u128(&mut buf, 1_000_000);
    /// assert_eq!(1_000_000, LittleEndian::read_u128(&buf));
    /// ```
    fn read_u128(buf: &[u8]) -> U128;

    /// Reads an unsigned n-bytes integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `nbytes < 1` or `nbytes > 8` or
    /// `buf.len() < nbytes`
    ///
    /// # Examples
    ///
    /// Write and read an n-byte number in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 3];
    /// LittleEndian::write_uint(&mut buf, 1_000_000, 3);
    /// assert_eq!(1_000_000, LittleEndian::read_uint(&buf, 3));
    /// ```
    fn read_uint(buf: &[u8], nbytes: usize) -> u64;

    /// Reads an unsigned n-bytes integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `nbytes < 1` or `nbytes > 16` or
    /// `buf.len() < nbytes`
    ///
    /// # Examples
    ///
    /// Write and read an n-byte number in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 3];
    /// LittleEndian::write_uint128(&mut buf, 1_000_000, 3);
    /// assert_eq!(1_000_000, LittleEndian::read_uint128(&buf, 3));
    /// ```
    fn read_uint128(buf: &[u8], nbytes: usize) -> u128;

    /// Writes an unsigned 16 bit integer `n` to `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 2`.
    ///
    /// # Examples
    ///
    /// Write and read `u16` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 2];
    /// LittleEndian::write_u16(&mut buf, 1_000);
    /// assert_eq!(1_000, LittleEndian::read_u16(&buf));
    /// ```
    fn write_u16(buf: &mut [u8], n: u16);

    /// Writes an unsigned 24 bit integer `n` to `buf`, stored in u32.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 3`.
    ///
    /// # Examples
    ///
    /// Write and read 24 bit `u32` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 3];
    /// LittleEndian::write_u24(&mut buf, 1_000_000);
    /// assert_eq!(1_000_000, LittleEndian::read_u24(&buf));
    /// ```
    fn write_u24(buf: &mut [u8], n: u32) {
        Self::write_uint(buf, n as u64, 3)
    }

    /// Writes an unsigned 32 bit integer `n` to `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 4`.
    ///
    /// # Examples
    ///
    /// Write and read `u32` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 4];
    /// LittleEndian::write_u32(&mut buf, 1_000_000);
    /// assert_eq!(1_000_000, LittleEndian::read_u32(&buf));
    /// ```
    fn write_u32(buf: &mut [u8], n: u32);

    /// Writes an unsigned 48 bit integer `n` to `buf`, stored in u64.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 6`.
    ///
    /// # Examples
    ///
    /// Write and read 48 bit `u64` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 6];
    /// LittleEndian::write_u48(&mut buf, 1_000_000_000_000);
    /// assert_eq!(1_000_000_000_000, LittleEndian::read_u48(&buf));
    /// ```
    fn write_u48(buf: &mut [u8], n: u64) {
        Self::write_uint(buf, n as u64, 6)
    }

    /// Writes an unsigned 64 bit integer `n` to `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 8`.
    ///
    /// # Examples
    ///
    /// Write and read `u64` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 8];
    /// LittleEndian::write_u64(&mut buf, 1_000_000);
    /// assert_eq!(1_000_000, LittleEndian::read_u64(&buf));
    /// ```
    fn write_u64(buf: &mut [u8], n: u64);

    /// Writes an unsigned 128 bit integer `n` to `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 16`.
    ///
    /// # Examples
    ///
    /// Write and read `u128` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 16];
    /// LittleEndian::write_u128(&mut buf, 1_000_000);
    /// assert_eq!(1_000_000, LittleEndian::read_u128(&buf));
    /// ```
    fn write_u128(buf: &mut [u8], n: u128);

    /// Writes an unsigned integer `n` to `buf` using only `nbytes`.
    ///
    /// # Panics
    ///
    /// If `n` is not representable in `nbytes`, or if `nbytes` is `> 8`, then
    /// this method panics.
    ///
    /// # Examples
    ///
    /// Write and read an n-byte number in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 3];
    /// LittleEndian::write_uint(&mut buf, 1_000_000, 3);
    /// assert_eq!(1_000_000, LittleEndian::read_uint(&buf, 3));
    /// ```
    fn write_uint(buf: &mut [u8], n: u64, nbytes: usize);

    /// Writes an unsigned integer `n` to `buf` using only `nbytes`.
    ///
    /// # Panics
    ///
    /// If `n` is not representable in `nbytes`, or if `nbytes` is `> 16`, then
    /// this method panics.
    ///
    /// # Examples
    ///
    /// Write and read an n-byte number in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 3];
    /// LittleEndian::write_uint128(&mut buf, 1_000_000, 3);
    /// assert_eq!(1_000_000, LittleEndian::read_uint128(&buf, 3));
    /// ```
    fn write_uint128(buf: &mut [u8], n: u128, nbytes: usize);

    /// Reads a signed 16 bit integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 2`.
    ///
    /// # Examples
    ///
    /// Write and read `i16` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 2];
    /// LittleEndian::write_i16(&mut buf, -1_000);
    /// assert_eq!(-1_000, LittleEndian::read_i16(&buf));
    /// ```
    #[inline]
    fn read_i16(buf: &[u8]) -> I16 {
        let res = Self::read_u16(buf).get() as i16;
        I16::new(res)
    }

    /// Reads a signed 24 bit integer from `buf`, stored in i32.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 3`.
    ///
    /// # Examples
    ///
    /// Write and read 24 bit `i32` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 3];
    /// LittleEndian::write_i24(&mut buf, -1_000_000);
    /// assert_eq!(-1_000_000, LittleEndian::read_i24(&buf));
    /// ```
    #[inline]
    fn read_i24(buf: &[u8]) -> i32 {
        Self::read_int(buf, 3) as i32
    }

    /// Reads a signed 32 bit integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 4`.
    ///
    /// # Examples
    ///
    /// Write and read `i32` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 4];
    /// LittleEndian::write_i32(&mut buf, -1_000_000);
    /// assert_eq!(-1_000_000, LittleEndian::read_i32(&buf));
    /// ```
    #[inline]
    fn read_i32(buf: &[u8]) -> I32 {
        I32::new(Self::read_u32(buf).get() as i32)
    }

    /// Reads a signed 48 bit integer from `buf`, stored in i64.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 6`.
    ///
    /// # Examples
    ///
    /// Write and read 48 bit `i64` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 6];
    /// LittleEndian::write_i48(&mut buf, -1_000_000_000_000);
    /// assert_eq!(-1_000_000_000_000, LittleEndian::read_i48(&buf));
    /// ```
    #[inline]
    fn read_i48(buf: &[u8]) -> i64 {
        Self::read_int(buf, 6) as i64
    }

    /// Reads a signed 64 bit integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 8`.
    ///
    /// # Examples
    ///
    /// Write and read `i64` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 8];
    /// LittleEndian::write_i64(&mut buf, -1_000_000_000);
    /// assert_eq!(-1_000_000_000, LittleEndian::read_i64(&buf));
    /// ```
    #[inline]
    fn read_i64(buf: &[u8]) -> i64 {
        Self::read_u64(buf).get() as i64
    }

    /// Reads a signed 128 bit integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 16`.
    ///
    /// # Examples
    ///
    /// Write and read `i128` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 16];
    /// LittleEndian::write_i128(&mut buf, -1_000_000_000);
    /// assert_eq!(-1_000_000_000, LittleEndian::read_i128(&buf));
    /// ```
    #[inline]
    fn read_i128(buf: &[u8]) -> i128 {
        Self::read_u128(buf).get() as i128
    }

    /// Reads a signed n-bytes integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `nbytes < 1` or `nbytes > 8` or
    /// `buf.len() < nbytes`
    ///
    /// # Examples
    ///
    /// Write and read n-length signed numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 3];
    /// LittleEndian::write_int(&mut buf, -1_000, 3);
    /// assert_eq!(-1_000, LittleEndian::read_int(&buf, 3));
    /// ```
    #[inline]
    fn read_int(buf: &[u8], nbytes: usize) -> i64 {
        extend_sign(Self::read_uint(buf, nbytes), nbytes)
    }

    /// Reads a signed n-bytes integer from `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `nbytes < 1` or `nbytes > 16` or
    /// `buf.len() < nbytes`
    ///
    /// # Examples
    ///
    /// Write and read n-length signed numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 3];
    /// LittleEndian::write_int128(&mut buf, -1_000, 3);
    /// assert_eq!(-1_000, LittleEndian::read_int128(&buf, 3));
    /// ```
    #[inline]
    fn read_int128(buf: &[u8], nbytes: usize) -> i128 {
        extend_sign128(Self::read_uint128(buf, nbytes), nbytes)
    }

    /// Reads a IEEE754 single-precision (4 bytes) floating point number.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 4`.
    ///
    /// # Examples
    ///
    /// Write and read `f32` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let e = 2.71828;
    /// let mut buf = [0; 4];
    /// LittleEndian::write_f32(&mut buf, e);
    /// assert_eq!(e, LittleEndian::read_f32(&buf));
    /// ```
    #[inline]
    fn read_f32(buf: &[u8]) -> f32 {
        f32::from_bits(Self::read_u32(buf).get())
    }

    /// Reads a IEEE754 double-precision (8 bytes) floating point number.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 8`.
    ///
    /// # Examples
    ///
    /// Write and read `f64` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let phi = 1.6180339887;
    /// let mut buf = [0; 8];
    /// LittleEndian::write_f64(&mut buf, phi);
    /// assert_eq!(phi, LittleEndian::read_f64(&buf));
    /// ```
    #[inline]
    fn read_f64(buf: &[u8]) -> f64 {
        f64::from_bits(Self::read_u64(buf).get())
    }

    /// Writes a signed 16 bit integer `n` to `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 2`.
    ///
    /// # Examples
    ///
    /// Write and read `i16` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 2];
    /// LittleEndian::write_i16(&mut buf, -1_000);
    /// assert_eq!(-1_000, LittleEndian::read_i16(&buf));
    /// ```
    #[inline]
    fn write_i16(buf: &mut [u8], n: i16) {
        Self::write_u16(buf, n as u16)
    }

    /// Writes a signed 24 bit integer `n` to `buf`, stored in i32.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 3`.
    ///
    /// # Examples
    ///
    /// Write and read 24 bit `i32` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 3];
    /// LittleEndian::write_i24(&mut buf, -1_000_000);
    /// assert_eq!(-1_000_000, LittleEndian::read_i24(&buf));
    /// ```
    #[inline]
    fn write_i24(buf: &mut [u8], n: i32) {
        Self::write_int(buf, n as i64, 3)
    }

    /// Writes a signed 32 bit integer `n` to `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 4`.
    ///
    /// # Examples
    ///
    /// Write and read `i32` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 4];
    /// LittleEndian::write_i32(&mut buf, -1_000_000);
    /// assert_eq!(-1_000_000, LittleEndian::read_i32(&buf));
    /// ```
    #[inline]
    fn write_i32(buf: &mut [u8], n: i32) {
        Self::write_u32(buf, n as u32)
    }

    /// Writes a signed 48 bit integer `n` to `buf`, stored in i64.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 6`.
    ///
    /// # Examples
    ///
    /// Write and read 48 bit `i64` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 6];
    /// LittleEndian::write_i48(&mut buf, -1_000_000_000_000);
    /// assert_eq!(-1_000_000_000_000, LittleEndian::read_i48(&buf));
    /// ```
    #[inline]
    fn write_i48(buf: &mut [u8], n: i64) {
        Self::write_int(buf, n as i64, 6)
    }

    /// Writes a signed 64 bit integer `n` to `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 8`.
    ///
    /// # Examples
    ///
    /// Write and read `i64` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 8];
    /// LittleEndian::write_i64(&mut buf, -1_000_000_000);
    /// assert_eq!(-1_000_000_000, LittleEndian::read_i64(&buf));
    /// ```
    #[inline]
    fn write_i64(buf: &mut [u8], n: i64) {
        Self::write_u64(buf, n as u64)
    }

    /// Writes a signed 128 bit integer `n` to `buf`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 16`.
    ///
    /// # Examples
    ///
    /// Write and read n-byte `i128` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 16];
    /// LittleEndian::write_i128(&mut buf, -1_000_000_000);
    /// assert_eq!(-1_000_000_000, LittleEndian::read_i128(&buf));
    /// ```
    #[inline]
    fn write_i128(buf: &mut [u8], n: i128) {
        Self::write_u128(buf, n as u128)
    }

    /// Writes a signed integer `n` to `buf` using only `nbytes`.
    ///
    /// # Panics
    ///
    /// If `n` is not representable in `nbytes`, or if `nbytes` is `> 8`, then
    /// this method panics.
    ///
    /// # Examples
    ///
    /// Write and read an n-byte number in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 3];
    /// LittleEndian::write_int(&mut buf, -1_000, 3);
    /// assert_eq!(-1_000, LittleEndian::read_int(&buf, 3));
    /// ```
    #[inline]
    fn write_int(buf: &mut [u8], n: i64, nbytes: usize) {
        Self::write_uint(buf, unextend_sign(n, nbytes), nbytes)
    }

    /// Writes a signed integer `n` to `buf` using only `nbytes`.
    ///
    /// # Panics
    ///
    /// If `n` is not representable in `nbytes`, or if `nbytes` is `> 16`, then
    /// this method panics.
    ///
    /// # Examples
    ///
    /// Write and read n-length signed numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut buf = [0; 3];
    /// LittleEndian::write_int128(&mut buf, -1_000, 3);
    /// assert_eq!(-1_000, LittleEndian::read_int128(&buf, 3));
    /// ```
    #[inline]
    fn write_int128(buf: &mut [u8], n: i128, nbytes: usize) {
        Self::write_uint128(buf, unextend_sign128(n, nbytes), nbytes)
    }

    /// Writes a IEEE754 single-precision (4 bytes) floating point number.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 4`.
    ///
    /// # Examples
    ///
    /// Write and read `f32` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let e = 2.71828;
    /// let mut buf = [0; 4];
    /// LittleEndian::write_f32(&mut buf, e);
    /// assert_eq!(e, LittleEndian::read_f32(&buf));
    /// ```
    #[inline]
    fn write_f32(buf: &mut [u8], n: f32) {
        Self::write_u32(buf, n.to_bits())
    }

    /// Writes a IEEE754 double-precision (8 bytes) floating point number.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() < 8`.
    ///
    /// # Examples
    ///
    /// Write and read `f64` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let phi = 1.6180339887;
    /// let mut buf = [0; 8];
    /// LittleEndian::write_f64(&mut buf, phi);
    /// assert_eq!(phi, LittleEndian::read_f64(&buf));
    /// ```
    #[inline]
    fn write_f64(buf: &mut [u8], n: f64) {
        Self::write_u64(buf, n.to_bits())
    }

    /// Reads unsigned 16 bit integers from `src` into `dst`.
    ///
    /// # Panics
    ///
    /// Panics when `src.len() != 2*dst.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `u16` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut bytes = [0; 8];
    /// let numbers_given = [1, 2, 0xf00f, 0xffee];
    /// LittleEndian::write_u16_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0; 4];
    /// LittleEndian::read_u16_into(&bytes, &mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    fn read_u16_into(src: &[u8], dst: &mut [u16]);

    /// Reads unsigned 32 bit integers from `src` into `dst`.
    ///
    /// # Panics
    ///
    /// Panics when `src.len() != 4*dst.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `u32` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut bytes = [0; 16];
    /// let numbers_given = [1, 2, 0xf00f, 0xffee];
    /// LittleEndian::write_u32_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0; 4];
    /// LittleEndian::read_u32_into(&bytes, &mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    fn read_u32_into(src: &[u8], dst: &mut [u32]);

    /// Reads unsigned 64 bit integers from `src` into `dst`.
    ///
    /// # Panics
    ///
    /// Panics when `src.len() != 8*dst.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `u64` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut bytes = [0; 32];
    /// let numbers_given = [1, 2, 0xf00f, 0xffee];
    /// LittleEndian::write_u64_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0; 4];
    /// LittleEndian::read_u64_into(&bytes, &mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    fn read_u64_into(src: &[u8], dst: &mut [u64]);

    /// Reads unsigned 128 bit integers from `src` into `dst`.
    ///
    /// # Panics
    ///
    /// Panics when `src.len() != 16*dst.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `u128` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut bytes = [0; 64];
    /// let numbers_given = [1, 2, 0xf00f, 0xffee];
    /// LittleEndian::write_u128_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0; 4];
    /// LittleEndian::read_u128_into(&bytes, &mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    fn read_u128_into(src: &[u8], dst: &mut [u128]);

    /// Reads signed 16 bit integers from `src` to `dst`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() != 2*dst.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `i16` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut bytes = [0; 8];
    /// let numbers_given = [1, 2, 0x0f, 0xee];
    /// LittleEndian::write_i16_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0; 4];
    /// LittleEndian::read_i16_into(&bytes, &mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    #[inline]
    fn read_i16_into(src: &[u8], dst: &mut [i16]) {
        let dst = unsafe { slice::from_raw_parts_mut(dst.as_mut_ptr() as *mut u16, dst.len()) };
        Self::read_u16_into(src, dst)
    }

    /// Reads signed 32 bit integers from `src` into `dst`.
    ///
    /// # Panics
    ///
    /// Panics when `src.len() != 4*dst.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `i32` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut bytes = [0; 16];
    /// let numbers_given = [1, 2, 0xf00f, 0xffee];
    /// LittleEndian::write_i32_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0; 4];
    /// LittleEndian::read_i32_into(&bytes, &mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    #[inline]
    fn read_i32_into(src: &[u8], dst: &mut [i32]) {
        let dst = unsafe { slice::from_raw_parts_mut(dst.as_mut_ptr() as *mut u32, dst.len()) };
        Self::read_u32_into(src, dst);
    }

    /// Reads signed 64 bit integers from `src` into `dst`.
    ///
    /// # Panics
    ///
    /// Panics when `src.len() != 8*dst.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `i64` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut bytes = [0; 32];
    /// let numbers_given = [1, 2, 0xf00f, 0xffee];
    /// LittleEndian::write_i64_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0; 4];
    /// LittleEndian::read_i64_into(&bytes, &mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    #[inline]
    fn read_i64_into(src: &[u8], dst: &mut [i64]) {
        let dst = unsafe { slice::from_raw_parts_mut(dst.as_mut_ptr() as *mut u64, dst.len()) };
        Self::read_u64_into(src, dst);
    }

    /// Reads signed 128 bit integers from `src` into `dst`.
    ///
    /// # Panics
    ///
    /// Panics when `src.len() != 16*dst.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `i128` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut bytes = [0; 64];
    /// let numbers_given = [1, 2, 0xf00f, 0xffee];
    /// LittleEndian::write_i128_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0; 4];
    /// LittleEndian::read_i128_into(&bytes, &mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    #[inline]
    fn read_i128_into(src: &[u8], dst: &mut [i128]) {
        let dst = unsafe { slice::from_raw_parts_mut(dst.as_mut_ptr() as *mut u128, dst.len()) };
        Self::read_u128_into(src, dst);
    }

    /// Reads IEEE754 single-precision (4 bytes) floating point numbers from
    /// `src` into `dst`.
    ///
    /// # Panics
    ///
    /// Panics when `src.len() != 4*dst.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `f32` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut bytes = [0; 16];
    /// let numbers_given = [1.0, 2.0, 31.312e31, -11.32e19];
    /// LittleEndian::write_f32_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0.0; 4];
    /// LittleEndian::read_f32_into(&bytes, &mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    #[inline]
    fn read_f32_into(src: &[u8], dst: &mut [f32]) {
        let dst = unsafe { slice::from_raw_parts_mut(dst.as_mut_ptr() as *mut u32, dst.len()) };
        Self::read_u32_into(src, dst);
    }

    /// **DEPRECATED**.
    ///
    /// This method is deprecated. Use `read_f32_into` instead.
    /// Reads IEEE754 single-precision (4 bytes) floating point numbers from
    /// `src` into `dst`.
    ///
    /// # Panics
    ///
    /// Panics when `src.len() != 4*dst.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `f32` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut bytes = [0; 16];
    /// let numbers_given = [1.0, 2.0, 31.312e31, -11.32e19];
    /// LittleEndian::write_f32_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0.0; 4];
    /// LittleEndian::read_f32_into_unchecked(&bytes, &mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    #[inline]
    #[deprecated(since = "1.3.0", note = "please use `read_f32_into` instead")]
    fn read_f32_into_unchecked(src: &[u8], dst: &mut [f32]) {
        Self::read_f32_into(src, dst);
    }

    /// Reads IEEE754 single-precision (4 bytes) floating point numbers from
    /// `src` into `dst`.
    ///
    /// # Panics
    ///
    /// Panics when `src.len() != 8*dst.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `f64` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut bytes = [0; 32];
    /// let numbers_given = [1.0, 2.0, 31.312e211, -11.32e91];
    /// LittleEndian::write_f64_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0.0; 4];
    /// LittleEndian::read_f64_into(&bytes, &mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    #[inline]
    fn read_f64_into(src: &[u8], dst: &mut [f64]) {
        let dst = unsafe { slice::from_raw_parts_mut(dst.as_mut_ptr() as *mut u64, dst.len()) };
        Self::read_u64_into(src, dst);
    }

    /// **DEPRECATED**.
    ///
    /// This method is deprecated. Use `read_f64_into` instead.
    ///
    /// Reads IEEE754 single-precision (4 bytes) floating point numbers from
    /// `src` into `dst`.
    ///
    /// # Panics
    ///
    /// Panics when `src.len() != 8*dst.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `f64` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut bytes = [0; 32];
    /// let numbers_given = [1.0, 2.0, 31.312e211, -11.32e91];
    /// LittleEndian::write_f64_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0.0; 4];
    /// LittleEndian::read_f64_into_unchecked(&bytes, &mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    #[inline]
    #[deprecated(since = "1.3.0", note = "please use `read_f64_into` instead")]
    fn read_f64_into_unchecked(src: &[u8], dst: &mut [f64]) {
        Self::read_f64_into(src, dst);
    }

    /// Writes unsigned 16 bit integers from `src` into `dst`.
    ///
    /// # Panics
    ///
    /// Panics when `dst.len() != 2*src.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `u16` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut bytes = [0; 8];
    /// let numbers_given = [1, 2, 0xf00f, 0xffee];
    /// LittleEndian::write_u16_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0; 4];
    /// LittleEndian::read_u16_into(&bytes, &mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    fn write_u16_into(src: &[u16], dst: &mut [u8]);

    /// Writes unsigned 32 bit integers from `src` into `dst`.
    ///
    /// # Panics
    ///
    /// Panics when `dst.len() != 4*src.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `u32` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut bytes = [0; 16];
    /// let numbers_given = [1, 2, 0xf00f, 0xffee];
    /// LittleEndian::write_u32_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0; 4];
    /// LittleEndian::read_u32_into(&bytes, &mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    fn write_u32_into(src: &[u32], dst: &mut [u8]);

    /// Writes unsigned 64 bit integers from `src` into `dst`.
    ///
    /// # Panics
    ///
    /// Panics when `dst.len() != 8*src.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `u64` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut bytes = [0; 32];
    /// let numbers_given = [1, 2, 0xf00f, 0xffee];
    /// LittleEndian::write_u64_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0; 4];
    /// LittleEndian::read_u64_into(&bytes, &mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    fn write_u64_into(src: &[u64], dst: &mut [u8]);

    /// Writes unsigned 128 bit integers from `src` into `dst`.
    ///
    /// # Panics
    ///
    /// Panics when `dst.len() != 16*src.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `u128` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut bytes = [0; 64];
    /// let numbers_given = [1, 2, 0xf00f, 0xffee];
    /// LittleEndian::write_u128_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0; 4];
    /// LittleEndian::read_u128_into(&bytes, &mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    fn write_u128_into(src: &[u128], dst: &mut [u8]);

    /// Writes signed 8 bit integers from `src` into `dst`.
    ///
    /// Note that since each `i8` is a single byte, no byte order conversions
    /// are used. This method is included because it provides a safe, simple
    /// way for the caller to write from a `&[i8]` buffer. (Without this
    /// method, the caller would have to either use `unsafe` code or convert
    /// each byte to `u8` individually.)
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() != src.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `i8` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
    ///
    /// let mut bytes = [0; 4];
    /// let numbers_given = [1, 2, 0xf, 0xe];
    /// LittleEndian::write_i8_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0; 4];
    /// bytes.as_ref().read_i8_into(&mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    fn write_i8_into(src: &[i8], dst: &mut [u8]) {
        let src = unsafe { slice::from_raw_parts(src.as_ptr() as *const u8, src.len()) };
        dst.copy_from_slice(src);
    }

    /// Writes signed 16 bit integers from `src` into `dst`.
    ///
    /// # Panics
    ///
    /// Panics when `buf.len() != 2*src.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `i16` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut bytes = [0; 8];
    /// let numbers_given = [1, 2, 0x0f, 0xee];
    /// LittleEndian::write_i16_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0; 4];
    /// LittleEndian::read_i16_into(&bytes, &mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    fn write_i16_into(src: &[i16], dst: &mut [u8]) {
        let src = unsafe { slice::from_raw_parts(src.as_ptr() as *const u16, src.len()) };
        Self::write_u16_into(src, dst);
    }

    /// Writes signed 32 bit integers from `src` into `dst`.
    ///
    /// # Panics
    ///
    /// Panics when `dst.len() != 4*src.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `i32` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut bytes = [0; 16];
    /// let numbers_given = [1, 2, 0xf00f, 0xffee];
    /// LittleEndian::write_i32_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0; 4];
    /// LittleEndian::read_i32_into(&bytes, &mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    fn write_i32_into(src: &[i32], dst: &mut [u8]) {
        let src = unsafe { slice::from_raw_parts(src.as_ptr() as *const u32, src.len()) };
        Self::write_u32_into(src, dst);
    }

    /// Writes signed 64 bit integers from `src` into `dst`.
    ///
    /// # Panics
    ///
    /// Panics when `dst.len() != 8*src.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `i64` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut bytes = [0; 32];
    /// let numbers_given = [1, 2, 0xf00f, 0xffee];
    /// LittleEndian::write_i64_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0; 4];
    /// LittleEndian::read_i64_into(&bytes, &mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    fn write_i64_into(src: &[i64], dst: &mut [u8]) {
        let src = unsafe { slice::from_raw_parts(src.as_ptr() as *const u64, src.len()) };
        Self::write_u64_into(src, dst);
    }

    /// Writes signed 128 bit integers from `src` into `dst`.
    ///
    /// # Panics
    ///
    /// Panics when `dst.len() != 16*src.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `i128` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut bytes = [0; 64];
    /// let numbers_given = [1, 2, 0xf00f, 0xffee];
    /// LittleEndian::write_i128_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0; 4];
    /// LittleEndian::read_i128_into(&bytes, &mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    fn write_i128_into(src: &[i128], dst: &mut [u8]) {
        let src = unsafe { slice::from_raw_parts(src.as_ptr() as *const u128, src.len()) };
        Self::write_u128_into(src, dst);
    }

    /// Writes IEEE754 single-precision (4 bytes) floating point numbers from
    /// `src` into `dst`.
    ///
    /// # Panics
    ///
    /// Panics when `src.len() != 4*dst.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `f32` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut bytes = [0; 16];
    /// let numbers_given = [1.0, 2.0, 31.312e31, -11.32e19];
    /// LittleEndian::write_f32_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0.0; 4];
    /// LittleEndian::read_f32_into(&bytes, &mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    fn write_f32_into(src: &[f32], dst: &mut [u8]) {
        let src = unsafe { slice::from_raw_parts(src.as_ptr() as *const u32, src.len()) };
        Self::write_u32_into(src, dst);
    }

    /// Writes IEEE754 double-precision (8 bytes) floating point numbers from
    /// `src` into `dst`.
    ///
    /// # Panics
    ///
    /// Panics when `src.len() != 8*dst.len()`.
    ///
    /// # Examples
    ///
    /// Write and read `f64` numbers in little endian order:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, LittleEndian};
    ///
    /// let mut bytes = [0; 32];
    /// let numbers_given = [1.0, 2.0, 31.312e211, -11.32e91];
    /// LittleEndian::write_f64_into(&numbers_given, &mut bytes);
    ///
    /// let mut numbers_got = [0.0; 4];
    /// LittleEndian::read_f64_into(&bytes, &mut numbers_got);
    /// assert_eq!(numbers_given, numbers_got);
    /// ```
    fn write_f64_into(src: &[f64], dst: &mut [u8]) {
        let src = unsafe { slice::from_raw_parts(src.as_ptr() as *const u64, src.len()) };
        Self::write_u64_into(src, dst);
    }

    /// Converts the given slice of unsigned 16 bit integers to a particular
    /// endianness.
    ///
    /// If the endianness matches the endianness of the host platform, then
    /// this is a no-op.
    ///
    /// # Examples
    ///
    /// Convert the host platform's endianness to big-endian:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, BigEndian};
    ///
    /// let mut numbers = [5, 65000];
    /// BigEndian::from_slice_u16(&mut numbers);
    /// assert_eq!(numbers, [5u16.to_be(), 65000u16.to_be()]);
    /// ```
    fn from_slice_u16(numbers: &mut [u16]);

    /// Converts the given slice of unsigned 32 bit integers to a particular
    /// endianness.
    ///
    /// If the endianness matches the endianness of the host platform, then
    /// this is a no-op.
    ///
    /// # Examples
    ///
    /// Convert the host platform's endianness to big-endian:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, BigEndian};
    ///
    /// let mut numbers = [5, 65000];
    /// BigEndian::from_slice_u32(&mut numbers);
    /// assert_eq!(numbers, [5u32.to_be(), 65000u32.to_be()]);
    /// ```
    fn from_slice_u32(numbers: &mut [u32]);

    /// Converts the given slice of unsigned 64 bit integers to a particular
    /// endianness.
    ///
    /// If the endianness matches the endianness of the host platform, then
    /// this is a no-op.
    ///
    /// # Examples
    ///
    /// Convert the host platform's endianness to big-endian:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, BigEndian};
    ///
    /// let mut numbers = [5, 65000];
    /// BigEndian::from_slice_u64(&mut numbers);
    /// assert_eq!(numbers, [5u64.to_be(), 65000u64.to_be()]);
    /// ```
    fn from_slice_u64(numbers: &mut [u64]);

    /// Converts the given slice of unsigned 128 bit integers to a particular
    /// endianness.
    ///
    /// If the endianness matches the endianness of the host platform, then
    /// this is a no-op.
    ///
    /// # Examples
    ///
    /// Convert the host platform's endianness to big-endian:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, BigEndian};
    ///
    /// let mut numbers = [5, 65000];
    /// BigEndian::from_slice_u128(&mut numbers);
    /// assert_eq!(numbers, [5u128.to_be(), 65000u128.to_be()]);
    /// ```
    fn from_slice_u128(numbers: &mut [u128]);

    /// Converts the given slice of signed 16 bit integers to a particular
    /// endianness.
    ///
    /// If the endianness matches the endianness of the host platform, then
    /// this is a no-op.
    ///
    /// # Examples
    ///
    /// Convert the host platform's endianness to big-endian:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, BigEndian};
    ///
    /// let mut numbers = [5, 6500];
    /// BigEndian::from_slice_i16(&mut numbers);
    /// assert_eq!(numbers, [5i16.to_be(), 6500i16.to_be()]);
    /// ```
    #[inline]
    fn from_slice_i16(src: &mut [i16]) {
        let src = unsafe { slice::from_raw_parts_mut(src.as_mut_ptr() as *mut u16, src.len()) };
        Self::from_slice_u16(src);
    }

    /// Converts the given slice of signed 32 bit integers to a particular
    /// endianness.
    ///
    /// If the endianness matches the endianness of the host platform, then
    /// this is a no-op.
    ///
    /// # Examples
    ///
    /// Convert the host platform's endianness to big-endian:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, BigEndian};
    ///
    /// let mut numbers = [5, 65000];
    /// BigEndian::from_slice_i32(&mut numbers);
    /// assert_eq!(numbers, [5i32.to_be(), 65000i32.to_be()]);
    /// ```
    #[inline]
    fn from_slice_i32(src: &mut [i32]) {
        let src = unsafe { slice::from_raw_parts_mut(src.as_mut_ptr() as *mut u32, src.len()) };
        Self::from_slice_u32(src);
    }

    /// Converts the given slice of signed 64 bit integers to a particular
    /// endianness.
    ///
    /// If the endianness matches the endianness of the host platform, then
    /// this is a no-op.
    ///
    /// # Examples
    ///
    /// Convert the host platform's endianness to big-endian:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, BigEndian};
    ///
    /// let mut numbers = [5, 65000];
    /// BigEndian::from_slice_i64(&mut numbers);
    /// assert_eq!(numbers, [5i64.to_be(), 65000i64.to_be()]);
    /// ```
    #[inline]
    fn from_slice_i64(src: &mut [i64]) {
        let src = unsafe { slice::from_raw_parts_mut(src.as_mut_ptr() as *mut u64, src.len()) };
        Self::from_slice_u64(src);
    }

    /// Converts the given slice of signed 128 bit integers to a particular
    /// endianness.
    ///
    /// If the endianness matches the endianness of the host platform, then
    /// this is a no-op.
    ///
    /// # Examples
    ///
    /// Convert the host platform's endianness to big-endian:
    ///
    /// ```rust
    /// use byteorder::{ByteOrder, BigEndian};
    ///
    /// let mut numbers = [5, 65000];
    /// BigEndian::from_slice_i128(&mut numbers);
    /// assert_eq!(numbers, [5i128.to_be(), 65000i128.to_be()]);
    /// ```
    #[inline]
    fn from_slice_i128(src: &mut [i128]) {
        let src = unsafe { slice::from_raw_parts_mut(src.as_mut_ptr() as *mut u128, src.len()) };
        Self::from_slice_u128(src);
    }

    /// Converts the given slice of IEEE754 single-precision (4 bytes) floating
    /// point numbers to a particular endianness.
    ///
    /// If the endianness matches the endianness of the host platform, then
    /// this is a no-op.
    fn from_slice_f32(numbers: &mut [f32]);

    /// Converts the given slice of IEEE754 double-precision (8 bytes) floating
    /// point numbers to a particular endianness.
    ///
    /// If the endianness matches the endianness of the host platform, then
    /// this is a no-op.
    fn from_slice_f64(numbers: &mut [f64]);
}

/// Defines big-endian serialization.
///
/// Note that this type has no value constructor. It is used purely at the
/// type level.
///
/// # Examples
///
/// Write and read `u32` numbers in big endian order:
///
/// ```rust
/// use byteorder::{ByteOrder, BigEndian};
///
/// let mut buf = [0; 4];
/// BigEndian::write_u32(&mut buf, 1_000_000);
/// assert_eq!(1_000_000, BigEndian::read_u32(&buf));
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BigEndian {}

impl Default for BigEndian {
    fn default() -> BigEndian {
        panic!("BigEndian default")
    }
}

/// A type alias for [`BigEndian`].
///
/// [`BigEndian`]: enum.BigEndian.html
pub type BE = BigEndian;

/// Defines little-endian serialization.
///
/// Note that this type has no value constructor. It is used purely at the
/// type level.
///
/// # Examples
///
/// Write and read `u32` numbers in little endian order:
///
/// ```rust
/// use byteorder::{ByteOrder, LittleEndian};
///
/// let mut buf = [0; 4];
/// LittleEndian::write_u32(&mut buf, 1_000_000);
/// assert_eq!(1_000_000, LittleEndian::read_u32(&buf));
/// ```
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LittleEndian {}

impl Default for LittleEndian {
    fn default() -> LittleEndian {
        panic!("LittleEndian default")
    }
}

/// A type alias for [`LittleEndian`].
///
/// [`LittleEndian`]: enum.LittleEndian.html
pub type LE = LittleEndian;

/// Defines network byte order serialization.
///
/// Network byte order is defined by [RFC 1700][1] to be big-endian, and is
/// referred to in several protocol specifications.  This type is an alias of
/// [`BigEndian`].
///
/// [1]: https://tools.ietf.org/html/rfc1700
///
/// Note that this type has no value constructor. It is used purely at the
/// type level.
///
/// # Examples
///
/// Write and read `i16` numbers in big endian order:
///
/// ```rust
/// use byteorder::{ByteOrder, NetworkEndian, BigEndian};
///
/// let mut buf = [0; 2];
/// BigEndian::write_i16(&mut buf, -5_000);
/// assert_eq!(-5_000, NetworkEndian::read_i16(&buf));
/// ```
///
/// [`BigEndian`]: enum.BigEndian.html
pub type NetworkEndian = BigEndian;

/// Defines system native-endian serialization.
///
/// Note that this type has no value constructor. It is used purely at the
/// type level.
///
/// On this platform, this is an alias for [`LittleEndian`].
///
/// [`LittleEndian`]: enum.LittleEndian.html
#[cfg(target_endian = "little")]
pub type NativeEndian = LittleEndian;

/// Defines system native-endian serialization.
///
/// Note that this type has no value constructor. It is used purely at the
/// type level.
///
/// On this platform, this is an alias for [`BigEndian`].
///
/// [`BigEndian`]: enum.BigEndian.html
#[cfg(target_endian = "big")]
pub type NativeEndian = BigEndian;
