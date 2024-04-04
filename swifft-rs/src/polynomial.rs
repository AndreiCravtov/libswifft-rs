use std::fmt::{Debug, Display, Formatter};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::fft::best_fft;

use crate::z257::Z257;

/// Element of polynomial quotient ring $\mathbb{Z}_{257}[\alpha]/(\alpha^{64} + 1)$
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Polynomial(Coefficients);

/// Type alias representing the coefficients of a polynomial
pub type Coefficients = [Z257; Polynomial::N];

/// Type alias representing a $64 \times 64$ matrix,
/// where each polynomial is interpreted a column
pub type Matrix = [Polynomial; Polynomial::N];

// STRUCT METHODS
impl Polynomial {
    // CONSTRUCTOR METHODS
    /// Create a polynomial from the coefficients provided
    pub const fn new(coefficients: Coefficients) -> Self {
        Self(coefficients)
    }

    /// Creates new polynomial from the coefficients provided
    pub const fn from_coefficients(coefficients: &[u16; Self::N]) -> Self {
        let mut values: Coefficients = [Z257::ZERO; Self::N];
        let mut i = 0; while i < Self::N {
            values[i] = Z257::new(coefficients[i]);
            i += 1
        }
        Self(values)
    }

    /// Create a polynomial from the powers of the given `point`,
    /// where $1, point, point^2, \dots, point^{63}$ are the coefficients
    pub const fn from_point_powers(point: &Z257) -> Self {
        let mut point_powers: Coefficients = [Z257::ZERO; Self::N];
        point_powers[0] = Z257::ONE;
        let mut i = 1; while i < Self::N {
            point_powers[i] = point_powers[i-1].cn_mul(point);
            i += 1
        }
        Self(point_powers)
    }

    // STRUCT FIELD METHODS
    /// Coefficients of the polynomial
    #[inline]
    pub const fn coefficients(&self) -> &Coefficients { &self.0 }

    // CONSTANT OPERATIONS
    pub const fn cn_neg(&self) -> Self {
        let mut result = Polynomial::ZERO;
        let mut i = 0; while i < Self::N {
            result.0[i] = self.0[i].cn_neg();
            i += 1
        }
        result
    }

    pub const fn cn_add(&self, rhs: &Self) -> Self {
        let mut result = Polynomial::ZERO;
        let mut i = 0; while i < Self::N {
            result.0[i] = self.0[i].cn_add(&rhs.0[i]);
            i += 1
        }
        result
    }

    pub const fn cn_sub(&self, rhs: &Self) -> Self {
        let mut result = Polynomial::ZERO;
        let mut i = 0; while i < Self::N {
            result.0[i] = self.0[i].cn_sub(&rhs.0[i]);
            i += 1
        }
        result
    }

    pub const fn scalar_mul(&self, scalar: &Z257) -> Self {
        let mut result = Polynomial::ZERO;
        let mut i = 0; while i < Self::N {
            result.0[i] = self.0[i].cn_mul(scalar);
            i += 1
        }
        result
    }

    /// Computes the dot-product product of `self` and `rhs` coefficients
    pub const fn dot_product(&self, rhs: &Self) -> Z257 {
        let mut dot_product = Z257::ZERO;
        let mut i = 0; while i < Self::N {
            dot_product = dot_product.cn_add(&self.0[i].cn_mul(&rhs.0[i]));
            i += 1
        }
        dot_product
    }

    /// Computes the Hadamard (point-wise) product of `self` and `rhs` coefficients
    pub const fn hadamard_product(&self, rhs: &Self) -> Self {
        let mut hadamard_product = Polynomial::ZERO;
        let mut i = 0; while i < Self::N {
            hadamard_product.0[i] = self.0[i].cn_mul(&rhs.0[i]);
            i += 1
        }
        hadamard_product
    }

    /// Increments the power of every $\alpha$ in this polynomial by $1$,
    /// and reduces it modulo $\alpha^{64} + 1$, returning the result
    ///
    /// This is equivalent to multiplying the polynomial by $\alpha$, or performing
    /// a negacyclic rotation on the coefficient vector
    pub const fn increment_power(&self) -> Self {
        let mut reduced_product = Polynomial::ZERO;
        reduced_product.0[0] = self.0[Self::N - 1].cn_neg();
        let mut i = 1; while i < Self::N {
            reduced_product.0[i] = self.0[i-1];
            i += 1
        }
        reduced_product
    }

    /// Evaluates this polynomial at some point
    ///
    /// This is equivalent to computing the dot product of the polynomial coefficient vector
    /// and [Polynomial::from_point_powers] vector - computed from the provided point
    #[inline]
    pub const fn evaluate_point(&self, point: &Z257) -> Z257 {
        self.dot_product(&Self::from_point_powers(point))
    }

    /// Produces the Toeplitz matrix that corresponds to the multiplication by this polynomial,
    /// where each polynomial in the resulting array is a column
    ///
    /// For the case of quotient ring $\mathbb{Z}_{257}[\alpha]/(\alpha^{64} + 1)$,
    /// this matrix represents a negacyclic convolution
    pub const fn toeplitz_matrix(&self) -> Matrix {
        let mut toeplitz_matrix = [Self::ZERO; Self::N];
        toeplitz_matrix[0] = *self;
        let mut i = 1; while i < Self::N {
            toeplitz_matrix[i] = toeplitz_matrix[i-1].increment_power();
            i += 1
        }
        toeplitz_matrix
    }

    /// Performs standard matrix multiplication in the field $Z_{257}$
    ///
    /// Treats the polynomials in `lhs` as columns of the matrix;
    /// treats the coefficients of `rhs` as a column vector;
    /// the result should be interpreted as a column vector

    pub const fn matrix_mul_col_vec(lhs: &Matrix, rhs: &Self) -> Self {
        let mut product: Coefficients = [Z257::ZERO; Self::N];
        let mut row = 0; while row < Self::N {
            let mut column = 0; while column < Self::N {
                if lhs[column].0[row].value() > 256 || rhs.0[column].value() > 256 {
                    panic!("AAAA")
                }
                product[row] = product[row].cn_add(
                    &lhs[column].0[row].cn_mul(&rhs.0[column]));
                column += 1
            }
            row += 1
        }
        Self(product)
    }

    /// Performs standard matrix multiplication in the field $Z_{257}$
    ///
    /// Treats the coefficients of `lhs` as a row vector;
    /// treats the polynomials in `rhs` as columns of the matrix;
    /// the result should be interpreted as a row vector
    pub const fn matrix_mul_row_vec(&self, rhs: &Matrix) -> Self {
        let mut product: Coefficients = [Z257::ZERO; Self::N];
        let mut column = 0; while column < Self::N {
            product[column] = self.dot_product(&rhs[column]);
            column += 1
        }
        Self(product)
    }

    /// Performs the naive algorithm for multiplying polynomials
    #[inline]
    pub const fn naive_mul(&self, rhs: &Self) -> Self {
        Self::matrix_mul_col_vec(&self.toeplitz_matrix(), rhs)
    }

    // NON-CONSTANT OPERATIONS
    pub fn neg_assign(&mut self) {
        for i in 0..Self::N {
            self.0[i] = -self.0[i]
        }
    }

    pub fn scalar_mul_assign(&mut self, scalar: &Z257) {
        for i in 0..Self::N {
            self.0[i] *= scalar
        }
    }

    /// Computes the Hadamard (point-wise) product of `self` and `rhs` coefficients
    pub fn hadamard_product_assign(&mut self, rhs: &Self) {
        for i in 0..Self::N {
            self.0[i] *= rhs[i]
        }
    }

    /// Increments the power of every $\alpha$ in this polynomial by $1$,
    /// and reduces it modulo $\alpha^{64} + 1$, returning the result
    ///
    /// This is equivalent to multiplying the polynomial by $\alpha$, or performing
    /// a negacyclic rotation on the coefficient vector
    pub fn increment_power_assign(&mut self) {
        let rotated_coefficient = -self[Self::N - 1];
        for i in 1..Self::N {
            self.0[i] = self[i-1]
        }
        self.0[0] = rotated_coefficient
    }


    /// Evaluates the polynomial at [`Polynomial::N`] ascending odd powers of [`Z257::OMEGA_ORDER_128`],
    /// which is $\omega_{128}, \omega_{128}^3, \dots, \omega_{128}^{127}$,
    /// and returns the resulting coefficient
    ///
    /// Equivalent to performing the isomorphism
    /// $$\left(\mathbb{Z}\_{257}\[\alpha\]/(\alpha^{64}+1), +, * \right) \cong \left(\mathbb{Z}_{257}^{64}, +, \circ \right)$$
    #[inline]
    pub fn fourier_coefficients(&self) -> Self {
        let mut fourier_coefficients = self.clone();
        fourier_coefficients.fourier_coefficients_assign();
        fourier_coefficients
    }

    /// Evaluates the polynomial at [`Polynomial::N`] ascending odd powers of [`Z257::OMEGA_ORDER_128`],
    /// which is $\omega_{128}, \omega_{128}^3, \dots, \omega_{128}^{127}$,
    /// and returns the resulting coefficient
    ///
    /// Equivalent to performing the isomorphism
    /// $$\left(\mathbb{Z}\_{257}\[\alpha\]/(\alpha^{64}+1), +, * \right) \cong \left(\mathbb{Z}_{257}^{64}, +, \circ \right)$$
    pub fn fourier_coefficients_assign(&mut self) {
        // multiply point-wise by [`OMEGA_ORDER_128_POWERS`]
        // and compute [`N`]-dimensional FFT of the result
        self.hadamard_product_assign(&Self::OMEGA_ORDER_128_POWERS);
        best_fft(
            &mut self.0, Z257::OMEGA_ORDER_64, Self::LOG2_N);
    }

    /// Interpolates the Fourier coefficients back into a polynomial
    ///
    /// Equivalent to undoing the isomorphism
    /// $$\left(\mathbb{Z}\_{257}\[\alpha\]/(\alpha^{64}+1), +, * \right) \cong \left(\mathbb{Z}_{257}^{64}, +, \circ \right)$$
    #[inline]
    pub fn interpolate_fourier_coefficients(&self) -> Self {
        let mut interpolated_polynomial = self.clone();
        interpolated_polynomial.interpolate_fourier_coefficients_assign();
        interpolated_polynomial
    }

    /// Interpolates the Fourier coefficients back into a polynomial
    ///
    /// Equivalent to undoing the isomorphism
    /// $$\left(\mathbb{Z}\_{257}\[\alpha\]/(\alpha^{64}+1), +, * \right) \cong \left(\mathbb{Z}_{257}^{64}, +, \circ \right)$$
    pub fn interpolate_fourier_coefficients_assign(&mut self) {
        // and compute [`N`]-dimensional inverse FFT of the result
        best_fft(
            &mut self.0, Self::OMEGA_ORDER_64_INV, Self::LOG2_N);

        // normalise the result, to get back the original polynomial
        self.hadamard_product_assign(&Self::FOURIER_NORMALISATION_COEFFICIENTS);
    }

    /// Performs the FFT algorithm for multiplying polynomials
    #[inline]
    pub fn fft_mul(&self, rhs: &Self) -> Self {
        let mut product = self.clone();
        product.fft_mul_assign(rhs);
        product
    }

    /// Performs the FFT algorithm for multiplying polynomials
    pub fn fft_mul_assign(&mut self, rhs: &Self) {
        // compute fourier coefficients of both
        self.fourier_coefficients_assign();
        let rhs_coefficients = rhs.fourier_coefficients();

        // compute point-wise product
        self.hadamard_product_assign(&rhs_coefficients);

        // interpolate the result back into a polynomial
        self.interpolate_fourier_coefficients_assign();
    }
}

impl Polynomial {
    /// The security parameter determining the maximum degree of polynomials,
    /// which is [`Polynomial::N`]`-1`
    pub const N: usize = 64;
    pub const LOG2_N: u32 = Self::N.ilog2();


    /// The zero polynomial, with all coefficients being ***0***
    /// It is the additive identity element, i.e. P + ZERO = P
    pub const ZERO: Self = Self([Z257::ZERO; Self::N]);

    /// The one polynomial, with the first coefficient being ***1***, and the rest ***0***
    /// It is the multiplicative identity element, i.e. P * ONE = P
    pub const ONE: Self = Self::from_coefficients(&[
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    /// The $\alpha$ polynomial
    pub const ALPHA: Self = Self::from_coefficients(&[
        0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ]);

    /// The inverse element of [`Z257::OMEGA_ORDER_64`]
    pub const OMEGA_ORDER_64_INV: Z257 = Z257::OMEGA_ORDER_64.cn_inv();

    /// The polynomial whose coefficients are ascending powers of [`Z257::OMEGA_ORDER_128`],
    /// which is $0, \omega_{128}, \omega_{128}^2, \dots, \omega_{128}^{63}$
    pub const OMEGA_ORDER_128_POWERS: Self = Self::from_point_powers(&Z257::OMEGA_ORDER_128);
    
    /// Coefficients that are used to normalise the result of applying the inverse Fourier transform
    /// to get back the original polynomial. Computed by finding the polynomial whose coefficients are
    /// ascending powers of the ***inverse*** of [`Z257::OMEGA_ORDER_128`], which is $0, \omega_{128}^{-1}, \omega_{128}^{-2}, \dots, \omega_{128}^{-63}$,
    /// and scaling it by the inverse of [`Z257::OMEGA_ORDER_64`]
    pub const FOURIER_NORMALISATION_COEFFICIENTS: Self = Self::from_point_powers(
        &Z257::OMEGA_ORDER_128.cn_inv()).scalar_mul(&Z257::new(Self::N as u16).cn_inv());
}

impl Display for Polynomial {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.0))
    }
}

impl<'a> Into<Polynomial> for &'a Polynomial {
    #[inline]
    fn into(self) -> Polynomial {
        *self
    }
}

impl Index<usize> for Polynomial {
    type Output = Z257;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Neg for Polynomial {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.cn_neg()
    }
}

impl<T: Into<Self>> Add<T> for Polynomial {
    type Output = Polynomial;
    fn add(self, rhs: T) -> Self::Output {
        self.cn_add(&rhs.into())
    }
}

impl<T: Into<Self>> AddAssign<T> for Polynomial {
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        for i in 0..Self::N {
            self.0[i] += rhs[i]
        }
    }
}

impl<T: Into<Self>> Sum<T> for Polynomial {
    fn sum<I: Iterator<Item=T>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, next| { acc + next.into() })
    }
}

impl<T: Into<Self>> Sub<T> for Polynomial {
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        self.cn_sub(&rhs.into())
    }
}

impl<T: Into<Self>> SubAssign<T> for Polynomial {
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        for i in 0..Self::N {
            self.0[i] -= rhs.0[i]
        }
    }
}

impl Mul<Z257> for Polynomial {
    type Output = Polynomial;
    fn mul(self, rhs: Z257) -> Self::Output {
        self.scalar_mul(&rhs.into())
    }
}

impl MulAssign<Z257> for Polynomial {
    fn mul_assign(&mut self, rhs: Z257) {
        self.scalar_mul_assign(&rhs)
    }
}

impl Mul<Polynomial> for Z257 {
    type Output = Polynomial;
    fn mul(self, rhs: Polynomial) -> Self::Output {
        rhs.scalar_mul(&self)
    }
}

impl Mul<&Matrix> for Polynomial {
    type Output = Polynomial;
    fn mul(self, rhs: &Matrix) -> Self::Output {
        self.matrix_mul_row_vec(rhs)
    }
}

impl Mul<&Polynomial> for &Matrix {
    type Output = Polynomial;
    fn mul(self, rhs: &Polynomial) -> Self::Output {
        Polynomial::matrix_mul_col_vec(self, rhs)
    }
}

impl<T: Into<Self>> Mul<T> for Polynomial {
    type Output = Polynomial;
    fn mul(self, rhs: T) -> Self::Output {
        self.fft_mul(&rhs.into())
    }
}

impl<T: Into<Self>> MulAssign<T> for Polynomial {
    fn mul_assign(&mut self, rhs: T) {
        self.fft_mul_assign(&rhs.into())
    }
}