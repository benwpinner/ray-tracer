use std::f64::consts::PI;

use approx::assert_abs_diff_eq;

use crate::engine::maths::{matrix::Matrix, tuple::Tuple};

#[test]
fn identical_matrices_are_equal() {
    let matrix1 = Matrix::new([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [4.0, 3.0, 2.0, 1.0],
    ]);
    let matrix2 = Matrix::new([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [4.0, 3.0, 2.0, 1.0],
    ]);
    assert_eq!(matrix1, matrix2);
    assert!(matrix1 == matrix2);
}

#[test]
fn not_identical_matrices_are_not_equal() {
    let matrix1 = Matrix::new([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [4.0, 3.0, 2.0, 1.0],
    ]);
    let matrix2 = Matrix::new([
        [3.0, 2.0, 3.0, 4.0],
        [5.0, 4.0, 7.0, 8.0],
        [9.0, 8.0, 5.0, 6.0],
        [4.0, 3.0, 2.0, 6.0],
    ]);
    assert_ne!(matrix1, matrix2);
    assert!(matrix1 != matrix2);
}

#[test]
fn multiplying_two_matrices() {
    let matrix1 = Matrix::new([
        [1.0, 2.0, 3.0, 4.0],
        [5.0, 6.0, 7.0, 8.0],
        [9.0, 8.0, 7.0, 6.0],
        [5.0, 4.0, 3.0, 2.0],
    ]);
    let matrix2 = Matrix::new([
        [-2.0, 1.0, 2.0, 3.0],
        [3.0, 2.0, 1.0, -1.0],
        [4.0, 3.0, 6.0, 5.0],
        [1.0, 2.0, 7.0, 8.0],
    ]);
    let matrix3 = matrix1 * matrix2;
    let expected_matrix = Matrix::new([
        [20.0, 22.0, 50.0, 48.0],
        [44.0, 54.0, 114.0, 108.0],
        [40.0, 58.0, 110.0, 102.0],
        [16.0, 26.0, 46.0, 42.0],
    ]);
    assert_eq!(expected_matrix, matrix3);
}

#[test]
fn multiplying_two_tuples() {
    let matrix = Matrix::new([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 4.0, 4.0, 2.0],
        [8.0, 6.0, 4.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    let tuple1 = Tuple::new(1.0, 2.0, 3.0, 1.0);
    let tuple2 = matrix * tuple1;
    let expected_tuple = Tuple::new(18.0, 24.0, 33.0, 1.0);
    assert_eq!(expected_tuple, tuple2);
}

#[test]
fn multiplying_matrix_by_identity() {
    let matrix = Matrix::new([
        [1.0, 2.0, 3.0, 4.0],
        [2.0, 4.0, 4.0, 2.0],
        [8.0, 6.0, 4.0, 1.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    let identity_matrix = Matrix::<4>::identity();
    let result = matrix * identity_matrix;
    assert_eq!(matrix, result);
}

#[test]
fn transpose_matrix() {
    let mut matrix = Matrix::new([
        [0.0, 9.0, 3.0, 0.0],
        [9.0, 8.0, 0.0, 8.0],
        [1.0, 8.0, 5.0, 3.0],
        [0.0, 0.0, 5.0, 8.0],
    ]);
    matrix.transpose();
    let expected_matrix = Matrix::new([
        [0.0, 9.0, 1.0, 0.0],
        [9.0, 8.0, 8.0, 0.0],
        [3.0, 0.0, 5.0, 5.0],
        [0.0, 8.0, 3.0, 8.0],
    ]);
    assert_eq!(expected_matrix, matrix);
}

#[test]
fn calculate_determinant() {
    let matrix = Matrix::new([[1.0, 5.0], [-3.0, 2.0]]);
    let determinant = matrix.determinant();
    assert_eq!(17.0, determinant);
}

#[test]
fn submatrix_of_3x3_matrix() {
    let matrix = Matrix::new([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
    let submatrix = matrix.submatrix(0, 2);
    let expected_matrix = Matrix::new([[-3.0, 2.0], [0.0, 6.0]]);
    assert_eq!(expected_matrix, submatrix);
}

#[test]
fn submatrix_of_4x4_matrix() {
    let matrix = Matrix::new([
        [-6.0, 1.0, 1.0, 6.0],
        [-8.0, 5.0, 8.0, 6.0],
        [-1.0, 0.0, 8.0, 2.0],
        [-7.0, 1.0, -1.0, 1.0],
    ]);
    let submatrix = matrix.submatrix(2, 1);
    let expected_matrix = Matrix::new([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);
    assert_eq!(expected_matrix, submatrix);
}

#[test]
fn minor_of_3x3_matrix() {
    let matrix = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
    let submatrix = matrix.submatrix(1, 0);
    assert_eq!(25.0, submatrix.determinant());
    assert_eq!(25.0, matrix.minor(1, 0));
}

#[test]
fn cofactor_of_3x3_matrix() {
    let matrix = Matrix::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
    assert_eq!(-12.0, matrix.minor(0, 0));
    assert_eq!(-12.0, matrix.cofactor(0, 0));
    assert_eq!(25.0, matrix.minor(1, 0));
    assert_eq!(-25.0, matrix.cofactor(1, 0));
}

#[test]
fn determinant_of_3x3_matrix() {
    let matrix = Matrix::new([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
    assert_eq!(56.0, matrix.cofactor(0, 0));
    assert_eq!(12.0, matrix.cofactor(0, 1));
    assert_eq!(-46.0, matrix.cofactor(0, 2));
    assert_eq!(-196.0, matrix.determinant());
}

#[test]
fn determinant_of_4x4_matrix() {
    let matrix = Matrix::new([
        [-2.0, -8.0, 3.0, 5.0],
        [-3.0, 1.0, 7.0, 3.0],
        [1.0, 2.0, -9.0, 6.0],
        [-6.0, 7.0, 7.0, -9.0],
    ]);
    assert_eq!(690.0, matrix.cofactor(0, 0));
    assert_eq!(447.0, matrix.cofactor(0, 1));
    assert_eq!(210.0, matrix.cofactor(0, 2));
    assert_eq!(51.0, matrix.cofactor(0, 3));
    assert_eq!(-4071.0, matrix.determinant());
}

#[test]
fn inverse_of_4x4_matrix() -> Result<(), String> {
    let matrix = Matrix::new([
        [-5.0, 2.0, 6.0, -8.0],
        [1.0, -5.0, 1.0, 8.0],
        [7.0, 7.0, -6.0, -7.0],
        [1.0, -3.0, 7.0, 4.0],
    ]);
    let inverse = matrix.inverse()?;
    let expected_inverse = Matrix::new([
        [0.21805, 0.45113, 0.24060, -0.04511],
        [-0.80827, -1.45677, -0.44361, 0.52068],
        [-0.07895, -0.22368, -0.05263, 0.19737],
        [-0.52256, -0.81391, -0.30075, 0.30639],
    ]);
    assert_abs_diff_eq!(532.0, matrix.determinant());
    assert_abs_diff_eq!(-160.0, matrix.cofactor(2, 3));
    assert_abs_diff_eq!(105.0, matrix.cofactor(3, 2));
    assert_abs_diff_eq!(expected_inverse, inverse);
    Ok(())
}

#[test]
fn inverse_of_4x4_matrix_2() -> Result<(), String> {
    let matrix = Matrix::new([
        [8.0, -5.0, 9.0, 2.0],
        [7.0, 5.0, 6.0, 1.0],
        [-6.0, 0.0, 9.0, 6.0],
        [-3.0, 0.0, -9.0, -4.0],
    ]);
    let inverse = matrix.inverse()?;
    let expected_inverse = Matrix::new([
        [-0.15385, -0.15385, -0.28205, -0.53846],
        [-0.07692, 0.12308, 0.02564, 0.03077],
        [0.35897, 0.35897, 0.43590, 0.92308],
        [-0.69231, -0.69231, -0.76923, -1.92308],
    ]);
    assert_abs_diff_eq!(expected_inverse, inverse);
    Ok(())
}

#[test]
fn inverse_of_4x4_matrix_3() -> Result<(), String> {
    let matrix = Matrix::new([
        [9.0, 3.0, 0.0, 9.0],
        [-5.0, -2.0, -6.0, -3.0],
        [-4.0, 9.0, 6.0, 4.0],
        [-7.0, 6.0, 6.0, 2.0],
    ]);
    let matrix2 = Matrix::new([
        [8.0, 2.0, 2.0, 2.0],
        [3.0, -1.0, 7.0, 0.0],
        [7.0, 0.0, 5.0, 4.0],
        [6.0, -2.0, 0.0, 5.0],
    ]);
    let result = matrix * matrix2;
    let inverse = matrix2.inverse()?;
    assert_abs_diff_eq!(matrix, result * inverse);
    Ok(())
}

#[test]
fn translate_point_via_matrix_multiplication() {
    let transform_matrix = Matrix::<4>::translation(5.0, -3.0, 2.0);
    let point = Tuple::new_point(-3.0, 4.0, 5.0);
    let expected_point = Tuple::new_point(2.0, 1.0, 7.0);
    assert_abs_diff_eq!(expected_point, transform_matrix * point);
}

#[test]
fn translate_point_via_inverse_matrix_multiplication() -> Result<(), String> {
    let transform_matrix = Matrix::<4>::translation(5.0, -3.0, 2.0).inverse()?;
    let point = Tuple::new_point(-3.0, 4.0, 5.0);
    let expected_point = Tuple::new_point(-8.0, 7.0, 3.0);
    assert_abs_diff_eq!(expected_point, transform_matrix * point);
    Ok(())
}

#[test]
fn vector_not_translated_via_matrix_multiplication() {
    let transform_matrix = Matrix::<4>::translation(5.0, -3.0, 2.0);
    let vector = Tuple::new_vector(-3.0, 4.0, 5.0);
    assert_abs_diff_eq!(vector, transform_matrix * vector);
}

#[test]
fn point_and_vector_scaled_via_matrix_multiplication() {
    let transform_matrix = Matrix::<4>::scaling(2.0, 3.0, 4.0);
    let point = Tuple::new_point(-4.0, 6.0, 8.0);
    let vector = Tuple::new_vector(-4.0, 6.0, 8.0);
    let expected_point = Tuple::new_point(-8.0, 18.0, 32.0);
    let expected_vector = Tuple::new_vector(-8.0, 18.0, 32.0);
    assert_abs_diff_eq!(expected_point, transform_matrix * point);
    assert_abs_diff_eq!(expected_vector, transform_matrix * vector);
}

#[test]
fn point_and_vector_scaled_via_inverse_matrix_multiplication() -> Result<(), String> {
    let transform_matrix = Matrix::<4>::scaling(2.0, 3.0, 4.0).inverse()?;
    let point = Tuple::new_point(-4.0, 6.0, 8.0);
    let vector = Tuple::new_vector(-4.0, 6.0, 8.0);
    let expected_point = Tuple::new_point(-2.0, 2.0, 2.0);
    let expected_vector = Tuple::new_vector(-2.0, 2.0, 2.0);
    assert_abs_diff_eq!(expected_point, transform_matrix * point);
    assert_abs_diff_eq!(expected_vector, transform_matrix * vector);
    Ok(())
}

#[test]
fn point_reflected_via_matrix_multiplication() -> Result<(), String> {
    let transform_matrix = Matrix::<4>::scaling(-2.0, 3.0, 4.0);
    let point = Tuple::new_point(2.0, 6.0, 8.0);
    let expected_point = Tuple::new_point(-4.0, 18.0, 32.0);
    assert_abs_diff_eq!(expected_point, transform_matrix * point);
    Ok(())
}

#[test]
fn point_rotated_around_x_via_matrix_multiplication() -> Result<(), String> {
    let transform_matrix1 = Matrix::rotation_x(PI / 4.0);
    let transform_matrix2 = Matrix::rotation_x(PI / 2.0);
    let point = Tuple::new_point(0.0, 1.0, 0.0);
    let expected_point1 = Tuple::new_point(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0);
    let expected_point2 = Tuple::new_point(0.0, 0.0, 1.0);
    let expected_point3 = Tuple::new_point(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
    assert_abs_diff_eq!(expected_point1, transform_matrix1 * point);
    assert_abs_diff_eq!(expected_point2, transform_matrix2 * point);
    assert_abs_diff_eq!(expected_point3, transform_matrix1.inverse()? * point);
    Ok(())
}

#[test]
fn point_rotated_around_y_via_matrix_multiplication() -> Result<(), String> {
    let transform_matrix1 = Matrix::rotation_y(PI / 4.0);
    let transform_matrix2 = Matrix::rotation_y(PI / 2.0);
    let point = Tuple::new_point(0.0, 0.0, 1.0);
    let expected_point1 = Tuple::new_point(f64::sqrt(2.0) / 2.0, 0.0, f64::sqrt(2.0) / 2.0);
    let expected_point2 = Tuple::new_point(1.0, 0.0, 0.0);
    let expected_point3 = Tuple::new_point(-f64::sqrt(2.0) / 2.0, 0.0, f64::sqrt(2.0) / 2.0);
    assert_abs_diff_eq!(expected_point1, transform_matrix1 * point);
    assert_abs_diff_eq!(expected_point2, transform_matrix2 * point);
    assert_abs_diff_eq!(expected_point3, transform_matrix1.inverse()? * point);
    Ok(())
}

#[test]
fn point_rotated_around_z_via_matrix_multiplication() -> Result<(), String> {
    let transform_matrix1 = Matrix::rotation_z(PI / 4.0);
    let transform_matrix2 = Matrix::rotation_z(PI / 2.0);
    let point = Tuple::new_point(0.0, 1.0, 0.0);
    let expected_point1 = Tuple::new_point(-f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0);
    let expected_point2 = Tuple::new_point(-1.0, 0.0, 0.0);
    let expected_point3 = Tuple::new_point(f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0);
    assert_abs_diff_eq!(expected_point1, transform_matrix1 * point);
    assert_abs_diff_eq!(expected_point2, transform_matrix2 * point);
    assert_abs_diff_eq!(expected_point3, transform_matrix1.inverse()? * point);
    Ok(())
}

#[test]
fn point_sheared_in_x_by_y() {
    let transform_matrix = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let point = Tuple::new_point(2.0, 3.0, 4.0);
    let expected_point = Tuple::new_point(5.0, 3.0, 4.0);
    assert_abs_diff_eq!(expected_point, transform_matrix * point);
}

#[test]
fn point_sheared_in_x_by_z() {
    let transform_matrix = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    let point = Tuple::new_point(2.0, 3.0, 4.0);
    let expected_point = Tuple::new_point(6.0, 3.0, 4.0);
    assert_abs_diff_eq!(expected_point, transform_matrix * point);
}

#[test]
fn point_sheared_in_y_by_x() {
    let transform_matrix = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    let point = Tuple::new_point(2.0, 3.0, 4.0);
    let expected_point = Tuple::new_point(2.0, 5.0, 4.0);
    assert_abs_diff_eq!(expected_point, transform_matrix * point);
}

#[test]
fn point_sheared_in_y_by_z() {
    let transform_matrix = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let point = Tuple::new_point(2.0, 3.0, 4.0);
    let expected_point = Tuple::new_point(2.0, 7.0, 4.0);
    assert_abs_diff_eq!(expected_point, transform_matrix * point);
}

#[test]
fn point_sheared_in_z_by_x() {
    let transform_matrix = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    let point = Tuple::new_point(2.0, 3.0, 4.0);
    let expected_point = Tuple::new_point(2.0, 3.0, 6.0);
    assert_abs_diff_eq!(expected_point, transform_matrix * point);
}

#[test]
fn point_sheared_in_z_by_y() {
    let transform_matrix = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    let point = Tuple::new_point(2.0, 3.0, 4.0);
    let expected_point = Tuple::new_point(2.0, 3.0, 7.0);
    assert_abs_diff_eq!(expected_point, transform_matrix * point);
}

#[test]
fn individual_transformations_performed_in_sequence() -> Result<(), String> {
    let rotation_matrix = Matrix::rotation_x(PI / 2.0);
    let scaling_matrix = Matrix::<4>::scaling(5.0, 5.0, 5.0);
    let translation_matrix = Matrix::<4>::translation(10.0, 5.0, 7.0);
    let mut point = Tuple::new_point(1.0, 0.0, 1.0);
    point = rotation_matrix * point;
    point = scaling_matrix * point;
    point = translation_matrix * point;
    let expected_point = Tuple::new_point(15.0, 0.0, 7.0);
    assert_abs_diff_eq!(expected_point, point);
    Ok(())
}

#[test]
fn chained_transformations_performed_in_reverse_order() -> Result<(), String> {
    let rotation_matrix = Matrix::rotation_x(PI / 2.0);
    let scaling_matrix = Matrix::<4>::scaling(5.0, 5.0, 5.0);
    let translation_matrix = Matrix::<4>::translation(10.0, 5.0, 7.0);
    let mut point = Tuple::new_point(1.0, 0.0, 1.0);
    point = (translation_matrix * scaling_matrix * rotation_matrix) * point;
    let expected_point = Tuple::new_point(15.0, 0.0, 7.0);
    assert_abs_diff_eq!(expected_point, point);
    Ok(())
}

// Scenario: Individual transformations are applied in sequence
// Given p ← point(1, 0, 1)
// And A ← rotation_x(π / 2)
// And B ← scaling(5, 5, 5)
// And C ← translation(10, 5, 7)
// # apply rotation first
// When p2 ← A * p
// Then p2 = point(1, -1, 0)
// # then apply scaling
// When p3 ← B * p2
// Then p3 = point(5, -5, 0)
// # then apply translation
// When p4 ← C * p3
// Then p4 = point(15, 0, 7)
// Scenario: Chained transformations must be applied in reverse order
// Given p ← point(1, 0, 1)
// And A ← rotation_x(π / 2)
// And B ← scaling(5, 5, 5)
// And C ← translation(10, 5, 7)
// When T ← C * B * A
// Then T * p = point(15, 0, 7)
