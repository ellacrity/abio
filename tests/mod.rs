extern crate aligned;

pub use aligned::integral::*;

mod integration;
pub use integration::Packet;

#[cfg(test)]
mod tests {
    use aligned::{Bytes, Pod};

    use super::*;

    #[test]
    fn bytes_can_be_constructed_from_any_byteslice() {
        let data = include_bytes!("../scratchpad/bytes.txt");
        println!("{}", String::from_utf8_lossy(&data[..]));

        let data = &data[..];

        let bytes = Bytes::new(data);
        let bytes = bytes.chunk().get(..Packet::SIZE);
        dbg!(bytes);
    }
}
