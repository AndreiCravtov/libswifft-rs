use std::fmt::{Display, format, Formatter};
use std::ops::{Add, Index, Mul, Sub};
use crate::reference::constant::{N, P};

/// Element of polynomial ring ***Z_[`P`] (A)/(A+1)***
#[derive(Copy, Clone, Debug)]
pub struct Polynomial {
    coefficients: [u16; N]
}

impl Polynomial {
    pub fn new(coefficients: [u16; N]) -> Self {
        Polynomial { coefficients: coefficients.map(|c| {
            c.rem_euclid(P)
        }) }
    }

    pub fn coefficients(&self) -> &[u16; N] {
        &self.coefficients
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

impl Add for Polynomial {
    type Output = Polynomial;
    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = [0; N];
        for i in 0..N {
            sum[i] = (self[i] + rhs[i]).rem_euclid(P)
        }
        Polynomial { coefficients: sum }
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut sum = [0; N];
        for i in 0..N {
            let negative = (-(rhs[i] as i32)).rem_euclid(P as i32) as u16;
            sum[i] = (self[i] + negative).rem_euclid(P)
        }
        Polynomial { coefficients: sum }
    }
}

impl Mul<u16> for Polynomial {
    type Output = Polynomial;
    fn mul(self, rhs: u16) -> Self::Output {
        let multiplier = rhs.rem_euclid(P);
        let mut result = [0; N];
        for i in 0..N {
            result[i] = (self[i] * multiplier).rem_euclid(P)
        }
        Polynomial { coefficients: result }
    }
}

impl Mul<Polynomial> for u16 {
    type Output = Polynomial;
    fn mul(self, rhs: Polynomial) -> Self::Output {
        rhs * self
    }
}

impl Mul for Polynomial {
    type Output = Polynomial;
    fn mul(self, rhs: Self) -> Self::Output {
        // regular polynomial product, modulo P
        let mut product = [0; 2*N-1];
        for i in 0..N {
            for j in 0..N {
                product[i + j] = (product[i + j] + self[i]*rhs[j]).rem_euclid(P);
            }
        }

        // reduction modulo a^N+1:
        // since a^N + 1 = 0, then a^N = -1 and
        let mut reduction = [0; N];
        for i in N..product.len() {
            reduction[i - N] = product[i - N] + product[i]
        }

        for i in 0..N {
            print!("{} ", product[i])
        }
        print!("\n");
        for i in N..product.len() {
            print!("{} ", product[i])
        }
        println!("\n");





        for i in 0..N {
            print!("{} ", product[i])
        }
        print!("\n");
        for i in N..product.len() {
            print!("{} ", product[i])
        }
        println!("\n");











        let mut result = [0; N];
        for k in 0..N {
            for i in 0..=k {
                let product = self[i] * rhs[k-i];
                result[k] = (result[k] + product).rem_euclid(P);
            }
        }
        println!("eeeee");
        Polynomial { coefficients: result }
    }
}