use crate::utils::clamp;

use super::vec3::Vec3;

pub type Color = Vec3;
impl Color {
    pub fn write_color(color: Color, samples_per_pixel: i32) {
        let r = color.x() / samples_per_pixel as f64;
        let g = color.y() / samples_per_pixel as f64;
        let b = color.z() / samples_per_pixel as f64;

        // gamma 2
        println!("{} {} {}", (clamp(r.sqrt(), 0.0, 0.999) * 256.0) as i32, 
                        (clamp(g.sqrt(), 0.0, 0.999) * 256.0) as i32, 
                        (clamp(b.sqrt(), 0.0, 0.999) * 256.0) as i32);
    }
}