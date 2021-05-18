use crate::{definitions::degrees_to_radians, Point3, Ray, Vec3};

/// Defines a data-structure used to store the geometry of the Camera.
#[derive(Clone, Copy)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Point3,
    vertical: Point3,
}

impl Camera {
    /// Used to set the geometric values of the Camera.
    pub fn new(
        position: &Point3,
        focus: &Point3,
        vup: &Vec3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (position - focus).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = *position;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
    /// Used to get the Ray corresponding to a Pixel and the Camera.
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
