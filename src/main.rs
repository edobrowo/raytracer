use raytracer::camera::Camera;
use raytracer::hittable::HittableList;
use raytracer::image;
use raytracer::material::{Lambertian, Metallic};
use raytracer::sphere::Sphere;
use raytracer::Color;
use raytracer::Point3;
use std::error::Error;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn Error>> {
    // World
    let mut world = HittableList::new();

    let mat_ground = Arc::new(Lambertian::new(&Color::new(0.8, 0.8, 0.0)));
    let mat_center = Arc::new(Lambertian::new(&Color::new(0.1, 0.2, 0.5)));
    let mat_left = Arc::new(Metallic::new(&Color::new(0.8, 0.8, 0.8), 0.3));
    let mat_right = Arc::new(Metallic::new(&Color::new(0.6, 0.6, 0.2), 1.0));

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        mat_ground,
    ));
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, mat_center));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left));
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right));

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth)?;

    // Render
    use std::time::Instant;
    let now = Instant::now();

    let data = camera.render(&world);

    let elapsed = now.elapsed();
    println!("Rendering: {:.2?}", elapsed);

    // Save
    let (image_width, image_height) = camera.dim();
    image::create_ppm("sample.ppm", &data, image_width, image_height)?;

    Ok(())
}
