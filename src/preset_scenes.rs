use std::{sync::Arc, path::Path};

use clap::clap_derive::ArgEnum;
use rand::random;

use crate::{scene::Scene, types::{vec3::Vec3, texture::{CheckerTexture, SolidColor, Texture, NoiseTexture, ImageTexture}, color, materials::Material, transform::TransformData, bvh::BVHNode}, camera::Camera, hittables::{hittable_list::HittableList, sphere::Sphere, moving_sphere::MovingSphere, aarect::{YZ, XZ, XY}, block::Block, instance::Instance, constant_medium::ConstantMedium, hittable::Hit, tri::Triangle, mesh::Mesh}, utils::{random_range, degrees_to_radians}, Background, hittable_list};
use crate::Material::*;

#[derive(Clone, ArgEnum)]
pub enum PresetScene {
    JumpingBalls,
    TwoSpheres,
    TwoPerlinSpheres,
    Earth,
    SimpleLight,
    CornellBox,
    TransformTest,
    CornellSmoke,
    FinalRender,
    TriangleTest,
    MeshTest,
}

impl PresetScene {
    pub fn get(&self, samples_per_pixel: u32) -> Scene {
        match self {
            PresetScene::JumpingBalls => random_scene(samples_per_pixel),
            PresetScene::TwoSpheres => two_spheres(samples_per_pixel),
            PresetScene::TwoPerlinSpheres => two_perlin_spheres(samples_per_pixel),
            PresetScene::Earth => earth(samples_per_pixel),
            PresetScene::SimpleLight => simple_light(samples_per_pixel),
            PresetScene::CornellBox => cornell_box(samples_per_pixel),
            PresetScene::TransformTest => transform_test(samples_per_pixel),
            PresetScene::CornellSmoke => cornell_smoke(samples_per_pixel),
            PresetScene::FinalRender => final_scene(samples_per_pixel),
            PresetScene::TriangleTest => triangle_test(samples_per_pixel),
            PresetScene::MeshTest => mesh_test(samples_per_pixel)
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
    let ground_material = Lambertian { 
        albedo: Arc::new(ground_texture)
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
                    sphere_material = Lambertian { albedo: Arc::new(texture) };
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
        albedo: Arc::new(texture)
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

fn diagonal_view(samples_per_pixel: u32, world: HittableList) -> Scene {
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

fn straight_view(samples_per_pixel: u32, world: HittableList) -> Scene {
    pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 1080;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    
    let look_from = Vec3(0.0, 0.0, 6.0);
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
        background: Background::VerticalGradient { bottom: Vec3(0.3, 0.42, 0.6), top: Vec3(0.7, 0.7, 0.7) } 
    }
}

pub fn two_spheres(samples_per_pixel: u32) -> Scene {
    let checker: Arc<dyn Texture> = Arc::new(CheckerTexture::make_solid_checkered(Vec3(0.2, 0.3, 0.1), Vec3(0.9, 0.9, 0.9)));
    let bottom_sphere = Sphere {
        center: Vec3(0.0, -10.0, 0.0),
        radius: 10.0,
        material: Lambertian { albedo: checker.clone() }
    };

    let top_checker = CheckerTexture::make_solid_checkered(Vec3(0.2, 0.3, 0.1), Vec3(0.9, 0.9, 0.9));
    let top_sphere = Sphere {
        center: Vec3(0.0, 10.0, 0.0),
        radius: 10.0,
        material: Lambertian { albedo: checker.clone() }
    };

    let world = hittable_list!(Box::new(bottom_sphere), Box::new(top_sphere));
    
    diagonal_view(samples_per_pixel, world)
}

pub fn two_perlin_spheres(samples_per_pixel: u32) -> Scene {
    let perlin = Arc::new(NoiseTexture::new(4.0));
    let bottom_sphere = Sphere {
        center: Vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian { albedo: perlin.clone() }
    };

    let top_sphere = Sphere {
        center: Vec3(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Lambertian { albedo: perlin.clone() }
    };

    let world = hittable_list!(Box::new(bottom_sphere), Box::new(top_sphere));

    diagonal_view(samples_per_pixel, world)
}

pub fn earth(samples_per_pixel: u32) -> Scene {
    let earth_texture = ImageTexture::from("textures/earthmap.jpg");
    let earth_material = Lambertian { albedo: Arc::new(earth_texture) };
    let globe = Sphere {
        center: Vec3(0.0, 0.0, 0.0),
        radius: 2.0,
        material: earth_material
    };
    
    let world = hittable_list!(Box::new(globe));

    diagonal_view(samples_per_pixel, world)
}

pub fn simple_light(samples_per_pixel: u32) -> Scene {
    let perlin = Arc::new(NoiseTexture::new(4.0));
    let bottom_sphere = Sphere {
        center: Vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Lambertian { albedo: perlin.clone() }
    };

    let top_perlin = NoiseTexture::new(4.0);
    let top_sphere = Sphere {
        center: Vec3(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Lambertian { albedo: perlin.clone() }
    };

    let diffuse_light = DiffuseLight { 
        emit: Arc::new(SolidColor::from(Vec3(4.0, 4.0, 4.0)))
    };
    let rect_light = XY {
        a0: 3.0,
        a1: 5.0,
        b0: 1.0,
        b1: 3.0,
        k: -2.0,
        material: diffuse_light
    };

    let world = hittable_list!(Box::new(bottom_sphere), Box::new(top_sphere), Box::new(rect_light));

    pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    let look_from = Vec3(26.0, 3.0, 6.0);
    let look_at = Vec3(0.0, 2.0, 0.0);
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
        background: Background::SolidColor(Vec3(0.0, 0.0, 0.0))
    }
}

pub fn cornell_box(samples_per_pixel: u32) -> Scene {
    let red = Lambertian {
        albedo: Arc::new(SolidColor::from(Vec3(0.65, 0.05, 0.05)))
    };

    let white_material = Arc::new(SolidColor::from(Vec3(0.73, 0.73, 0.73)));
    let white = Lambertian {
        albedo: white_material
    };
    let green = Lambertian {
        albedo: Arc::new(SolidColor::from(Vec3(0.12, 0.45, 0.15)))
    };
    let light = DiffuseLight {
        emit: Arc::new(SolidColor::from(Vec3(15.0, 15.0, 15.0)))
    };

    let wall0 = YZ {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 555.0,
        material: green
    };
    let wall1 = YZ {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 0.0,
        material: red
    };
    let wall2 = XZ {
        a0: 213.0,
        a1: 343.0,
        b0: 227.0,
        b1: 332.0,
        k: 554.0,
        material: light
    };
    let wall3 = XZ {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 0.0,
        material: white.clone()
    };
    let wall4 = XZ {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 555.0,
        material: white.clone()
    };
    let wall5 = XY {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 555.0,
        material: white.clone()
    };

    let block0 = Block::new(
        Vec3(0.0, 0.0, 0.0),
        Vec3(165.0, 330.0, 165.0),
        white.clone()
    );
    let block1 = Block::new(
        Vec3(0.0, 0.0, 0.0),
        Vec3(165.0, 165.0, 165.0),
        white.clone()
    );

    let block0 = Instance::new(
        Box::new(block0),
        TransformData::identity()
                                 .rotate_angle_axis(Vec3(0.0, 1.0, 0.0), degrees_to_radians(15.0))
                                 .translate(Vec3(265.0, 0.0, 295.0))
    );

    let block1 = Instance::new(
        Box::new(block1),
        TransformData::identity()
                                 .rotate_angle_axis(Vec3(0.0, 1.0, 0.0), degrees_to_radians(-18.0))
                                 .translate(Vec3(130.0, 0.0, 65.0))
                                 
    );

    let world = hittable_list!(
        Box::new(wall0), 
        Box::new(wall1), 
        Box::new(wall2), 
        Box::new(wall3), 
        Box::new(wall4), 
        Box::new(wall5),
        Box::new(block0),
        Box::new(block1)
    );

    pub const ASPECT_RATIO: f64 = 1.0;
    const IMAGE_WIDTH: u32 = 600;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    let look_from = Vec3(278.0, 278.0, -800.0);
    let look_at = Vec3(278.0, 278.0, 0.0);
    let focus_dist = 10.0;
    let camera = Camera::custom(
        look_from,
        look_at,
        Vec3(0.0, 1.0, 0.0), 
        ASPECT_RATIO, 
        40.0,
        0.0,
        focus_dist,
        0.0,
        0.0
    );
    
    Scene { 
        camera, 
        world, 
        aspect_ratio: ASPECT_RATIO, 
        height: IMAGE_HEIGHT, 
        width: IMAGE_WIDTH, 
        samples_per_pixel,
        background: Background::SolidColor(Vec3(0.0, 0.0, 0.0))
    }
}

pub fn transform_test(samples_per_pixel: u32) -> Scene {
    let red = Lambertian {
        albedo: Arc::new(SolidColor::from(Vec3(0.65, 0.05, 0.05)))
    };

    let white_material = Arc::new(SolidColor::from(Vec3(0.73, 0.73, 0.73)));
    let white = Lambertian {
        albedo: white_material
    };
    let green = Lambertian {
        albedo: Arc::new(SolidColor::from(Vec3(0.12, 0.45, 0.15)))
    };
    let light = DiffuseLight {
        emit: Arc::new(SolidColor::from(Vec3(15.0, 15.0, 15.0)))
    };

    let wall0 = YZ {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 555.0,
        material: green
    };
    let wall1 = YZ {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 0.0,
        material: red
    };
    let wall2 = XZ {
        a0: 163.0,
        a1: 393.0,
        b0: 177.0,
        b1: 382.0,
        k: 554.0,
        material: light
    };
    let wall3 = XZ {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 0.0,
        material: white.clone()
    };
    let wall4 = XZ {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 555.0,
        material: white.clone()
    };
    let wall5 = XY {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 555.0,
        material: white.clone()
    };

    let block0 = Block::new(
        Vec3(0.0, 0.0, 0.0),
        Vec3(165.0, 330.0, 165.0),
        white.clone()
    );

    let block0 = Instance::new(
        Box::new(block0),
        TransformData::identity().translate(Vec3(265.0, 0.0, 295.0))
                                            .rotate_angle_axis(Vec3(0.0, 1.0, 0.0), degrees_to_radians(15.0))
    );

    let world = hittable_list!(
        Box::new(wall0), 
        Box::new(wall1), 
        Box::new(wall2), 
        Box::new(wall3), 
        Box::new(wall4), 
        Box::new(wall5),
        Box::new(block0)
    );

    pub const ASPECT_RATIO: f64 = 1.0;
    const IMAGE_WIDTH: u32 = 600;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    let look_from = Vec3(278.0, 278.0, -800.0);
    let look_at = Vec3(278.0, 278.0, 0.0);
    let focus_dist = 10.0;
    let camera = Camera::custom(
        look_from,
        look_at,
        Vec3(0.0, 1.0, 0.0), 
        ASPECT_RATIO, 
        40.0,
        0.0,
        focus_dist,
        0.0,
        0.0
    );
    
    Scene { 
        camera, 
        world, 
        aspect_ratio: ASPECT_RATIO, 
        height: IMAGE_HEIGHT, 
        width: IMAGE_WIDTH, 
        samples_per_pixel,
        background: Background::SolidColor(Vec3(0.0, 0.0, 0.0))
    }
}

pub fn cornell_smoke(samples_per_pixel: u32) -> Scene {
    let red = Lambertian {
        albedo: Arc::new(SolidColor::from(Vec3(0.65, 0.05, 0.05)))
    };

    let white_material = Arc::new(SolidColor::from(Vec3(0.73, 0.73, 0.73)));
    let white = Lambertian {
        albedo: white_material
    };
    let green = Lambertian {
        albedo: Arc::new(SolidColor::from(Vec3(0.12, 0.45, 0.15)))
    };
    let light = DiffuseLight {
        emit: Arc::new(SolidColor::from(Vec3(7.0, 7.0, 7.0)))
    };

    let wall0 = YZ {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 555.0,
        material: green
    };
    let wall1 = YZ {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 0.0,
        material: red
    };
    let wall2 = XZ {
        a0: 113.0,
        a1: 443.0,
        b0: 127.0,
        b1: 432.0,
        k: 554.0,
        material: light
    };
    let wall3 = XZ {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 0.0,
        material: white.clone()
    };
    let wall4 = XZ {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 555.0,
        material: white.clone()
    };
    let wall5 = XY {
        a0: 0.0,
        a1: 555.0,
        b0: 0.0,
        b1: 555.0,
        k: 555.0,
        material: white.clone()
    };

    let block0 = Block::new(
        Vec3(0.0, 0.0, 0.0),
        Vec3(165.0, 330.0, 165.0),
        white.clone()
    );
    let block1 = Block::new(
        Vec3(0.0, 0.0, 0.0),
        Vec3(165.0, 165.0, 165.0),
        white.clone()
    );

    let block0 = Instance::new(
        Box::new(block0),
        TransformData::identity()
                                    .rotate_angle_axis(Vec3(0.0, 1.0, 0.0), degrees_to_radians(15.0))
                                    .translate(Vec3(265.0, 0.0, 295.0))
    );

    let block1 = Instance::new(
        Box::new(block1),
        TransformData::identity()
                                    .rotate_angle_axis(Vec3(0.0, 1.0, 0.0), degrees_to_radians(-18.0))
                                    .translate(Vec3(130.0, 0.0, 65.0))
                                    
    );

    let block0 = ConstantMedium::solid(Box::new(block0), 0.01, Vec3(0.0, 0.0, 0.0));
    let block1 = ConstantMedium::solid(Box::new(block1), 0.01, Vec3(1.0, 1.0, 1.0));

    let world = hittable_list!(
        Box::new(wall0), 
        Box::new(wall1), 
        Box::new(wall2), 
        Box::new(wall3), 
        Box::new(wall4), 
        Box::new(wall5),
        Box::new(block0),
        Box::new(block1)
    );

    pub const ASPECT_RATIO: f64 = 1.0;
    const IMAGE_WIDTH: u32 = 600;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    let look_from = Vec3(278.0, 278.0, -800.0);
    let look_at = Vec3(278.0, 278.0, 0.0);
    let focus_dist = 10.0;
    let camera = Camera::custom(
        look_from,
        look_at,
        Vec3(0.0, 1.0, 0.0), 
        ASPECT_RATIO, 
        40.0,
        0.0,
        focus_dist,
        0.0,
        0.0
    );
    
    Scene { 
        camera, 
        world, 
        aspect_ratio: ASPECT_RATIO, 
        height: IMAGE_HEIGHT, 
        width: IMAGE_WIDTH, 
        samples_per_pixel,
        background: Background::SolidColor(Vec3(0.0, 0.0, 0.0))
    }
}

pub fn final_scene(samples_per_pixel: u32) -> Scene {
    let mut boxes1: Vec<Box<dyn Hit>> = Vec::new();
    let ground = Lambertian { 
        albedo: Arc::new(SolidColor::from(Vec3(0.48, 0.83, 0.53)))
    };
    const BOXES_PER_SIDE: i32 = 20;
    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_range(1.0, 101.0);
            let z1 = z0 + w;

            let block = Block::new(Vec3(x0, y0, z0), Vec3(x1, y1, z1), ground.clone());
            boxes1.push(Box::new(block));
        }
    }
    
    let mut objects = HittableList::new();
    objects.add(Box::new(BVHNode::make(boxes1, 0.0, 1.0)));

    let light = DiffuseLight { 
        emit: Arc::new(SolidColor::from(Vec3(7.0, 7.0, 7.0)))  
    };

    objects.add(Box::new(XZ {
        material: light,
        a0: 123.0,
        a1: 423.0,
        b0: 147.0,
        b1: 412.0,
        k: 554.0,
    }));

    let center1 = Vec3(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3(30.0, 0.0, 0.0);
    let moving_sphere_material = Lambertian { 
        albedo: Arc::new(SolidColor::from(Vec3(0.7, 0.3, 0.1)))
    };
    objects.add(Box::new(MovingSphere {
        start_position: center1,
        end_position: center2,
        start_time: 0.0,
        end_time: 1.0,
        radius: 50.0,
        material: moving_sphere_material,
    }));

    objects.add(Box::new(Sphere {
        center: Vec3(260.0, 150.0, 45.0),
        radius: 50.0,
        material: Dielectric { index_of_refraction: 1.5 },
    }));

    objects.add(Box::new(Sphere {
        center: Vec3(0.0, 150.0, 145.0),
        radius: 50.0,
        material: Metal { albedo: Vec3(0.8, 0.8, 0.9), fuzz: 1.0 }
    }));

    let boundary = Box::new(Sphere {
        center: Vec3(360.0, 150.0, 145.0),
        radius: 70.0,
        material: Dielectric { index_of_refraction: 1.5 }
    });

    objects.add(boundary);

    let boundary = Box::new(Sphere {
        center: Vec3(360.0, 150.0, 145.0),
        radius: 70.0,
        material: Dielectric { index_of_refraction: 1.5 }
    });
    objects.add(Box::new(ConstantMedium::solid(
        boundary,
        0.2,
        Vec3(0.2, 0.4, 0.9)
    )));

    let boundary = Box::new(Sphere {
        center: Vec3(0.0, 0.0, 0.0),
        radius: 5000.0,
        material: Dielectric { index_of_refraction: 1.5 }
    });
    objects.add(Box::new(ConstantMedium::solid(
        boundary, 
        0.0001, 
        Vec3(1.0, 1.0, 1.0)
    )));

    let emat = Lambertian { 
        albedo: Arc::new(ImageTexture::from("textures/earthmap.jpg")) 
    };

    objects.add(Box::new(Sphere {
        center: Vec3(400.0, 200.0, 400.0),
        radius: 100.0,
        material: emat
    }));
    
    let perlin_texture = NoiseTexture::new(0.1);
    objects.add(Box::new(Sphere {
        center: Vec3(220.0, 280.0, 300.0),
        radius: 80.0,
        material: Lambertian { 
            albedo: Arc::new(perlin_texture)
        }
    }));

    let mut boxes2: Vec<Box<dyn Hit>> = Vec::new();
    let white = Lambertian { 
        albedo: Arc::new(SolidColor::from(Vec3(0.73, 0.73, 0.73)))
    };

    let ns: i32 = 1000;
    for _ in 0..ns {
        boxes2.push(Box::new(Sphere {
            center: Vec3::random_vec_bounded(0.0, 165.0),
            radius: 10.0,
            material: white.clone()
        }));
    }

    objects.add(Box::new(Instance::new(
        Box::new(BVHNode::make(boxes2, 0.0, 1.0)),
        TransformData::identity().rotate_angle_axis(Vec3(0.0, 1.0, 0.0), degrees_to_radians(15.0))
                                            .translate(Vec3(-100.0, 270.0, 395.0))
    )));


    pub const ASPECT_RATIO: f64 = 1.0;
    const IMAGE_WIDTH: u32 = 800;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    let look_from = Vec3(478.0, 278.0, -600.0);
    let look_at = Vec3(278.0, 278.0, 0.0);
    let focus_dist = 10.0;
    let camera = Camera::custom(
        look_from,
        look_at,
        Vec3(0.0, 1.0, 0.0), 
        ASPECT_RATIO, 
        40.0,
        0.0,
        focus_dist,
        0.0,
        1.0
    );
    
    Scene { 
        camera, 
        world: objects, 
        aspect_ratio: ASPECT_RATIO, 
        height: IMAGE_HEIGHT, 
        width: IMAGE_WIDTH, 
        samples_per_pixel,
        background: Background::SolidColor(Vec3(0.0, 0.0, 0.0))
    }
}

pub fn triangle_test(samples_per_pixel: u32) -> Scene {
    let noise = NoiseTexture::new(0.1);
    let tri_mat = Material::Lambertian { albedo: Arc::new(noise) };

    let triangle = Triangle::new(
        Vec3(0.0, 0.0, 0.0),
        Vec3(1.0, 0.0, 0.0),
        Vec3(0.0, 1.0, 0.0),
        tri_mat
    );

    let world = hittable_list!(Box::new(triangle));

    straight_view(samples_per_pixel, world)
}

pub fn mesh_test(samples_per_pixel: u32) -> Scene {
    let mesh = Mesh::from_file(Path::new("meshes/suzanne.obj"));
    let mesh = mesh.expect("unable to load mesh");

    let world = hittable_list!(Box::new(mesh));

    straight_view(samples_per_pixel, world)
}