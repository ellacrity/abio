extern crate aligned_rs as aligned;

use aligned::integral::{U16, U32};
use aligned::{Aligned, Decodable, Result, Zeroable};

#[repr(C)]
// #[derive(aligned_derive::Aligned, aligned_derive::Zeroable, aligned_derive::Decodable)]
#[derive(Clone, Copy, Debug)]
pub struct Packet {
    header: U32,
    length: U16,
    tag: U16,
    body: [u8; 64],
}

impl Packet {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let (parser, tail) = Self::read_bytes(bytes)?;
        assert!(tail.is_empty());
        Ok(parser)
    }

    pub fn header(&self) -> U32 {
        self.header
    }
}

unsafe impl Aligned for Packet {}
unsafe impl Zeroable for Packet {}

impl Decodable for Packet {
    fn read_bytes(bytes: &[u8]) -> aligned::Result<(Self, &[u8])> {
        let mut pos = 0;
        let (header, tail) = U32::read_bytes(bytes)?;
        pos += 4;

        let (length, tail) = U16::read_bytes(tail)?;
        pos += 2;

        let (tag, tail) = U16::read_bytes(tail)?;
        pos += 2;

        let position = U32::SIZE + U16::SIZE + U16::SIZE;
        assert_eq!(pos, position);
        println!("position: {position}\nLength: {length}");
        let body = bytes[position..].try_into().map_err(|err| {
            eprintln!("Cannot convert bytes into array, error: {err:?}");
            aligned::Error::size_mismatch(position + length.get() as usize - position, position)
        })?;
        dbg!(body, tail);
        Ok((Self { header, length, tag, body }, tail))
    }
}

#[cfg(test)]
mod tests {
    use aligned::is_without_padding;

    use super::*;

    #[test]
    fn safe_transmute_checks_layout() {
        let header = U32::new(0xff80_08c8);
        let length = U16::new(32);
        let tag =
            U16::from_bytes(b"MZ").expect("buffer should simply contain MZ, which is valid U16");
        let body = *b"message body is of expected len\x08\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        let packet = Packet { header, length, tag, body };
        dbg!(packet);

        let has_padding = is_without_padding!(struct -> Packet, U32, U16, U16, [u8; 64]);
        assert!(!has_padding);

        assert_eq!(packet.header(), 4286580936.into());

        let mut buf = Vec::new();
        buf.extend_from_slice(&header.to_le_bytes()[..]);
        buf.extend_from_slice(&length.to_le_bytes()[..]);
        buf.extend_from_slice(&tag.to_le_bytes()[..]);
        buf.extend_from_slice(&body[..]);
        dbg!(buf.len());

        let (packet, _) =
            Packet::read_bytes(&buf[..]).expect("Packet should be readable from bytes");

        assert!(!packet.body.is_empty());
    }

    #[test]
    fn creating_packet_directly_from_bytes() {
        const BUFFER_SIZE: usize = Packet::SIZE;
        let mut buf = [0u8; BUFFER_SIZE];

        getrandom::getrandom(&mut buf)
            .expect("getrandom should have filled a buffer with random bytes.");

        println!("{:?}", buf);
        let packet = Packet::parse(&buf[..]).expect(
            "failed to parse Packet
    from random bytes",
        );
        dbg!(packet);
    }
}
