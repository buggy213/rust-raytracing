use crate::{hittables::sphere::Sphere, types::vec3::Vec3};
use crate::types::ray::Ray;
use super::hittable::{Hittable, HitRecord};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>
}

#[macro_export]
macro_rules! from {
    ( $( $a:expr ), * ) => {
        HittableList { objects: vec![$($a, )*] }
    }
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: Vec::new() }
    }   
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut return_value: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for hittable in &self.objects {
            if let Some(hit_record) = hittable.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                return_value = Some(hit_record);
            }
        }
        return_value
    }
}