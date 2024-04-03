//! Parameters: n=64, m=32, q=257

use crate::sys::{
    SWIFFT_Set, SWIFFT_SetMultiple, SWIFFT_Add, SWIFFT_AddMultiple, SWIFFT_ConstAdd,
    SWIFFT_ConstAddMultiple, SWIFFT_ConstMul, SWIFFT_ConstMulMultiple, SWIFFT_ConstSet,
    SWIFFT_ConstSetMultiple, SWIFFT_ConstSub, SWIFFT_ConstSubMultiple, SWIFFT_Mul,
    SWIFFT_MulMultiple, SWIFFT_Sub, SWIFFT_SubMultiple
};
use crate::buffer::{Output, Outputs};

/// Sets a SWIFFT hash value to another, element-wise.
/// 
/// # Arguments
/// * `output` - the hash value of SWIFFT to modify
/// * `operand` - the hash value to set to
pub fn set(output: &mut Output, operand: &Output) {
    unsafe {
        SWIFFT_Set(output.0[0].as_mut_ptr(), operand.0[0].as_ptr())
    }
}

/// Sets a SWIFFT hash value to another, element-wise, for multiple blocks.
/// 
/// # Arguments
/// * `NUM_BLOCKS` - the number of blocks to operate on
/// * `output` - the hash value of SWIFFT to modify
/// * `operand` - the hash value to set to
pub fn set_multiple<const NUM_BLOCKS: usize>(output: &mut Outputs<NUM_BLOCKS>, operand: &Outputs<NUM_BLOCKS>) {
    unsafe {
        SWIFFT_SetMultiple(NUM_BLOCKS.try_into().unwrap(), output.0[0].as_mut_ptr(), operand.0[0].as_ptr())
    }
}

/// Adds a SWIFFT hash value to another, element-wise.
/// 
/// # Arguments
/// * `output` - the hash value of SWIFFT to modify
/// * `operand` - the hash value to add
pub fn add(output: &mut Output, operand: &Output) {
    unsafe {
        SWIFFT_Add(output.0[0].as_mut_ptr(), operand.0[0].as_ptr())
    }
}

/// Adds a SWIFFT hash value to another, element-wise, for multiple blocks.
/// 
/// # Arguments
/// * `NUM_BLOCKS` - the number of blocks to operate on
/// * `output` - the hash value of SWIFFT to modify
/// * `operand` - the hash value to add
pub fn add_multiple<const NUM_BLOCKS: usize>(output: &mut Outputs<NUM_BLOCKS>, operand: &Outputs<NUM_BLOCKS>) {
    unsafe {
        SWIFFT_AddMultiple(NUM_BLOCKS.try_into().unwrap(), output.0[0].as_mut_ptr(), operand.0[0].as_ptr())
    }
}

/// Subtracts a SWIFFT hash value from another, element-wise.
/// 
/// # Arguments
/// * `output` - the hash value of SWIFFT to modify
/// * `operand` - the hash value to subtract
pub fn sub(output: &mut Output, operand: &Output) {
    unsafe {
        SWIFFT_Sub(output.0[0].as_mut_ptr(), operand.0[0].as_ptr())
    }
}
/// Subtracts a SWIFFT hash value from another, element-wise, for multiple blocks.
/// 
/// # Arguments
/// * `NUM_BLOCKS` - the number of blocks to operate on
/// * `output` - the hash value of SWIFFT to modify
/// * `operand` - the hash value to subtract
pub fn sub_multiple<const NUM_BLOCKS: usize>(output: &mut Outputs<NUM_BLOCKS>, operand: &Outputs<NUM_BLOCKS>) {
    unsafe {
        SWIFFT_SubMultiple(NUM_BLOCKS.try_into().unwrap(), output.0[0].as_mut_ptr(), operand.0[0].as_ptr())
    }
}

/// Multiplies a SWIFFT hash value from another, element-wise.
/// 
/// # Arguments
/// * `output` - the hash value of SWIFFT to modify
/// * `operand` - the hash value to multiply by
pub fn mul(output: &mut Output, operand: &Output) {
    unsafe {
        SWIFFT_Mul(output.0[0].as_mut_ptr(), operand.0[0].as_ptr())
    }
}

/// Multiplies a SWIFFT hash value from another, element-wise, for multiple blocks.
/// 
/// # Arguments
/// * `NUM_BLOCKS` - the number of blocks to operate on
/// * `output` - the hash value of SWIFFT to modify
/// * `operand` - the hash value to multiply by
pub fn mul_multiple<const NUM_BLOCKS: usize>(output: &mut Outputs<NUM_BLOCKS>, operand: &Outputs<NUM_BLOCKS>) {
    unsafe {
        SWIFFT_MulMultiple(NUM_BLOCKS.try_into().unwrap(), output.0[0].as_mut_ptr(), operand.0[0].as_ptr())
    }
}

/// Sets a constant value at each SWIFFT hash value element.
/// 
/// # Arguments
/// * `output` - the hash value of SWIFFT to modify
/// * `operand` - operand the constant value to set
pub fn const_set(output: &mut Output, operand: i16) {
    unsafe {
        SWIFFT_ConstSet(output.0[0].as_mut_ptr(), operand.rem_euclid(257));
    }
}

/// Sets a constant value at each SWIFFT hash value element for multiple blocks.
/// 
/// # Arguments
/// * `NUM_BLOCKS` - the number of blocks to operate on
/// * `output` - the hash value of SWIFFT to modify, per block
/// * `operand` - the constant value to set, per block
pub fn const_set_multiple<const NUM_BLOCKS: usize>(output: &mut Outputs<NUM_BLOCKS>, operand: &[i16; NUM_BLOCKS]) {
    unsafe {
        SWIFFT_ConstSetMultiple(NUM_BLOCKS.try_into().unwrap(), 
            output.0[0].as_mut_ptr(), operand.map(|i| { i.rem_euclid(257) }).as_ptr())
    }
}

/// Adds a constant value to each SWIFFT hash value element.
/// 
/// # Arguments
/// * `output` - the hash value of SWIFFT to modify
/// * `operand` - the constant value to add
pub fn const_add(output: &mut Output, operand: i16) {
    unsafe {
        SWIFFT_ConstAdd(output.0[0].as_mut_ptr(), operand)
    }
}

/// Adds a constant value to each SWIFFT hash value element for multiple blocks.
/// 
/// # Arguments
/// * `NUM_BLOCKS` - the number of blocks to operate on
/// * `output` - the hash value of SWIFFT to modify, per block
/// * `operand` - the constant value to add, per block
pub fn const_add_multiple<const NUM_BLOCKS: usize>(output: &mut Outputs<NUM_BLOCKS>, operand: &[i16; NUM_BLOCKS]) {
    unsafe {
        SWIFFT_ConstAddMultiple(NUM_BLOCKS.try_into().unwrap(), 
            output.0[0].as_mut_ptr(), operand.map(|i| { i.rem_euclid(257) }).as_ptr())
    }
}

/// Subtracts a constant value from each SWIFFT hash value element.
/// 
/// # Arguments
/// * `output` - the hash value of SWIFFT to modify
/// * `operand` - the constant value to subtract
pub fn const_sub(output: &mut Output, operand: i16) {
    unsafe {
        SWIFFT_ConstSub(output.0[0].as_mut_ptr(), operand)
    }
}

/// Subtracts a constant value from each SWIFFT hash value element for multiple blocks.
/// 
/// # Arguments
/// * `NUM_BLOCKS` - the number of blocks to operate on
/// * `output` - the hash value of SWIFFT to modify, per block
/// * `operand` - the constant value to subtract, per block
pub fn const_sub_multiple<const NUM_BLOCKS: usize>(output: &mut Outputs<NUM_BLOCKS>, operand: &[i16; NUM_BLOCKS]) {
    unsafe {
        SWIFFT_ConstSubMultiple(NUM_BLOCKS.try_into().unwrap(), 
            output.0[0].as_mut_ptr(), operand.map(|i| { i.rem_euclid(257) }).as_ptr())
    }
}

/// Multiply a constant value into each SWIFFT hash value element.
/// 
/// # Arguments
/// * `output` - the hash value of SWIFFT to modify
/// * `operand` - the constant value to multiply by
pub fn const_mul(output: &mut Output, operand: i16) {
    unsafe {
        SWIFFT_ConstMul(output.0[0].as_mut_ptr(), operand)
    }
}

/// Multiply a constant value into each SWIFFT hash value element for multiple blocks.
/// 
/// # Arguments
/// * `NUM_BLOCKS` - the number of blocks to operate on
/// * `output` - the hash value of SWIFFT to modify, per block
/// * `operand` - the constant value to multiply by, per block
pub fn const_mul_multiple<const NUM_BLOCKS: usize>(output: &mut Outputs<NUM_BLOCKS>, operand: &[i16; NUM_BLOCKS]) {
    unsafe {
        SWIFFT_ConstMulMultiple(NUM_BLOCKS.try_into().unwrap(), 
            output.0[0].as_mut_ptr(), operand.map(|i| { i.rem_euclid(257) }).as_ptr())
    }
}