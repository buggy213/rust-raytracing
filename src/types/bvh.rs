use std::{cmp::Ordering, mem::swap};

use rand::Rng;

use crate::hittables::hittable::{Hittable, HitRecord};

use super::{aabb::AABB, ray::Ray};

/// A node in the bounding volume hierarchy

pub enum BVHNode {
    Leaf {
        val: Box<dyn Hittable>,
        bounding_box: AABB
    },
    Branch {
        left: Box<dyn Hittable>,
        right: Box<dyn Hittable>,
        bounding_box: AABB
    }
}

impl BVHNode {
    pub fn make(mut objects: Vec<Box<dyn Hittable>>, t0: f64, t1: f64) -> BVHNode {
        let comparator = match rand::thread_rng().gen_range(0..3) {
            0 => |a: AABB, b: AABB| if a.minimum.0 < b.minimum.0 { Ordering::Less } else { Ordering::Greater },
            1 => |a: AABB, b: AABB| if a.minimum.1 < b.minimum.1 { Ordering::Less } else { Ordering::Greater },
            _ => |a: AABB, b: AABB| if a.minimum.2 < b.minimum.2 { Ordering::Less } else { Ordering::Greater },
        };
        if objects.len() == 1 {
            let leaf = objects.remove(0);
            let bounding_box = leaf.bounding_box(t0, t1).expect("objects in BVH must be boundable");
            BVHNode::Leaf {
                val: leaf,
                bounding_box: bounding_box
            }
        }
        else if objects.len() == 2 {
            let mut a_AABB = objects[0].bounding_box(t0, t1).expect("objects in BVH must be boundable");
            let mut b_AABB = objects[1].bounding_box(t0, t1).expect("objects in BVH must be boundable");
            if comparator(a_AABB, b_AABB) == Ordering::Greater {
                objects.swap(0, 1);
                swap(&mut a_AABB, &mut b_AABB)
            }
            BVHNode::Branch { 
                left: Box::new(BVHNode::Leaf {
                    val: objects.remove(0),
                    bounding_box: a_AABB
                }),
                right: Box::new(BVHNode::Leaf {
                    val: objects.remove(0), 
                    bounding_box: b_AABB
                }),
                bounding_box: AABB::surrounding_box(a_AABB, b_AABB)
            }
        }
        else {
            objects.sort_by(|a, b| {
                comparator(a.bounding_box(t0, t1).expect("objects in BVH must be boundable"),
                           b.bounding_box(t0, t1).expect("objects in BVH must be boundable"))
            });
            let right = objects.split_off(objects.len() / 2);
            let left = BVHNode::make(objects, t0, t1);
            let right = BVHNode::make(right, t0, t1);
            let bounding_box = AABB::surrounding_box(left.get_bounding_box(), right.get_bounding_box());
            BVHNode::Branch { 
                left: Box::new(left),
                right: Box::new(right),
                bounding_box
            }
        }

    }
    fn get_bounding_box(&self) -> AABB {
        match self {
            Self::Leaf { bounding_box, .. } => {
                *bounding_box
            },
            Self::Branch { bounding_box, .. } => {
                *bounding_box
            }
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Self::Leaf { val, bounding_box } => {
                if !bounding_box.hit(r, t_min, t_max) {
                    None
                }
                else {
                    val.hit(r, t_min, t_max)
                }
            },
            Self::Branch { left, right, bounding_box } => {
                if !bounding_box.hit(r, t_min, t_max) {
                    None
                }
                else {
                    let left = left.hit(r, t_min, t_max);
                    let t_max = if let Some(ref left_hitrecord) = left { left_hitrecord.t } else { t_max };
                    let right = right.hit(r, t_min, t_max);
                    if right.is_some() {
                        right
                    }
                    else if left.is_some() {
                        left
                    }
                    else {
                        None
                    }
                }
            }
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        match self {
            Self::Leaf { bounding_box, .. } => {
                Some(*bounding_box)
            },
            Self::Branch { bounding_box, .. } => {
                Some(*bounding_box)
            }
        }
    }
}