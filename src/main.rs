mod types;

use types::vec3::{Vec3, Point};
use types::color::Color;
use types::ray::Ray;

fn ray_color(r: &Ray) -> Color {
    match hit_sphere(Vec3(0.0, 0.0, -1.0), 0.5, r) {
        Some(t) => {
            let normal: Vec3 = Vec3::normalized(r.at(t) - Vec3(0.0, 0.0, -1.0));
            0.5 * Vec3(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0)
        },
        None => {
            let unit_direction: Vec3 = Vec3::normalized(r.direction);
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
        }
    }
}

fn hit_sphere(center: Point, radius: f64, ray: &Ray) -> Option<f64> {
    let oc: Vec3 = ray.origin - center;
    let a = ray.direction.square_magnitude();
    let b = 2.0 * Vec3::dot(oc, ray.direction);
    let c = oc.square_magnitude() - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant > 0.0 {
        Some((-b - discriminant.sqrt()) / (2.0 * a))
    }
    else {
        None
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

    
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray: Ray = Ray { origin: ORIGIN, direction: LOWER_LEFT + u * HORIZONTAL + v * VERTICAL - ORIGIN };
            let color: Color = ray_color(&ray);
            Color::write_color(color);
        }
    }

    eprintln!("Done");
}
