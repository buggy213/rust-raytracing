use std::f64::consts::PI;
use rand::random;

pub fn degrees_to_radians(deg: f64) -> f64 {
    2.0 * PI * deg / 360.0
}

pub fn random_range(min: f64, max: f64) -> f64 {
    random::<f64>() * (max - min) + min
}

pub trait Clamp<T> {
    fn clamp(&self, min: T, max: T) -> T;
}

impl Clamp<f64> for f64 {
    fn clamp(&self, min: f64, max: f64) -> f64 {
        if *self < min { min } else if *self > max { max } else { *self }
    }
}

pub mod perlin {
    use rand::{Rng};

    use crate::types::vec3::{Point, Vec3};

    const POINT_COUNT: usize = 256;

    pub struct Perlin {
        perm_x: [i32; POINT_COUNT],
        perm_y: [i32; POINT_COUNT],
        perm_z: [i32; POINT_COUNT],
        ranvec: [Vec3; POINT_COUNT]
    }

    impl Perlin {

        pub fn new() -> Perlin {
            let mut ranvec = [Vec3(0.0, 0.0, 0.0); POINT_COUNT];
            for i in 0..POINT_COUNT {
                ranvec[i] = Vec3::normalized(Vec3::random_vec_bounded(-1.0, 1.0));
            }

            Perlin { 
                perm_x: Perlin::generate_perm(), 
                perm_y: Perlin::generate_perm(), 
                perm_z: Perlin::generate_perm(),
                ranvec
            }
        }

        pub fn noise(&self, p: Point) -> f64 {
            let u = p.x() - p.x().floor();
            let v = p.y() - p.y().floor();
            let w = p.z() - p.z().floor();

            let i = p.x().floor() as i32;
            let j = p.y().floor() as i32;
            let k = p.z().floor() as i32;

            let mut c = [[[Vec3(0.0, 0.0, 0.0); 2]; 2]; 2];
            for di in 0..2 {
                for dj in 0..2 {
                    for dk in 0..2 {
                        c[di][dj][dk] = self.ranvec[
                            (self.perm_x[((i + di as i32) & 255) as usize] ^
                            self.perm_y[((j + dj as i32) & 255) as usize] ^
                            self.perm_z[((k + dk as i32) & 255) as usize]) as usize
                        ];
                    }
                }
            }
            
            Perlin::perlin_interp(c, u, v, w)
        }

        pub fn turbulence(&self, p: Point, depth: u32) -> f64 {
            let mut accum = 0.0;
            let mut temp_p = p;
            let mut weight = 1.0;
            for _ in 0..depth {
                accum += weight * self.noise(temp_p);
                weight *= 0.5;
                temp_p *= 2.0;
            }
            accum.abs()
        }

        fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
            let mut accum = 0.0;
            let uu = u * u * (3.0 - 2.0 * u);
            let vv = v * v * (3.0 - 2.0 * v);
            let ww = w * w * (3.0 - 2.0 * w);

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let i = i as f64;
                        let j = j as f64;
                        let k = k as f64;
                        let weight_vec = Vec3(u - i, v - j, w - k);
                        accum += (i * uu + (1.0 - i) * (1.0 - uu)) *
                                 (j * vv + (1.0 - j) * (1.0 - vv)) *
                                 (k * ww + (1.0 - k) * (1.0 - ww)) *
                                 Vec3::dot(c[i as usize][j as usize][k as usize], weight_vec)
                    }
                }
            }
            accum
        }

        fn generate_perm() -> [i32; POINT_COUNT] {
            let mut p: [i32; POINT_COUNT] = [0; POINT_COUNT];
            for i in 0..POINT_COUNT {
                p[i] = i as i32;
            }
            Perlin::permute(&mut p, POINT_COUNT);
            p
        }

        fn permute(perm: &mut [i32; POINT_COUNT], n: usize) {
            for i in (1..n).rev() {
                let target = rand::thread_rng().gen_range(0..=i);
                perm.swap(i, target);
            }
        }
    }
}