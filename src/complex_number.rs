use std::ops::{Add, Mul, Sub, Div};
use macroquad::math::Vec2;

#[derive(Copy, Clone)]
pub struct CNumber {
    pub real: f64,
    pub imaginary: f64,
}

impl CNumber {
    pub const fn new(im: f64, re: f64) -> Self{
        CNumber {
            real: re,
            imaginary: im,
        }
    }
    pub const fn zero() -> Self {
        CNumber { real: 0.0, imaginary: 0.0 }
    }
    pub fn abs(&self) -> f64 {
        (self.imaginary*self.imaginary+self.real*self.real).sqrt()
    }
}
impl Mul for CNumber {
    type Output = CNumber;

    fn mul(self, rhs: Self) -> CNumber {
        let out = CNumber::new(self.real*rhs.imaginary+self.imaginary*rhs.real,
                        self.real*rhs.real-self.imaginary*rhs.imaginary);
        return out
    }
}

impl Mul<Vec2> for CNumber {
    type Output = CNumber;

    fn mul(self, rhs: Vec2) -> Self::Output {
        CNumber { real: self.real*rhs.x as f64, imaginary: self.imaginary*rhs.y as f64}
    }
}
impl Div<Vec2> for CNumber {
    type Output = CNumber;

    fn div(self, rhs: Vec2) -> Self::Output {
        CNumber { real: self.real/rhs.x as f64, imaginary: self.imaginary/rhs.y as f64 }
    }
}
impl Add<f64> for CNumber {
    type Output = CNumber;

    fn add(self, rhs: f64) -> Self::Output {
        CNumber::new(self.imaginary, self.real+rhs)
    }
}
impl Add for CNumber {
    type Output = CNumber;

    fn add(self, rhs: Self) -> Self::Output {
        CNumber::new(self.imaginary+rhs.imaginary, self.real+rhs.real)
    }
}

impl Sub for CNumber {
    type Output = CNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        CNumber::new(self.imaginary-rhs.imaginary, self.real-rhs.real)
    }
}