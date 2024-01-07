pub mod camera;
pub mod color;
pub mod hittable;
pub mod interval;
pub mod netbpm;
pub mod ray;
pub mod sphere;
pub mod vec3;

pub use color::Color;
pub use interval::Interval;
pub use ray::Ray;
pub use vec3::{Point3, Vec3};

use std::error::Error;
use std::fs::File;
use std::path::Path;

pub fn create_ppm<P>(path: P, data: &Vec<Color>, w: u32, h: u32) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let data = data.iter().map(|color| color.to_rgb24()).collect();
    let file = File::create(path)?;
    let mut ppmstream = netbpm::PpmWriter::new(file);
    ppmstream.write(data, w, h, 255)?;
    Ok(())
}
