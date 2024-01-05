use raytracer_ow::Color;
use raytracer_ow::PpmWriter;
use raytracer_ow::Ray;
use raytracer_ow::{Point3, Vec3};
use std::error::Error;
use std::fs::File;

fn gen_ppm(path: &str, data: &Vec<Color>, width: u32, height: u32) -> Result<(), Box<dyn Error>> {
    let data = data.iter().map(|color| color.to_bytes()).collect();
    let file = File::create(path)?;
    let mut ppmstream = PpmWriter::new(file);
    ppmstream.write(data, width, height, 255)?;
    Ok(())
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = *ray.origin() - center;
    let a = ray.direction().len_sqr();
    let half_b = Vec3::dot(&oc, ray.direction());
    let c = oc.len_sqr() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - f64::sqrt(discriminant)) / a
    }
}

fn ray_color(ray: Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, &ray);
    if t > 0.0 {
        let normal = Vec3::unit(&(ray.at(t) - Vec3::new(0.0, 0.0, -1.0)));
        return 0.5 * Color::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
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

            let pixel_color = ray_color(r);
            data.push(pixel_color);
        }
    }

    gen_ppm("test.ppm", &data, image_width, image_height)?;

    Ok(())
}
