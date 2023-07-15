extern crate abio;

// Provides some test helper functions, such as `gen_random_bytes`.
include!("../resources/test_helpers.rs");

use core::slice;
use std::mem::size_of;

use abio::signed::I32;
use abio::unsigned::*;
use abio::{
    Abi, Bytes, BytesExt, Deserialize, Deserializer, Error, LittleEndian, Result, Source, Zeroable,
    LE,
};

#[repr(C)]
#[derive(Abi, Clone, Copy, Debug, Zeroable)]
pub struct Header {
    magic: U32,
    len: U16,
    tag: U16,
    exe_offset: I32,
}

impl Header {
    pub fn new(bytes: &[u8], offset: usize) -> Result<Self> {
        Ok(Self { magic, len, tag, exe_offset })
    }
}

impl Deserialize<LE> for Header {
    fn deserialize(bytes: &[u8], endian: LE) -> Result<Self> {
        let saved = Bytes::new(bytes);
        let x = saved.read_slice(0, u32::SIZE)?.as_chunk();
        let mut pos = 0;

        let magic = endian.decode_u32(bytes);
        pos += u32::SIZE;

        let bytes = &bytes[pos..];

        let len = endian.decode_u16(bytes);

        pos += u32::SIZE;
        let bytes = &bytes[pos..];

        let bytes = advance::<u16>(bytes, pos)?;

        let tag = endian.decode_u16(bytes);
        let bytes = advance::<u16>(bytes, pos)?;

        let exe_offset = endian.deserialize_i32(bytes);
        let bytes = advance::<i32>(bytes, pos)?;
        assert!(bytes.len() < saved.len());
        let header = Header {
            magic: magic.into(),
            len: len.into(),
            tag: tag.into(),
            exe_offset: exe_offset.into(),
        };
        Ok(header)
    }
}

fn advance<T>(bytes: &[u8], mut pos: usize) -> Result<&[u8]> {
    let size = size_of::<T>();
    pos += size;
    bytes.get(pos..).ok_or_else(|| Error::out_of_bounds(pos, bytes.len()))
}

#[test]
fn safe_transmute_checks_layout() {
    let data = *b"MZ\x45\x50\x00\x00\x20\x00\x4d\x5a\xe8\x00\x00\x00";
    // let mut pos = 0;

    // let header = U32::from_bytes(&data).unwrap();
    // assert_eq!(header, 0x00004550.into());
    // pos += U32::SIZE;

    // let length = U16::from_bytes(&data[pos..]).unwrap();
    // assert_eq!(length.get(), 0x20);
    // pos += U16::SIZE;

    // let tag = U16::from_bytes(&data[pos..]).unwrap();
    // assert_eq!(tag.get(), 0x5a4d);
    // pos += U16::SIZE;

    // let body = <[u8; 32]>::try_from(&data[pos..])
    //     .expect("body should have been read since it has no layout requirements except
    // size"); assert_eq!(&body, &b"message body has expected length"[..]);

    // FIXME: parse not currently working due to bug / fixable issue
    let parsed = Header::from_bytes(&data[..])?;
    let header = Header::deserialize(&data[..], Aligned::Little)
        .expect("Packet should be readable from bytes");
    assert!(tail.is_empty());

    assert!(header.is_valid_align());
    assert!(header.verify_layout());
}

#[test]
fn creating_packet_directly_from_bytes() {
    const BUFFER_SIZE: usize = Header::SIZE;

    let buf = gen_random_bytes::<BUFFER_SIZE>();

    println!("{:?}", buf);
    let packet = Header::parse(&buf[..]).expect(
        "failed to parse Packet
    from random bytes",
    );
    dbg!(packet);
}
