use std::{
    io::{stderr, Write},
    sync::Arc,
};

use rtrcrs::{
    camera::Camera, color::Color, definitions::random_double, hittable_list::HittableList,
    ray::Point3, sphere::Sphere,
};

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;

    //World
    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));

    // Camera
    let camera = Camera::new();

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let (u, v) = (
                    (i as f64 + random_double()) / (IMAGE_WIDTH - 1) as f64,
                    (j as f64 + random_double()) / (IMAGE_HEIGHT - 1) as f64,
                );
                pixel_color += camera.get_ray(u, v).color(&world);
            }
            println!("{}", pixel_color.anti_aliased(SAMPLES_PER_PIXEL));
        }
    }

    eprintln!("\rImage Generated.");
    stderr().flush().unwrap();
}
