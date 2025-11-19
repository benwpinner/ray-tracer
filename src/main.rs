use std::time::Instant;

use ray_tracer_rs::engine::{canvas::Canvas, color::Color, maths::tuple::Tuple};

use crate::{
    example_projects::projectile_simulator::{
        projectile::Projectile, projectile_simulator::ProjectileSimulator,
    },
    ppm_renderer::renderer::PpmRenderer,
};

pub mod example_projects;

pub mod ppm_renderer;

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    let projectile = Projectile::new(
        Tuple::new_point(0.0, 1.0, 0.0),
        Tuple::new_vector(2.0, 5.0, 0.0),
    );
    let mut projectile_simulator = ProjectileSimulator::new(
        Tuple::new_vector(0.0, -0.1, 0.0),
        Tuple::new_vector(-0.01, 0.01, 0.0),
        projectile,
    );

    while projectile_simulator.projectile.pos[1] + projectile_simulator.projectile.vel[1] > 0.0 {
        projectile_simulator.tick();
    }

    projectile_simulator.render();
    let duration = start.elapsed();
    println!("ELAPSED {}", duration.as_millis());
    Ok(())
}
