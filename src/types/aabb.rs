use super::{vec3::Vec3, ray::Ray};

/// Axis-aligned bounding box
/// Defined by 2 points
#[derive(Clone, Copy)]
pub struct AABB {
    pub minimum: Vec3,
    pub maximum: Vec3
}

impl AABB {
    pub fn new(minimum: Vec3, maximum: Vec3) -> AABB {
        AABB { minimum, maximum }
    }

    fn check_slab(min: f64, max: f64, ray_origin: f64, ray_dir: f64, t_min: f64, t_max: f64) -> bool {
        let x = (min - ray_origin) / ray_dir;
        let y =  (max - ray_origin) / ray_dir;
        let t0 = f64::min(x, y);
        let t1 = f64::max(x, y);

        let t_min = f64::max(t0, t_min);
        let t_max = f64::min(t1, t_max);

        t_min < t_max
    }

    /// Returns a box which surrounds both a and b
    pub fn surrounding_box(a: AABB, b: AABB) -> AABB {
        AABB { 
            minimum: Vec3(
                f64::min(a.minimum.0, b.minimum.0),
                f64::min(a.minimum.1, b.minimum.1),
                f64::min(a.minimum.2, b.minimum.2)
            ), 
            maximum: Vec3(
                f64::max(a.minimum.0, b.minimum.0),
                f64::max(a.minimum.1, b.minimum.1),
                f64::max(a.minimum.2, b.minimum.2)
            ) 
        }
    }

    pub fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> bool {
        AABB::check_slab(self.minimum.0, self.maximum.0, r.origin.0, r.direction.0, t_min, t_max)
        && AABB::check_slab(self.minimum.1, self.maximum.1, r.origin.1, r.direction.1, t_min, t_max)
        && AABB::check_slab(self.minimum.2, self.maximum.2, r.origin.2, r.direction.2, t_min, t_max)
    }
}
