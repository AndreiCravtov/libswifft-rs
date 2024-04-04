use rayon::prelude::*;

use crate::multiplier::MULTIPLIER_POLYNOMIAL_COEFFICIENTS;
use crate::polynomial::{Coefficients, Polynomial};
use crate::z257::Z257;

// CONSTANTS
/// Efficiency and security parameter representing the number of vectors
/// (*from vector space $\mathbb{Z}_{257}^{64}$*),
/// that are taken as input to the hash function
pub const M: usize = 16;

/// The total size of the input, calculated by multiplying
/// [`Polynomial::N`] _(the number of elements in an input vector)_ and
/// [`M`] _(the number of input vectors)_
pub const INPUT_SIZE: usize = Polynomial::N * M;

/// The total size of an input block that consists of [`u8`] elements,
/// calculated by dividing [`INPUT_SIZE`] by [`u8::BITS`].
///
/// Each of the [`Polynomial::N`] elements *(in each of the [`M`] input vectors)*,
/// is represented by `1` bit; `8` elements per byte
pub const INPUT_BLOCK_SIZE: usize = INPUT_SIZE / u8::BITS as usize;


// HELPER METHODS
/// Parses input block of $16$ binary polynomials
pub const fn parse_input_block(input: &[u8; INPUT_BLOCK_SIZE]) -> SwifftInput {
    // parse inputs into binary polynomial coefficients
    let mut input_coefficients: [Coefficients; M] = [[Z257::ZERO; Polynomial::N]; M];
    let mut byte_index = 0; while byte_index < INPUT_BLOCK_SIZE {
        let mut bit_position = 0; while bit_position < u8::BITS {
            let input_position = (byte_index as u32 * u8::BITS + bit_position) as usize;
            let input_index = input_position / Polynomial::N;
            let coefficient_index = input_position % Polynomial::N;
            let bit = (input[byte_index] >> bit_position) & 1 != 0;
            input_coefficients[input_index][coefficient_index] = Z257::from_bool(bit);
            bit_position += 1
        }
        byte_index += 1
    }

    // instantiate input polynomials
    let mut input_polynomials: [Polynomial; M] = [Polynomial::ZERO; M];
    let mut i = 0; while i < M {
        input_polynomials[i] = Polynomial::new(input_coefficients[i]);
        i += 1
    }
    input_polynomials
}

pub fn swifft_hash_naive(input: &SwifftInput) -> Polynomial {
    // compute linear combination of products a_i * x_i
    let mut hash_polynomial = Polynomial::ZERO;
    for i in 0..M {
        hash_polynomial += MULTIPLIER_POLYNOMIALS[i].naive_mul(&input[i])
    }
    hash_polynomial
}

pub fn swifft_hash_fft_simple(input: &SwifftInput) -> Polynomial {
    // compute linear combination of products a_i * x_i
    let mut hash_polynomial = Polynomial::ZERO;
    for i in 0..M {
        hash_polynomial += MULTIPLIER_POLYNOMIALS[i] * input[i]
    }
    hash_polynomial
}

// SWIFFT HASH FUNCTION
/// Type alias representing the input to the SWIFFT hash function
pub type SwifftInput = [Polynomial; M];

/// Standard SWIFFT hash function, processing a single input
pub fn swifft_hash(input: &SwifftInput) -> Polynomial {
    // Compute 16 individual Polynomial products A_i * X_i
    // in the Fourier coefficients representation
    let mut product_fourier_coefficients = input.clone();
    product_fourier_coefficients.par_iter_mut().enumerate()
        .for_each(|(i, input)| {
            // compute Fourier coefficients of input
            input.fourier_coefficients_assign();

            // compute hadamard product of input and multiplier Fourier coefficients
            input.hadamard_product_assign(&MULTIPLIER_FOURIER_COEFFICIENTS[i]);
        });
    
    // Compute linear combination of those products
    let mut digest = Polynomial::ZERO;
    for product in product_fourier_coefficients {
        digest += product
    }
    
    // interpolate resulting Fourier coefficients,
    // and return result
    digest.interpolate_fourier_coefficients_assign();
    digest
}

// PRECOMPUTED CONSTANTS
/// The instantiation of [`MULTIPLIER_POLYNOMIAL_COEFFICIENTS`] as [`Polynomial`]s
pub const MULTIPLIER_POLYNOMIALS: [Polynomial; M] = compute_multiplier_polynomials(); const fn compute_multiplier_polynomials() -> [Polynomial; M] {
    let mut multiplier_polynomials = [Polynomial::ZERO; M];
    let mut i = 0; while i < M {
        multiplier_polynomials[i] = Polynomial::from_coefficients(&MULTIPLIER_POLYNOMIAL_COEFFICIENTS[i]);
        i += 1
    }
    multiplier_polynomials
}

/// The [`MULTIPLIER_POLYNOMIALS`] evaluated at [`Polynomial::N`] ascending odd powers of [`Z257::OMEGA_ORDER_128`],
/// which is $\omega_{128}, \omega_{128}^3, \dots, \omega_{128}^{127}$
///
/// Equivalent to performing the isomorphism
/// $\left(\mathbb{Z}_{257}[\alpha]/(\alpha^{64} + 1), +, *\right) \cong \left(\mathbb{Z}_{257}^{64}, +, \circ\right)$
pub const MULTIPLIER_FOURIER_COEFFICIENTS: [Polynomial; M] = compute_multiplier_fourier_coefficients(); const fn compute_multiplier_fourier_coefficients() -> [Polynomial; M] {
    // then point-wise multiply with powers of $\omega_{128}$
    let mut augmented_multiplier_polynomials: [Polynomial; M] = [Polynomial::ZERO; M];
    let mut i = 0; while i < M {
        augmented_multiplier_polynomials[i] = MULTIPLIER_POLYNOMIALS[i].hadamard_product(&Polynomial::OMEGA_ORDER_128_POWERS);
        i += 1
    }

    // then evaluate at powers of $\omega_{128}^2$, to get the evaluation at odd powers of $\omega_{128}$
    let mut fourier_coefficients: [Coefficients; M] = [[Z257::ZERO; Polynomial::N]; M];
    let mut i = 0; while i < M {
        let mut j = 0; while j < Polynomial::N {
            let omega_order_64_power = Z257::OMEGA_ORDER_64.cn_pow(&Z257::new(j as u16));
            fourier_coefficients[i][j] = augmented_multiplier_polynomials[i].evaluate_point(&omega_order_64_power);
            j += 1
        }
        i += 1
    }
    
    // instantiate polynomials with coefficients
    let mut multiplier_fourier_coefficients: [Polynomial; M] = [Polynomial::ZERO; M];
    let mut i = 0; while i < M {
        multiplier_fourier_coefficients[i] = Polynomial::new(fourier_coefficients[i]);
        i += 1
    }
    multiplier_fourier_coefficients
}
