use crate::engine::core::shapes::shape::Shape;

pub struct Intersection<'a> {
    pub t: f64,
    pub shape: &'a dyn Shape,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, shape: &'a dyn Shape) -> Self {
        Self { t, shape }
    }

    pub fn hit(intersections: Vec<&'a Intersection>) -> Option<&'a Self> {
        let mut result: Option<&Intersection<'_>> = None;

        for intersection in intersections {
            if intersection.t > 0.0 && (result.is_none() || intersection.t < result.unwrap().t) {
                result = Some(intersection);
            }
        }
        result
    }
}
