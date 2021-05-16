use crate::{Point3, Ray, Vec3};

/// Defines a record data-structure to store the information about Rays hitting multiple objects.
#[derive(Copy, Clone)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    /// Used to initialise record data-structure.
    pub fn default() -> Self {
        Self {
            point: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        }
    }
    /// Used to set values in record with given record.
    pub fn set(&mut self, rec: &Self) {
        self.t = rec.t;
        self.point = rec.point;
        self.normal = rec.normal;
        self.front_face = rec.front_face;
    }

    /// Used to set the outward normal of the surface.
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

/// Defines the interfaces that can be implemented on any kind of object abstraction.
pub trait Hittable {
    /// Defines the hit operation on the object with the provided Ray.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}