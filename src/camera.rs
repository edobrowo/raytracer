use crate::{hittable::Hittable, vec3, Color, Error, Interval, Point3, Ray, Vec3};
use rand::{self, Rng};

pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: u32,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32) -> Result<Self, Error> {
        if aspect_ratio <= 0.0 {
            return Err(Error::new_camera(&format!(
                "aspect ratio must be greater than 0 (given {aspect_ratio})"
            )));
        }
        if image_width == 0 {
            return Err(Error::new_camera(&format!(
                "image width must be greater than 0 (given {image_width})"
            )));
        }
        if samples_per_pixel == 0 {
            return Err(Error::new_camera(&format!(
                "samples per pixel must be greater than 0 (given {samples_per_pixel})"
            )));
        }

        // Image
        let image_height = f64::max(image_width as f64 / aspect_ratio, 1.0) as u32;

        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
        let center = Point3::new(0.0, 0.0, 0.0);

        // Horizontal and verical viewport vectors
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Pixel delta vectors
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Upper-left pixel
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
            samples_per_pixel,
        })
    }

    pub fn dim(&self) -> (u32, u32) {
        (self.image_width, self.image_height)
    }

    pub fn render<T: Hittable>(&self, world: &T) -> Vec<Color> {
        let mut data: Vec<Color> = Vec::new();
        for row in 0..self.image_height {
            for col in 0..self.image_width {
                let mut pixel_color = Color::new_rgb(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(row, col);
                    pixel_color += Camera::ray_color(ray, world);
                }
                data.push(pixel_color / self.samples_per_pixel as f64);
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

        let unit_dir = Vec3::unit(ray.direction());
        let a = 0.5 * (unit_dir.y() + 1.0);
        (1.0 - a) * Color::new_rgb(1.0, 1.0, 1.0) + a * Color::new_rgb(0.5, 0.7, 1.0)
    }
}
