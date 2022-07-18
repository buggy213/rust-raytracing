use std::{
    sync::Arc, fmt::Debug
};

use rand::random;

use crate::hittables::hittable::HitRecord;

use super::{
    ray::Ray, 
    color::Color, 
    vec3::{
        Vec3, 
        Point
    }, 
    texture::Texture
};

// TODO: implement a "mix shader"
// similar to this: https://docs.blender.org/manual/en/2.79/render/cycles/nodes/types/shaders/mix.html
#[derive(Clone)]
pub enum Material {
    Lambertian {
        albedo: Arc<dyn Texture>
    },
    Metal {
        albedo: Color,
        fuzz: f64
    },
    Dielectric {
        index_of_refraction: f64
    },
    DiffuseLight {
        emit: Arc<dyn Texture>
    },
    Isotropic {
        albedo: Arc<dyn Texture>
    }
}

impl Debug for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Material").finish()
    }
}

impl Material {

    fn reflectance(cosine: f64, ior: f64) -> f64 {
        let mut r0 = (1.0 - ior) / (1.0 + ior);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
    }

    pub fn scatter(&self, ray_in: Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = record.normal + Vec3::random_unit_vector();

                if scatter_direction.near_zero() {
                    scatter_direction = record.normal;
                }

                let scattered = Ray { origin: record.p, direction: scatter_direction, ..ray_in };
                Some((albedo.value(record.u, record.v, record.p), scattered))
            }

            Material::Metal { albedo, fuzz } => {
                let reflected = Vec3::reflect(Vec3::normalized(ray_in.direction), record.normal);
                let scattered = Ray { origin: record.p, direction: reflected + *fuzz * Vec3::random_in_unit_sphere(), ..ray_in };
                if Vec3::dot(scattered.direction, record.normal) > 0.0 { Some((*albedo, scattered)) } else { None }
            }

            Material::Dielectric { index_of_refraction } => {
                let refraction_ratio = if record.front_face { 1.0 / index_of_refraction } else { *index_of_refraction };
                let unit_direction = Vec3::normalized(ray_in.direction);
                
                let cos_theta = f64::min(1.0, Vec3::dot(-unit_direction, record.normal));
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                
                let direction = if cannot_refract || Material::reflectance(cos_theta, refraction_ratio) > random() {
                    Vec3::reflect(unit_direction, record.normal)
                }
                else {                    
                    Vec3::refract(unit_direction, record.normal, refraction_ratio)
                };
                let scattered = Ray { origin: record.p, direction: direction, ..ray_in };
                Some((Vec3(1.0, 1.0, 1.0), scattered))
            },
            Material::Isotropic { albedo } => {
                let scattered = Ray {
                    origin: record.p,
                    direction: Vec3::random_in_unit_sphere(),
                    time: ray_in.time
                };
                let attenuation = albedo.value(record.u, record.v, record.p);
                Some((attenuation, scattered))
            },
            _ => {
                None
            }
        }
    }
    pub fn emitted(&self, u: f64, v: f64, p: Point) -> Option<Color> {
        match self {
            Material::DiffuseLight { emit } => {
                Some(emit.value(u, v, p))
            }
            _ => {
                None
            }
        }
    }
}


