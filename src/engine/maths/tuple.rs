use std::ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign, Sub, SubAssign};

use approx::AbsDiffEq;

#[derive(Debug, PartialEq, Clone)]
pub struct Tuple(pub Vec<f64>);

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self(vec![x, y, z, w])
    }

    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Self(vec![x, y, z, 1.0])
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Self(vec![x, y, z, 0.0])
    }

    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
    }

    pub fn w(&self) -> f64 {
        self[3]
    }

    pub fn negate(&mut self) {
        self[0] = -self[0];
        self[1] = -self[1];
        self[2] = -self[2];
        self[3] = -self[3];
    }

    pub fn magnitude(&self) -> f64 {
        f64::sqrt((self[0] * self[0]) + (self[1] * self[1]) + (self[2] * self[2]))
    }

    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        self[0] /= magnitude;
        self[1] /= magnitude;
        self[2] /= magnitude;
        self[3] /= magnitude;
    }

    pub fn dot_product(lhs: &Tuple, rhs: &Tuple) -> f64 {
        return lhs[0] * rhs[0] + lhs[1] * rhs[1] + lhs[2] * rhs[2] + lhs[3] * rhs[3];
    }

    pub fn cross_product(lhs: &Tuple, rhs: &Tuple) -> Self {
        return Self::new_vector(
            lhs[1] * rhs[2] - lhs[2] * rhs[1],
            lhs[2] * rhs[0] - lhs[0] * rhs[2],
            lhs[0] * rhs[1] - lhs[1] * rhs[0],
        );
    }
}

impl Eq for Tuple {}

impl Add<Self> for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(vec![
            self[0] + rhs[0],
            self[1] + rhs[1],
            self[2] + rhs[2],
            self[3] + rhs[3],
        ])
    }
}

impl AddAssign<Self> for Tuple {
    fn add_assign(&mut self, rhs: Self) {
        self[0] = self[0] + rhs[0];
        self[1] = self[1] + rhs[1];
        self[2] = self[2] + rhs[2];
        self[3] = self[3] + rhs[3];
    }
}

impl Sub<Self> for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(vec![
            self[0] - rhs[0],
            self[1] - rhs[1],
            self[2] - rhs[2],
            self[3] - rhs[3],
        ])
    }
}

impl SubAssign<Self> for Tuple {
    fn sub_assign(&mut self, rhs: Self) {
        self[0] = self[0] - rhs[0];
        self[1] = self[1] - rhs[1];
        self[2] = self[2] - rhs[2];
        self[3] = self[3] - rhs[3];
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple(vec![
            self[0] * rhs,
            self[1] * rhs,
            self[2] * rhs,
            self[3] * rhs,
        ])
    }
}

impl MulAssign<f64> for Tuple {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] = self[0] * rhs;
        self[1] = self[1] * rhs;
        self[2] = self[2] * rhs;
        self[3] = self[3] * rhs;
    }
}

impl Deref for Tuple {
    type Target = Vec<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Tuple {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AbsDiffEq for Tuple {
    type Epsilon = f64;

    fn default_epsilon() -> f64 {
        1e-5
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
        for i in 0..self.len() {
            if !f64::abs_diff_eq(&self[i], &other[i], epsilon) {
                return false;
            }
        }
        true
    }
}
