use super::{hittable_list::HittableList, hittable::{Hittable, HitRecord}, aarect::{XY, XZ, YZ}};
use crate::{types::{vec3::{Vec3, Point}, ray::Ray, aabb::AABB, materials::Material}, hittable_list};
pub struct Block {
    min: Point,
    max: Point,
    sides: HittableList
}

impl Block {
    pub fn new(min: Point, max: Point, material: Material) -> Block {
        let back = XY {
            a0: min.x(),
            a1: max.x(),
            b0: min.y(),
            b1: max.y(),
            k: max.z(),
            material: material.clone()
        };
        let front = XY {
            a0: min.x(),
            a1: max.x(),
            b0: min.y(),
            b1: max.y(),
            k: min.z(),
            material: material.clone()
        };
        let top = XZ {
            a0: min.x(),
            a1: max.x(),
            b0: min.z(),
            b1: max.z(),
            k: max.y(),
            material: material.clone()
        };
        let bottom = XZ {
            a0: min.x(),
            a1: max.x(),
            b0: min.z(),
            b1: max.z(),
            k: min.y(),
            material: material.clone()
        };
        let left = YZ {
            a0: min.y(),
            a1: max.y(),
            b0: min.z(),
            b1: max.z(),
            k: min.x(),
            material: material.clone()
        };
        let right = YZ {
            a0: min.y(),
            a1: max.y(),
            b0: min.z(),
            b1: max.z(),
            k: min.x(),
            material: material.clone()
        };
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

impl Hittable for Block {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB::new(self.min, self.max))
    }
}