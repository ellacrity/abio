#![allow(unused)]

extern crate abio;

use core::mem::size_of;

use abio::{integer::*, Abi, Codec, Decode, Limit, Slice};
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

// Bytes originating from a dynamic library with the PE file format.
const LIBRARY_BYTES: &[u8] = &[
    77, 90, 144, 0, 3, 0, 0, 0, 4, 0, 0, 0, 255, 255, 0, 0, 184, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    232, 0, 0, 0, 14, 31, 186, 14, 0, 180, 9, 205, 33, 184, 1, 76, 205, 33, 84, 104, 105, 115, 32,
    112, 114, 111, 103, 114, 97, 109, 32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 114, 117,
    110, 32, 105, 110, 32, 68, 79, 83, 32, 109, 111, 100, 101, 46, 13, 13, 10, 36, 0, 0, 0, 0, 0,
    0, 0, 7, 167, 104, 106, 67, 198, 6, 57, 67, 198, 6, 57, 67, 198, 6, 57, 87, 173, 6, 56, 66,
    198, 6, 57, 87, 173, 5, 56, 96, 198, 6, 57, 87, 173, 2, 56, 194, 198, 6, 57, 87, 173, 11, 56,
    92, 199, 6, 57, 87, 173, 3, 56, 88, 198, 6, 57, 87, 173, 249, 57, 66, 198, 6, 57, 87, 173, 4,
    56, 66, 198, 6, 57, 82, 105, 99, 104, 67, 198, 6, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 80, 69, 0, 0, 100, 134, 10, 0, 23, 91, 113, 47, 0, 0, 0, 0, 0, 0,
    0, 0, 240, 0, 34, 32, 11, 2, 14, 20, 0, 154, 17, 0, 0, 98, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    16, 0, 0, 0, 0, 0, 128, 1, 0, 0, 0, 0, 16, 0, 0, 0, 2, 0, 0, 10, 0, 0, 0, 10, 0, 0, 0, 10, 0,
    0, 0, 0, 0, 0, 0, 0, 128, 31, 0, 0, 4, 0, 0, 47, 180, 31, 0, 3, 0, 96, 65, 0, 0, 4, 0, 0, 0, 0,
    0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16,
    0, 0, 0, 112, 33, 21, 0, 165, 46, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96, 24, 0, 8, 5, 7, 0, 0,
    32, 23, 0, 240, 228, 0, 0, 0, 138, 30, 0, 128, 107, 0, 0, 0, 112, 31, 0, 72, 5, 0, 0, 96, 104,
    18, 0, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    128, 219, 17, 0, 24, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 116, 101, 120, 116, 0, 0, 0, 206,
    145, 17, 0, 0, 16, 0, 0,
];

#[test]
fn safe_transmute_checks_layout() {
    let bytes = &LIBRARY_BYTES[..512];
    println!("{:?}", bytes);

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
