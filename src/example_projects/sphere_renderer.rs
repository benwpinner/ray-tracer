use std::{sync::Arc, thread};

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

pub struct SphereRenderer {
    sphere: Sphere,
    camera: Camera,
}

impl SphereRenderer {
    pub fn new() -> Self {
        Self {
            sphere: Sphere::new(1, 1.0),
            camera: Camera::new(
                Matrix::<4>::translation(0.0, 0.0, 4.0),
                Canvas::new(1920, 1080),
            ),
        }
    }

    pub fn render(&mut self) -> Result<(), String> {
        let width = self.camera.canvas.lock().unwrap().width;
        let height = self.camera.canvas.lock().unwrap().height;
        for x in 0..width {
            for y in 0..height {
                println!("X {}, Y {}", x, y);
                let mut ray = self.camera.get_ray(x, y);
                match Intersection::hit(self.sphere.intersect(&mut ray)?.iter().collect()) {
                    Some(_) => self.camera.canvas.lock().unwrap().write_pixel(
                        x,
                        y,
                        Color::new(1.0, 0.0, 0.0),
                    ),
                    None => continue,
                };
            }
        }
        let renderer = PpmRenderer::from(&self.camera.canvas.lock().unwrap().clone());
        renderer
            .render("sphere.ppm")
            .map_err(|_| "Failed in PpmRenderer")?;
        Ok(())
    }

    pub fn render_with_threading(&mut self) -> Result<(), String> {
        let mut handles = vec![];
        let height = self.camera.canvas.lock().unwrap().height;
        let width = self.camera.canvas.lock().unwrap().width;
        let rows_per_tile = height / 22; // Number of rows each thread handles

        let mut row_tiles = vec![];

        for y_start in (0..height).step_by(rows_per_tile) {
            let y_end = (y_start + rows_per_tile).min(height);
            row_tiles.push((y_start, y_end));
        }

        for (y_start, y_end) in row_tiles {
            let canvas_clone = Arc::clone(&self.camera.canvas);
            let sphere = self.sphere.clone();
            let camera = self.camera.clone();

            let handle = thread::spawn(move || {
                let mut buffer = Vec::with_capacity((y_end - y_start) * width);

                for y in y_start..y_end {
                    for x in 0..width {
                        let mut ray = camera.get_ray(x, y);

                        if let Ok(intersections) = sphere.intersect(&mut ray) {
                            if Intersection::hit(intersections.iter().collect()).is_some() {
                                buffer.push(Color::new(1.0, 0.0, 0.0));
                            } else {
                                buffer.push(Color::new(0.0, 0.0, 0.0));
                            }
                        }
                    }
                }

                // Merge the buffer into the shared canvas once
                let mut canvas_lock = canvas_clone.lock().unwrap();
                for (i, pixel) in buffer.iter().enumerate() {
                    let target_index = (y_start * width) + i;
                    canvas_lock.buffer[target_index] = *pixel;
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().map_err(|_| "Thread panicked")?;
        }

        let renderer = PpmRenderer::from(&self.camera.canvas.lock().unwrap().clone());
        renderer
            .render("sphere_threaded.ppm")
            .map_err(|_| "Failed in PpmRenderer")?;
        Ok(())
    }
}
