use crate::types::{vec3::{Point, Vec3}, ray::Ray, materials::Material, aabb::AABB};
pub trait Hittable: Send + Sync {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    /// Returns a bounding box around this hittable from time t0 to t1
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
}

// Material lifetime should outlive HitRecord lifetime
pub struct HitRecord<'a> {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a Material,
    pub u: f64,
    pub v: f64
}

impl HitRecord<'_> {
    // Instead of having normal of hitrecord always point towards outside of the object, it will always
    // point against the direction of the ray. Apparently this makes things easier? (though I'm used to the other option)
    pub fn construct<'a>(p: Point, outward_normal: Vec3, t: f64, ray: Ray, material: &'a Material, u: f64, v: f64) -> HitRecord<'a> {
        let front_face = Vec3::dot(ray.direction, outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        
        HitRecord { p: p, normal: normal, t: t, front_face: front_face, material, u, v }
    }
}