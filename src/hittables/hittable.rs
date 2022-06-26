use crate::types::{vec3::{Point, Vec3}, ray::Ray};
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool
}

impl HitRecord {
    // Instead of having normal of hitrecord always point towards outside of the object, it will always
    // point against the direction of the ray. Apparently this makes things easier? (though I'm used to the other option)
    pub fn construct(p: Point, outward_normal: Vec3, t: f64, ray: &Ray) -> HitRecord {
        let front_face = Vec3::dot(ray.direction, outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        HitRecord { p: p, normal: normal, t: t, front_face: front_face }
    }
}