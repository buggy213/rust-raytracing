use std::fmt::Debug;

use crate::types::{ray::Ray, aabb::AABB};
use super::hittable::{Hittable, HitRecord};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl Debug for HittableList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("HittableList"))
    }
}

#[macro_export]
macro_rules! hittable_list {
    ( $( $a:expr ), * ) => {
        HittableList { objects: vec![$($a, )*] }
    }
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: Vec::new() }
    }   

    pub fn add(&mut self, x: Box<dyn Hittable>) {
        self.objects.push(x);
    }
}

impl Hittable for HittableList {
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