use core::slice;

use abio::integer::U32;
use abio::Codec;
use abio::{Abi, Endian};

include!("../resources/test_helpers.rs");

fn generate_int() -> U32 {
    let mut buf = [0u8; U32::SIZE];
    getrandom::getrandom(&mut buf).unwrap();
    U32::read_aligned(&buf, Endian::Little).expect("failed to decode little endian U32")
}

#[test]
// #[cfg(not(miri))]
fn converting_to_byte_array() {
    use std::thread;

    use abio::{BytesOf, Chunk};

    #[inline]
    pub const unsafe fn read_byte_array_unchecked<const SIZE: usize>(
        bytes: &[u8],
        offset: usize,
    ) -> [u8; SIZE] {
        let data = bytes.as_ptr().add(offset);
        let raw_slice = slice::from_raw_parts(data, SIZE);
        raw_slice.as_ptr().cast::<[u8; SIZE]>().read()
    }

    let codec = Codec::default();

    let handle1 = thread::spawn(|| {
        let mut array = [[0u8; 4]; 1024 * 8];
        array.iter_mut().for_each(|item| *item = generate_int().to_le_bytes());
        array
    });

    let handle2 = thread::spawn(|| {
        let mut array = [[0u8; 4]; 1024 * 8];
        array.iter_mut().for_each(|item| *item = generate_int().to_le_bytes());
        array
    });

    let out1 = handle1.join().expect("failed to join handle1 with current thread");
    let out2 = handle2.join().expect("failed to join handle1 with current thread");
    let output = [out1, out2].concat();

    for (index, value) in output.into_iter().enumerate() {
        let chunk = Chunk::from(value);

        let value = U32::read_aligned(chunk.as_bytes(), codec.endian())
            .expect("U32 could not run decode with given Codec");
        let value2 =
            U32::from_le_bytes(unsafe { read_byte_array_unchecked::<4>(chunk.as_bytes(), 0) });
        assert!(!value.bytes_of().is_empty(), "index {index} has an empty value.");
        assert_eq!(
            value.get(Endian::Little),
            value2.get_le(),
            "{value} != {value2}; expected both values to be equal"
        );
    }
}
