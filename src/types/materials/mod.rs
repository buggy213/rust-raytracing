use crate::hittables::hittable::HitRecord;

use super::{ray::Ray, color::Color, vec3::Vec3};

pub enum Material {
    Lambertian(Color),
    Metal(Color, f64),
    Dielectric(f64),
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

            Material::Metal(albedo, fuzz) => {
                let reflected = Vec3::reflect(Vec3::normalized(ray_in.direction), record.normal);
                let scattered = Ray { origin: record.p, direction: reflected + *fuzz * Vec3::random_in_unit_sphere() };
                if Vec3::dot(scattered.direction, record.normal) > 0.0 { Some((*albedo, scattered)) } else { None }
            }

            Material::Dielectric(index_of_refraction) => {
                let refraction_ratio = if record.front_face { 1.0 / index_of_refraction } else { *index_of_refraction };
                let unit_direction = Vec3::normalized(ray_in.direction);
                let refracted = Vec3::refract(unit_direction, record.normal, refraction_ratio);
                let scattered = Ray { origin: record.p, direction: refracted };
                Some((Vec3(1.0, 1.0, 1.0), scattered))
            }
        }
    }
}


