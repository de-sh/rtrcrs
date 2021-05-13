use crate::{Color, HitRecord, Hittable, HittableList, Vec3, INFINITY};

/// Defines an alias for Vec3, used to define a point in 3-dimensional co-ordinate space.
pub type Point3 = Vec3;

/// Defines a Ray using a reference starting point and a direction vector.
#[derive(Clone, Copy)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    /// This function creates a new Ray.
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    /// Returns the origin of the given Ray.
    pub fn origin(&self) -> Point3 {
        self.orig
    }

    /// Returns the direction of the given Ray.
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    /// Returns point along Ray at `t`.
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }

    /// Returns the expected color at the intersection of any ray and the object(s).
    pub fn color(&self, world: &HittableList) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(self, 0.0, INFINITY, &mut rec) {
            0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))
        } else {
            let unit_dir = Vec3::unit_vector(self.dir);
            let t = 0.5 * (unit_dir.y() + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }

    /// Returns a decision variable as an f64, describing whether a ray hits the corresponding sphere.
    pub fn hit_sphere(&self, center: Point3, radius: f64) -> f64 {
        let oc = self.orig - center;
        let (a, half_b, c) = (
            self.dir.length_squared(),
            oc.dot(self.dir),
            oc.length_squared() - radius.powi(2),
        );
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            -1.0
        } else {
            (-half_b - discriminant.sqrt()) / a
        }
    }
}
