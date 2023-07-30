#![allow(unused)]

extern crate abio;

// Provides some test helper functions, such as `gen_random_bytes`.
include!("../resources/test_helpers.rs");
include!("../resources/link_dylib.rs");

use core::mem::size_of;

use abio::{integer::*, Abi, Bytes, Codec, Decode, Limit};
use abio::{BytesOf, Result, Zeroable};

#[repr(C)]
#[derive(Abi, BytesOf, Copy, Clone, Debug, Zeroable)]
pub struct ImageDosHeader {
    pub e_magic: u16,
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    pub e_res: [u16; 4],
    pub e_oemid: u16,
    pub e_oeminfo: u16,
    pub e_res2: [u16; 10],
    pub e_lfanew: i32,
}

impl Decode for ImageDosHeader {
    fn decode(bytes: &[u8], codec: Codec) -> Result<Self> {
        assert!(!bytes.is_empty());
        let mut pos = 0;

        let e_magic = u16::decode(bytes, codec)?;
        pos += e_magic.size();

        let e_cblp = u16::decode(&bytes[pos..], codec)?;
        pos += e_cblp.size();
        let e_cp = u16::decode(&bytes[pos..], codec)?;
        pos += e_cp.size();
        let e_crlc = u16::decode(&bytes[pos..], codec)?;
        pos += e_crlc.size();
        let e_cparhdr = u16::decode(&bytes[pos..], codec)?;
        pos += e_cparhdr.size();
        let e_minalloc = u16::decode(&bytes[pos..], codec)?;
        pos += e_minalloc.size();
        let e_maxalloc = u16::decode(&bytes[pos..], codec)?;
        pos += e_maxalloc.size();
        let e_ss = u16::decode(&bytes[pos..], codec)?;
        pos += e_ss.size();
        let e_sp = u16::decode(&bytes[pos..], codec)?;
        pos += e_sp.size();
        let e_csum = u16::decode(&bytes[pos..], codec)?;
        pos += e_csum.size();
        let e_ip = u16::decode(&bytes[pos..], codec)?;
        pos += e_ip.size();
        let e_cs = u16::decode(&bytes[pos..], codec)?;
        pos += e_cs.size();
        let e_lfarlc = u16::decode(&bytes[pos..], codec)?;
        pos += e_lfarlc.size();
        let e_ovno = u16::decode(&bytes[pos..], codec)?;
        pos += e_ovno.size();

        let e_res = <[u16; 4]>::decode(&bytes[pos..], codec)?;
        pos += e_res.size();
        let e_oemid = u16::decode(&bytes[pos..], codec)?;
        pos += e_oemid.size();
        let e_oeminfo = u16::decode(&bytes[pos..], codec)?;
        pos += e_oeminfo.size();
        let e_res2 = <[u16; 10]>::decode(&bytes[pos..], codec)?;
        pos += e_res2.size();

        let e_lfanew = i32::decode(&bytes[pos..], codec)?;
        pos += e_lfanew.size();

        assert_eq!(
            pos,
            ImageDosHeader::SIZE,
            "Cursor (pos) should have advanced to same size as IMAGE_DOS_HEADER"
        );

        Ok(ImageDosHeader {
            e_magic,
            e_cblp,
            e_cp,
            e_crlc,
            e_cparhdr,
            e_minalloc,
            e_maxalloc,
            e_ss,
            e_sp,
            e_csum,
            e_ip,
            e_cs,
            e_lfarlc,
            e_ovno,
            e_res,
            e_oemid,
            e_oeminfo,
            e_res2,
            e_lfanew,
        })
    }
}

fn display_state<T: Abi>(pos: usize) {
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

#[test]
fn safe_transmute_checks_layout() {
    let bytes = &LIBRARY_BYTES[..0x1000];

    let codec =
        Codec::builder().with_little_endian().with_limit(Limit::default()).try_build().unwrap();

    println!(
        "\nDecoding header with size | alignment:\n{} | {}",
        ImageDosHeader::SIZE,
        ImageDosHeader::ALIGN
    );
    println!("Using bytes: {:?}\nwith codec: {:?}", &bytes[..ImageDosHeader::SIZE], codec);
    let header =
        ImageDosHeader::decode(bytes, codec).expect("failed to parse IMAGE_DOS_HEADER from bytes");
    assert_eq!(
        header.e_magic,
        u16::from_le_bytes(*b"MZ"),
        "IMAGE_DOS_HEADER size does not match cursor position"
    );

    dbg!(header);
}
