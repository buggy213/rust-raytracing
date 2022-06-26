mod types;
mod hittables;
mod utils;

use hittables::hittable::Hittable;
use types::vec3::{Vec3};
use types::color::Color;
use types::ray::Ray;

use crate::hittables::hittable_list::HittableList;
use crate::hittables::sphere::Sphere;

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    match world.hit(r, 0.0, f64::INFINITY) {
        Some(record) => {
            0.5 * (record.normal + Vec3(1.0, 1.0, 1.0))
        },
        None => {
            let unit_direction: Vec3 = Vec3::normalized(r.direction);
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_HEIGHT: i32 = 400;
    const IMAGE_WIDTH: i32 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as i32;

    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
    const FOCAL_LENGTH: f64 = 1.0;

    const ORIGIN: Vec3 = Vec3(0.0, 0.0, 0.0);
    const HORIZONTAL: Vec3 = Vec3(VIEWPORT_WIDTH, 0.0, 0.0);
    const VERTICAL: Vec3 = Vec3(0.0, VIEWPORT_HEIGHT, 0.0);
    let LOWER_LEFT: Vec3 = ORIGIN - HORIZONTAL / 2.0 - VERTICAL / 2.0 - Vec3(0.0, 0.0, FOCAL_LENGTH);

    let near = Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5
    };
    let far = Sphere {
        center: Vec3(0.0, -100.5, -1.0),
        radius: 100.0
    };
    let world = from!(Box::new(near), Box::new(far)); 

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray: Ray = Ray { origin: ORIGIN, direction: LOWER_LEFT + u * HORIZONTAL + v * VERTICAL - ORIGIN };
            let color: Color = ray_color(&ray, &world);
            Color::write_color(color);
        }
    }

    eprintln!("Done");
}
