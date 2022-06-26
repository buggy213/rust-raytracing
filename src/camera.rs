use crate::{types::{vec3::Vec3, ray::Ray}, utils::degrees_to_radians};

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left: Vec3
}

impl Camera {
    pub fn default(aspect_ratio: f64, vfov: f64) -> Camera {

        let theta = degrees_to_radians(vfov);
        let h = theta.tan();

        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = viewport_height * aspect_ratio;
        let focal_length: f64 = 1.0;
    
        let origin: Vec3 = Vec3(0.0, 0.0, 0.0);
        let horizontal: Vec3 = Vec3(viewport_width, 0.0, 0.0);
        let vertical: Vec3 = Vec3(0.0, viewport_height, 0.0);
        let lower_left: Vec3 = origin - horizontal / 2.0 - vertical / 2.0 - Vec3(0.0, 0.0, focal_length);
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray { origin: self.origin, direction: self.lower_left + u * self.horizontal + v * self.vertical - self.origin }
    }
}