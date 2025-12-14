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
        self.origin.clone() + (self.direction.clone() * t)
    }

    pub fn transform(&self, transform_matrix: &Matrix) -> Self {
        Self {
            origin: transform_matrix.clone() * self.origin.clone(),
            direction: transform_matrix.clone() * self.direction.clone(),
        }
    }
}
