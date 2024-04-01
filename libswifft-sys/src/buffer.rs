//! Parameters: n=64, m=32, q=257

#[repr(C, align(64))]
pub struct AlignedBuffer<const CHUNK_SIZE: usize, const NUM_CHUNKS: usize>(pub [[u8; CHUNK_SIZE]; NUM_CHUNKS]);

/// 32 input vectors, each in `Z_2^{64}`,
/// corresponding to `2048`-bit input size,
/// where each element in a vector takes `1` bit
pub type Input = Inputs<1>;

/// An array of inputs
pub type Inputs<const NUM_INPUTS: usize> = AlignedBuffer<256, NUM_INPUTS>;

/// An input buffer treated as a sign buffer,
/// where each bit corresponds to a sign.
/// `0 = +ve` and `1 = -ve`.
/// When paired with a normal input buffer,
/// allows for an input domain of `{-1, 0, 1}`
pub type SignInput = SignInputs<1>;

/// An array of sign inputs
pub type SignInputs<const NUM_INPUTS: usize> = Inputs<NUM_INPUTS>;

/// An output vector in `Z_{257}^{64}`,
/// where each element in the vector takes `16` bits
pub type Output = Outputs<1>;

// An array of output vectors
pub type Outputs<const NUM_OUTPUTS: usize> = AlignedBuffer<128, NUM_OUTPUTS>;

/// An output vector in `Z_{256}^{64}`,
/// corresponding to `512`-bit output size,
/// where each element in the vector takes `64` bits
pub type CompactOutput = CompactOutputs<1>;

// An array of compact outputs
pub type CompactOutputs<const NUM_OUTPUTS: usize> = AlignedBuffer<64, NUM_OUTPUTS>;

// IMPLEMENT BLOCKS
impl<const CHUNK_SIZE: usize, const NUM_CHUNKS: usize> AlignedBuffer<CHUNK_SIZE, NUM_CHUNKS> {
    /// Creates a `value`-initialized `AlignedBuffer`
    pub fn new(value: u8) -> Self {
        Self([[value; CHUNK_SIZE]; NUM_CHUNKS])
    }
}

impl<const CHUNK_SIZE: usize, const NUM_CHUNKS: usize> Default for AlignedBuffer<CHUNK_SIZE, NUM_CHUNKS> {
    /// Creates a zero-initialized `AlignedBuffer`
    fn default() -> Self {
        Self([[0u8; CHUNK_SIZE]; NUM_CHUNKS])
    }
}