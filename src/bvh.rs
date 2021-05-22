use std::sync::Arc;

use crate::{Aabb, Hittable, HittableList, definitions::random_double};

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bvh_box: Aabb,
}

impl BvhNode {
    pub fn new(src_objects: Arc<dyn Hittable>, time0: f64, time1: f64, start:f64, end:f64) -> Self {
        let mut object = list.objects.into_iter();
        let axis = random_double(0.0,2.0) as u8;
        let compar
        Self {}
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let None = self.bvh_box.hit(ray, t_min, t_max) {
            None
        } else {
            let (mut rec, mut t) = (HitRecord::default(), false);
            if let Some(hit_left) = self.left.hit(ray, t_min, t_max) {
                rec = hit_left;
                t = true;
            }
            if let Some(hit_right) = self.right.hit(ray, t_min, if t { rec.t } else { t_max }) {
                rec = hit_right;
                t = true;
            }
            if t {
                Some(rec)
            } else {
                None
            }
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        Some(self.bvh_box)
    }
}
