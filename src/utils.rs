use std::f64::consts::PI;
use rand::random;
use crate::Material::*;
use crate::Vec3;
use crate::Sphere;
use crate::Material;
use crate::camera::Camera;
use crate::color;

use crate::hittables::hittable_list::HittableList;
use crate::hittables::moving_sphere::MovingSphere;
use crate::scene::Scene;
use crate::types::texture::CheckerTexture;
use crate::types::texture::SolidColor;

pub fn degrees_to_radians(deg: f64) -> f64 {
    2.0 * PI * deg / 360.0
}

pub fn random_range(min: f64, max: f64) -> f64 {
    random::<f64>() * (max - min) + min
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { min } else if x > max { max } else { x }
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
        samples_per_pixel
    }
}