use crate::engine::{
    canvas::Canvas,
    core::rays::rays::Ray,
    maths::{matrix::Matrix, tuple::Tuple},
};

pub struct Camera {
    pub transform: Matrix<4>,
    pub canvas: Canvas,
}

impl Camera {
    pub fn new(transform: Matrix<4>, canvas: Canvas) -> Self {
        Self { transform, canvas }
    }

    pub fn get_ray(&self, x: usize, y: usize) -> Ray {
        let pixel_size = self.canvas.pixel_size();
        let aspect_ratio = self.canvas.aspect_ratio();
        let pixel_pos = Tuple::new_point(
            (x as f64 * pixel_size) + (pixel_size / 2.0) - (1.0 / aspect_ratio),
            1.0 - (y as f64 * pixel_size) - (pixel_size / 2.0),
            -1.0,
        );
        let mut eye_to_pixel = pixel_pos - Tuple::new_point(0.0, 0.0, 0.0);
        eye_to_pixel.normalize();
        let mut ray = Ray::new(Tuple::new_point(0.0, 0.0, 0.0), eye_to_pixel);
        ray.transform(&self.transform);
        ray
    }
}
