use crate::engine::{
    core::rays::{intersection::Intersection, rays::Ray},
    maths::matrix::Matrix,
};

pub trait Shape {
    fn intersect(&self, ray: &mut Ray) -> Result<Vec<Intersection>, String>;
    fn set_transform(&mut self, transform_matrix: &Matrix<4>);
}
