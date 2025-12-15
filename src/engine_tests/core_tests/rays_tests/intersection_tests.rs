use crate::engine::core::{rays::intersection::Intersection, shapes::primitives::sphere::Sphere};

#[test]
fn hit_when_two_positive_ts() {
    let s = Sphere::new(1, 1.0);
    let i1 = Intersection::new(1.0, &s);
    let i2 = Intersection::new(2.0, &s);
    let hit = Intersection::hit(vec![&i1, &i2]);
    assert_eq!(hit.unwrap().t, i1.t);
}

#[test]
fn hit_when_one_negative_t() {
    let s = Sphere::new(1, 1.0);
    let i1 = Intersection::new(-1.0, &s);
    let i2 = Intersection::new(1.0, &s);
    let hit = Intersection::hit(vec![&i1, &i2]);
    assert_eq!(hit.unwrap().t, i2.t);
}

#[test]
fn hit_when_two_negative_ts() {
    let s = Sphere::new(1, 1.0);
    let i1 = Intersection::new(-2.0, &s);
    let i2 = Intersection::new(-1.0, &s);
    let hit = Intersection::hit(vec![&i1, &i2]);
    assert!(hit.is_none());
}

#[test]
fn hit_lowest_nonnegative_t() {
    let s = Sphere::new(1, 1.0);
    let i1 = Intersection::new(5.0, &s);
    let i2 = Intersection::new(7.0, &s);
    let i3 = Intersection::new(-3.0, &s);
    let i4 = Intersection::new(2.0, &s);
    let hit = Intersection::hit(vec![&i1, &i2, &i3, &i4]);
    assert_eq!(hit.unwrap().t, i4.t);
}
