use std::{
    fs::File,
    io::{Result, Write},
};

use ray_tracer_rs::engine::{canvas::Canvas, color::Color};

pub struct PpmRenderer {
    width: usize,
    height: usize,
    pub canvas: Canvas,
}

impl PpmRenderer {
    pub fn render(&self, filename: &str) -> Result<()> {
        let mut ppm = "P3".to_string();
        ppm += format!("\n{} {}", self.width, self.height).as_str();
        ppm += format!("\n255").as_str();
        for y in 0..self.height {
            ppm += "\n";
            for (i, x) in (0..self.width).enumerate() {
                let pixel = self.canvas.pixel_at(x, y);
                ppm += format!(
                    "{} {} {}",
                    (pixel.r * 255.0).clamp(0.0, 255.0).round(),
                    (pixel.g * 255.0).clamp(0.0, 255.0).round(),
                    (pixel.b * 255.0).clamp(0.0, 255.0).round()
                )
                .as_str();
                if i < self.width as usize {
                    ppm += " ";
                }
            }
        }
        let mut file = File::create(filename)?;
        file.write_all(ppm.as_bytes())?;
        Ok(())
    }
}

impl From<&Canvas> for PpmRenderer {
    fn from(value: &Canvas) -> Self {
        Self {
            width: value.width,
            height: value.height,
            canvas: value.clone(),
        }
    }
}
