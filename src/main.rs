mod types;
mod hittables;
mod utils;
mod camera;

use hittables::hittable::Hittable;
use rand::random;
use types::vec3::{Vec3};
use types::color::Color;
use types::ray::Ray;
use types::materials::Material::{Lambertian, Metal, Dielectric};

use crate::camera::Camera;
use crate::hittables::hittable_list::HittableList;
use crate::hittables::sphere::Sphere;

fn base_gradient(r: &Ray) -> Color {
    let unit_direction: Vec3 = Vec3::normalized(r.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    
    if depth <= 0 {
        return Vec3(0.0, 0.0, 0.0);
    }

    match world.hit(r, 0.001, f64::INFINITY) {
        Some(record) => {
            match record.material.scatter(&r, &record) {
                Some((attenuation, scattered)) => {
                    attenuation * ray_color(&scattered, world, depth - 1)
                }
                // Absorbed
                None => {
                    Vec3(0.0, 0.0, 0.0)
                }
            }
        },
        None => {
            base_gradient(r)
        }
    }
}
pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
fn main() {
    
    const IMAGE_HEIGHT: i32 = 400;
    const IMAGE_WIDTH: i32 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    let camera = Camera::default();

    let material_ground = Lambertian(Vec3(0.8, 0.8, 0.0));
    let material_center = Lambertian(Vec3(0.1, 0.2, 0.5));
    let material_left = Dielectric(1.5);
    let material_hollow = Dielectric(1.5);
    let material_right = Metal(Vec3(0.8, 0.6, 0.2), 0.0);
    let center = Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
        material: material_center
    };
    let left_hollow = Sphere {
        center: Vec3(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: material_left
    };
    let left_inner = Sphere {
        center: Vec3(-1.0, 0.0, -1.0),
        radius: -0.4,
        material: material_hollow
    };
    let right = Sphere {
        center: Vec3(1.0, 0.0, -1.0),
        radius: 0.5,
        material: material_right
    };
    let ground = Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0,
        material: material_ground
    };
    let world = from!(Box::new(ground), Box::new(center), Box::new(left_hollow), Box::new(left_inner), Box::new(right)); 

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {            
            let mut color: Color = Vec3(0.0, 0.0, 0.0);

            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (random::<f64>() + i as f64) / (IMAGE_WIDTH - 1) as f64;
                let v = (random::<f64>() + j as f64) / (IMAGE_HEIGHT - 1) as f64;
                let ray: Ray = camera.get_ray(u, v);
                color += ray_color(&ray, &world, MAX_DEPTH);
            }
            Color::write_color(color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("Done");
}
