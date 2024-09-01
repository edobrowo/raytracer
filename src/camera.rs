use crate::{hittable::Hittable, Color, Error, Interval, Point3, Ray, Vec3};
use rand::{self, Rng};

/// Camera information that defines the viewport into worldspace.
pub struct Camera {
    /// Image plane aspect ratio.
    pub aspect_ratio: f64,

    /// Image plane width in pixels.
    pub image_width: u32,

    /// Image plane height in pixels.
    pub image_height: u32,

    /// Maximum number of ray bounces.
    pub max_depth: u32,

    /// Number of sampels to be taken per pixel region.
    pub samples_per_pixel: u32,

    /// Vertical FOV.
    pub vfov: f64,

    /// Point camera is looking from.
    pub look_from: Point3,

    /// Point camera is looking at.
    pub look_at: Point3,

    /// Camera-relative "up" direction.
    pub vup: Vec3,

    /// Camera coordinates.
    center: Point3,

    /// Position of upper-left pixel, i.e., pixel at (0, 0).
    pixel00_loc: Point3,

    /// Horizontal delta between two pixels.
    pixel_delta_u: Vec3,

    /// Vertical delta between two pixels.
    pixel_delta_v: Vec3,

    /// Camera coordinate frame basis.
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    // Use a non-zero lower bound to prevent shadow acne.
    const INITIAL_T_BOUND: Interval = Interval::new(0.001, f64::INFINITY);

    /// Create a new camera.
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        vfov: f64,
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
    ) -> Result<Self, Error> {
        if aspect_ratio <= 0.0 {
            return Err(Error::new_camera(&format!(
                "aspect_ratio must be greater than 0 (given {aspect_ratio})"
            )));
        }
        if image_width == 0 {
            return Err(Error::new_camera(&format!(
                "image_width must be greater than 0 (given {image_width})"
            )));
        }
        if samples_per_pixel == 0 {
            return Err(Error::new_camera(&format!(
                "samples_per_pixel must be greater than 0 (given {samples_per_pixel})"
            )));
        }
        if max_depth == 0 {
            return Err(Error::new_camera(&format!(
                "max_depth must be greater than 0 (given {samples_per_pixel})"
            )));
        }

        // Determine image height with the width and aspect ratio.
        let image_height = f64::max(image_width as f64 / aspect_ratio, 1.0) as u32;

        let center = look_from;

        // Viewport dimensions.
        let focal_length = (look_from - look_at).len();
        let theta = vfov.to_radians();
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Camera basis vectors.
        let w = (look_from - look_at).unit();
        let u = Vec3::cross(&vup, &w).unit();
        let v = Vec3::cross(&w, &u);

        // Horizontal and verical viewport vectors.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Pixel delta vectors.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Upper-left pixel.
        let viewport_upper_left = center - (focal_length * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Ok(Self {
            aspect_ratio,
            image_width,
            image_height,
            max_depth,
            samples_per_pixel,
            vfov,
            look_from,
            look_at,
            vup,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            u,
            v,
            w,
        })
    }

    /// Retrieve image plane pixel dimensions.
    pub fn dim(&self) -> (u32, u32) {
        (self.image_width, self.image_height)
    }

    /// Render the image given a world of hittable objects.
    pub fn render<T: Hittable>(&self, world: &T) -> Vec<Color> {
        let mut data: Vec<Color> = Vec::new();

        for row in 0..self.image_height {
            for col in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(row, col);
                    pixel_color += Camera::ray_color(&ray, self.max_depth, world);
                }

                data.push(pixel_color / self.samples_per_pixel as f32);
            }
        }

        data
    }

    /// Build the ray from the camera to a particular pixel.
    fn get_ray(&self, row: u32, col: u32) -> Ray {
        // Build a vector to the center of the pixel.
        let pixel_u = col as f64 * self.pixel_delta_u;
        let pixel_v = row as f64 * self.pixel_delta_v;
        let pixel_center = self.pixel00_loc + pixel_u + pixel_v;

        // Sample the pixel.
        let pixel_sample = pixel_center + self.pixel_sample_square();

        // Build the ray to that pixel.
        let ray_origin = self.center;
        let ray_direction = pixel_sample - self.center;

        Ray::new(ray_origin, ray_direction)
    }

    /// Sample within a pixel square.
    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 * rand::thread_rng().gen::<f64>();
        let py = -0.5 * rand::thread_rng().gen::<f64>();

        px * self.pixel_delta_u + py * self.pixel_delta_v
    }

    /// Determine the color of a ray.
    fn ray_color<T: Hittable>(ray: &Ray, depth: u32, world: &T) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(ray, &Self::INITIAL_T_BOUND) {
            return if let Some((scattered, attenuation)) = rec.material.scatter(ray, &rec) {
                attenuation * Camera::ray_color(&scattered, depth - 1, world)
            } else {
                Color::new(0.0, 0.0, 0.0)
            };
        }

        let unit_dir = ray.direction().unit();
        let a = (0.5 * (unit_dir.y() + 1.0)) as f32;
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
