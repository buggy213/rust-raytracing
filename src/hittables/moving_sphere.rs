use crate::types::{vec3::Vec3, materials::Material, ray::Ray, aabb::AABB};

use super::{hittable::{Hittable, HitRecord}, sphere::Sphere};

pub struct MovingSphere {
    pub start_position: Vec3,
    pub end_position: Vec3,
    
    pub start_time: f64,
    pub end_time: f64,

    pub radius: f64,
    pub material: Material
}

impl MovingSphere {
    fn center(&self, time: f64) -> Vec3 {
        let fraction = (time - self.start_time) / (self.end_time - self.start_time);
        Vec3::lerp(self.start_position, self.end_position, fraction)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some((root, outward_normal)) = Sphere::hit_sphere(self.center(ray.time), self.radius, ray, t_min, t_max) {
            let (u, v) = Sphere::get_sphere_uv(outward_normal);
            let record: HitRecord = HitRecord::construct(ray.at(root), outward_normal, root, ray, &self.material, u, v);
            Some(record)
        }
        else {
            None
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let r = Vec3(self.radius, self.radius, self.radius);
        let box0 = AABB::new(self.center(t0) - r, self.center(t0) + r);
        let box1 = AABB::new(self.center(t1) - r, self.center(t1) + r);
        Some(AABB::surrounding_box(box0, box1))
    }
}