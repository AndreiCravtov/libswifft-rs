use std::convert::Into;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Rem, RemAssign, Neg, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use std::iter::{Product, Sum};
use num_traits::{CheckedDiv, Pow, Inv, Bounded, Zero, ConstZero, ConstOne, One, Num, Unsigned};
use ff::{Field, PrimeField, WithSmallOrderMulGroup};

/// This represents an element of $\mathbb{Z}_{257}$
#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct Z257(u16);

// STRUCT METHODS
impl Z257 {
    // CONSTRUCTOR METHODS
    /// Creates a new element of $\mathbb{Z}_{257}$,
    /// with the value provided
    #[inline]
    pub const fn new(value: u16) -> Self {
        Self(value % (Self::P as u16))
    }

    /// Creates a new element of $\mathbb{Z}_{257}$,
    /// from the provided byte value
    #[inline]
    pub const fn from_u64(value: u64) -> Self {
        Self((value % (Self::P as u64)) as u16)
    }

    /// Creates a new element of $\mathbb{Z}_{257}$,
    /// from the provided byte value
    #[inline]
    pub const fn from_u8(value: u8) -> Self {
        Self((value % (Self::P as u8)) as u16)
    }

    /// Creates a new element of $\mathbb{Z}_{257}$,
    /// from the provided boolean value
    #[inline]
    pub const fn from_bool(value: bool) -> Self {
        if value { Z257::ONE } else { Self::ZERO }
    }
    
    // PROPERTY METHODS
    #[inline]
    pub const fn value(&self) -> u16 {
        self.0
    }
    
    // CONSTANT OPERATIONS
    #[inline]
    pub const fn cn_is_zero(&self) -> bool {
        self.0 == 0
    }

    #[inline]
    pub const fn cn_is_one(&self) -> bool {
        self.0 == 1
    }
    
    #[inline]
    pub const fn cn_neg(&self) -> Self {
        if self.cn_is_zero() {
            Self::ZERO
        } else { 
            Self(Self::P - self.0)
        }
    }
    
    #[inline]
    pub const fn cn_add(&self, rhs: &Self) -> Self {
        let result = self.0 + rhs.0;
        if result >= Self::P {
            Self(result - Self::P)
        } else { 
            Self(result)
        }
    }

    #[inline]
    pub const fn cn_sub(&self, rhs: &Self) -> Self {
        if self.0 >= rhs.0 {
            Self(self.0 - rhs.0)
        } else { 
            Self(self.0 + Self::P - rhs.0)
        }
    }

    #[inline]
    pub const fn cn_mul(&self, rhs: &Self) -> Self {
        Self(((self.0 as u32 * rhs.0 as u32) % Self::P as u32) as u16)
    }
    
    #[inline]
    pub const fn cn_div(&self, rhs: &Self) -> Self {
        if rhs.cn_is_zero() {
            panic!("Cannot divide by zero")
        } else {
            Self(((self.0 as u32 * Self::INV[rhs.0 as usize] as u32) % Self::P as u32) as u16)
        }
    }

    #[inline]
    pub const fn cn_checked_div(&self, rhs: &Self) -> Option<Self> {
        if rhs.cn_is_zero() {
            None
        } else { 
            Some(Self(((self.0 as u32 * Self::INV[rhs.0 as usize] as u32) % Self::P as u32) as u16))
        }
    }

    #[inline]
    pub const fn cn_pow(&self, rhs: &Self) -> Self {
        Self(Self::POW[self.0 as usize][rhs.0 as usize])
    }

    #[inline]
    pub const fn cn_inv(&self) -> Self {
        if self.cn_is_zero() {
            panic!("Cannot invert zero")
        } else {
            Self(Self::INV[self.0 as usize])
        }
    }


    #[inline]
    pub const fn cn_inv_checked(&self) -> Option<Self> {
        if self.cn_is_zero() {
            None
        } else {
            Some(Self(Self::INV[self.0 as usize]))
        }
    }
    
    // NON-CONSTANT OPS
    #[inline]
    pub fn neg_assign(&mut self) {
        self.0 = if self.cn_is_zero() {
            0
        } else {
            Self::P - self.0
        }
    }

    #[inline]
    pub fn checked_div_assign(&mut self, rhs: &Self) -> Option<()> {
        if rhs.cn_is_zero() {
            None
        } else {
            self.0 = ((self.0 as u32 * Self::INV[rhs.0 as usize] as u32) % Self::P as u32) as u16;
            Some(())
        }
    }

    #[inline]
    pub fn pow_assign(&mut self, rhs: &Self) {
        self.0 = Self::POW[self.0 as usize][rhs.0 as usize]
    }

    #[inline]
    pub fn inv_assign(&mut self) {
        if self.cn_is_zero() {
            panic!("Cannot invert zero")
        } else {
            self.0 = Self::INV[self.0 as usize]
        }
    }

    #[inline]
    pub fn inv_assign_checked(&mut self) -> Option<()> {
        if self.cn_is_zero() {
            None
        } else {
            self.0 = Self::INV[self.0 as usize];
            Some(())
        }
    }
}

// STRUCT CONSTS
impl Z257 {
    // PUBLIC CONSTANTS
    pub const P: u16 = 257;
    
    // NUMBER CONSTS
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);
    pub const TWO: Self = Self(2);
    pub const THREE: Self = Self(3);

    // BOUND CONSTS
    pub const MIN: Self = Self::ZERO;
    pub const MAX: Self = Self(256);
    
    // MULTIPLICATIVE SUBGROUP CONSTS
    /// Least primitive root of unity in $\mathbb{Z}_{257}$,
    /// used as the generator point for multiplicative subgroup of order $256$
    pub const LEAST_PRIMITIVE_ROOT: Self = Self::THREE;

    /// Generator element of multiplicative subgroup of order $128$,
    /// containing the `128`th roots of unity in $\mathbb{Z}_{257}$
    pub const OMEGA_ORDER_128: Self = Self::LEAST_PRIMITIVE_ROOT.cn_pow(&Self::TWO);

    /// Generator element of multiplicative subgroup of order $64$,
    /// containing the `64`th roots of unity in $\mathbb{Z}_{257}$
    pub const OMEGA_ORDER_64: Self = Self::OMEGA_ORDER_128.cn_pow(&Self::TWO);

    /// Generator element of multiplicative subgroup of order $32$,
    /// containing the `32`th roots of unity in $\mathbb{Z}_{257}$
    pub const OMEGA_ORDER_32: Self = Self::OMEGA_ORDER_64.cn_pow(&Self::TWO);

    /// Generator element of multiplicative subgroup of order $16$,
    /// containing the `16`th roots of unity in $\mathbb{Z}_{257}$
    pub const OMEGA_ORDER_16: Self = Self::OMEGA_ORDER_32.cn_pow(&Self::TWO);

    /// Generator element of multiplicative subgroup of order $8$,
    /// containing the `8`th roots of unity in $\mathbb{Z}_{257}$
    pub const OMEGA_ORDER_8: Self = Self::OMEGA_ORDER_16.cn_pow(&Self::TWO);

    /// Generator element of multiplicative subgroup of order $4$,
    /// containing the `4`th roots of unity in $\mathbb{Z}_{257}$
    pub const OMEGA_ORDER_4: Self = Self::OMEGA_ORDER_8.cn_pow(&Self::TWO);

    /// Generator element of multiplicative subgroup of order $2$,
    /// containing the `2`nd roots of unity in $\mathbb{Z}_{257}$
    pub const OMEGA_ORDER_2: Self = Self::OMEGA_ORDER_4.cn_pow(&Self::TWO);
    
    // PRIVATE CONSTANTS
    const POW: [[u16; Self::P as usize]; Self::P as usize] = Self::compute_pow(); const fn compute_pow() -> [[u16; Self::P as usize]; Self::P as usize] {
        let mut pow: [[u16; Self::P as usize]; Self::P as usize] = [[0; Self::P as usize]; Self::P as usize];
        let mut n = 0; while n < Self::P {
            pow[n  as usize][0] = 1;
            let mut i = 1; while i < Self::P {
                pow[n  as usize][i  as usize] = ((pow[n  as usize][(i - 1)  as usize] as u32 * n as u32) % (Self::P as u32)) as u16;
                i += 1
            }
            n += 1
        }
        pow
    }
    const INV: [u16; Self::P as usize] = Self::compute_invert(); const fn compute_invert() -> [u16; Self::P as usize] {
        let mut invert: [u16; Self::P as usize] = [0; Self::P as usize];
        let mut n = 0; while n < Self::P {
            invert[n  as usize] = Self::POW[n  as usize][(Self::P as usize) - 2];
            n += 1
        }
        invert
    }
    const SQRT: [Option<u16>; Self::P as usize] = Self::compute_sqrt(); const fn compute_sqrt() -> [Option<u16>; Self::P as usize] {
        let mut sqrt: [Option<u16>; Self::P as usize] = [None; Self::P as usize];
        let mut n = 0; while n < Self::P {
            let mut m = 0; while m < Self::P {
                if Self::POW[m  as usize][2] == n {
                    sqrt[n  as usize] = Some(m);
                }
                m += 1
            }
            n += 1
        }
        sqrt
    }
}

// `std` TRAITS
impl Default for Z257 {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Into<u16> for Z257 {
    #[inline]
    fn into(self) -> u16 {
        self.value()
    }
}

impl<'a> Into<Z257> for &'a Z257 {
    #[inline]
    fn into(self) -> Z257 {
        *self
    }
}

impl From<bool> for Z257 {
    #[inline]
    fn from(value: bool) -> Self {
        Self::from_bool(value)
    }
}

impl From<u8> for Z257 {
    #[inline]
    fn from(value: u8) -> Self {
        Self::from_u8(value)
    }
}

impl From<u16> for Z257 {
    #[inline]
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl From<u64> for Z257 {
    #[inline]
    fn from(value: u64) -> Self {
        Self::from_u64(value)
    }
}

impl Display for Z257 {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl Debug for Z257 {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl<T: Into<Self>> Rem<T> for Z257 {
    type Output = Self;
    /// The remainder operator `%`.
    ///
    /// # Warning
    ///
    /// This method is part of the [`Rem`] trait implementation and does not make sense
    /// for finite fields, where arithmetic is inherently modulo the field size. Using
    /// this method is discouraged and may lead to confusing results.
    #[inline]
    fn rem(self, rhs: T) -> Self::Output {
        Self(self.0 % rhs.into().0)
    }
}

impl<T: Into<Self>> RemAssign<T> for Z257 {
    /// The remainder assignment operator `%=`.
    ///
    /// # Warning
    ///
    /// This method is part of the [`RemAssign`] trait implementation and does not make sense
    /// for finite fields, where arithmetic is inherently modulo the field size. Using
    /// this method is discouraged and may lead to confusing results.
    #[inline]
    fn rem_assign(&mut self, rhs: T) {
        self.0 %= rhs.into().0
    }
}

impl Neg for Z257 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        self.cn_neg()
    }
}

impl<T: Into<Self>> Add<T> for Z257 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        self.cn_add(&rhs.into())
    }
}

impl<T: Into<Self>> AddAssign<T> for Z257 {
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.0 += rhs.0;
        if self.0 >= Self::P {
            self.0 -= Self::P
        }
    }
}

impl<T: Into<Self>> Sub<T> for Z257 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        self.cn_sub(&rhs.into())
    }
}

impl<T: Into<Self>> SubAssign<T> for Z257 {
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        if self.0 >= rhs.0 {
            self.0 -= rhs.0
        } else {
            self.0 += Self::P - rhs.0
        }
    }
}

impl<T: Into<Self>> Sum<T> for Z257 {
    #[inline]
    fn sum<I: Iterator<Item=T>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |acc, next| { acc + next.into() })
    }
}

impl<T: Into<Self>> Mul<T> for Z257 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        self.cn_mul(&rhs.into())
    }
}

impl<T: Into<Self>> MulAssign<T> for Z257 {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.0 = ((self.0 as u32 * rhs.into().0 as u32) % Self::P as u32) as u16
    }
}

impl<T: Into<Self>> Product<T> for Z257 {
    #[inline]
    fn product<I: Iterator<Item=T>>(iter: I) -> Self {
        iter.fold(Self::ONE, |acc, next| { acc * next.into() })
    }
}

impl<T: Into<Self>> Div<T> for Z257 {
    type Output = Self;

    /// Performs the `/` operation.
    ///
    /// # WARNING
    ///
    /// This will panic if dividing by zero.
    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        self.cn_div(&rhs.into())
    }
}

impl<T: Into<Self>> DivAssign<T> for Z257 {
    /// Performs the `/=` operation.
    ///
    /// # WARNING
    ///
    /// This will panic if dividing by zero.
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        if rhs.cn_is_zero() {
            panic!("Cannot divide by zero")
        } else {
            self.0 = ((self.0 as u32 * Self::INV[rhs.0 as usize] as u32) % Self::P as u32) as u16
        }
    }
}

// `num_traits` TRAITS
impl CheckedDiv for Z257 {
    /// Returns the result of `self / rhs`, or [`None`] if dividing by zero.
    #[inline]
    fn checked_div(&self, rhs: &Self) -> Option<Self> {
        self.cn_checked_div(rhs)
    }
}

impl<T: Into<Self>> Pow<T> for Z257 {
    type Output = Self;
    #[inline]
    fn pow(self, rhs: T) -> Self::Output {
        self.cn_pow(&rhs.into())
    }
}

impl Inv for Z257 {
    type Output = Self;

    /// Unary operator for retrieving the multiplicative inverse, or reciprocal, of a value.
    ///
    /// This will panic if inverting zero.
    #[inline]
    fn inv(self) -> Self::Output {
        self.cn_inv()
    }
}

impl Bounded for Z257 {
    #[inline]
    fn min_value() -> Self {
        Self::MIN
    }

    #[inline]
    fn max_value() -> Self {
        Self::MAX
    }
}

impl Zero for Z257 {
    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }

    #[inline]
    fn set_zero(&mut self) {
        self.0 = 0
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.cn_is_zero()
    }
}

impl ConstZero for Z257 {
    const ZERO: Self = Self::ZERO;
}

impl One for Z257 {
    #[inline]
    fn one() -> Self {
        Self::ONE
    }

    #[inline]
    fn set_one(&mut self) {
        self.0 = 1
    }

    #[inline]
    fn is_one(&self) -> bool where Self: PartialEq {
        self.cn_is_one()
    }
}

impl ConstOne for Z257 {
    const ONE: Self = Self::ONE;
}

impl Num for Z257 {
    type FromStrRadixErr = std::num::ParseIntError;
    #[inline]
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        match u16::from_str_radix(str, radix) {
            Ok(value) => Ok(Self::new(value)),
            Err(err) => Err(err)
        }
    }
}

impl Unsigned for Z257 {}

// `ff` TRAITS
impl ff::derive::subtle::ConstantTimeEq for Z257 {
    #[inline]
    fn ct_eq(&self, other: &Self) -> ff::derive::subtle::Choice {
        if self == other {
            ff::derive::subtle::Choice::from(1)
        } else {
            ff::derive::subtle::Choice::from(0)
        }
    }
}

impl ff::derive::subtle::ConditionallySelectable for Z257 {
    #[inline]
    fn conditional_select(a: &Self, b: &Self, choice: ff::derive::subtle::Choice) -> Self {
        match choice.unwrap_u8() {
            0 => Z257(a.0),
            1 => Z257(b.0),
            choice => unreachable!("A choice should either be 0 or 1, instead found: {}", choice)
        }
    }
}

impl Field for Z257 {
    const ZERO: Self = Self::ZERO;
    const ONE: Self = Self::ONE;

    #[inline]
    fn random(mut rng: impl ff::derive::rand_core::RngCore) -> Self {
        Self((rng.next_u32() % Self::P as u32) as u16)
    }

    #[inline]
    fn square(&self) -> Self {
        Self(Self::POW[self.0 as usize][2])
    }

    #[inline]
    fn double(&self) -> Self {
        *self * Self::TWO
    }

    #[inline]
    fn invert(&self) -> ff::derive::subtle::CtOption<Self> {
        match self.cn_inv_checked() {
            Some(value) => ff::derive::subtle::CtOption::new(
                value, ff::derive::subtle::Choice::from(1)),
            _ => ff::derive::subtle::CtOption::new(
                Self::ZERO, ff::derive::subtle::Choice::from(0))
        }
    }
    
    fn sqrt_ratio(num: &Self, div: &Self) -> (ff::derive::subtle::Choice, Self) {
        if num.cn_is_zero() {
            (ff::derive::subtle::Choice::from(1), Self::ZERO)
        } else if div.cn_is_zero() {
            (ff::derive::subtle::Choice::from(0), Self::ZERO)
        } else {
            let num_div = *num / div;
            match Self::SQRT[num_div.0 as usize] {
                Some(sqrt) => (ff::derive::subtle::Choice::from(1), Self(sqrt)),

                // I set $G_S = \textsf{num}/\textsf{div}$ since it is a non-square,
                // so $\sqrt{G_S \cdot \textsf{num}/\textsf{div}} = \textsf{num}/\textsf{div}$
                _ => (ff::derive::subtle::Choice::from(0), num_div)
            }
        }
    }
}

impl PrimeField for Z257 {
    const S: u32 = 8;
    const DELTA: Self = Self::ONE;
    const TWO_INV: Self = Self::TWO.cn_inv();
    const MULTIPLICATIVE_GENERATOR: Self = Self::LEAST_PRIMITIVE_ROOT;
    const ROOT_OF_UNITY: Self = Self::MULTIPLICATIVE_GENERATOR;
    const ROOT_OF_UNITY_INV: Self = Self(Self::INV[Self::ROOT_OF_UNITY.0 as usize]);
    const NUM_BITS: u32 = 9;
    const CAPACITY: u32 = Self::NUM_BITS - 1;
    const MODULUS: &'static str = "257";

    type Repr = [u8; (u16::BITS / u8::BITS) as usize];

    fn from_repr(repr: Self::Repr) -> ff::derive::subtle::CtOption<Self> {
        let value = u16::from_le_bytes(repr);
        if value < Self::P as u16 {
            ff::derive::subtle::CtOption::new(
                Self(value), ff::derive::subtle::Choice::from(1))
        } else {
            ff::derive::subtle::CtOption::new(
                Self::ZERO, ff::derive::subtle::Choice::from(0))
        }
    }

    #[inline]
    fn to_repr(&self) -> Self::Repr {
        self.0.to_le_bytes()
    }

    #[inline]
    fn is_odd(&self) -> ff::derive::subtle::Choice {
        ff::derive::subtle::Choice::from(
            (self.0 % 2) as u8)
    }
}

impl WithSmallOrderMulGroup<128> for Z257 {
    const ZETA: Self = Self::OMEGA_ORDER_128;
}

impl WithSmallOrderMulGroup<64> for Z257 {
    const ZETA: Self = Self::OMEGA_ORDER_64;
}

impl WithSmallOrderMulGroup<32> for Z257 {
    const ZETA: Self = Self::OMEGA_ORDER_32;
}

impl WithSmallOrderMulGroup<16> for Z257 {
    const ZETA: Self = Self::OMEGA_ORDER_16;
}

impl WithSmallOrderMulGroup<8> for Z257 {
    const ZETA: Self = Self::OMEGA_ORDER_8;
}


impl WithSmallOrderMulGroup<4> for Z257 {
    const ZETA: Self = Self::OMEGA_ORDER_4;
}

impl WithSmallOrderMulGroup<2> for Z257 {
    const ZETA: Self = Self::OMEGA_ORDER_2;
}