use crate::hittables::hittable::HitRecord;

use super::{ray::Ray, color::Color, vec3::Vec3};

pub enum Material {
    Lambertian(Color),
    Metal(Color)
}

impl Material {
    pub fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Material::Lambertian(albedo) => {
                let mut scatter_direction = record.normal + Vec3::random_unit_vector();

                if scatter_direction.near_zero() {
                    scatter_direction = record.normal;
                }

                let scattered = Ray { origin: record.p, direction: scatter_direction };
                Some((*albedo, scattered))
            }

            Material::Metal(albedo) => {
                let reflected = Vec3::reflect(Vec3::normalized(ray_in.direction), record.normal);
                let scattered = Ray { origin: record.p, direction: reflected };
                Some((*albedo, scattered))
            }
        }
    }
}


