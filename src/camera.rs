use rand::{self, Rng};
use std::error::Error;
use std::fmt;

use crate::{hittable::Hittable, vec3, Color, Interval, Point3, Ray, Vec3};

#[derive(Debug, Clone)]
pub struct CameraError {
    message: String,
}

impl CameraError {
    pub fn from(message: &str) -> CameraError {
        CameraError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for CameraError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PPM error: {}", self.message)
    }
}

impl Error for CameraError {}

#[derive(Clone, Copy)]
pub struct ImageDim(pub u32);

impl ImageDim {
    pub fn new(val: u32) -> Result<ImageDim, CameraError> {
        if val > 0 {
            Ok(ImageDim(val))
        } else {
            Err(CameraError::from("image width must be greater than 0"))
        }
    }
}

struct AspectRatio(f64);

impl AspectRatio {
    pub fn new(val: f64) -> Result<AspectRatio, CameraError> {
        if val > 0.0 {
            Ok(AspectRatio(val))
        } else {
            Err(CameraError::from("aspect ratio must be greater than 0"))
        }
    }
}

struct SamplesCount(u32);

impl SamplesCount {
    pub fn new(val: u32) -> Result<SamplesCount, CameraError> {
        if val > 0 {
            Ok(SamplesCount(val))
        } else {
            Err(CameraError::from("aspect ratio must be greater than 0"))
        }
    }
}

pub struct Camera {
    aspect_ratio: AspectRatio,
    image_width: ImageDim,
    image_height: ImageDim,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: SamplesCount,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
    ) -> Result<Camera, CameraError> {
        let image_width = ImageDim::new(image_width)?;
        let aspect_ratio = AspectRatio::new(aspect_ratio)?;
        let samples_per_pixel = SamplesCount::new(samples_per_pixel)?;

        // Image
        let image_height =
            ImageDim::new(f64::max(image_width.0 as f64 / aspect_ratio.0, 1.0) as u32)?;

        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width.0 as f64 / image_height.0 as f64);
        let center = Point3::new(0.0, 0.0, 0.0);

        // Horizontal and verical viewport vectors
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Pixel delta vectors
        let pixel_delta_u = viewport_u / image_width.0 as f64;
        let pixel_delta_v = viewport_v / image_height.0 as f64;

        // Upper-left pixel
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Ok(Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
        })
    }

    pub fn dim(&self) -> (ImageDim, ImageDim) {
        (self.image_width, self.image_height)
    }

    pub fn render<T: Hittable>(&self, world: &T) -> Vec<Color> {
        let mut data: Vec<Color> = Vec::new();
        for row in 0..self.image_height.0 {
            for col in 0..self.image_width.0 {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel.0 {
                    let ray = self.get_ray(row, col);
                    pixel_color += Camera::ray_color(ray, world);
                }
                data.push(pixel_color / self.samples_per_pixel.0 as f64);
            }
        }
        data
    }

    fn get_ray(&self, row: u32, col: u32) -> Ray {
        let pixel_center = self.pixel00_loc
            + (col as f64 * self.pixel_delta_u)
            + (row as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - self.center;

        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 * rand::thread_rng().gen::<f64>();
        let py = -0.5 * rand::thread_rng().gen::<f64>();
        px * self.pixel_delta_u + py * self.pixel_delta_v
    }

    fn ray_color<T: Hittable>(ray: Ray, world: &T) -> Color {
        if let Some(rec) = world.hit(&ray, Interval::new(0.0, f64::INFINITY)) {
            let n = rec.normal;
            let direction = vec3::random_on_hemisphere(&n);
            return 0.5 * Camera::ray_color(Ray::new(rec.p, direction), world);
        }

        let unit_dir = Vec3::unit(&ray.direction());
        let a = 0.5 * (unit_dir.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
