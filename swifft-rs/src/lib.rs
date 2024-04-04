use rand::random;
use crate::hash::{INPUT_BLOCK_SIZE, parse_input_block, swifft_hash};

pub mod multiplier;
pub mod hash;
pub mod polynomial;
pub mod z257;
pub mod fft;

// fn main() {
//     let a = parse_input_block(&[random(); INPUT_BLOCK_SIZE]);
//     swifft_hash(&a);
// }