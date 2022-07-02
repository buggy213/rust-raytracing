use std::f64::consts::PI;
use clap::clap_derive::ArgEnum;
use rand::random;
use crate::Background;
use crate::Material::*;
use crate::Vec3;
use crate::Sphere;
use crate::Material;
use crate::camera::Camera;
use crate::color;

use crate::hittable_list;
use crate::hittables::hittable_list::HittableList;
use crate::hittables::moving_sphere::MovingSphere;
use crate::scene::Scene;
use crate::types::texture::CheckerTexture;
use crate::types::texture::ImageTexture;
use crate::types::texture::NoiseTexture;
use crate::types::texture::SolidColor;

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

#[derive(Clone, ArgEnum)]
pub enum PresetScene {
    JumpingBalls,
    TwoSpheres,
    TwoPerlinSpheres,
    Earth
}

impl PresetScene {
    pub fn get(&self, samples_per_pixel: u32) -> Scene {
        match self {
            PresetScene::JumpingBalls => random_scene(samples_per_pixel),
            PresetScene::TwoSpheres => two_spheres(samples_per_pixel),
            PresetScene::TwoPerlinSpheres => two_perlin_spheres(samples_per_pixel),
            PresetScene::Earth => earth(samples_per_pixel)
        }
    }
}

pub fn random_scene(samples_per_pixel: u32) -> Scene {
    pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    

    let look_from = Vec3(13.0, 2.0, 3.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    let focus_dist = 10.0;
    let camera = Camera::custom(
        look_from,
        look_at,
        Vec3(0.0, 1.0, 0.0), 
        ASPECT_RATIO, 
        20.0,
        0.1,
        focus_dist,
        0.0,
        1.0
    );

    let mut world = HittableList::new();
    let ground_texture: CheckerTexture = CheckerTexture::make_solid_checkered(Vec3(0.2, 0.3, 0.1), Vec3(0.9, 0.9, 0.9));
    let ground_material= Lambertian { 
        albedo: Box::new(ground_texture)
    };
    world.add(Box::new(Sphere {
        center: Vec3(0.0, -1000.0, 0.0), 
        radius: 1000.0, 
        material: ground_material 
    }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random::<f64>();
            let center = Vec3(a as f64 + 0.9 * random::<f64>(), 0.2, b as f64 + 0.9 * random::<f64>());

            if (center - Vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Material;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = color::random_color() * color::random_color();
                    let texture: SolidColor = albedo.into();
                    sphere_material = Lambertian { albedo: Box::new(texture) };
                    let center2 = center + Vec3(0.0, random_range(0.0, 0.5), 0.0);
                    world.add(Box::new(MovingSphere {
                        start_position: center,
                        end_position: center2,
                        start_time: 0.0,
                        end_time: 1.0, 
                        radius: 0.2, 
                        material: sphere_material
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_vec_bounded(0.5, 1.0);
                    let fuzz = random_range(0.0, 0.5);
                    sphere_material = Metal { albedo, fuzz };
                    world.add(Box::new(Sphere {
                        center, 
                        radius: 0.2, 
                        material: sphere_material
                    }));
                } else {
                    // glass
                    sphere_material = Dielectric { index_of_refraction: 1.5 };
                    world.add(Box::new(Sphere {
                        center, 
                        radius: 0.2, 
                        material: sphere_material
                    }));
                }
            }
        }
    }

    let material1 =  Dielectric {
        index_of_refraction: 1.5
    };
    world.add(Box::new(Sphere { 
        center: Vec3(0.0, 1.0, 0.0), 
        radius: 1.0, 
        material: material1
    }));

    let texture: SolidColor = Vec3(0.4, 0.2, 0.1).into();
    let material2 = Lambertian {
        albedo: Box::new(texture)
    };
    world.add(Box::new(Sphere { 
        center: Vec3(-4.0, 1.0, 0.0), 
        radius: 1.0, 
        material: material2
    }));

    let material3 = Metal {
        albedo: Vec3(0.7, 0.6, 0.5), 
        fuzz: 0.0
    };
    world.add(Box::new(Sphere { 
        center: Vec3(4.0, 1.0, 0.0), 
        radius: 1.0, 
        material: material3
    }));

    Scene {
        world,
        camera,
        aspect_ratio: ASPECT_RATIO,
        height: IMAGE_HEIGHT,
        width: IMAGE_WIDTH,
        samples_per_pixel,
        background: Background::VerticalGradient { bottom: Vec3(0.5, 0.7, 1.0), top: Vec3(1.0, 1.0, 1.0) }
    }
}

pub fn diagonal_view(samples_per_pixel: u32, world: HittableList) -> Scene {
    pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    

    let look_from = Vec3(13.0, 2.0, 3.0);
    let look_at = Vec3(0.0, 0.0, 0.0);
    let focus_dist = 10.0;
    let camera = Camera::custom(
        look_from,
        look_at,
        Vec3(0.0, 1.0, 0.0), 
        ASPECT_RATIO, 
        20.0,
        0.0,
        focus_dist,
        0.0,
        1.0
    );
    
    Scene { 
        camera, 
        world, 
        aspect_ratio: ASPECT_RATIO, 
        height: IMAGE_HEIGHT, 
        width: IMAGE_WIDTH, 
        samples_per_pixel,
        background: Background::VerticalGradient { bottom: Vec3(0.5, 0.7, 1.0), top: Vec3(1.0, 1.0, 1.0) } 
    }
}

pub fn two_spheres(samples_per_pixel: u32) -> Scene {
    let bottom_checker = CheckerTexture::make_solid_checkered(Vec3(0.2, 0.3, 0.1), Vec3(0.9, 0.9, 0.9));
    let bottom_sphere = Sphere {
        center: Vec3(0.0, -10.0, 0.0),
        radius: 10.0,
        material: Lambertian { albedo: Box::new(bottom_checker) }
    };

    let top_checker = CheckerTexture::make_solid_checkered(Vec3(0.2, 0.3, 0.1), Vec3(0.9, 0.9, 0.9));
    let top_sphere = Sphere {
        center: Vec3(0.0, 10.0, 0.0),
        radius: 10.0,
        material: Lambertian { albedo: Box::new(top_checker) }
    };

    let world = hittable_list!(Box::new(bottom_sphere), Box::new(top_sphere));
    
    diagonal_view(samples_per_pixel, world)
}

pub fn two_perlin_spheres(samples_per_pixel: u32) -> Scene {
    let bottom_perlin = NoiseTexture::new(4.0);
    let bottom_sphere = Sphere {
        center: Vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian { albedo: Box::new(bottom_perlin) }
    };

    let top_perlin = NoiseTexture::new(4.0);
    let top_sphere = Sphere {
        center: Vec3(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Lambertian { albedo: Box::new(top_perlin) }
    };

    let world = hittable_list!(Box::new(bottom_sphere), Box::new(top_sphere));

    diagonal_view(samples_per_pixel, world)
}

pub fn earth(samples_per_pixel: u32) -> Scene {
    let earth_texture = ImageTexture::from("textures/earthmap.jpg");
    let earth_material = Lambertian { albedo: Box::new(earth_texture) };
    let globe = Sphere {
        center: Vec3(0.0, 0.0, 0.0),
        radius: 2.0,
        material: earth_material
    };
    
    let world = hittable_list!(Box::new(globe));

    diagonal_view(samples_per_pixel, world)
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