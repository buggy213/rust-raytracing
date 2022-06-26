use super::vec3::Vec3;

pub type Color = Vec3;
impl Color {
    pub fn write_color(color: Color) {
        println!("{} {} {}", (color.x() * 255.999) as i32, 
                        (color.y() * 255.999) as i32, 
                        (color.z() * 255.999) as i32);
    }
}