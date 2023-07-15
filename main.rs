use std::process::Command;

use abio::{Abi, Result, Source, Zeroable};

fn main() -> Result<()> {
    let output = Command::new("curl")
        .arg("https://ifconfig.me")
        .output()
        .map_err(|_| abio::Error::internal_failure())?;
    let buf = if output.status.success() { output.stdout } else { output.stderr };
    let as_utf8 = String::from_utf8_lossy(&buf[..]);
    println!("{as_utf8}");
    Ok(())
}

#[repr(C)]
#[derive(Abi, Clone, Copy, Debug, Zeroable)]
pub struct DosHeader {
    e_magic: U16,
    __padding: [u8; 58],
    e_lfanew: I32,
}
