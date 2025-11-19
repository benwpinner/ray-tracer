use std::ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tuple([f64; 4]);

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self([x, y, z, w])
    }

    pub fn new_point(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z, 1.0])
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z, 0.0])
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
        Self([
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
        Self([
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
        Tuple([self[0] * rhs, self[1] * rhs, self[2] * rhs, self[3] * rhs])
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

// impl Mul<Self> for Color {
//     type Output = Self;

//     fn mul(self, rhs: Self) -> Self::Output {
//         Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
//     }
// }

// impl MulAssign<Self> for Color {
//     fn mul_assign(&mut self, rhs: Self) {
//         self.r = self.r * rhs.r;
//         self.g = self.g * rhs.g;
//         self.b = self.b * rhs.b;
//     }
// }

impl Deref for Tuple {
    type Target = [f64; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Tuple {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
