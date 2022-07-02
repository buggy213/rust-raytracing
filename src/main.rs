mod types;
mod hittables;
mod utils;
mod camera;
mod scene;

use std::fs::File;
use std::io;
use std::path::Path;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use clap::Parser;
use hittables::hittable::Hittable;
use rand::random;
use scene::Scene;
use types::vec3::{Vec3};
use types::color::Color;
use types::ray::Ray;
use types::materials::Material::{self};
use utils::PresetScene;
use crate::hittables::sphere::Sphere;
use crate::types::color;
use crate::utils::random_scene;

fn base_gradient(r: Ray) -> Color {
    let unit_direction: Vec3 = Vec3::normalized(r.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3(1.0, 1.0, 1.0) + t * Vec3(0.5, 0.7, 1.0)
}

fn ray_color(r: Ray, world: &dyn Hittable, depth: u32) -> Color {
    
    if depth <= 0 {
        return Vec3(0.0, 0.0, 0.0);
    }

    match world.hit(r, 0.001, f64::INFINITY) {
        Some(record) => {
            match record.material.scatter(r, &record) {
                Some((attenuation, scattered)) => {
                    attenuation * ray_color(scattered, world, depth - 1)
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


#[derive(Parser)]
struct Arguments {
    #[clap(short='s', long="samples")]
    num_samples: u32,
    #[clap(short='m', long="multithreaded")]
    multithreaded: bool,
    #[clap(short='o', long="output")]
    output_file: Option<String>,
    #[clap(long="scene", arg_enum, value_parser, default_value_t=PresetScene::JumpingBalls)]
    preset_scene: PresetScene
}

const MAX_DEPTH: u32 = 50;
fn render(scene: &Scene, identifier: u32) -> Vec<Color> {
    let mut color_data: Vec<Color> = Vec::new();

    for j in (0..scene.height).rev() {
        eprintln!("[{}] Scanlines remaining: {}", identifier, j);
        for i in 0..scene.width {            
            let mut color: Color = Vec3(0.0, 0.0, 0.0);
            
            for _s in 0..scene.samples_per_pixel {
                let u = (random::<f64>() + i as f64) / (scene.width - 1) as f64;
                let v = (random::<f64>() + j as f64) / (scene.height - 1) as f64;
                let ray: Ray = scene.camera.get_ray(u, v);
                color += ray_color(ray, &scene.world, MAX_DEPTH) / scene.samples_per_pixel.into();
            }
            color_data.push(color);
        }
    }
    eprintln!("[{}] Done", identifier);

    color_data
}

fn average_color_data(color_data_vec: &Vec<Vec<Color>>, size: usize) -> Vec<Color> {
    let mut result = vec![Vec3(0.0, 0.0, 0.0); size];
    for color_data in color_data_vec.iter() {
        for (i, color) in color_data.iter().enumerate() {
            result[i] += *color;
        }
    }
    result = result.iter().map(|x: &Color| *x / color_data_vec.len() as f64).collect();
    result
}

fn main() {
    let Arguments { num_samples, multithreaded, output_file, preset_scene  } = Arguments::parse();
    eprintln!("num_samples: {}, multithreaded: {}", num_samples, multithreaded);
    let mut scene = preset_scene.get(num_samples);
    let color_data;
    if !multithreaded {
        color_data = render(&scene, 0);
    }
    else {
        let mut color_data_vec: Vec<Vec<Color>> = Vec::new();
        let mut children: Vec<JoinHandle<Vec<Color>>> = Vec::new();

        let cores = num_cpus::get() as u32;

        scene.samples_per_pixel /= cores;
        scene.samples_per_pixel += 1;

        let scene_ref = Arc::new(scene);

        
        for i in 0..cores {
            let shared_scene = scene_ref.clone();
            children.push(thread::spawn(move || {
                render(&shared_scene, i)
            }));
        }
        for child in children.into_iter() {
            color_data_vec.push(child.join().expect("thread failed"));
        }

        color_data = average_color_data(&color_data_vec, (scene_ref.width * scene_ref.height) as usize);
        scene = Arc::try_unwrap(scene_ref).expect("unable to move scene out of shared ownership");
    }

    if let Some(filename) = output_file {
        let file = File::create(Path::new(&filename)).expect("unable to create file");
        scene.print_ppm(&color_data, file).expect("failed to print output");
    }
    else {
        scene.print_ppm(&color_data, io::stdout()).expect("failed to print output");
    }
}


