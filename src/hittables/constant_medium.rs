use std::{
    sync::Arc, 
    f64::{
        INFINITY, 
        consts::E
    }
};

use rand::random;

use crate::types::{
    materials::Material, 
    texture::{
        Texture, 
        SolidColor
    }, 
    color::Color, 
    aabb::AABB, 
    ray::Ray, 
    vec3::Vec3
};

use super::hittable::{
    Hit, 
    HitRecord
};

/// A "participating media", such as fog or smoke
/// # Fields
/// `boundary` - the boundary of this participating media
/// 
/// `phase_function` - Material which describes how light is scattered
/// 
/// `neg_inv_density` - (-1 / density) of media. The higher the density of the media,
/// the more likely it is that a ray will get scattered by it.
pub struct ConstantMedium {
    boundary: Box<dyn Hit>,
    phase_function: Material,
    neg_inv_density: f64,
}

impl ConstantMedium {
    /// Create a new "participating media" object
    pub fn new(b: Box<dyn Hit>, d: f64, a: Arc::<dyn Texture>) -> ConstantMedium {
        ConstantMedium {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function: Material::Isotropic { 
                albedo: a
            }
        }
    }

    /// Usually, participating media have constant color
    pub fn solid(b: Box<dyn Hit>, d: f64, c: Color) -> ConstantMedium {
        ConstantMedium::new(b, d, Arc::new(SolidColor::from(c)))
    }
}

impl Hit for ConstantMedium {
    /// This currently assumes boundary is convex, so a ray can only
    /// pass through the media once.
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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

                    // The probability that the ray will scatter is based on its density and 
                    // how long that ray is in the boundary of the media
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
