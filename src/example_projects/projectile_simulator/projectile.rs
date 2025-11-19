use ray_tracer_rs::engine::maths::tuple::Tuple;

#[derive(Debug, Clone, Copy)]
pub struct Projectile {
    pub pos: Tuple,
    pub vel: Tuple,
}

impl Projectile {
    pub fn new(pos: Tuple, vel: Tuple) -> Self {
        Self { pos, vel }
    }
}
