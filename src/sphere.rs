use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::{Interval, Point3, Ray, Vec3};

/// Sphere object in world space and material.
#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    /// Creates a new sphere.
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // Use discriminant to determine number of intersections
        let oc = ray.origin() - &self.center;
        let a = ray.direction().len_sqr();
        let half_b = Vec3::dot(&oc, ray.direction());
        let c = oc.len_sqr() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f64::sqrt(discriminant);

        // Take the first root where there is a hit
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        // Compute the normal, i.e. the reflected ray
        let t = root;
        let p = ray.at(root);
        let outward_normal = (&p - &self.center) / self.radius;

        Some(HitRecord::new(&p, &outward_normal, t, ray, &*self.material))
    }
}
