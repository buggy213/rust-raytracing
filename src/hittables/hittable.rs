use crate::types::{vec3::{Point, Vec3}, ray::Ray, materials::Material, aabb::AABB};

/// Any type that implements Hit can be Hit by 
/// In addition, these must be safe to send across threads 
/// (since they're essentially just some geometry data and material references, they should be)
pub trait Hit: Send + Sync {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    /// Returns a bounding box around this hittable from time t0 to t1
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
}

/// A HitRecord bundles together information about a ray hitting something that implements Hit
/// # Fields
/// `p` - the point of intersection
/// 
/// `normal` - the normal vector of the thing which was hit; always points against the direction of the ray
/// 
/// `t` - how far along the ray did the hit occur
/// 
/// `front_face` - whether the ray hit the front or the back of the object, as determined
/// by a dot product with the ray and the outward facing normal of the object
/// 
/// `material` - a borrowed reference to a Material
/// Material lifetime should outlive HitRecord lifetime - HitRecords are basically ephemeral,
/// so this should be a non-issue
/// 
/// `u`, `v` - the uv coordinates of the hit. Used for texture mapping
#[derive(Debug)]
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

    /// Construct a hitrecord from a normal that was interpolated from vertex normals. Solves the problem of certain rays 
    /// being rendered as if they were hitting the back side of an object due to vertex normal interpolation even though the underlying
    /// geometry means they were hitting the front - provide "correct" face normal instead to do the calculation
    pub fn construct_from_interpolated_normal<'a>(p: Point, interpolated_normal: Vec3, front_face: bool, 
            t: f64, ray: Ray, material: &'a Material, u: f64, v: f64) -> HitRecord<'a> {
        let normal = if front_face { interpolated_normal } else { -interpolated_normal };
        
        HitRecord { p: p, normal: normal, t: t, front_face: front_face, material, u, v }
    }
}