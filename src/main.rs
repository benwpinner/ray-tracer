use std::time::Instant;

use crate::example_projects::sphere_renderer::SphereRender;

pub mod example_projects;

pub mod ppm_renderer;

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    let mut sphere_renderer = SphereRender::new();
    sphere_renderer.render();
    let duration = start.elapsed();
    println!("ELAPSED {}", duration.as_millis());
    Ok(())
}
