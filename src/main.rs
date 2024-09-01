use raytracer::camera::Camera;
use raytracer::hittable::HittableList;
use raytracer::material::{Dielectric, Lambertian, Metallic};
use raytracer::sphere::Sphere;
use raytracer::Color;
use raytracer::Point3;
use raytracer::{image, Vec3};
use std::error::Error;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn Error>> {
    // World setup.
    let mut world = HittableList::new();

    let mat_ground = Arc::new(Lambertian::new(&Color::new(0.8, 0.8, 0.0)));
    let mat_center = Arc::new(Lambertian::new(&Color::new(0.1, 0.2, 0.5)));
    let mat_left = Arc::new(Dielectric::new(1.5));
    let mat_bubble = Arc::new(Dielectric::new(1.0 / 1.5));
    let mat_right = Arc::new(Metallic::new(&Color::new(0.6, 0.6, 0.2), 1.0));

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        mat_ground,
    ));
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, mat_center));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, mat_bubble));
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right));

    // Camera setup.
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let vfov = 20.0;
    let look_from = Point3::new(-2.0, 2.0, 1.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        look_from,
        look_at,
        vup,
    )?;

    // Renderer setup.
    use std::time::Instant;
    let now = Instant::now();

    let data = camera.render(&world);

    let elapsed = now.elapsed();
    println!("Rendering: {:.2?}", elapsed);

    // Save the rendered image.
    let (image_width, image_height) = camera.dim();
    image::create_ppm("sample.ppm", &data, image_width, image_height)?;

    Ok(())
}
