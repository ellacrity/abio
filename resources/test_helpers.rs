pub fn gen_random_bytes<const LEN: usize>() -> [u8; LEN] {
    let mut buf = [0u8; LEN];

    getrandom::getrandom(&mut buf)
        .expect("getrandom should have filled a buffer with random bytes.");
    buf
}
