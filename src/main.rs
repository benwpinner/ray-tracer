use std::time::Instant;

use crate::example_projects::clock::Clock;

pub mod example_projects;

pub mod ppm_renderer;

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    let mut clock = Clock::new();
    clock.populate_notches();
    clock.render();
    let duration = start.elapsed();
    println!("ELAPSED {}", duration.as_millis());
    Ok(())
}
