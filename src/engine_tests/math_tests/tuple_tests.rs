use crate::engine::maths::tuple::Tuple;

#[test]
fn create_point_and_vector() {
    let point = Tuple::new_point(1.0, 2.0, 3.0);
    let vector = Tuple::new_vector(1.0, 2.0, 3.0);
    assert_eq!(point[3], 1.0);
    assert_eq!(vector[3], 0.0);
}

#[test]
fn add_point_and_vector() {
    let point = Tuple::new_point(1.0, 2.0, 3.0);
    let vector = Tuple::new_vector(1.0, 1.0, 4.0);
    let expected = Tuple::new(2.0, 3.0, 7.0, 1.0);
    assert_eq!(expected, point + vector);
}

#[test]
fn add_point_and_vector2() {
    let point = Tuple::new_point(1.0, 2.0, 3.0);
    let vector = Tuple::new_vector(1.0, 1.0, 4.0);
    let expected = Tuple::new(2.0, 3.0, 7.0, 1.0);
    assert_eq!(expected, point + vector);
}

#[test]
fn add_assign_point_and_vector() {
    let mut point = Tuple::new_point(1.0, 2.0, 3.0);
    let vector = Tuple::new_vector(1.0, 1.0, 4.0);
    let expected = Tuple::new(2.0, 3.0, 7.0, 1.0);
    point += vector;
    assert_eq!(expected, point);
}

#[test]
fn sub_point_from_point() {
    let point1 = Tuple::new_point(1.0, 2.0, 3.0);
    let point2 = Tuple::new_point(1.0, 1.0, 4.0);
    let expected = Tuple::new(0.0, 1.0, -1.0, 0.0);
    assert_eq!(expected, point1 - point2);
}

#[test]
fn sub_assign_point_from_point() {
    let mut point1 = Tuple::new_point(1.0, 2.0, 3.0);
    let point2 = Tuple::new_point(1.0, 1.0, 4.0);
    let expected = Tuple::new(0.0, 1.0, -1.0, 0.0);
    point1 -= point2;
    assert_eq!(expected, point1);
}

#[test]
fn sub_vector_from_point() {
    let point = Tuple::new_point(3.0, 2.0, 6.0);
    let vector = Tuple::new_vector(1.0, 1.0, 4.0);
    let expected = Tuple::new(2.0, 1.0, 2.0, 1.0);
    assert_eq!(expected, point - vector);
}

#[test]
fn sub_assign_vector_from_point() {
    let mut point1 = Tuple::new_point(3.0, 2.0, 6.0);
    let point2 = Tuple::new_vector(1.0, 1.0, 4.0);
    let expected = Tuple::new(2.0, 1.0, 2.0, 1.0);
    point1 -= point2;
    assert_eq!(expected, point1);
}

#[test]
fn sub_vector_from_vector() {
    let vector1 = Tuple::new_vector(3.0, 2.0, 6.0);
    let vector2 = Tuple::new_vector(1.0, 1.0, 4.0);
    let expected = Tuple::new(2.0, 1.0, 2.0, 0.0);
    assert_eq!(expected, vector1 - vector2);
}

#[test]
fn sub_assign_vector_from_vector() {
    let mut vector1 = Tuple::new_vector(3.0, 2.0, 6.0);
    let vector2 = Tuple::new_vector(1.0, 1.0, 4.0);
    let expected = Tuple::new(2.0, 1.0, 2.0, 0.0);
    vector1 -= vector2;
    assert_eq!(expected, vector1);
}

#[test]
fn mult_tuple_by_scalar() {
    let vector1 = Tuple::new(3.0, -2.0, -6.0, 3.0);
    let expected = Tuple::new(10.5, -7.0, -21.0, 10.5);
    assert_eq!(expected, vector1 * 3.5);
}

#[test]
fn mult_assign_vector_from_vector() {
    let mut vector1 = Tuple::new(3.0, -2.0, -6.0, 3.0);
    let expected = Tuple::new(10.5, -7.0, -21.0, 10.5);
    vector1 *= 3.5;
    assert_eq!(expected, vector1);
}

#[test]
fn negate_vector_and_point() {
    let mut vector = Tuple::new_vector(3.0, -2.0, 6.0);
    let mut point = Tuple::new_point(1.0, 1.0, -4.0);
    let expected_vector = Tuple::new(-3.0, 2.0, -6.0, 0.0);
    let expected_point = Tuple::new(-1.0, -1.0, 4.0, -1.0);
    vector.negate();
    point.negate();
    assert_eq!(expected_vector, vector);
    assert_eq!(expected_point, point);
}

#[test]
fn magnitude_of_vectors() {
    let vector1 = Tuple::new_vector(1.0, 0.0, 0.0);
    let vector2 = Tuple::new_vector(0.0, 1.0, 0.0);
    let vector3 = Tuple::new_vector(0.0, 0.0, 1.0);
    let vector4 = Tuple::new_vector(1.0, 2.0, 3.0);
    let vector5 = Tuple::new_vector(-1.0, -2.0, -3.0);
    assert_eq!(1.0, vector1.magnitude());
    assert_eq!(1.0, vector2.magnitude());
    assert_eq!(1.0, vector3.magnitude());
    assert_eq!(f64::sqrt(14.0), vector4.magnitude());
    assert_eq!(f64::sqrt(14.0), vector5.magnitude());
}

#[test]
fn normalize_vector() {
    let mut vector1 = Tuple::new_vector(4.0, 0.0, 0.0);
    let mut vector2 = Tuple::new_vector(1.0, 2.0, 3.0);
    let expected_vector1 = Tuple::new_vector(1.0, 0.0, 0.0);
    let expected_vector2 = Tuple::new_vector(
        1.0 / f64::sqrt(14.0),
        2.0 / f64::sqrt(14.0),
        3.0 / f64::sqrt(14.0),
    );
    vector1.normalize();
    vector2.normalize();
    assert_eq!(expected_vector1, vector1);
    assert_eq!(expected_vector2, vector2);
    assert_eq!(1.0, vector1.magnitude());
    assert_eq!(1.0, vector2.magnitude());
}

#[test]
fn dot_product_of_two_vectors() {
    let vector1 = Tuple::new_vector(1.0, 2.0, 3.0);
    let vector2 = Tuple::new_vector(2.0, 3.0, 4.0);
    let expected = 20.0;
    assert_eq!(expected, Tuple::dot_product(&vector1, &vector2));
}

#[test]
fn cross_product_of_two_vectors() {
    let vector1 = Tuple::new_vector(1.0, 2.0, 3.0);
    let vector2 = Tuple::new_vector(2.0, 3.0, 4.0);
    let expected_vector1 = Tuple::new_vector(-1.0, 2.0, -1.0);
    let expected_vector2 = Tuple::new_vector(1.0, -2.0, 1.0);
    assert_eq!(expected_vector1, Tuple::cross_product(&vector1, &vector2));
    assert_eq!(expected_vector2, Tuple::cross_product(&vector2, &vector1));
}
