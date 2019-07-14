const fn bytes_for_bits(bits: usize) -> usize {
    (bits + 7) / 8
}

pub struct Bitmap<const BITS: usize> {
    bytes: [u8; bytes_for_bits(BITS)],
}

impl<const BITS: usize> Bitmap<{ BITS }> {
    fn new() -> Self {
        Bitmap {
            bytes: [0; bytes_for_bits(BITS)],
        }
    }
}
