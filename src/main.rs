use raytracer_ow::{Camera, HittableList, Point3, Sphere};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // World
    let mut world = HittableList::new();

    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 10;
    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel)?;

    // Render
    let data = camera.render(&world);

    // Save
    let (image_width, image_height) = camera.dim();
    raytracer_ow::create_ppm("scene.ppm", &data, image_width.0, image_height.0)?;

    Ok(())
}
