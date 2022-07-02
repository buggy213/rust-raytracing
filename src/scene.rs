use std::io::Write;

use crate::{hittables::hittable_list::HittableList, camera::Camera, types::color::Color, utils::clamp};

#[derive(Debug)]
pub struct Scene {
    pub camera: Camera,
    pub world: HittableList,
    pub aspect_ratio: f64,
    pub height: u32,
    pub width: u32,
    pub samples_per_pixel: u32
}

impl Scene {
    pub fn print_ppm(&self, color_data: &Vec<Color>, mut output: impl Write) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(output, "P3")?;
        writeln!(output, "{} {}", self.width, self.height)?;
        writeln!(output, "255")?;
        for pixel in color_data.iter() {
            writeln!(output, "{} {} {}", 
                    (clamp(pixel.0.sqrt(), 0.0, 1.0) * 256.0) as i32, 
                    (clamp(pixel.1.sqrt(), 0.0, 1.0) * 256.0) as i32,
                    (clamp(pixel.2.sqrt(), 0.0, 1.0) * 256.0) as i32)?;
        }

        Ok(())
    }
}