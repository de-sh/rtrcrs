use std::sync::Arc;

use crate::{Aabb, HitRecord, Hittable, Material, Point3, Ray, Vec3};

/// Defines a geometrically Spherical object.
pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: f64,
    time1: f64,
    radius: f64,
    material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }
    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    /// Provides a definition of hit() for spherical objects.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(ray.time());
        let (a, half_b, c) = (
            ray.direction().dot(&ray.direction()),
            oc.dot(&ray.direction()),
            oc.dot(&oc) - self.radius.powi(2),
        );

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lied in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        let mut rec = HitRecord {
            point,
            normal: Vec3::new(0.0, 0.0, 0.0),
            t,
            front_face: false,
            material: self.material.clone(),
        };

        let outward_normal = (point - self.center(ray.time())) / self.radius;
        rec.set_face_normal(ray, &outward_normal);
        rec.material = self.material.clone();

        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &Aabb) -> Option<Aabb> {
        let box0 = Aabb::new(
            self.center(time0) - Point3::new(self.radius, self.radius, self.radius),
            self.center(time0) + Point3::new(self.radius, self.radius, self.radius),
        );
        let box1 = Aabb::new(
            self.center(time1) - Point3::new(self.radius, self.radius, self.radius),
            self.center(time1) + Point3::new(self.radius, self.radius, self.radius),
        );

        Some(Aabb::surrounding_box(&box0, &box1))
    }
}
