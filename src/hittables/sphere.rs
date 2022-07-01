use super::hittable::Hittable;
use crate::types::materials::Material;
use crate::types::ray::Ray;
use crate::types::vec3::Point;
use crate::types::vec3::Vec3;
use super::hittable::HitRecord;
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Material
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin - self.center;
        let a = ray.direction.square_magnitude();
        let half_b = Vec3::dot(oc, ray.direction);
        let c = oc.square_magnitude() - self.radius * self.radius;
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
        let outward_normal = (ray.at(root) - self.center) / self.radius;
        let record: HitRecord = HitRecord::construct(ray.at(root), outward_normal, root, ray, &self.material);
        Some(record)
    }
}