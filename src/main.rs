use raytracer_ow::Color;
use raytracer_ow::Interval;
use raytracer_ow::Ray;
use raytracer_ow::{Hittable, HittableList, Sphere};
use raytracer_ow::{Point3, Vec3};
use std::error::Error;

fn ray_color<T: Hittable>(ray: Ray, world: &HittableList<T>) -> Color {
    if let Some(rec) = world.hit(
        &ray,
        Interval {
            min: 0.0,
            max: f64::INFINITY,
        },
    ) {
        let n = rec.normal;
        return 0.5 * (Color::new(n[0], n[1], n[2]) + Color::new(1.0, 1.0, 1.0));
    }

    let unit_dir = Vec3::unit(&ray.direction());
    let a = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = f64::max(image_width as f64 / aspect_ratio, 1.0) as u32;

    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Horizontal and verical viewport vectors
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Pixel vectors
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Upper-left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    let mut data: Vec<Color> = Vec::new();
    for row in 0..image_height {
        for col in 0..image_width {
            let pixel_center =
                pixel00_loc + (col as f64 * pixel_delta_u) + (row as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(r, &world);
            data.push(pixel_color);
        }
    }

    raytracer_ow::create_ppm("scene.ppm", &data, image_width, image_height)?;

    Ok(())
}
