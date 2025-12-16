use std::sync::{Arc, Mutex};

use crate::engine::{
    canvas::Canvas,
    core::rays::rays::Ray,
    maths::{matrix::Matrix, tuple::Tuple},
};

#[derive(Clone)]
pub struct Camera {
    pub transform: Matrix<4>,
    pub canvas: Arc<Mutex<Canvas>>,
    pixel_size: f64,
    aspect_ratio: f64,
}

impl Camera {
    pub fn new(transform: Matrix<4>, canvas: Canvas) -> Self {
        Self {
            transform,
            pixel_size: canvas.pixel_size(),
            aspect_ratio: canvas.aspect_ratio(),
            canvas: Arc::new(Mutex::new(canvas)),
        }
    }

    pub fn get_ray(&self, x: usize, y: usize) -> Ray {
        let pixel_pos = Tuple::new_point(
            (x as f64 * self.pixel_size) + (self.pixel_size / 2.0) - (1.0 / self.aspect_ratio),
            1.0 - (y as f64 * self.pixel_size) - (self.pixel_size / 2.0),
            -1.0,
        );
        let mut eye_to_pixel = pixel_pos - Tuple::new_point(0.0, 0.0, 0.0);
        eye_to_pixel.normalize();
        let mut ray = Ray::new(Tuple::new_point(0.0, 0.0, 0.0), eye_to_pixel);
        ray.transform(&self.transform);
        ray
    }
}
