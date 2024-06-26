use raytracer_ow::camera::Camera;
use raytracer_ow::hittable::HittableList;
use raytracer_ow::sphere::Sphere;
use raytracer_ow::Point3;
use std::error::Error;
use raytracer_ow::image;

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
    image::create_ppm("sample.ppm", &data, image_width, image_height)?;

    Ok(())
}
