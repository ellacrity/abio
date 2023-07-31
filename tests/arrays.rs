#![cfg(miri)]

use abio::integer::U32;
use abio::{Codec, Endian};

fn gen_aligned_u32() -> U32 {
    let mut buf = [0u8; 4];
    getrandom::getrandom(&mut buf).expect("getrandom failed to fill the buffer.");
    U32::read_aligned(&buf, Codec::default()).expect("failed to decode little endian U32")
}

// ISSUE #3: Move this to benchmarking suite. https://github.com/ellacrity/abio/issues/3

#[test]
fn converting_to_byte_array() {
    use std::thread;

    use abio::{BytesOf, Chunk};

    let codec = Codec::default();

    let handle1 = thread::spawn(|| {
        let mut array = [[0u8; 4]; 1024 * 8];
        array.iter_mut().for_each(|item| *item = gen_aligned_u32().to_le_bytes());
        array
    });

    let handle2 = thread::spawn(|| {
        let mut array = [[0u8; 4]; 1024 * 8];
        array.iter_mut().for_each(|item| *item = gen_aligned_u32().to_le_bytes());
        array
    });

    let out1 = handle1.join().expect("failed to join handle1 with current thread");
    let out2 = handle2.join().expect("failed to join handle1 with current thread");
    let output = [out1, out2].concat();

    for (index, value) in output.into_iter().enumerate() {
        let chunk = Chunk::from(value);

        let value = U32::read_aligned(chunk.as_bytes(), codec)
            .expect("U32 could not run decode with given Codec");
        let value2 = U32::from_le_bytes(chunk.into_array());
        assert!(!value.bytes_of().is_empty(), "index {index} has an empty value.");
        assert_eq!(
            value.get(Endian::Little),
            value2.get_le(),
            "{value} != {value2}; expected both values to be equal"
        );
    }
}
