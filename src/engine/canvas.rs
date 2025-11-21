use super::color::Color;

use std::fmt::Display;

#[derive(Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            buffer: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }

    pub fn write_pixel(self: &mut Self, x: usize, y: usize, color: Color) {
        self.buffer[x + (self.width * y)] = color;
    }

    pub fn pixel_at(self: &Self, x: usize, y: usize) -> &Color {
        &self.buffer[x + (self.width * y)]
    }

    pub fn fill(self: &mut Self, color: Color) {
        self.buffer = (0..self.width * self.height).map(|_i| color).collect();
    }
}

impl Display for Canvas {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            let mut row: String = "".to_string();
            for x in 0..self.width {
                row += format!(
                    "{} {} {} ",
                    self.pixel_at(x, y).r,
                    self.pixel_at(x, y).g,
                    self.pixel_at(x, y).b
                )
                .as_str();
            }
            println!("{row}");
        }
        Ok(())
    }
}
