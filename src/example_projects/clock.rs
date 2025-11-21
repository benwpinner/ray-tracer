use ray_tracer_rs::engine::{
    canvas::Canvas,
    color::Color,
    maths::{matrix::Matrix, tuple::Tuple},
};
use std::f64::consts::PI;
use std::io::Result;

use crate::ppm_renderer::renderer::PpmRenderer;

pub struct Clock {
    notches: Vec<Tuple>,
}

impl Clock {
    pub fn new() -> Self {
        Self { notches: vec![] }
    }

    pub fn populate_notches(&mut self) {
        let mut point = Tuple::new_point(0.0, 1.0, 0.0);
        self.notches.push(point.clone());
        for _is_pivot_zero in 0..11 {
            point = Matrix::rotation_z(PI / 6.0) * point.clone();
            self.notches.push(point.clone());
        }
    }

    pub fn render(&self) -> Result<()> {
        let mut canvas = Canvas::new(300, 300);
        canvas.fill(Color::new(0.0, 0.0, 0.0));
        for point in &self.notches {
            let shifted_point = point.clone() * 140.0;
            canvas.write_pixel(
                (shifted_point.x() + 150.0).round() as usize,
                (shifted_point.y() + 150.0).round() as usize,
                Color::new(1.0, 0.0, 0.0),
            );
        }
        let ppm_renderer = PpmRenderer::from(&canvas);
        ppm_renderer.render()
    }
}
