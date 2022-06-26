use crate::{types::{vec3::Vec3, ray::Ray}, utils::degrees_to_radians};

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left: Vec3
}

impl Camera {
    pub fn default() -> Camera {
        Camera::custom(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, -1.0), Vec3(0.0, 1.0, 0.0), crate::ASPECT_RATIO, 90.0)
    }

    pub fn custom(look_from: Vec3, look_at: Vec3, v_up: Vec3, aspect_ratio: f64, vfov: f64) -> Camera {

        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();

        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = viewport_height * aspect_ratio;
        let focal_length: f64 = 1.0;

        let w = Vec3::normalized(look_from - look_at);
        let u = Vec3::normalized(Vec3::cross(v_up, w));
        let v = Vec3::cross(w, u);
    
        let origin: Vec3 = look_from;
        let horizontal: Vec3 = viewport_width * u;
        let vertical: Vec3 = viewport_height * v;
        let lower_left: Vec3 = origin - horizontal / 2.0 - vertical / 2.0 - w * focal_length;
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