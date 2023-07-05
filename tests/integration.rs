use aligned::integral::{U16, U32};
use aligned::{AsBytes, FromBytes, Parser, Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Packet {
    header: U32,
    length: U16,
    tag: U16,
    body: [u8; 64],
}

impl Packet {
    pub fn parse(bytes: &[u8]) -> Option<Self> {
        let mut parser = Parser::new(bytes);
        let packet = parser.read::<Packet>()?;
        Some(packet)
    }

    pub fn header(&self) -> U32 {
        self.header
    }
}

unsafe impl AsBytes for Packet {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of_val(self),
            )
        }
    }
}

unsafe impl Pod for Packet {}
unsafe impl Zeroable for Packet {}

unsafe impl FromBytes for Packet {
    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut pos = 0;
        let header = unsafe { bytes[pos..].as_ptr().cast::<U32>().read() };
        pos += U32::SIZE;
        dbg!(header, pos);

        let length = U16::from_bytes(&bytes[pos..])?;
        pos += U16::SIZE;
        dbg!(length, pos);

        let tag = U16::from_bytes(&bytes[pos..])?;
        pos += U16::SIZE;
        dbg!(tag, pos);

        let body = bytes[pos..pos + length.get() as usize].try_into().ok()?;
        dbg!(body, pos);
        Some(Self { header, length, tag, body })
    }
}

#[cfg(test)]
mod tests {
    use aligned::{struct_has_padding, AsBytes, FromBytes};

    use super::*;

    #[test]
    fn safe_transmute_checks_layout() {
        let packet = Packet {
            header: U32::new(0xff80_08c8),
            length: U16::new(32),
            tag: U16::from_bytes(b"MZ").expect("buffer should simply contain MZ, which is valid U16"),
            body: *b"message body is of expected len\x08\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        };
        dbg!(packet);

        let has_padding = struct_has_padding!(Packet, U32, U16, U16, [u8; 64]);
        assert!(!has_padding);

        assert_eq!(packet.header(), 4286580936.into());

        let bytes = packet.as_bytes();
        Packet::from_bytes(bytes);
    }

    // #[test]
    // fn creating_packet_directly_from_bytes() {
    //     const BUFFER_SIZE: usize = Packet::SIZE;
    //     let mut buffer = [0u8; BUFFER_SIZE];

    //     println!("{:?}", buffer);
    //     let packet = Packet::parse(&buffer[..]).expect("failed to parse Packet
    // from random bytes");     dbg!(packet);
    // }
}
