use std::{io, fmt::format};

use image::{io::Reader as ImageReader, DynamicImage, Rgb, GenericImageView};

use crate::utils::{perlin::Perlin};

use super::{vec3::{Point, Vec3}, color::Color};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Point) -> Color;
}

pub struct SolidColor {
    color: Color
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Point) -> Color {
        self.color
    }
}

impl From<Color> for SolidColor {
    fn from(color: Color) -> Self {
        SolidColor { color }
    }
}

pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point) -> Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 { self.odd.value(u, v, p) } else { self.even.value(u, v, p) }
    }
}

impl CheckerTexture {
    pub fn make_solid_checkered(odd: Color, even: Color) -> CheckerTexture {
        let odd: SolidColor = odd.into();
        let even: SolidColor = even.into();
        CheckerTexture {
            odd: Box::new(odd),
            even: Box::new(even)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point) -> Color {
        Vec3(1.0, 1.0, 1.0) * 0.5 * (1.0 + f64::sin(self.scale * p.z() + 10.0 * self.noise.turbulence(p, 7)))
    }
}

impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture { noise: Perlin::new(), scale }
    }
}

pub struct ImageTexture {
    pub width: u32,
    pub height: u32,
    pub data: DynamicImage
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: Point) -> Color {
        let u = u.clamp(0.0, 1.0);
        
        // image coordinates have origin at top left, we have origin at bottom left
        let v = 1.0 - v.clamp(0.0, 1.0);

        let i = ((u * self.width as f64) as u32).clamp(0, self.width - 1);
        let j = ((v * self.height as f64) as u32).clamp(0, self.height - 1);
        
        let pixel = self.data.get_pixel(i, j);
        Vec3(pixel.0[0] as f64 / 255.0, pixel.0[1] as f64 / 255.0, pixel.0[2] as f64 / 255.0)
    }
}

impl From<&str> for ImageTexture {
    fn from(filename: &str) -> ImageTexture {
        let img = ImageReader::open(filename).expect("failed to open texture file");
        let img = img.decode().expect("failed to decode texture file");

        ImageTexture {  
            width: img.width(),
            height: img.height(),
            data: img
        }
    }
}