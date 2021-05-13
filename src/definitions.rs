use crate::{Point3, Ray, Vec3};

pub use std::f64::consts::PI;
pub const INFINITY: f64 = std::f64::MAX;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
