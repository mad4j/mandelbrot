use std::ops;

#[derive(Clone, Copy)]
/// A complex number in Cartesian form (i.e. x+yi).
pub struct Z {
    /// Real part of the complex number
    re: f64,
    /// Imaginary part of the complex number
    im: f64,
}

impl Z {
    pub const ZERO: Z = Z { re: 0.0, im: 0.0 };

    pub fn new(re: f64, im: f64) -> Self {
        Z { re, im }
    }

    pub fn re(self) -> f64 {
        self.re
    }

    pub fn im(self) -> f64 {
        self.im
    }

    pub fn norm_sqr(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

impl ops::Add for Z {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl ops::Mul for Z {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}
