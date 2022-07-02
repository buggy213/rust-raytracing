use crate::{types::{vec3::Vec3, ray::Ray}, utils::{degrees_to_radians, random_range}};
#[derive(Debug)]
pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left: Vec3,

    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,

    // shutter open / close
    time0: f64,
    time1: f64
}

impl Camera {
    pub fn default() -> Camera {
        Camera::custom(
            Vec3(0.0, 0.0, 0.0), 
            Vec3(0.0, 0.0, -1.0), 
            Vec3(0.0, 1.0, 0.0), 
            16.0 / 9.0, 
            90.0, 
            0.0, 
            1.0,
            0.0,
            0.0
        )
    }

    pub fn custom(look_from: Vec3, 
                  look_at: Vec3, 
                  v_up: Vec3, 
                  aspect_ratio: f64, 
                  vfov: f64, 
                  aperture: f64, 
                  focus_distance: f64,
                  time0: f64,
                  time1: f64) -> Camera {

        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();

        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = viewport_height * aspect_ratio;

        let w = Vec3::normalized(look_from - look_at);
        let u = Vec3::normalized(Vec3::cross(v_up, w));
        let v = Vec3::cross(w, u);
    
        let origin: Vec3 = look_from;
        let horizontal: Vec3 = focus_distance * viewport_width * u;
        let vertical: Vec3 = focus_distance * viewport_height * v;
        let lower_left: Vec3 = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_distance;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left,
            u,
            v,
            w,
            lens_radius,
            time0,
            time1
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rand = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rand.x() + self.v * rand.y();
        Ray { 
            origin: self.origin + offset, 
            direction: self.lower_left + s * self.horizontal + t * self.vertical - self.origin - offset,
            time: random_range(self.time0, self.time1)
        }
    }
}