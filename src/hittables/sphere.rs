use std::f64::consts::PI;

use super::hittable::Hittable;
use crate::types::materials::Material;
use crate::types::ray::Ray;
use crate::types::vec3::Point;
use crate::types::vec3::Vec3;
use crate::types::aabb::AABB;
use super::hittable::HitRecord;

// a sphere should indeed outlive the material attached to it
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Material
}

impl Sphere {
    /// Returns an Option<(f64, Vec3)> containing the outward facing normal and length along ray if the ray hit a sphere, and None otherwise
    pub fn hit_sphere(center: Vec3, radius: f64, ray: Ray, t_min: f64, t_max: f64) -> Option<(f64, Vec3)> {
        let oc: Vec3 = ray.origin - center;
        let a = ray.direction.square_magnitude();
        let half_b = Vec3::dot(oc, ray.direction);
        let c = oc.square_magnitude() - radius * radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if t_min > root || root > t_max {
            root = (-half_b + sqrtd) / a;
            if t_min > root || root > t_max {
                return None;
            }
        }
        let outward_normal = (ray.at(root) - center) / radius;
        Some((root, outward_normal))
    }

    /// p: point on unit sphere; returns spherical UV coordinates (u, v) where u corresponds to latitude and v to longitude 
    /// (both ranging from 0 to 1)
    pub fn get_sphere_uv(p: Point) -> (f64, f64) {
        let theta = f64::acos(-p.y());
        let phi = f64::atan2(-p.z(), p.x()) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some((root, outward_normal)) = Sphere::hit_sphere(self.center, self.radius, ray, t_min, t_max) {
            let (u, v) = Sphere::get_sphere_uv(outward_normal);
            let record: HitRecord = HitRecord::construct(ray.at(root), outward_normal, root, ray, &self.material, u, v);
            Some(record)
        }
        else {
            None
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        let r = Vec3(self.radius, self.radius, self.radius);
        Some(AABB::new(self.center - r, self.center + r))
    }
}