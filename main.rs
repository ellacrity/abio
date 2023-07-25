use std::process::Command;

// include!("resources/src/bindings.rs");

// use abio::endian::{Endian, LittleEndian, LE};
// use abio::integer::I32;
// use abio::{Abi, AsBytes, Chunk, Decode, Result, Source, Zeroable};

// #[derive(Abi, Clone, Copy, Debug, Zeroable)]
// #[repr(transparent)]
// pub struct ImageDosHeaders(IMAGE_DOS_HEADER);

// impl Decode for ImageDosHeaders {
//     fn decode<E: Endian>(source: &[u8], endian: E) -> Result<T> {}
// }

// impl ImageDosHeaders {
//     pub fn parse(bytes: &[u8]) -> Result<Self> {
//         let e_magic = LE::read_u16(bytes, 0)?;
//         let pos = e_magic.runtime_size();
//         let size = IMAGE_DOS_HEADER::SIZE;
//         pos += size - I32::SIZE;
//         let (val, tail) = bytes.read_array::<_, Chunk<4>>(pos)?;
//         assert!(!tail.is_empty());
//         let value = I32::decode::<LE>(bytes, pos)?;
//     }
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("curl").arg("https://ifconfig.me").output()?;
    let buf = if output.status.success() { output.stdout } else { output.stderr };
    let as_utf8 = String::from_utf8_lossy(&buf[..]);
    println!("{as_utf8}");
    Ok(())
}
