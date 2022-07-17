use std::f64::EPSILON;

use crate::types::{
    vec3::{
        Point, 
        Vec3
    }, 
    aabb::AABB, 
    ray::Ray, 
    materials::Material
};

use super::hittable::{
    Hit, 
    HitRecord
};
/// A single triangle - for a polygonal mesh, refer to mesh.rs
/// Defined by three points; normal is defined by (v1 - v0) × (v2 - v0)
/// following right-hand convention
pub struct Triangle {
    v0: Point,
    v1: Point,
    v2: Point,
    material: Material
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material: Material) -> Triangle {
        Triangle { v0, v1, v2, material }
    }

    /// Implementation of Moller-Trumbore ray-triangle intersection algorithm
    /// Returns: (t, u, v) - t where ray intersects triangle, u / v canonical barycentric coordinates
    /// or None if ray does not intersect triangle 
    pub fn moller_trumbore(v0: Vec3, v1: Vec3, v2: Vec3, r: Ray, t_min: f64, t_max: f64)
        -> Option<(f64, f64, f64)> {
        let e1 = v1 - v0;
        let e2 = v2 - v0;
        
        let P = Vec3::cross(r.direction, e2);
        let denom = Vec3::dot(P, e1);

        if denom < EPSILON && denom > -EPSILON{
            return None;
        }

        let T = r.origin - v0;

        let u = Vec3::dot(P, T) / denom;
        if u < 0.0 || u > 1.0 {
            return None;
        }
        
        let Q = Vec3::cross(T, e1);
        let v = Vec3::dot(Q, r.direction) / denom;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = Vec3::dot(Q, e2) / denom;
        if t < t_min || t > t_max {
            None
        }
        else {
            Some((t, u, v))
        }
    }
}

impl Hit for Triangle {
    

    #[allow(non_snake_case)]
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        
        let (t, u, v) = Triangle::moller_trumbore(self.v0, self.v1, self.v2, r, t_min, t_max)?;
        let e1 = self.v1 - self.v0;
        let e2 = self.v2 - self.v0;

        let normal = Vec3::normalized(Vec3::cross(e1, e2));

        Some(HitRecord::construct(
            r.at(t), 
            normal, 
            t, 
            r, 
            &self.material, 
            u, 
            v
        ))
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        let x_min = f64::min(self.v0.0, f64::min(self.v1.0, self.v2.0));
        let y_min = f64::min(self.v0.1, f64::min(self.v1.1, self.v2.1));
        let z_min = f64::min(self.v0.2, f64::min(self.v1.2, self.v2.2));
        let x_max = f64::max(self.v0.0, f64::max(self.v1.0, self.v2.0));
        let y_max = f64::max(self.v0.1, f64::max(self.v1.1, self.v2.1));
        let z_max = f64::max(self.v0.2, f64::max(self.v1.2, self.v2.2));
        
        Some(AABB::new(Vec3(x_min, y_min, z_min), Vec3(x_max, y_max, z_max)))
    }
}