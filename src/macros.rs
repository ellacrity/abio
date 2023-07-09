// /// Reads an aligned integer from a slice of bytes, given an `Endianness`, a
// reader /// function and a target, or `to` type.
// macro_rules! read_aligned {
//     (
//         $bytes:expr, $aligned:tt->$method:tt<$endianness:ty>() $(,)?
//     ) => {{ let size = ::core::mem::size_of::<$aligned>(); if $bytes.len() < size
//       { return Err($crate::error::SizeMismatchError::new(size, $bytes.len())); }

//         let value = ::byteorder::ByteOrder::<$endianness>::$method($bytes);
//         if $bytes.is_ptr_aligned::<$aligned>() {
//             Ok(<$aligned>::new(value))
//         } else {
//             Err($crate::error::SizeMismatchError::new(size, $bytes.len()))
//         }
//     }};
//     (
//         $bytes:expr, $aligned:tt->$method:tt $(,)?
//     ) => {{ let size = ::core::mem::size_of::<$aligned>(); if $bytes.len() < size
//       { return Err($crate::error::SizeMismatchError::new(size, $bytes.len())); }

//         let value = ::byteorder::LittleEndian::$method($bytes);
//         if $bytes.is_ptr_aligned::<$aligned>() {
//             Ok(<$aligned>::new(value))
//         } else {
//             Err($crate::error::SizeMismatchError::new(size, $bytes.len()))
//         }
//     }};
//     () => {};
// }

#[macro_export]
macro_rules! assert_preconditions {
    ($bytes:ident, $ty:ty) => {{
        let _: () = assert!(!$bytes.is_empty());
        let _: () = assert!($bytes.len() >= core::mem::size_of::<$ty>());
    }};
}
