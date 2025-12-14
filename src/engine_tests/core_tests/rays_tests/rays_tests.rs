use approx::assert_abs_diff_eq;

use crate::engine::{
    core::rays::rays::Ray,
    maths::{matrix::Matrix, tuple::Tuple},
};

#[test]
fn compute_point_along_ray_at_point_in_time() {
    let origin = Tuple::new_point(2.0, 3.0, 4.0);
    let direction = Tuple::new_vector(1.0, 0.0, 0.0);
    let ray = Ray::new(origin, direction);
    let expected_position1 = Tuple::new_point(2.0, 3.0, 4.0);
    let expected_position2 = Tuple::new_point(3.0, 3.0, 4.0);
    let expected_position3 = Tuple::new_point(1.0, 3.0, 4.0);
    let expected_position4 = Tuple::new_point(4.5, 3.0, 4.0);
    assert_abs_diff_eq!(expected_position1, ray.position(0.0));
    assert_abs_diff_eq!(expected_position2, ray.position(1.0));
    assert_abs_diff_eq!(expected_position3, ray.position(-1.0));
    assert_abs_diff_eq!(expected_position4, ray.position(2.5));
}

#[test]
fn translate_a_ray() {
    let origin = Tuple::new_point(1.0, 2.0, 3.0);
    let direction = Tuple::new_vector(0.0, 1.0, 0.0);
    let ray = Ray::new(origin, direction);
    let transform_matrix = Matrix::translation(3.0, 4.0, 5.0);
    let transformed_ray = ray.transform(&transform_matrix);
    let expected_origin = Tuple::new_point(4.0, 6.0, 8.0);
    let expected_direction = Tuple::new_vector(0.0, 1.0, 0.0);
    assert_abs_diff_eq!(expected_origin, transformed_ray.origin);
    assert_abs_diff_eq!(expected_direction, transformed_ray.direction);
}

#[test]
fn scaling_a_ray() {
    let origin = Tuple::new_point(1.0, 2.0, 3.0);
    let direction = Tuple::new_vector(0.0, 1.0, 0.0);
    let ray = Ray::new(origin, direction);
    let transform_matrix = Matrix::scaling(2.0, 3.0, 4.0);
    let transformed_ray = ray.transform(&transform_matrix);
    let expected_origin = Tuple::new_point(2.0, 6.0, 12.0);
    let expected_direction = Tuple::new_vector(0.0, 3.0, 0.0);
    assert_abs_diff_eq!(expected_origin, transformed_ray.origin);
    assert_abs_diff_eq!(expected_direction, transformed_ray.direction);
}
