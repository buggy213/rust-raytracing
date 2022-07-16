use super::{
    hittable_list::HittableList, 
    hittable::{
        Hit, 
        HitRecord
    }, 
    aarect::{
        XY, 
        XZ, 
        YZ
    }
};
use crate::{
    types::{
        vec3::{
            Point
        }, 
        ray::Ray, 
        aabb::AABB, 
        materials::Material
    }, 
    hittable_list
};

/// A rectangular prism constructed out of axis-aligned rectangles
/// # Fields
/// `min` - the corner of the prism with the smallest values of X, Y, and Z
/// 
/// `max` - the corner of the prism with the largest values of X, Y, and Z
/// 
/// `sides` - contains the six sides of the prism in a `HittableList`
pub struct Block {
    min: Point,
    max: Point,
    sides: HittableList
}

impl Block {
    /// Create a new block with given material
    pub fn new(min: Point, max: Point, material: Material) -> Block {
        let back = XY::new(
            material.clone(),
            min.x(),
            max.x(),
            min.y(),
            max.y(),
            max.z(),
        );
        let front = XY::new(
            material.clone(),
            min.x(),
            max.x(),
            min.y(),
            max.y(),
            min.z(),
        );
        let top = XZ::new(
            material.clone(),
            min.x(),
            max.x(),
            min.z(),
            max.z(),
            max.y(),
        );
        let bottom = XZ::new(
            material.clone(),
            min.x(),
            max.x(),
            min.z(),
            max.z(),
            min.y(),
        );
        let left = YZ::new(
            material.clone(),
            min.y(),
            max.y(),
            min.z(),
            max.z(),
            min.x(),
        );
        let right = YZ::new(
            material.clone(),
            min.y(),
            max.y(),
            min.z(),
            max.z(),
            min.x(),
        );
        let sides = hittable_list!(
            Box::new(left), 
            Box::new(right), 
            Box::new(top), 
            Box::new(bottom), 
            Box::new(back), 
            Box::new(front)
        );
        Block { min, max, sides }
    }
}

impl Hit for Block {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB::new(self.min, self.max))
    }
}