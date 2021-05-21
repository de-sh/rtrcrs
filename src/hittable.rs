use std::sync::Arc;

use crate::{Aabb, Color, Lambertian, Material, Point3, Ray, Vec3};

/// Defines a record data-structure to store the information about Rays hitting multiple objects.
#[derive(Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    /// Used to initialise record data-structure.
    pub fn default() -> Self {
        Self {
            point: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            material: Arc::new(Lambertian::new(Color::new(0.0, 0.0, 0.0))),
        }
    }
    /// Used to set the outward normal of the surface.
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -outward_normal
        };
    }
}

/// Defines the interfaces that can be implemented on any kind of object abstraction.
pub trait Hittable: Sync + Send {
    /// Defines the hit operation on the object with the provided Ray.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &Aabb) -> Option<Aabb>;
}
