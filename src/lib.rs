//! Definition of a 3-dimensional model rendering engine using ray tracing.
pub type Vec3 = nalgebra::Vector3<f64>;

pub mod color;
use color::Color;

pub mod ray;
use ray::{Point3, Ray};

pub mod hittable;
use hittable::{HitRecord, Hittable};

pub mod sphere;
use sphere::Sphere;

pub mod hittable_list;
use hittable_list::HittableList;

pub mod definitions;
use definitions::{degrees_to_radians, random_double, random_in_unit_sphere, INFINITY};

pub mod camera;

pub mod material;
use material::{Dielectric, Lambertian, Material, Metal};

pub mod moving;
use moving::MovingSphere;

pub mod aabb;
use aabb::Aabb;

pub mod bvh;
use bvh::BvhNode;
