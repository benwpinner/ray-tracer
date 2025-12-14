use crate::engine::{
    core::{
        rays::rays::Ray,
        shapes::{primitives::sphere::Sphere, shape::Shape},
    },
    maths::{matrix::Matrix, tuple::Tuple},
};

#[test]
fn ray_intersects_at_two_points() -> Result<(), String> {
    let origin = Tuple::new_point(0.0, 0.0, -5.0);
    let direction = Tuple::new_vector(0.0, 0.0, 1.0);
    let ray = Ray::new(origin, direction);
    let sphere = Sphere::new(1, 1.0);
    let xs = sphere.intersect(ray)?;
    assert_eq!(2, xs.len());
    assert_eq!(4.0, xs[0].t);
    assert_eq!(6.0, xs[1].t);
    Ok(())
}

#[test]
fn ray_intersects_at_a_tangent() -> Result<(), String> {
    let origin = Tuple::new_point(0.0, 1.0, -5.0);
    let direction = Tuple::new_vector(0.0, 0.0, 1.0);
    let ray = Ray::new(origin, direction);
    let sphere = Sphere::new(1, 1.0);
    let xs = sphere.intersect(ray)?;
    assert_eq!(1, xs.len());
    assert_eq!(5.0, xs[0].t);
    Ok(())
}

#[test]
fn ray_misses() -> Result<(), String> {
    let origin = Tuple::new_point(0.0, 2.0, -5.0);
    let direction = Tuple::new_vector(0.0, 0.0, 1.0);
    let ray = Ray::new(origin, direction);
    let sphere = Sphere::new(1, 1.0);
    let xs = sphere.intersect(ray)?;
    assert_eq!(0, xs.len());
    Ok(())
}

#[test]
fn ray_originates_inside_sphere() -> Result<(), String> {
    let origin = Tuple::new_point(0.0, 0.0, 0.0);
    let direction = Tuple::new_vector(0.0, 0.0, 1.0);
    let ray = Ray::new(origin, direction);
    let sphere = Sphere::new(1, 1.0);
    let xs = sphere.intersect(ray)?;
    assert_eq!(2, xs.len());
    assert_eq!(-1.0, xs[0].t);
    assert_eq!(1.0, xs[1].t);
    Ok(())
}

#[test]
fn sphere_is_behind_ray_origin() -> Result<(), String> {
    let origin = Tuple::new_point(0.0, 0.0, 5.0);
    let direction = Tuple::new_vector(0.0, 0.0, 1.0);
    let ray = Ray::new(origin, direction);
    let sphere = Sphere::new(1, 1.0);
    let xs = sphere.intersect(ray)?;
    assert_eq!(2, xs.len());
    assert_eq!(-6.0, xs[0].t);
    assert_eq!(-4.0, xs[1].t);
    Ok(())
}

#[test]
fn sphere_is_created_with_identity_transform() {
    let sphere = Sphere::new(1, 1.0);
    assert_eq!(Matrix::identity(), sphere.transform);
}

#[test]
fn transforming_a_sphere() {
    let mut sphere = Sphere::new(1, 1.0);
    let translation_matrix = Matrix::translation(2.0, 3.0, 4.0);
    sphere.set_transform(&translation_matrix);
    assert_eq!(translation_matrix, sphere.transform);
}
