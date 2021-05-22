use crate::{HitRecord, Hittable, Point3, Ray};

pub struct Aabb {
    min: Point3,
    max: Point3,
}

impl Aabb {
    pub fn default() -> Self {
        Self::new(Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0))
    }

    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
        let min = Point3::new(
            f64::min(box0.min.x, box1.min.x),
            f64::min(box0.min.y, box1.min.y),
            f64::min(box0.min.z, box1.min.z),
        );
        let max = Point3::new(
            f64::max(box0.max.x, box1.max.x),
            f64::max(box0.max.y, box1.max.y),
            f64::max(box0.max.z, box1.max.z),
        );
        Aabb { min, max }
    }
}

impl Hittable for Aabb {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        for a in 0..3 {
            let invd = 1.0 / ray.direction()[a];
            let t0 = (self.min[a] - ray.origin()[a]) * invd;
            let t1 = (self.max[a] - ray.origin()[a]) * invd;
            let (t1, t0) = if invd < 0.0 { (t0, t1) } else { (t1, t0) };
            let t_min = t0.max(t_min);
            let t_max = t1.min(t_max);

            if t_max <= t_min {
                return None;
            }
        }
        Some(HitRecord::default())
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        None
    }
}
