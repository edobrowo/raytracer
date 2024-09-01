use crate::hittable::{HitRecord, Orientation};
use crate::{util, Color, Ray, Vec3};

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
        Self { albedo: *albedo }
    }
}

impl Material for Lambertian {
    #[allow(unused)]
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        // Generate the reflected ray in the unit circle from the surface normal.
        let scatter_direction = rec.normal + Vec3::random_unit();

        // Use the surface normal if the generated ray is degenerate.
        if !scatter_direction.almost_zero() {
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
    p: f64,
}

impl LambertianRandom {
    /// Create a Lambertian material. Rays will scatter with probability `p`.
    pub fn new(albedo: &Color, p: f64, is_attenuated: bool) -> Self {
        assert!((0.0..=1.0).contains(&p));

        let albedo = if is_attenuated {
            // When attenuated, scale the albedo by `p`.
            albedo / p as f32
        } else {
            *albedo
        };

        Self { albedo, p }
    }
}

impl Material for LambertianRandom {
    #[allow(unused)]
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        // Random test on whether to scatter
        let r = util::gen_unit();
        if r <= self.p {
            return None;
        }

        // Generate the reflected ray in the unit circle from the surface normal.
        let scatter_direction = rec.normal + Vec3::random_unit();

        // Use the surface normal if the generated ray is degenerate.
        if !scatter_direction.almost_zero() {
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
        let reflected = Vec3::reflect(ray.direction(), &rec.normal);

        // Fuzz the reflected ray within a fuzz sphere.
        let reflected = reflected.unit() + (self.fuzz * Vec3::random_unit());

        let scattered = Ray::new(rec.p, reflected);

        // If the scattered ray would return back to the surface, just absorb it.
        if Vec3::dot(scattered.direction(), &rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

/// Dielectric material.
#[derive(Debug, Clone)]
pub struct Dielectric {
    /// Refractive index in a vacuum.
    refractive_index: f64,
}

impl Dielectric {
    /// Creates a new dielectric material.
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }

    /// Compute reflectance using Schlick approximation.
    /// `cosine` should be the dot of a vector and a surface normal, both normalized.
    pub fn reflectance_schlick(cosine: f64, refractive_index: f64) -> f64 {
        let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let ri = if rec.orientation == Orientation::Exterior {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = ray.direction().unit();
        let cos_theta = f64::min(Vec3::dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let total_internal_reflection = ri * sin_theta > 1.0;

        let schlick = Dielectric::reflectance_schlick(cos_theta, ri);
        let reflect_schlick = schlick > util::gen_unit();

        let direction = if total_internal_reflection || reflect_schlick {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, ri)
        };

        let scattered = Ray::new(rec.p, direction);
        let attenuation = Color::new(1.0, 1.0, 1.0);
        Some((scattered, attenuation))
    }
}

/// Normal map with Lambertian scattering.
#[derive(Debug, Clone)]
pub struct NormalMap {}

impl NormalMap {
    /// Creates a new dielectric material.
    pub fn new() -> Self {
        Self {}
    }
}

impl Material for NormalMap {
    #[allow(unused)]
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let n = rec.normal;
        let scattered = Ray::new(rec.p, n);

        let attenuation = Color::new(n.x() as f32, n.y() as f32, n.z() as f32);

        // Generate the reflected ray in the unit circle from the surface normal.
        let scatter_direction = rec.normal + Vec3::random_unit();

        // Use the surface normal if the generated ray is degenerate.
        if !scatter_direction.almost_zero() {
            Some((Ray::new(rec.p, scatter_direction), attenuation))
        } else {
            Some((Ray::new(rec.p, rec.normal), attenuation))
        }
    }
}
