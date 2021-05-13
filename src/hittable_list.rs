use std::sync::Arc;

use crate::{HitRecord, Hittable, Ray};

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = None;

        for object in self.objects.iter() {
            hit_anything = object.hit(ray, t_min, t_max)
        }

        hit_anything
    }
}
