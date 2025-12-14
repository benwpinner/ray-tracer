use crate::engine::maths::{matrix::Matrix, tuple::Tuple};

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, t: f64) -> Tuple {
        &self.origin + &&(self.direction * t)
    }

    pub fn transform(&mut self, transform_matrix: &Matrix<4>) {
        self.origin = transform_matrix * &self.origin;
        self.direction = transform_matrix * &self.direction;
    }
}
