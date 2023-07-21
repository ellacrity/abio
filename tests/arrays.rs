use core::slice;
use std::sync::OnceLock;
use std::time::SystemTime;

use abio::integer::U128;

include!("../resources/test_helpers.rs");

pub struct TimeGuard {
    inner: SystemTime,
}

impl TimeGuard {
    pub fn new() -> Self {
        let inner = SystemTime::now();
        Self { inner }
    }

    pub fn elapsed(&self) -> U128 {
        U128::new(
            self.inner
                .elapsed()
                .expect("SystemTime should produce an elapsed value since creation")
                .as_millis(),
        )
    }
}

impl Default for TimeGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for TimeGuard {
    fn drop(&mut self) {
        let elapsed = self.elapsed();
        println!("TimeGuard dropped after: {elapsed}ms");
    }
}

#[test]
fn converting_to_byte_array() {
    pub static DATA: OnceLock<[[u8; 256]; 1000]> = OnceLock::new();

    #[inline]
    pub const unsafe fn read_byte_array_unchecked<const SIZE: usize>(
        bytes: &[u8],
        offset: usize,
    ) -> [u8; SIZE] {
        let data = bytes.as_ptr().add(offset);
        let raw_slice = slice::from_raw_parts(data, SIZE);
        raw_slice.as_ptr().cast::<[u8; SIZE]>().read()
    }

    let timer = SystemTime::now();
    let data = DATA.get_or_init(|| {
        let mut array = [[0u8; 256]; 1000];
        array.iter_mut().for_each(|item| *item = gen_random_bytes::<256>());
        array
    });

    for (_, array) in data.iter().enumerate() {
        let array = unsafe { read_byte_array_unchecked::<128>(array, 64) };
        assert_eq!(array.len(), 128);
    }
    println!("Elapsed: {}ms", timer.elapsed().unwrap().as_millis());
}
