use super::super::engine::canvas::*;
use super::super::engine::color::*;

#[test]
fn create_canvas() {
    let canvas = Canvas::new(10, 20);
    let expected_color = Color::new(0f32, 0f32, 0f32);
    for pixel in canvas.buffer {
        assert_eq!(expected_color, pixel);
    }
}

#[test]
fn write_pixel_to_canvas() {
    let mut canvas = Canvas::new(10, 20);
    let expected_color = Color::new(1f32, 0f32, 0f32);
    canvas.write_pixel(2, 3, Color::new(1f32, 0f32, 0f32));
    assert_eq!(&expected_color, canvas.pixel_at(2, 3));
}
