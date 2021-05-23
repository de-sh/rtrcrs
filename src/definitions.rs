use crate::{
    BvhNode, Color, Dielectric, HittableList, Lambertian, Metal, MovingSphere, Point3, Sphere, Vec3,
};
use rand::Rng;
use std::sync::Arc;

/// Re-exports the definition of the constant PI.
pub use std::f64::consts::PI;

/// Re-exports the definition of the constant MAX as INFINITY.
pub const INFINITY: f64 = std::f64::MAX;

/// Provides logic for converting Degrees to Radians.
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Provides logic for generating random numbers of type f64 from 0.0 to 1.0 .
pub fn random_double(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

/// Provides logic for generating a random Vec of type Vector3.
pub fn random_vec(min: f64, max: f64) -> Vec3 {
    let rd = || random_double(min, max);
    Vec3::new(rd(), rd(), rd())
}

/// Provides logic for generating random Vectors inside a Unit sphere.
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec(-1.0, 1.0);
        if p.dot(&p) >= 1.0 {
            continue;
        }
        return p;
    }
}

/// Provides logic for generating random Vectors inside a hemishpere.
pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    // In the same hemisphere as the normal
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

/// Provides logic for generating a random Unit Vector.
pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalize()
}

/// Provides logic for returning true if a Vector is very close to zero in all dimensions.
pub fn near_zero(v: Vec3) -> bool {
    let s = 10f64.powi(-8);
    v.x.abs() < s && v.y.abs() < s && v.z.abs() < s
}

/// Provides logic for Reflection.
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

/// Provides logic for Refraction.
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = n.dot(&-uv).min(1.0);
    let r_out_perpendicular = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perpendicular.dot(&r_out_perpendicular))
        .abs()
        .sqrt())
        * n;
    r_out_perpendicular + r_out_parallel
}

/// Using Schlick's approximation for Reflectance.
pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

pub fn random_scene() -> HittableList {
    let mut objects = HittableList::default();
    let mut world = HittableList::default();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));
    let rand = || random_double(0.0, 1.0);
    let rand_color = |min, max| {
        Color::new(
            random_double(min, max),
            random_double(min, max),
            random_double(min, max),
        )
    };
    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(a as f64 + 0.9 * rand(), 0.2, b as f64 + 0.9 * rand());
            let point = center - Point3::new(4.0, 0.2, 0.0);
            if point.dot(&point).sqrt() > 0.9 {
                match (rand() * 100.0) as u8 {
                    0..=79 => {
                        let albedo = rand_color(0.0, 1.0);
                        let sphere_material = Arc::new(Lambertian::new(albedo));
                        objects.add(Arc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                        let center2 = center + Point3::new(0.0, random_double(0.0, 0.5), 0.0);
                        objects.add(Arc::new(MovingSphere::new(
                            center,
                            center2,
                            0.0,
                            1.0,
                            0.2,
                            sphere_material,
                        )));
                    }
                    8..=94 => {
                        let albedo = rand_color(0.5, 1.0);
                        let fuzz = random_double(0.0, 0.5);
                        let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                        objects.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                    _ => {
                        let sphere_material = Arc::new(Dielectric::new(1.5));
                        objects.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                    }
                }
            }
        }
    }
    let material1 = Arc::new(Dielectric::new(1.5));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    objects.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    objects.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world.add(Arc::new(BvhNode::new(&mut objects, 0.0, 1.0)));
    world
}
