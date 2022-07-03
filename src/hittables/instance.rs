use crate::types::{transform::{TransformData, Transform, InverseTransform}, aabb::AABB, vec3::Vec3, ray::Ray};

use super::hittable::{Hittable, HitRecord};

pub struct Instance {
    transform: TransformData,
    object: Box<dyn Hittable>
}

macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = max!($($z),*);
        if $x > y {
            $x
        } else {
            y
        }
    }}
}
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = min!($($z),*);
        if $x < y {
            $x
        } else {
            y
        }
    }}
}

impl Hittable for Instance {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let transformed_ray = r.inverse_transform(self.transform);
        if let Some(hitrecord) = self.object.hit(transformed_ray, t_min, t_max) {
            let transformed_p = hitrecord.p.transform(self.transform);
            let transformed_normal = (hitrecord.p + hitrecord.normal).transform(self.transform) - transformed_p;
            Some(HitRecord::construct(
                transformed_p,
                transformed_normal,
                hitrecord.t,
                r,
                hitrecord.material,
                hitrecord.u,
                hitrecord.v
            ))
        }
        else {
            None
        }
    }
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        // check all 8 corners of bounding box
        if let Some(aabb) = self.object.bounding_box(t0, t1) {
            let a0 = aabb.minimum;
            let a1 = Vec3(aabb.maximum.x(), aabb.minimum.y(), aabb.minimum.z());
            let a2 = Vec3(aabb.maximum.x(), aabb.minimum.y(), aabb.maximum.z());
            let a3 = Vec3(aabb.minimum.x(), aabb.minimum.y(), aabb.maximum.z());
            let a4 = Vec3(aabb.minimum.x(), aabb.maximum.y(), aabb.minimum.z());
            let a5 = Vec3(aabb.maximum.x(), aabb.maximum.y(), aabb.minimum.z());
            let a6 = aabb.maximum;
            let a7 = Vec3(aabb.minimum.x(), aabb.maximum.y(), aabb.maximum.z());
            let a0 = a0.transform(self.transform);
            let a1 = a1.transform(self.transform);
            let a2 = a2.transform(self.transform);
            let a3 = a3.transform(self.transform);
            let a4 = a4.transform(self.transform);
            let a5 = a5.transform(self.transform);
            let a6 = a6.transform(self.transform);
            let a7 = a7.transform(self.transform);
            let min = Vec3(
                min!(a0.0, a1.0, a2.0, a3.0, a4.0, a5.0, a6.0, a7.0),
                min!(a0.1, a1.1, a2.1, a3.1, a4.1, a5.1, a6.1, a7.1),
                min!(a0.2, a1.2, a2.2, a3.2, a4.2, a5.2, a6.2, a7.2)
            );
            let max = Vec3(
                max!(a0.0, a1.0, a2.0, a3.0, a4.0, a5.0, a6.0, a7.0),
                max!(a0.1, a1.1, a2.1, a3.1, a4.1, a5.1, a6.1, a7.1),
                max!(a0.2, a1.2, a2.2, a3.2, a4.2, a5.2, a6.2, a7.2)
            );
            Some(AABB::new(min, max))
        }
        else {
            None
        }
    }
}

impl Instance {
    pub fn new(object: Box<dyn Hittable>, transform: TransformData) -> Instance {
        Instance { transform, object }
    }
}