use std::io::stderr;
use std::io::Write;


mod vector;
pub use vector::Vec3;
use Vec3 as Point3;

mod color;
use color::Color;

fn main() {
    // Image

    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    // Render

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(i as f64 / (IMAGE_WIDTH - 1) as f64, j as f64 / (IMAGE_HEIGHT - 1) as f64, 0.25);
            println!("{}", pixel_color);
        }
    }

    eprintln!("\rImage Generated.");
    stderr().flush().unwrap();

    let red = Color::new(0.1, 0.2, 0.3);
    let green = Color::new(0.3, 0.2, 0.1);
    println!("Add: {}", red + green);
    println!("Sub: {}", red - green);
    println!("Mul: {}", red * 2.0);
    println!("VecMul: {}", green * red);
    println!("Div: {}", green / 2.0);
    println!("Dot: {}", green.dot(red));
    println!("Cross: {}", green.cross(red));
    println!("Unit: {}", Color::unit_vector(green));
}
