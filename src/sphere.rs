use crate::hittable::{HitRecord, Hittable};
use crate::{Interval, Point3, Ray, Vec3};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = *ray.origin() - self.center;
        let a = ray.direction().len_sqr();
        let half_b = Vec3::dot(&oc, ray.direction());
        let c = oc.len_sqr() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f64::sqrt(discriminant);

        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = ray.at(root);
        let outward_normal = (p - self.center) / self.radius;

        Some(HitRecord::new(p, outward_normal, t, ray))
    }
}
