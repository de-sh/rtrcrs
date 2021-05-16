use crate::{Color, Vec3};

/// Defines an alias for Vec3, used to define a point in 3-dimensional co-ordinate space.
pub type Point3 = Vec3;

/// Defines a Ray using a reference starting point and a direction vector.
#[derive(Debug, PartialEq, Copy, Clone)]
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
    pub fn origin(self) -> Point3 {
        self.orig
    }

    /// Returns the direction of the given Ray.
    pub fn direction(self) -> Vec3 {
        self.dir
    }

    /// Returns point along Ray at `t`.
    pub fn at(self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }

    /// Returns the expected color at the intersection of any ray and the object(s).
    pub fn color(self) -> Color {
        let mut t = self.hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5);
        if t > 0.0 {
            let n = Vec3::unit_vector(self.at(t) - Vec3::new(0.0, 0.0, -1.0));
            0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)
        } else {
            let unit_dir = Vec3::unit_vector(self.dir);
            t = 0.5 * (unit_dir.y() + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }

    /// Returns a decision variable as an f64, describing whether a ray hits the corresponding sphere.
    pub fn hit_sphere(&self, center: Point3, radius: f64) -> f64 {
        let oc = self.orig - center;
        let (a, b, c) = (
            self.dir.dot(self.dir),
            2.0 * oc.dot(self.dir),
            oc.dot(oc) - radius * radius,
        );
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            -1.0
        } else {
            (-b - discriminant.sqrt()) / (2.0 * a)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_test() {
        let origin = Point3::new(3.0, 2.0, 1.0);
        let dir = Vec3::new(2.0, 3.0, 5.0);
        let ray = Ray::new(origin, dir);
        assert_eq!(ray.origin(), origin);
        assert_eq!(ray.direction(), dir);
        assert_eq!(ray.at(3.0), origin + dir * 3.0);
    }
    #[test]
    fn color_test() {
        let origin = Point3::new(3.0, 2.0, 1.0);
        let dir = Vec3::new(2.0, 3.0, 5.0);
        let color = Ray::new(origin, dir).color();
        assert_eq!(color, Color::new(0.6283339341519281, 0.7770003604911568, 1.0));
    }
}
