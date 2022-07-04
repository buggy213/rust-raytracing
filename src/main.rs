mod types;
mod hittables;
mod utils;
mod camera;
mod scene;
mod preset_scenes;
mod cli;

use std::fs::File;
use std::io;
use std::path::Path;
use std::sync::mpsc::{Sender, Receiver, TryRecvError};
use std::sync::{Arc, mpsc};
use std::thread::{self, JoinHandle, sleep, sleep_ms};
use std::time::Duration;

use clap::Parser;
use hittables::hittable::Hittable;
use rand::random;
use scene::Scene;
use types::vec3::{Vec3};
use types::color::Color;
use types::ray::Ray;
use types::materials::Material::{self};

use crate::cli::{CliArguments, MultithreadedSettings, RenderStrategy};

#[derive(Debug)]
pub enum Background {
    SolidColor(Color),
    VerticalGradient {
        bottom: Color,
        top: Color
    }
}

impl Background {
    fn get_color(&self, r: Ray) -> Color {
        match self {
            Background::SolidColor(color) => {
                *color
            }
            Background::VerticalGradient { bottom, top } => {
                let unit_direction: Vec3 = Vec3::normalized(r.direction);
                let t = 0.5 * (unit_direction.y() + 1.0);
                (1.0 - t) * (*top) + t * (*bottom)
            }
        }
    }
}

fn ray_color(r: Ray, world: &dyn Hittable, depth: u32, background: &Background) -> Color {
    if depth <= 0 {
        return Vec3(0.0, 0.0, 0.0);
    }

    match world.hit(r, 0.001, f64::INFINITY) {
        Some(record) => {
            let emitted = record.material.emitted(record.u, record.v, record.p).unwrap_or_default();
            
            match record.material.scatter(r, &record) {
                Some((attenuation, scattered)) => {
                    emitted + attenuation * ray_color(scattered, world, depth - 1, background)
                }
                // Absorbed
                None => {
                    emitted
                }
            }
        },
        None => {
            background.get_color(r)
        }
    }
}

const MAX_DEPTH: u32 = 50;
fn render(scene: &Scene, identifier: u32) -> Vec<Color> {
    let mut color_data: Vec<Color> = Vec::with_capacity((scene.width * scene.height) as usize);
    
    for j in (0..scene.height).rev() {
        eprintln!("[{}] scanlines remaining: {}", identifier, j);
        for i in 0..scene.width {            
            let mut color: Color = Vec3(0.0, 0.0, 0.0);
            for _s in 0..scene.samples_per_pixel {
                let u = (random::<f64>() + i as f64) / (scene.width - 1) as f64;
                let v = (random::<f64>() + j as f64) / (scene.height - 1) as f64;
                let ray: Ray = scene.camera.get_ray(u, v);
                color += ray_color(ray, &scene.world, MAX_DEPTH, &scene.background) / scene.samples_per_pixel.into();
            }
            color_data.push(color);
        }
    }

    eprintln!("[{}] done", identifier);
    
    color_data
}

fn async_render(scene: &Scene, job: RenderJobMessage, transmit_progress: &Sender<RenderResultMessage>) {
    let mut scanline: Vec<(Pixel, Color)> = Vec::new();
    let RenderJobMessage { top_right, bottom_left, samples_per_pixel } = job;
    for j in (bottom_left.y..top_right.y).rev() {
        for i in bottom_left.x..top_right.x {            
            let mut color: Color = Vec3(0.0, 0.0, 0.0);
            
            for _s in 0..samples_per_pixel {
                let u = (random::<f64>() + i as f64) / (scene.width - 1) as f64;
                let v = (random::<f64>() + j as f64) / (scene.height - 1) as f64;
                let ray: Ray = scene.camera.get_ray(u, v);
                color += ray_color(ray, &scene.world, MAX_DEPTH, &scene.background) / job.samples_per_pixel.into();
            }
            scanline.push((Pixel { x: i, y: j }, color));
        }     

        // transmit at end of each scanline
        transmit_progress.send(RenderResultMessage::Result { rendered_pixels: scanline })
                            .expect("unable to send data to coordinating thread");
        scanline = Vec::new();
    }
}

#[derive(Clone, Copy)]
struct Pixel {
    x: u32,
    y: u32
}
enum RenderResultMessage {
    Result {
        rendered_pixels: Vec<(Pixel, Vec3)>
    },
    Done
}

#[derive(Clone, Copy)]
struct RenderJobMessage {
    top_right: Pixel,
    bottom_left: Pixel,
    samples_per_pixel: u32
}

struct RenderThread {
    handle: JoinHandle<()>,
    send_job: Sender<RenderJobMessage>,
    receive_result: Receiver<RenderResultMessage>,
    send_done: Sender<()>
}

fn main() {
    let CliArguments { 
        num_samples, 
        multithreaded, 
        output_file, 
        preset_scene, 
        multithreaded_settings
    } = CliArguments::parse();

    eprintln!("num_samples: {}, multithreaded: {}", num_samples, multithreaded);
    let mut scene = preset_scene.get(num_samples);
    let mut color_data;

    if !multithreaded {
        color_data = render(&scene, 0);
    }
    else {
        color_data = vec![Vec3(0.0, 0.0, 0.0); (scene.width * scene.height) as usize];
        let mut children: Vec<RenderThread> = Vec::new();

        let cores = num_cpus::get() as u32;

        let MultithreadedSettings { interactive, render_strategy, tile_size } = multithreaded_settings;
        let horizontal_tiles;
        let vertical_tiles;

        match render_strategy {
            RenderStrategy::TileFull | RenderStrategy::TileAverage => {
                horizontal_tiles = if scene.width % tile_size == 0 { scene.width / tile_size } else { (scene.width / tile_size) + 1 }; 
                vertical_tiles = if scene.height % tile_size == 0 { scene.height / tile_size } else { (scene.height / tile_size) + 1 };
            }
            RenderStrategy::ProgressiveAverage => {
                horizontal_tiles = 1;
                vertical_tiles = 1;
            }
        }

        let samples_per_pixel;
        let jobs_per_tile;
        match render_strategy {
            RenderStrategy::ProgressiveAverage | RenderStrategy::TileAverage => {
                samples_per_pixel = (scene.samples_per_pixel / cores) + 1;
                jobs_per_tile = cores;
            },
            RenderStrategy::TileFull => {
                samples_per_pixel = scene.samples_per_pixel;
                jobs_per_tile = 1;
            }
        };
        let mut jobs = Vec::with_capacity((horizontal_tiles * vertical_tiles) as usize);

        // create render jobs
        for j in (0..vertical_tiles).rev() {
            for i in 0..horizontal_tiles {
                for _ in 0..jobs_per_tile {
                    jobs.push(RenderJobMessage {
                        
                        
                        top_right: if render_strategy == RenderStrategy::ProgressiveAverage {
                            Pixel {
                                x: scene.width,
                                y: scene.height
                            }
                        }
                        else {
                            Pixel { 
                                x: u32::min((i + 1) * tile_size, scene.width), 
                                y: u32::min((j + 1) * tile_size, scene.height) 
                            }
                        },
                        bottom_left: Pixel { x: i * tile_size, y: j * tile_size },
                        samples_per_pixel
                    });
                }
            }
        }

        let scene_ref = Arc::new(scene);
        
        for i in 0..cores {
            let shared_scene = scene_ref.clone();
            let (result_transmit, result_receive) = mpsc::channel();
            let (job_transmit, job_receive) = mpsc::channel();
            let (done_transmit, done_receive) = mpsc::channel();
            let thread = thread::spawn(move || {
                sleep(Duration::from_millis(500));
                while let Ok(job_message) = job_receive.try_recv() {
                    async_render(&shared_scene, job_message, &result_transmit);
                    eprintln!("[{}] finished job", i);
                }

                result_transmit.send(RenderResultMessage::Done).expect("failed to send message back to main thread");
                done_receive.recv().expect("failed to receive done signal");
            });
            children.push(RenderThread { handle: thread, send_job: job_transmit, receive_result: result_receive, send_done: done_transmit });
        }

        // each should have at least one job
        assert!(children.len() <= jobs.len());

        // assign initial jobs to each thread
        for (i, job) in jobs.iter().enumerate() {
            children[i % children.len()].send_job.send(*job).expect("failed to assign job");
        }
        
        const POLLING_INTERVAL: u64 = 1; // 100ms polling interval
        let mut completed_threads = 0;
        while completed_threads < children.len() {
            sleep(Duration::from_millis(POLLING_INTERVAL));
            for child in children.iter() {
                match child.receive_result.try_recv() {
                    Ok(render_result) => {
                        match render_result { 
                            RenderResultMessage::Result { rendered_pixels } => {
                                for rendered_pixel in rendered_pixels {
                                    let index = (scene_ref.height - 1 - rendered_pixel.0.y) * scene_ref.width + rendered_pixel.0.x;
                                    color_data[index as usize] += rendered_pixel.1 / jobs_per_tile as f64;
                                }
                            },
                            RenderResultMessage::Done => {
                                completed_threads += 1;
                            }
                        }
                    },
                    Err(_) => {
                        continue;
                    },
                }
            }
            
        }
        
        // join up to ensure all threads are finished
        for child in children {
            child.send_done.send(()).expect("failed to send done message to child thread");
            child.handle.join().expect("failed to join child thread");
        }
        
        scene = Arc::try_unwrap(scene_ref).expect("unable to move scene out of shared ownership");
    }

    if let Some(filename) = output_file {
        let path = Path::new(&filename);
        
        match filename.split('.').last().unwrap() {
            "png" => scene.save_png(&color_data, path),
            "ppm" | _ => {
                let file = File::create(path).expect("unable to create file");
                scene.print_ppm(&color_data, file).expect("failed to print output")
            }
        }
    }
    else {
        scene.print_ppm(&color_data, io::stdout()).expect("failed to print output");
    }
}


