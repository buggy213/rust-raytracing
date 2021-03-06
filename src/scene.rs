use std::{io::{Write}, path::Path};

use crate::{hittables::hittable_list::HittableList, camera::Camera, types::color::Color, Background};

#[derive(Debug)]
pub struct Scene {
    pub camera: Camera,
    pub world: HittableList,
    pub aspect_ratio: f64,
    pub height: u32,
    pub width: u32,
    pub samples_per_pixel: u32,
    pub background: Background
}

impl Scene {
    pub fn print_ppm(&self, color_data: &Vec<Color>, mut output: impl Write) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(output, "P3")?;
        writeln!(output, "{} {}", self.width, self.height)?;
        writeln!(output, "255")?;
        for pixel in color_data.iter() {
            writeln!(output, "{} {} {}", 
                    (pixel.0.sqrt().clamp(0.0, 1.0) * 256.0) as i32, 
                    (pixel.1.sqrt().clamp(0.0, 1.0) * 256.0) as i32,
                    (pixel.2.sqrt().clamp(0.0, 1.0) * 256.0) as i32)?;
        }

        Ok(())
    }

    pub fn save_png(&self, color_data: &Vec<Color>, output: &Path) {
        let buf: Vec<u8> = color_data.iter().flat_map(|x| {
            [(x.0.sqrt().clamp(0.0, 1.0) * 256.0) as u8, 
             (x.1.sqrt().clamp(0.0, 1.0) * 256.0) as u8, 
             (x.2.sqrt().clamp(0.0, 1.0) * 256.0) as u8]
        }).collect();
        image::save_buffer_with_format(
            output, 
            buf.as_slice(),
            self.width, 
            self.height, 
            image::ColorType::Rgb8, 
            image::ImageFormat::Png
        ).expect("failed to write png");
    }
}