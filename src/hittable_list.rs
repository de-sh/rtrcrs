use std::sync::Arc;

use crate::{Aabb, HitRecord, Hittable, Ray};

/// Defines a data-structure to store all the Hittable objects.
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    /// Creates an empty object list.
    pub fn default() -> Self {
        Self { objects: vec![] }
    }

    /// Used to add a new instance of an Hittable object to the list.  
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    /// Used to clear all the Hittable objects of the corresponding Ray.
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn objects(&mut self) -> &mut Vec<Arc<dyn Hittable>> {
        &mut self.objects
    }
}

impl Hittable for HittableList {
    /// Provides a direct interface to run hit() on a list of [Hittable](Hittable) objects.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(temp_rec) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                hit_anything = Some(temp_rec);
            }
        }

        hit_anything
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.objects.is_empty() {
            None
        } else {
            let (mut first_box, mut output_box) = (true, Aabb::default());
            for object in self.objects.iter() {
                if let Some(temp_box) = object.bounding_box(time0, time1) {
                    output_box = if first_box {
                        first_box = false;
                        temp_box
                    } else {
                        Aabb::surrounding_box(&output_box, &temp_box)
                    };
                } else {
                    return None;
                }
            }
            Some(output_box)
        }
    }
}
