use crate::{
    Material, 
    types::{
        aabb::AABB, 
        vec3::Vec3, 
        ray::Ray
    }
};
use super::hittable::{
    Hit, 
    HitRecord
};

// Axis-aligned rectangles - used a macro to avoid having to define all 3 separately
macro_rules! aarect {
    ($name:ident, $first_axis:tt, $second_axis:tt, $third_axis:tt) => {
        /// An axis-aligned rectangle
        /// # Fields:
        /// `a0`, `a1` - minimum and maximum bounds on "first axis" (e.g X axis for XY aligned rectangle)
        /// 
        /// `b0`, `b1` - minimum and maximum bounds on "second axis" (e.g Y axis for XY aligned rectangle)
        /// 
        /// `k` - position of rectangle along the axis perpendicular to the rectangle, the "third axis" 
        /// (Z for XY aligned rectangle)
        pub struct $name {
            pub material: Material,
            pub a0: f64,
            pub a1: f64,
            pub b0: f64,
            pub b1: f64,
            pub k: f64
        }

        impl $name {
            pub fn new(material: Material, a0: f64, a1: f64, b0: f64, b1: f64, k: f64) -> $name {
                $name {
                    material, a0, a1, b0, b1, k
                }
            }
        }
        
        impl Hit for $name {
            /// Check for when a ray's value for "third axis"
            /// (e.g Z axis for an XY-aligned rectangle) is equal to our position along
            /// that third axis, then determine whether it's positions along the other two axes
            /// we are aligned to are within our bounds - if so, then it's a hit
            fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
                let t = (self.k - r.origin.$third_axis) / r.direction.$third_axis;
                if t < t_min || t > t_max {
                    return None;
                }
                let a = r.origin.$first_axis + t * r.direction.$first_axis;
                let b = r.origin.$second_axis + t * r.direction.$second_axis;
                if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
                    return None;
                }
                let mut outward_normal = Vec3(0.0, 0.0, 0.0);
                outward_normal.$third_axis = 1.0;
                let u = (a - self.a0) / (self.a1 - self.a0);
                let v = (b - self.b0) / (self.b1 - self.b0);
                let hit_record = HitRecord::construct(r.at(t), outward_normal, t, r, &self.material, u, v);
                Some(hit_record)
            }

            fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
                // Nonzero (but very small) thickness to avoid issues with BVH
                let mut min = Vec3(0.0, 0.0, 0.0);
                let mut max = Vec3(0.0, 0.0, 0.0);
                min.$first_axis = self.a0;
                min.$second_axis = self.b0;
                min.$third_axis = self.k - 0.0001;
                max.$first_axis = self.a0;
                max.$second_axis = self.b0;
                max.$third_axis = self.k + 0.0001;
                Some(
                    AABB::new(min, max)
                )
            }
        }
    };
}

// Create the three types using the macro
aarect!(XY, 0, 1, 2);
aarect!(XZ, 0, 2, 1);
aarect!(YZ, 1, 2, 0);