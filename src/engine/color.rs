use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use approx::AbsDiffEq;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}

impl Add<Self> for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl AddAssign<Self> for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r = self.r + rhs.r;
        self.g = self.g + rhs.g;
        self.b = self.b + rhs.b;
    }
}

impl Sub<Self> for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl SubAssign<Self> for Color {
    fn sub_assign(&mut self, rhs: Self) {
        self.r = self.r - rhs.r;
        self.g = self.g - rhs.g;
        self.b = self.b - rhs.b;
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.r = self.r * rhs;
        self.g = self.g * rhs;
        self.b = self.b * rhs;
    }
}

impl Mul<Self> for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl MulAssign<Self> for Color {
    fn mul_assign(&mut self, rhs: Self) {
        self.r = self.r * rhs.r;
        self.g = self.g * rhs.g;
        self.b = self.b * rhs.b;
    }
}

impl AbsDiffEq for Color {
    type Epsilon = f32;

    fn default_epsilon() -> f32 {
        1e-5
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: f32) -> bool {
        if !f32::abs_diff_eq(&self.r, &other.r, epsilon)
            || !f32::abs_diff_eq(&self.g, &other.g, epsilon)
            || !f32::abs_diff_eq(&self.b, &other.b, epsilon)
        {
            return false;
        }
        true
    }
}
