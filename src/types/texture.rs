use crate::utils::perlin::Perlin;

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