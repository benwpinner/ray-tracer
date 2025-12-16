use std::time::Instant;

use crate::example_projects::sphere_renderer::SphereRenderer;

pub mod example_projects;

pub mod ppm_renderer;

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    let mut sphere_renderer = SphereRenderer::new();
    sphere_renderer.render();
    let duration = start.elapsed();
    println!("ELAPSED {}", duration.as_millis());
    let start_threaded = Instant::now();
    let mut sphere_renderer = SphereRenderer::new();
    sphere_renderer.render_with_threading();
    let duration_threaded = start_threaded.elapsed();
    println!("ELAPSED THREADED {}", duration_threaded.as_millis());
    Ok(())
}
