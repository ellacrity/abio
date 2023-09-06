extern crate abio_derive;
use abio::{Abi, AsBytes, Decode, Zeroable};

pub fn something() {
    mpmc::
}

#[derive(Abi, AsBytes, Zeroable)]
pub struct Packet {
    prefix: u32,
    length: u16,
    tag: u16,
    payload: [u8; 248],
}
