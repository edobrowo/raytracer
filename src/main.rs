use raytracer::camera::Camera;
use raytracer::hittable::HittableList;
use raytracer::image;
use raytracer::sphere::Sphere;
use raytracer::Point3;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // World
    let mut world = HittableList::new();

    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 1;
    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel)?;

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
