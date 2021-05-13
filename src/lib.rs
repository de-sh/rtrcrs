//! Definition of a 3-dimensional model rendering engine using ray tracing.
pub mod vector;
use vector::Vec3;

pub mod color;
use color::Color;

pub mod ray;
use ray::{Ray, Point3};

pub mod hittable;
use hittable::{HitRecord, Hittable};

pub mod sphere;
use sphere::Sphere;

pub mod hittable_list;
use hittable_list::HittableList;
