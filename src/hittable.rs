use crate::{material::Material, Interval, Point3, Ray, Vec3};

/// Indicates a particular side of a closed polyhedron.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Orientation {
    /// The inside of a closed polyhedron.
    Interior,

    /// The outside of a closed polyhedron.
    Exterior,
}

/// Information recorded at a ray-object intersection.
#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    /// The point of intersection.
    pub p: Point3,

    /// The surface normal at `p`.
    pub normal: Vec3,

    /// Reference to the object material.
    pub material: &'a dyn Material,

    /// Ray parameter on intersect.
    t: f64,

    /// Orientation at which the intersection occurs.
    pub orientation: Orientation,
}

impl<'a> HitRecord<'a> {
    /// Creates a new hit record.
    pub fn new(p: &Point3, normal: &Vec3, t: f64, ray: &Ray, material: &'a dyn Material) -> Self {
        // Enforce the normal and ray to be in the same hemisphere.
        let (normal, orientation) = if Vec3::dot(ray.direction(), &normal) < 0.0 {
            (normal.clone(), Orientation::Exterior)
        } else {
            (-normal.clone(), Orientation::Interior)
        };

        Self {
            p: p.clone(),
            normal,
            material,
            t,
            orientation,
        }
    }
}

/// Specifies how rays intersect geometry.
pub trait Hittable {
    /// Produces a hit record when an intersection occurs.
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}

/// List of objects that can be hit by rays.
pub struct HittableList<T: Hittable> {
    objects: Vec<T>,
}

impl<T: Hittable> HittableList<T> {
    /// Creates a new hittable list.
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    /// Adds to hittable to the list.
    pub fn add(&mut self, object: T) {
        self.objects.push(object);
    }

    /// Clears the list.
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
