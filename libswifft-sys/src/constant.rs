pub const N: usize = 64;
pub const M: usize = 32;
pub const Q: usize = 257;
pub const INPUT_SIZE: usize = N * M;
pub const INPUT_BLOCK_SIZE: usize = INPUT_SIZE / u8::BITS as usize;
pub const OUTPUT_BLOCK_SIZE: usize = 2*N;
pub const COMPACT_OUTPUT_BLOCK_SIZE: usize = 512 / u8::BITS as usize;
