use crate::reference::constant::{M, MULTIPLIER_POLYNOMIALS};
use crate::reference::polynomial::Polynomial;

/// Hashes a standard hash input, produces a standard hash
pub fn swifft_hash(input: &[Polynomial; M]) -> Polynomial {
    // compute linear combination of products a_i * x_i
    let mut hash_polynomial = Polynomial::ZERO;
    for i in 0..M {
        hash_polynomial += &MULTIPLIER_POLYNOMIALS[i] * &input[i]
    }
    hash_polynomial
}