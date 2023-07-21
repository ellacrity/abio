extern crate abio;

// Provides some test helper functions, such as `gen_random_bytes`.
include!("../resources/test_helpers.rs");

use abio::bytes::Bytes;
use abio::endian::{Endian, LittleEndian, LE};
use abio::integer::*;
use abio::{Abi, Decode, Result, Span, Zeroable};

#[repr(C)]
#[derive(Abi, Clone, Copy, Debug, Zeroable)]
pub struct Header {
    magic: U32,
    len: U16,
    tag: U16,
    exe_offset: I32,
}

impl Header {
    pub fn new(bytes: &[u8], offset: usize) -> Result<(Self, usize)> {
        let mut pos = offset;
        let magic = LittleEndian::read_u32(bytes, pos)?;
        pos += magic.runtime_size();

        let len = LittleEndian::read_u16(bytes, pos)?;
        pos += len.runtime_size();

        let tag = LittleEndian::read_u16(bytes, pos)?;
        pos += tag.runtime_size();

        let exe_offset = LittleEndian::read_i32(bytes, pos)?;
        pos += exe_offset.runtime_size();

        Ok((Self { magic, len, tag, exe_offset }, pos))
    }
}

impl Decode for Header {
    type Offset = Span;

    fn decode<E: Endian>(source: &[u8], offset: Self::Offset, endian: E) -> Result<Self> {
        println!("Decoding header.");
        let mut pos = offset;

        println!("Decoding U32");

        // figure out if there is a hack that lets us abuse type system to encode / decode
        // BE/LE and use all params. **needs to be endian-aware**
        let magic = LittleEndian::read_u32(source, offset.start())?;
        pos += 4;

        let len = U16::decode(source, pos, endian)?;
        pos += 2;

        let tag = LE::read_u16(source, pos.start())?;
        pos += 2;

        let exe_offset = I32::decode(source, pos, endian)?;
        pos += 4;

        let header = Header { magic, len, tag, exe_offset };
        Ok(header)
    }
}

#[test]
fn safe_transmute_checks_layout() {
    let data = &b"MZ\x45\x50\x00\x00\x20\x00\x4d\x5a\xe8\x00\x00\x00"[..];
    let header = Header::decode(data, Span::new(0, data.len()), LittleEndian)
        .expect("Packet should be readable from bytes");

    dbg!(header);
}

#[test]
fn creating_header_directly_from_bytes() {
    const BUFFER_SIZE: usize = Header::SIZE;

    let buf = gen_random_bytes::<BUFFER_SIZE>();

    println!("{:?}", buf);
    let packet = Header::decode(&buf[..], 0.into(), LittleEndian)
        .expect("failed to parse Packet from random bytes");
    dbg!(packet);
}
