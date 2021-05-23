use crate::{degrees_to_radians, random_double, random_in_unit_sphere, Point3, Ray, Vec3};
/// Defines a data-structure used to store the geometry of the Camera.
#[derive(Clone, Copy)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Point3,
    vertical: Point3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    /// Used to set the geometric values of the Camera.
    pub fn new(
        position: &Point3,
        focus: &Point3,
        vup: &Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        mut _time0: f64,
        mut _time1: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        _time0 = 0.0;
        _time1 = 1.0;

        let w = (position - focus).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);
        let lens_radius = aperture / 2.0;
        let origin = *position;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;
        let time0 = _time0;
        let time1 = _time1;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius,
            time0,
            time1,
        }
    }
    /// Used to get the Ray corresponding to a Pixel and the Camera.
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_sphere();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            random_double(self.time0, self.time1),
        )
    }
}
