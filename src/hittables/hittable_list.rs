use std::fmt::Debug;

use crate::types::{
    ray::Ray, 
    aabb::AABB
};
use super::hittable::{
    Hit, 
    HitRecord
};

/// A collection of hittable objects
pub struct HittableList {
    objects: Vec<Box<dyn Hit>>
}

/// Dummy implementation of Debug for HittableList
/// TODO: actually provide helpful debug information
impl Debug for HittableList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("HittableList"))
    }
}

#[macro_export]
/// Convenience macro to create a HittableList from hittables
macro_rules! hittable_list {
    ( $( $a:expr ), * ) => {
        HittableList::from_vec(vec![$($a, )*])
    }
}

impl HittableList {
    /// Creates a empty HittableList
    pub fn new() -> HittableList {
        HittableList { objects: Vec::new() }
    }

    /// Creates a HittableList and populates it with some objects - primarily used by 
    /// `hittable_list` macro
    pub fn from_vec(vec: Vec<Box<dyn Hit>>) -> HittableList {
        HittableList { objects: vec }
    }

    /// Add a Hittable object to this HittableList
    pub fn add(&mut self, x: Box<dyn Hit>) {
        self.objects.push(x);
    }
}

impl Hit for HittableList {
    /// Checks every object in HittableList, then returns the HitRecord from
    /// the one that was hit first (or None if nothing was hit)
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut return_value: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for hittable in self.objects.iter() {
            if let Some(hit_record) = hittable.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                return_value = Some(hit_record);
            }
        }
        return_value
    }

    /// Create a bounding box encompassing every object in this HittableList
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }
        let aabb = self.objects[0].bounding_box(t0, t1);
        if aabb.is_none() {
            return None;
        }
        let mut aabb = aabb.unwrap();
        for hittable in self.objects.iter().skip(1) {
            if let Some(bounding_box) = hittable.bounding_box(t0, t1) {
                aabb = AABB::surrounding_box(aabb, bounding_box)
            }
            else {
                return None;
            }
        }
        Some(aabb)
    }
}