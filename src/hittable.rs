use crate::{Interval, Point3, Ray, Vec3};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Face {
    Outside,
    Inside,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    t: f64,
    face: Face,
}

impl HitRecord {
    pub fn new(p: Point3, outward_normal: Vec3, t: f64, ray: &Ray) -> Self {
        if Vec3::dot(ray.direction(), &outward_normal) < 0.0 {
            Self {
                p,
                normal: outward_normal,
                t,
                face: Face::Outside,
            }
        } else {
            Self {
                p,
                normal: -outward_normal,
                t,
                face: Face::Inside,
            }
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

pub struct HittableList<T: Hittable> {
    objects: Vec<T>,
}

impl<T: Hittable> HittableList<T> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: T) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl<T: Hittable> Default for HittableList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let (rec, _): (Option<HitRecord>, f64) =
            self.objects
                .iter()
                .fold((None, ray_t.max()), |(rec, tmax), object| {
                    if let Some(rec) = object.hit(ray, Interval::new(ray_t.min(), tmax)) {
                        let tmax = rec.t;
                        return (Some(rec), tmax);
                    }
                    (rec, tmax)
                });
        rec
    }
}
