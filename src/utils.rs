use std::f64::consts::PI;

pub fn degrees_to_radians(deg: f64) -> f64 {
    return 2.0 * PI * deg / 360.0;
}