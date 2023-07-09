extern crate aligned_rs as aligned;

pub use aligned::integral::*;

mod integration;
pub use integration::Packet;

#[cfg(test)]
mod tests {

    use aligned::{Aligned, Bytes};

    use super::*;

    #[test]
    fn bytes_can_be_constructed_from_any_byteslice() {
        let data = include_bytes!("../scratchpad/traits.txt");
        println!("{}", String::from_utf8_lossy(&data[..]));

        let data = &data[..];

        let bytes = Bytes::new(data);
        let bytes = bytes.chunk().get(..Packet::SIZE);
        dbg!(bytes);

        let bytes = include_bytes!("./integration.rs");

        let target = std::env::var("CARGO_BUILD_TARGET").unwrap();
        println!("target: {target}");
    }
}
