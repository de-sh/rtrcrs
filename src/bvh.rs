use std::{cmp::Ordering, sync::Arc};

use crate::{random_double, Aabb, HitRecord, Hittable, HittableList, Ray};

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bvh_box: Aabb,
}

impl BvhNode {
    pub fn new(list: &mut HittableList, time0: f64, time1: f64) -> Self {
        let len = list.objects().len();
        Self::_new(list.objects(), time0, time1, 0, len)
    }

    pub fn _new(
        objects: &mut Vec<Arc<dyn Hittable>>,
        time0: f64,
        time1: f64,
        start: usize,
        end: usize,
    ) -> Self {
        fn box_compare(
            time0: f64,
            time1: f64,
            axis: usize,
        ) -> impl FnMut(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> Ordering {
            move |a, b| {
                if let (Some(box_a), Some(box_b)) =
                    (a.bounding_box(time0, time1), b.bounding_box(time0, time1))
                {
                    let cmp = |x: Aabb| x.min()[axis] + x.min()[axis];
                    cmp(box_a).partial_cmp(&cmp(box_b)).unwrap()
                } else {
                    panic!("No bounding box in bvh_node constructor.");
                }
            }
        }

        let axis = random_double(0.0, 2.9999) as usize;
        let object_span = end - start;

        objects.sort_unstable_by(box_compare(time0, time1, axis));

        let (left, right) = match object_span {
            1 => (objects[start].clone(), objects[start].clone()),
            2 => (objects[start].clone(), objects[start + 1].clone()),
            _ => {
                let mid = start + object_span / 2;
                let left: Arc<dyn Hittable> =
                    Arc::new(BvhNode::_new(objects, time0, time1, start, mid));
                let right: Arc<dyn Hittable> =
                    Arc::new(BvhNode::_new(objects, time0, time1, mid, end));
                (left, right)
            }
        };

        if let (Some(box_left), Some(box_right)) = (
            left.bounding_box(time0, time1),
            right.bounding_box(time0, time1),
        ) {
            Self {
                left,
                right,
                bvh_box: Aabb::surrounding_box(&box_left, &box_right),
            }
        } else {
            panic!("No bounding box in bvh_node constructor.")
        }
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

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(self.bvh_box.clone())
    }
}
