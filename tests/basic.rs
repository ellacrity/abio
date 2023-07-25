extern crate abio;

// Provides some test helper functions, such as `gen_random_bytes`.
include!("../resources/test_helpers.rs");
include!("../resources/link_dylib.rs");

use core::mem::size_of;

use abio::config::{Config, LittleEndian, LE};
use abio::{integer::*, Bytes};
use abio::{BytesOf, Decode, Memory, Result, Zeroable};

#[repr(C)]
#[derive(Abi, AsBytes, Clone, Copy, Debug, Default, Zeroable)]
pub struct Header {
    prefix: U16,
    length: U16,
    magic: [u8; 4],
    exe_offset: I32,
}

impl Header {
    pub fn parse<E: Config>(bytes: &[u8], offset: usize) -> Result<(Self, usize)> {
        let bytes = Bytes::new(bytes);

        let mut pos = offset;
        display_state::<U16>(pos);
        let prefix = E::read_u16(bytes)?;
        pos += prefix.size();

        display_state::<U16>(pos);
        let length = E::read_u16(&bytes[pos..])?;
        pos += length.size();

        display_state::<U32>(pos);
        let magic = E::read_u32(&bytes[pos..])?;
        pos += magic.size();

        display_state::<I32>(pos);
        let exe_offset = E::read_i32(&bytes[pos..])?;
        pos += exe_offset.size();

        Ok((Header { prefix, length, magic: magic.to_le_bytes(), exe_offset }, pos))
    }
}

fn display_state<T: Memory>(pos: usize) {
    println!(
        "\nPosition in bytes: {}\nReading next {} bytes into {} (alignment: {})",
        pos,
        size_of::<T>(),
        core::any::type_name::<T>(),
        T::ALIGN
    );
}

const LIBRARY_BYTES_RAW: &[u8] = include_bytes!("../resources/ntdll.dll");
const LIB_BYTES_LEN: usize = LIBRARY_BYTES_RAW.len();

#[link_section = ".text"]
static LIBRARY_BYTES: [u8; LIB_BYTES_LEN] = *include_bytes!("../resources/ntdll.dll");

// const HEADER_BYTES: [u8; 12] =
// *b"\x4d\x5a\x00\x0C\x00\x00\x45\x50\x00\x00\x00\xe8";

#[test]
fn safe_transmute_checks_layout() {
    let bytes = Bytes::new(&LIBRARY_BYTES[..]);

    let prefix = U16::decode::<LE>(&bytes[..2]);
    assert_eq!(prefix, Ok(U16::new(23117)));
    println!("prefix: {prefix:?}");

    println!("\nDecoding header with size | alignment:\n{} | {}", Header::SIZE, Header::ALIGN);
    let (header, pos) =
        Header::parse::<LE>(bytes.bytes_of(), 0).expect("failed to parse Header from bytes");
    assert_eq!(pos, header.size(), "Header size does not match cursor position");

    dbg!(header);
}
