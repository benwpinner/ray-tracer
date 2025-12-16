use crate::engine::{
    core::{
        rays::{intersection::Intersection, rays::Ray},
        shapes::shape::Shape,
    },
    maths::{matrix::Matrix, tuple::Tuple},
};

#[derive(Clone, Copy)]
pub struct Sphere {
    pub id: i32,
    pub r: f64,
    pub transform: Matrix<4>,
}

impl Sphere {
    pub fn new(id: i32, r: f64) -> Self {
        Self {
            id,
            r,
            transform: Matrix::<4>::identity(),
        }
    }

    // b^2 - 4ac
    // a = 1 as a = D . D, D should be a normalized vector so the dot product should be 1
    // b = 2D . (O - C)
    // c = ((O - C) . (O - C)) -
    fn _ray_discriminant(&self, ray: &mut Ray) -> Result<f64, String> {
        ray.transform(&self.transform.inverse()?);
        let sphere_to_ray = &ray.origin;
        let a = Tuple::dot_product(&ray.direction, &ray.direction);
        let b = 2.0 * Tuple::dot_product(&ray.direction, sphere_to_ray);
        let c = Tuple::dot_product(sphere_to_ray, sphere_to_ray) - (self.r * self.r);
        Ok((b * b) - (4.0 * a * c))
    }

    fn discriminant(&self, a: f64, b: f64, c: f64) -> f64 {
        (b * b) - (4.0 * a * c)
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &mut Ray) -> Result<Vec<Intersection>, String> {
        ray.transform(&self.transform.inverse()?);
        let sphere_to_ray = ray.origin - Tuple::new_point(0.0, 0.0, 0.0);
        let a = Tuple::dot_product(&ray.direction, &ray.direction);
        let b = 2.0 * Tuple::dot_product(&ray.direction, &sphere_to_ray);
        let c = Tuple::dot_product(&sphere_to_ray, &sphere_to_ray) - (self.r * self.r);
        let discriminant = self.discriminant(a, b, c);
        if discriminant > 0.0 {
            return Ok(vec![
                Intersection::new((-b - f64::sqrt(discriminant)) / (2.0 * a), self),
                Intersection::new((-b + f64::sqrt(discriminant)) / (2.0 * a), self),
            ]);
        } else if discriminant == 0.0 {
            return Ok(vec![Intersection::new((-b) / (2.0 * a), self)]);
        }
        Ok(vec![])
    }

    fn set_transform(&mut self, transform_matrix: &Matrix<4>) {
        self.transform = &self.transform * transform_matrix;
    }
}
