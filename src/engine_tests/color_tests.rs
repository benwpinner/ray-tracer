use super::super::engine::color::*;

#[test]
fn create_color() {
    let color = Color::new(0.2, 0.4, 0.8);
    assert_eq!(0.2, color.r);
    assert_eq!(0.4, color.g);
    assert_eq!(0.8, color.b);
}

#[test]
fn add_two_colors() {
    let color1 = Color::new(0.2, 0.4, 0.8);
    let color2 = Color::new(0.7, 0.6, 0.3);
    let expected_color = Color::new(0.9, 1.0, 1.1);
    assert_eq!(expected_color, color1 + color2);
}

#[test]
fn add_assign_two_colors() {
    let mut color1 = Color::new(0.2, 0.4, 0.8);
    let color2 = Color::new(0.7, 0.6, 0.3);
    let expected_color = Color::new(0.9, 1.0, 1.1);
    color1 += color2;
    assert_eq!(expected_color, color1);
}

#[test]
fn subtract_two_colors() {
    let color1 = Color::new(0.2, 0.4, 0.8);
    let color2 = Color::new(0.7, 0.6, 0.3);
    let expected_color = Color::new(0.5, 0.2, -0.5);
    assert_eq!(expected_color, color2 - color1);
}

#[test]
fn subtract_assign_two_colors() {
    let mut color1 = Color::new(0.2, 0.4, 0.8);
    let color2 = Color::new(0.7, 0.6, 0.3);
    let expected_color = Color::new(0.5, 0.2, -0.5);
    color1 -= color2;
    assert_eq!(expected_color, color1);
}

#[test]
fn multiply_two_colors() {
    let color1 = Color::new(1f32, 0.5, 0.75);
    let color2 = Color::new(0.7, 0.6, 0.25);
    let expected_color = Color::new(0.7, 0.3, -0.5);
    assert_eq!(expected_color, color2 * color1);
}

#[test]
fn multiply_assign_two_colors() {
    let mut color1 = Color::new(1f32, 0.5, 0.75);
    let color2 = Color::new(0.7, 0.6, 0.25);
    let expected_color = Color::new(0.5, 0.2, -0.5);
    color1 *= color2;
    assert_eq!(expected_color, color1);
}
