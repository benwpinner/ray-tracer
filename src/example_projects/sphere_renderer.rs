use ray_tracer_rs::engine::{
    camera::Camera,
    canvas::Canvas,
    color::Color,
    core::{
        rays::intersection::Intersection,
        shapes::{primitives::sphere::Sphere, shape::Shape},
    },
    maths::matrix::Matrix,
};

use crate::ppm_renderer::renderer::PpmRenderer;

pub struct SphereRender {
    sphere: Sphere,
    camera: Camera,
}

impl SphereRender {
    pub fn new() -> Self {
        Self {
            sphere: Sphere::new(1, 1.0),
            camera: Camera::new(Matrix::translation(0.0, 0.0, 5.0), Canvas::new(1920, 1080)),
        }
    }

    pub fn render(&mut self) -> Result<(), String> {
        for x in 0..self.camera.canvas.width {
            for y in 0..self.camera.canvas.height {
                println!("X {}, Y {}", x, y);
                let ray = self.camera.get_ray(x, y);
                match Intersection::hit(self.sphere.intersect(ray)?.iter().collect()) {
                    Some(_) => self
                        .camera
                        .canvas
                        .write_pixel(x, y, Color::new(1.0, 0.0, 0.0)),
                    None => continue,
                };
            }
        }
        let renderer = PpmRenderer::from(&self.camera.canvas);
        renderer.render().map_err(|_| "Failed in PpmRenderer")?;
        Ok(())
    }
}
