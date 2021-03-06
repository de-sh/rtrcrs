use rayon::prelude::*;
use std::{
    io::{stderr, Write},
    sync::Arc,
};

use rtrcrs::{
    camera::Camera,
    color::{anti_aliased, Color},
    definitions::{random_double, random_scene},
    material::{Dielectric, Lambertian, Metal},
    ray::Point3,
    sphere::Sphere,
    Vec3,
};

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    //World
    let mut world = random_scene();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.45,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));

    // Camera
    let camera = Camera::new(
        &Point3::new(13.0, 2.0, 3.0),
        &Point3::new(0.0, 0.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        10.0,
    );

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    std::env::set_var("RAYON_NUM_THREADS", "4");
    let image = (0..IMAGE_HEIGHT)
        .into_par_iter()
        .rev()
        .flat_map(|j| {
            eprint!("\rScanlines remaining: {} ", j);
            stderr().flush().unwrap();
            (0..IMAGE_WIDTH)
                .flat_map(|i| {
                    let pixel_color: Color = (0..SAMPLES_PER_PIXEL)
                        .map(|_| {
                            let (u, v) = (
                                (i as f64 + random_double(0.0, 1.0)) / (IMAGE_WIDTH - 1) as f64,
                                (j as f64 + random_double(0.0, 1.0)) / (IMAGE_HEIGHT - 1) as f64,
                            );
                            camera.get_ray(u, v).color(&world, MAX_DEPTH)
                        })
                        .sum();
                    anti_aliased(pixel_color, SAMPLES_PER_PIXEL)
                        .iter()
                        .map(|c| (c * 255.9) as u8)
                        .collect::<Vec<u8>>()
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    for pc in image.chunks(3) {
        println!("{} {} {}", pc[0], pc[1], pc[2]);
    }

    eprintln!("\rImage Generated.");
    stderr().flush().unwrap();
}
