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

    /// Camera coordinates.
    pub center: Point3,

    /// Position of upper-left pixel, i.e., pixel at (0, 0).
    pixel00_loc: Point3,

    /// Horizontal delta between two pixels.
    pixel_delta_u: Vec3,

    /// Vertical delta between two pixels.
    pixel_delta_v: Vec3,

    /// Maximum number of ray bounces.
    max_depth: u32,

    /// Number of sampels to be taken per pixel region.
    samples_per_pixel: u32,
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

        // Camera parameters.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let center = Point3::new(0.0, 0.0, 0.0);

        // Horizontal and verical viewport vectors.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Pixel delta vectors.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Upper-left pixel.
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Ok(Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            max_depth,
            samples_per_pixel,
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
