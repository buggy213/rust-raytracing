use std::{sync::Arc, f64::{INFINITY, consts::E}};

use rand::random;

use crate::types::{materials::Material, texture::{Texture, SolidColor}, color::Color, aabb::AABB, ray::{Ray, self}, vec3::Vec3};

use super::hittable::{Hittable, HitRecord};

pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    phase_function: Material,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(b: Box<dyn Hittable>, d: f64, a: Arc::<dyn Texture>) -> ConstantMedium {
        ConstantMedium {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Material::Isotropic { 
                albedo: a
            }
        }
    }

    pub fn solid(b: Box<dyn Hittable>, d: f64, c: Color) -> ConstantMedium {
        ConstantMedium::new(b, d, Arc::new(SolidColor::from(c)))
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // assumes boundary is convex
        if let Some(record1) = self.boundary.hit(r, -INFINITY, INFINITY) {
            if let Some(record2) = self.boundary.hit(r, record1.t + 0.0001, INFINITY) {
                let mut t1 = record1.t;
                let mut t2 = record2.t;
                t1 = f64::max(t1, t_min);
                t2 = f64::min(t2, t_max);
                if t1 < t2 {
                    if t1 < 0.0 {
                        t1 = 0.0;
                    }

                    let ray_length = r.direction.length();
                    let distance_inside_boundary = (t2 - t1) * ray_length;
                    let hit_distance = self.neg_inv_density * random::<f64>().log(E);
                    if hit_distance <= distance_inside_boundary {
                        let t = t1 + hit_distance / ray_length;
                        return Some(HitRecord::construct(
                            r.at(t), 
                            Vec3(1.0, 0.0, 0.0), // arbitrary 
                            t, 
                            r, 
                            &self.phase_function, 
                            0.0,  // arbitrary
                            0.0 // arbitrary
                        ));
                    }
                }
            }
        }
        None
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.boundary.bounding_box(t0, t1)
    }
}
