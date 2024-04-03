use std::fmt::{Display, Formatter};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Index, Mul, Neg, Sub, SubAssign};
use num_traits::Inv;
use crate::constant::{INPUT_BLOCK_SIZE, M, N, P};
use crate::z257::Z257;

/// Element of polynomial ring ***[`Z257`] (A)/(A+1)***
#[derive(Eq, Copy, Debug)]
pub struct Polynomial {
    coefficients: [u16; N]
}

impl Polynomial {
    /// The zero polynomial, with all coefficients being ***0***
    /// It is the additive identity element, i.e. P + ZERO = P
    pub const ZERO: Polynomial = Polynomial { coefficients: [0; N] };

    /// The one polynomial, with the first coefficient being ***1***, and the rest ***0***
    /// It is the multiplicative identity element, i.e. P * ONE = P
    pub const ONE: Polynomial = Polynomial { coefficients: [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ] };

    pub const fn new(coefficients: &[u16; N]) -> Self {
        let mut coefficients_mod: [u16; N] = [0; N];
        let mut i = 0; while i < N {
            coefficients_mod[i] = coefficients[i].rem_euclid(P);
            i += 1
        }
        Polynomial { coefficients: coefficients_mod }
    }

    /// Parses input block of [`M`] binary polynomials
    pub fn from_input_block(input: &[u8; INPUT_BLOCK_SIZE]) -> [Polynomial; M] {
        // parse inputs into polynomials
        let mut input_coefficients: [[u16; N]; M] = [[0; N]; M];
        for byte_index in 0..INPUT_BLOCK_SIZE {
            for bit_position in 0..u8::BITS {
                let input_position = (byte_index as u32 * u8::BITS + bit_position) as usize;
                let input_index = input_position / N;
                let coefficient_index = input_position % N;
                input_coefficients[input_index][coefficient_index] = ((input[byte_index] >> bit_position) & 1) as u16;
            }
        }
        input_coefficients.map(|coefficients| { Polynomial::new(&coefficients) })
    }

    /// For any coefficient ***c***, returns ***-c (mod [`P`])***
    pub fn neg_coefficient(coefficient: &u16) -> u16 {
        (-(*coefficient as i32)).rem_euclid(P as i32) as u16
    }

    pub fn coefficients(&self) -> &[u16; N] {
        &self.coefficients
    }
    
    /// Computes the Hadamard (point-wise) product of the coefficient vector
    pub fn hadamard_product(&self, rhs: &Self) -> Self {
        let mut hadamard_product = [0; N];
        for i in 1..N {
            hadamard_product[i] = (self[i] * rhs[i]) % Z257::P as u16
        }
        Self { coefficients: hadamard_product }
    }

    /// Increments the power of every ***a*** in this polynomial by 1,
    /// and reduces it modulo ***a^[`N`] + 1***, returning the result
    ///
    /// This is equivalent to multiplying the polynomial by ***a***
    pub fn increment_power(&self) -> Polynomial {
        // equivalent to rotating the coefficients, but the rotated value on the other end is negative
        let mut reduced_product = [0; N];
        reduced_product[0] = Polynomial::neg_coefficient(&self[N-1]);
        for i in 1..N {
            reduced_product[i] = self[i-1]
        }
        Polynomial { coefficients: reduced_product }
    }

    /// Evaluates this polynomial at some point, modulo [`P`]
    pub fn evaluate_point(&self, point: u16) -> u16 {
        // exponentiate point correctly
        let mut point_powers: [u16; N] = [0; N];
        point_powers[0] = 1;
        for i in 1..N {
            point_powers[i] = (point_powers[i-1] as u32 * point as u32).rem_euclid(P as u32) as u16;
        }

        // compute dot product between point powers and coefficients
        let mut evaluation = 0;
        for i in 0..N {
            evaluation = (evaluation + point_powers[i] * self[i]).rem_euclid(P);
        }
        evaluation
    }

    /// Produces a toeplitz matrix which corresponds to the multiplication by this polynomial,
    /// where each resulting of the array is a column
    ///
    /// In this particular case, the matrix represents a negacyclic convolution
    pub fn toeplitz_matrix(&self) -> [Polynomial; N] {
        let mut toeplitz_matrix = [Polynomial::ZERO; N];
        toeplitz_matrix[0] = self.clone();
        for i in 1..N {
            toeplitz_matrix[i] = toeplitz_matrix[i-1].increment_power()
        }
        toeplitz_matrix
    }
    
    pub fn correct_multiply(&self, rhs: &Self) -> Self { // this performs the transformation correctly
        &self.toeplitz_matrix() * rhs
    }
    
    pub fn fft_multiply(&self, rhs: &Polynomial) -> Polynomial {
        // powers of OMEGA_ORDER_128 (should be precomputed)
        let mut omega_powers: [u16; N] = [0; N];
        omega_powers[0] = 1;
        for i in 1..N {
            omega_powers[i] = (omega_powers[i-1] * Z257::OMEGA_ORDER_128) % Z257::P as u16;
        }
        let omega_polynomial = Self { coefficients: omega_powers };
        
        // preprocess `self` and `rhs`
        let mut self_processed = self.hadamard_product(&omega_polynomial).coefficients.map(|value| {Z257::new(value)});
        let mut rhs_processed = rhs.hadamard_product(&omega_polynomial).coefficients.map(|value| {Z257::new(value)});
        
        // compute `N`-dimensional FFT of processed vectors
        halo2_proofs::arithmetic::best_fft::<Z257, Z257>(&mut self_processed, Z257::OMEGA_ORDER_64.into(), N.ilog2());
        halo2_proofs::arithmetic::best_fft::<Z257, Z257>(&mut rhs_processed, Z257::OMEGA_ORDER_64.into(), N.ilog2());
        
        // compute point-wise product (modulo 257)
        let mut product_fft = self_processed;
        for i in 0..N {
            product_fft[i] *= rhs_processed[i];
        }
        
        // compute `N`-dimensional IFFT of result
        for i in 0..N {
            product_fft[i] /= Z257::new(omega_powers[i]);
        }
        halo2_proofs::arithmetic::best_fft::<Z257, Z257>(&mut product_fft, Z257::new(Z257::OMEGA_ORDER_64).inv().unwrap(), N.ilog2());
        for mut value in product_fft {
            value /= Z257::new(N as u16);
        }
        
        // turn back into polynomial
        Self { coefficients: product_fft.map(|value| { value.into() }) }
    }
}

impl Clone for Polynomial {
    fn clone(&self) -> Self {
        Polynomial{ coefficients: self.coefficients.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        self.coefficients = source.coefficients.clone()
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.coefficients.iter().enumerate()
            .map(|(i, &c)| format!("{c}*a^{i}"))
            .collect::<Vec<_>>().join(" + "))
    }
}

impl Index<usize> for Polynomial {
    type Output = u16;
    fn index(&self, index: usize) -> &Self::Output {
        &self.coefficients[index]
    }
}

impl PartialEq<Self> for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..N {
            if self[i] != other[i] {
                return false
            }
        }
        return true
    }
}

impl Sum for Polynomial {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.sum()
    }
}

impl Add for &Polynomial {
    type Output = Polynomial;
    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = [0; N];
        for i in 0..N {
            sum[i] = (self[i] + rhs[i]).rem_euclid(P)
        }
        Polynomial { coefficients: sum }
    }
}

impl AddAssign for Polynomial {
    fn add_assign(&mut self, rhs: Self) {
        let mut coefficients = self.coefficients;
        for i in 0..N {
            coefficients[i] += rhs[i]
        }
    }
}

impl Neg for &Polynomial {
    type Output = Polynomial;
    fn neg(self) -> Self::Output {
        let mut negative = [0; N];
        for i in 0..N {
            let negative_coefficient = (-(self[i] as i32)).rem_euclid(P as i32) as u16;
            negative[i] = negative_coefficient;
        }
        Polynomial { coefficients: negative }
    }
}

impl Sub for &Polynomial {
    type Output = Polynomial;
    fn sub(self, rhs: Self) -> Self::Output {
        let negative = -rhs;
        self + &negative
    }
}

impl SubAssign for Polynomial {
    fn sub_assign(&mut self, rhs: Self) {
        let mut coefficients = self.coefficients;
        for i in 0..N {
            coefficients[i] -= rhs[i]
        }
    }
}

impl Mul<&u16> for &Polynomial {
    type Output = Polynomial;
    fn mul(self, rhs: &u16) -> Self::Output {
        let multiplier = rhs.rem_euclid(P);
        let mut result = [0; N];
        for i in 0..N {
            result[i] = (self[i] as u32 * multiplier as u32).rem_euclid(P as u32) as u16
        }
        Polynomial { coefficients: result }
    }
}

impl Mul<&Polynomial> for u16 {
    type Output = Polynomial;
    fn mul(self, rhs: &Polynomial) -> Self::Output {
        rhs * &self
    }
}

/// Treats the array of polynomials as a matrix, where
/// each element is the corresponding column; treats the polynomial
/// to be multiplied as a vector, and performs standard matrix multiplication
/// in the field ***Z_[`P`]***
impl Mul<&Polynomial> for &[Polynomial; N] {
    type Output = Polynomial;
    fn mul(self, rhs: &Polynomial) -> Self::Output {
        let mut product: [u16; N] = [0; N];
        for row in 0..N {
            for column in 0..N {
                product[row] = (product[row] as u32 + self[column][row] as u32 * rhs[column] as u32).rem_euclid(P as u32) as u16;
            }
        }
        Polynomial { coefficients: product }
    }
}

impl Mul for &Polynomial {
    type Output = Polynomial;
    fn mul(self, rhs: Self) -> Self::Output { // THIS IS INCORRECT, the correct multiply is 
        self.fft_multiply(rhs)
    }
}