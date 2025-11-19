use ray_tracer_rs::engine::{canvas::Canvas, color::Color, maths::tuple::Tuple};

use crate::{
    example_projects::projectile_simulator::projectile::Projectile,
    ppm_renderer::{self, renderer::PpmRenderer},
};

pub struct ProjectileSimulator {
    gravity: Tuple,
    wind: Tuple,
    pub projectile: Projectile,
    path: Vec<Tuple>,
    min_bounds: [f64; 2],
    max_bounds: [f64; 2],
}

impl ProjectileSimulator {
    pub fn new(gravity: Tuple, wind: Tuple, projectile: Projectile) -> Self {
        Self {
            gravity,
            wind,
            projectile,
            path: vec![projectile.pos],
            min_bounds: [projectile.pos[0], projectile.pos[1]],
            max_bounds: [projectile.pos[0], projectile.pos[1]],
        }
    }

    pub fn tick(&mut self) {
        self.projectile.pos += self.projectile.vel;
        self.projectile.vel += self.gravity + self.wind;
        self.path.push(self.projectile.pos);
        if self.projectile.pos[0] < self.min_bounds[0] {
            self.min_bounds[0] = self.projectile.pos[0];
        } else if self.projectile.pos[0] > self.min_bounds[0] {
            self.max_bounds[0] = self.projectile.pos[0];
        }

        if self.projectile.pos[1] < self.min_bounds[1] {
            self.min_bounds[1] = self.projectile.pos[1];
            println!("MINB {} {}", self.projectile.pos[1], self.min_bounds[1]);
        } else if self.projectile.pos[1] > self.max_bounds[1] {
            self.max_bounds[1] = self.projectile.pos[1];
            println!("MAXB {} {}", self.projectile.pos[1], self.max_bounds[1]);
        }
    }

    pub fn render(&mut self) {
        let mut canvas = Canvas::new(300, 300);
        let height = self.max_bounds[1] - self.min_bounds[1];
        let width = self.max_bounds[0] - self.min_bounds[0];
        let ratio = (300.0 - 20.0) / height.max(width);
        println!("{} {}", self.min_bounds[0], self.max_bounds[0]);
        println!("{} {}", self.min_bounds[1], self.max_bounds[1]);
        println!("{} {} {}", height, width, ratio);

        let mut shift_x = 10.0;
        let mut shift_y = 10.0;
        if self.min_bounds[0] < 0.0 {
            shift_x -= self.min_bounds[0];
            // println!("{shift_x}");
        }
        if self.min_bounds[1] < 0.0 {
            shift_y -= self.min_bounds[1];
            println!("{shift_y}");
        }
        for point in &self.path {
            let x = point[0] * ratio;
            let y = (point[1] * ratio).round();
            // println!("UNSHIFTED {} {} {} {}", point[1], ratio, y, shift_y);
            // println!("SHIFTED {}", y + shift_y);
            // println!("RATIO {}, {}", x, (point[1] * ratio).round());
            let shifted_x = (x + shift_x).round() as usize;
            let shifted_y = canvas.height - (y - shift_y) as usize;
            if shifted_x > 0
                && shifted_x < canvas.width
                && shifted_y > 0
                && shifted_y < canvas.height
            {
                canvas.write_pixel(shifted_x, shifted_y, Color::new(1.0, 0.0, 0.0));
            }
        }
        let ppm_renderer = PpmRenderer::from(&canvas);
        ppm_renderer.render();
    }
}
