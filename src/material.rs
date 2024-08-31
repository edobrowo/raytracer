use crate::{hittable::HitRecord, Color, Ray, Vec3};

pub trait Material {
    /// Determines the reflected ray and color produced by a particular hit.
    #[allow(unused)]
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        None
    }
}

// TODO: add sometimes-scatter and attentuated scatter modes

/// Lambertian diffuse material.
#[derive(Debug, Clone)]
pub struct Lambertian {
    /// Fractional reflectance color.
    albedo: Color,
}

impl Lambertian {
    /// Create a Lambertian material. Rays will always scatter.
    pub fn new(albedo: &Color) -> Self {
        Self {
            albedo: albedo.clone(),
        }
    }
}

impl Material for Lambertian {
    #[allow(unused)]
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        // Generate the reflected ray in the unit circle from the surface normal.
        let scatter_direction = rec.normal + Vec3::random_unit();

        // Use the surface normal if the generated ray is degenerate.
        if !scatter_direction.is_almost_zero() {
            Some((Ray::new(rec.p, scatter_direction), self.albedo))
        } else {
            Some((Ray::new(rec.p, rec.normal), self.albedo))
        }
    }
}

/// Metallic material.
#[derive(Debug, Clone)]
pub struct Metallic {
    albedo: Color,
}

impl Metallic {
    // Creates a new metallic material.
    pub fn new(albedo: &Color) -> Self {
        Metallic {
            albedo: albedo.clone(),
        }
    }
}

impl Material for Metallic {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflect(ray.direction(), &rec.normal);
        Some((Ray::new(rec.p, reflected), self.albedo))
    }
}
