use crate::{material::Material, Interval, Point3, Ray, Vec3};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Orientation {
    Interior,
    Exterior,
}

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
    t: f64,
    orientation: Orientation,
}

impl<'a> HitRecord<'a> {
    pub fn new(p: Point3, normal: Vec3, t: f64, ray: &Ray, material: &'a dyn Material) -> Self {
        let (normal, orientation) = if Vec3::dot(ray.direction(), &normal) < 0.0 {
            (normal, Orientation::Exterior)
        } else {
            (-normal, Orientation::Interior)
        };

        Self {
            p,
            normal,
            material,
            t,
            orientation,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;
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
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        self.objects
            .iter()
            .fold((None, ray_t.max()), |(rec, t_max), object| {
                if let Some(rec) = object.hit(ray, &Interval::new(ray_t.min(), t_max)) {
                    let t = rec.t;
                    (Some(rec), t)
                } else {
                    (rec, t_max)
                }
            })
            .0
    }
}
