use itertools::Itertools;
use rand::{Rng, prelude::Distribution, distributions::Standard};

use crate::hittables::hittable::{Hittable, HitRecord};

use super::{aabb::AABB, ray::Ray};

/// A node in the bounding volume hierarchy
pub struct BVHNode {
    left: Box<dyn Hittable>,
    right: Option<Box<dyn Hittable>>,
    aabb: AABB
}

impl Hittable for BVHNode {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.aabb.hit(r, t_min, t_max) {
            return None;
        }
        let hit_left = self.left.hit(r, t_min, t_max);
        let hit_right = if self.right.is_none() {
            None
        } 
        else {
            let t_max = if let Some(ref rec) = hit_left { 
                rec.t 
            }
            else { 
                t_max 
            };
            self.right.as_ref().unwrap().hit(r, t_min, t_max)
        };
        if hit_right.is_some() {
            hit_right
        }
        else if hit_left.is_some() {
            hit_left
        }
        else {
            None
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(self.aabb)
    }
}

impl BVHNode {
    // TODO: make this less bad
    /*pub fn new(hittables: Vec<Box<dyn Hittable>>, t0: f64, t1: f64) -> BVHNode {
        
    }*/
}