#[derive(Abi, Clone, Copy, Debug, Decode)]
#[decode(BE)]
struct MagicSequence<const N: usize> {
    inner: [u8; N],
}

#[derive(Abi, AsBytes, Zeroable)]
pub struct Message<const N: usize> {
    magic: MagicSequence<N>,
}

impl<const N: usize> Message<N> {
    pub fn new(bytes: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let magic = MagicSequence::decode_slice(bytes)?;
    }
}
