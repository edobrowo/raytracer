use crate::{hittable::HitRecord, Color, Ray, Vec3};
use rand::{self, Rng};

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

/// Lambertian probabilistic diffuse material.
#[derive(Debug, Clone)]
pub struct LambertianRandom {
    /// Fractional reflectance color.
    albedo: Color,

    /// Probability of scattering.
    p: f32,
}

impl LambertianRandom {
    /// Create a Lambertian material. Rays will scatter with probability `p`.
    pub fn new(albedo: &Color, p: f32, is_attenuated: bool) -> Self {
        assert!(0.0 <= p && p <= 1.0);

        let albedo = if is_attenuated {
            // When attenuated, scale the albedo by `p`.
            albedo / p
        } else {
            albedo.clone()
        };

        Self {
            albedo: albedo.clone(),
            p,
        }
    }
}

impl Material for LambertianRandom {
    #[allow(unused)]
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        // Random test on whether to scatter
        let r = rand::thread_rng().gen::<f32>();
        if r <= self.p {
            return None;
        }

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
