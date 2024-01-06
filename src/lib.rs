mod camera;
mod color;
mod hittable;
mod interval;
mod netbpm;
mod ray;
mod sphere;
mod vec3;

pub use camera::Camera;
pub use color::Color;
pub use hittable::{HitRecord, Hittable, HittableList};
pub use interval::Interval;
pub use netbpm::PpmWriter;
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::{Point3, Vec3};

use std::error::Error;
use std::fs::File;

pub fn create_ppm(path: &str, data: &Vec<Color>, w: u32, h: u32) -> Result<(), Box<dyn Error>> {
    let data = data.iter().map(|color| color.to_rgb24()).collect();
    let file = File::create(path)?;
    let mut ppmstream = PpmWriter::new(file);
    ppmstream.write(data, w, h, 255)?;
    Ok(())
}
