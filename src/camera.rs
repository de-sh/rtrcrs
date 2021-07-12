use crate::{Point3, Ray};

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
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Point3::new(viewport_width, 0.0, 0.0);
        let vertical = Point3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Point3::new(0.0, 0.0, focal_length);

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
