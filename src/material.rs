use crate::{hittable::HitRecord, Color, Ray, Vec3};

/// Specifies how rays scatter off of geometry.
pub trait Material {
    /// Determines the reflected ray and color produced by a particular hit.
    #[allow(unused)]
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        None
    }
}

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
    /// Fractional reflectance color.
    albedo: Color,

    /// Fuzz radius. Specifies a sphere around a perfect reflected ray
    /// in which the actual reflected ray can be generated.
    fuzz: f64,
}

impl Metallic {
    // Creates a new metallic material.
    pub fn new(albedo: &Color, fuzz: f64) -> Self {
        Metallic {
            albedo: *albedo,
            fuzz: f64::min(fuzz, 1.0),
        }
    }
}

impl Material for Metallic {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray.direction().reflect(&rec.normal);

        // Fuzz the reflected ray within a fuzz sphere.
        let reflected = reflected.unit() + (self.fuzz * &Vec3::random_unit());

        let scattered = Ray::new(rec.p, reflected);

        // If the scattered ray would return back to the surface, just absorb it.
        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}
