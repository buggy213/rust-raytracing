use rand::random;
use super::vec3::Vec3;

pub type Color = Vec3;

pub fn random_color() -> Color {
    Vec3(random(), random(), random())
}