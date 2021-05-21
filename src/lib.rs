//! Definition of a 3-dimensional model rendering engine using ray tracing.
pub type Vec3 = nalgebra::Vector3<f64>;

pub mod color;
use color::Color;

pub mod ray;
use ray::{Point3, Ray};

pub mod hittable;
use hittable::{HitRecord, Hittable};

pub mod sphere;

pub mod hittable_list;
use hittable_list::HittableList;

pub mod definitions;
use definitions::INFINITY;

pub mod camera;

pub mod material;
use material::{Lambertian, Material};

pub mod moving;
use moving::MovingSphere;

pub mod aabb;
use aabb::Aabb;
