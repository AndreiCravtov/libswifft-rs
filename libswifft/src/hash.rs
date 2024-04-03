//! Parameters: n=64, m=32, q=257
//!
//! INPUTS AND OUTPUTS SHOULD BE LITTLE ENDIAN
//! 0th pos = 0th power of polynomial
//! 0th pos = 0th power of 257

use crate::sys::{
    SWIFFT_Compact, SWIFFT_CompactMultiple, SWIFFT_Compute, SWIFFT_ComputeMultiple,
    SWIFFT_ComputeMultipleSigned, SWIFFT_ComputeSigned
};
use crate::buffer::{
    CompactOutput, CompactOutputs, Input, Inputs, Output, Outputs, SignInput, SignInputs
};

/// Computes the result of a SWIFFT operation.
/// The result is composable with other hash values.
/// 
/// # Arguments
/// * `input` - the input of 256 bytes (2048 bit)
/// * `output` - the resulting hash value of SWIFFT, of size 128 bytes (1024 bit)
pub fn compute(input: &Input, output: &mut Output) {
    unsafe {
        SWIFFT_Compute(input.0[0].as_ptr(), output.0[0].as_mut_ptr())
    }
}

/// Computes the result of multiple SWIFFT operations.
/// The result is composable with other hash values.
/// 
/// # Arguments
/// * `NUM_BLOCKS` - the number of blocks to operate on
/// * `input` - the blocks of input, each of 256 bytes (2048 bit)
/// * `output` - the resulting blocks of hash values of SWIFFT, each of size 128 bytes (1024 bit)
pub fn compute_multiple<const NUM_BLOCKS: usize>(input: &Inputs<NUM_BLOCKS>,
                                                 output: &mut Outputs<NUM_BLOCKS>) {
    unsafe {
        SWIFFT_ComputeMultiple(NUM_BLOCKS.try_into().unwrap(), input.0[0].as_ptr(), output.0[0].as_mut_ptr())
    }
}

/// Computes the result of a SWIFFT operation.
/// The result is composable with other hash values.
/// 
/// # Arguments
/// * `input` - the input of 256 bytes (2048 bit)
/// * `sign_input` - the sign bits corresponding to the input of 256 bytes (2048 bit)
/// * `output` - the resulting hash value of SWIFFT, of size 128 bytes (1024 bit)
pub fn compute_signed(input: &Input, sign_input: &SignInput, output: &mut Output) {
    unsafe {
        SWIFFT_ComputeSigned(input.0[0].as_ptr(), sign_input.0[0].as_ptr(), output.0[0].as_mut_ptr())
    }
}

/// Computes the result of multiple SWIFFT operations.
/// The result is composable with other hash values.
/// 
/// # Arguments
/// * `NUM_BLOCKS` - the number of blocks to operate on
/// * `input` - the blocks of input, each of 256 bytes (2048 bit)
/// * `sign_input` - the blocks of sign bits corresponding to blocks of input of 256 bytes (2048 bit)
/// * `output` - the resulting blocks of hash values of SWIFFT, each of size 128 bytes (1024 bit)
pub fn compute_multiple_signed<const NUM_BLOCKS: usize>(input: &Inputs<NUM_BLOCKS>, sign_input: &SignInputs<NUM_BLOCKS>, output: &mut Outputs<NUM_BLOCKS>) {
    unsafe {
        SWIFFT_ComputeMultipleSigned(NUM_BLOCKS.try_into().unwrap(), input.0[0].as_ptr(), sign_input.0[0].as_ptr(), output.0[0].as_mut_ptr())
    }
}

/// Compacts a hash value of SWIFFT.
/// The result is not composable with other compacted hash values.
/// 
/// # Arguments
/// * `output` - the hash value of SWIFFT, of size 128 bytes (1024 bit)
/// * `compact_output` - the compacted hash value of SWIFFT, of size 64 bytes (512 bit)
pub fn compact(output: &Output, compact_output: &mut CompactOutput) {
    unsafe {
        SWIFFT_Compact(output.0[0].as_ptr(), compact_output.0[0].as_mut_ptr())
    }
}

/// Compacts a hash value of SWIFFT for multiple blocks.
/// The result is not composable with other compacted hash values.
/// 
/// # Arguments
/// * `NUM_BLOCKS` - the number of blocks to operate on
/// * `output` - the hash value of SWIFFT, of size 128 bytes (1024 bit)
/// * `compact_output` - the compacted hash value of SWIFFT, of size 64 bytes (512 bit)
pub fn compact_multiple<const NUM_BLOCKS: usize>(output: &Outputs<NUM_BLOCKS>, compact_output: &mut CompactOutputs<NUM_BLOCKS>) {
    unsafe {
        SWIFFT_CompactMultiple(NUM_BLOCKS.try_into().unwrap(), output.0[0].as_ptr(), compact_output.0[0].as_mut_ptr())
    }
}