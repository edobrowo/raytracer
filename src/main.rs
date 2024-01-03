use raytracer_ow::Color;
use raytracer_ow::PpmWriter;
//use raytracer_ow::Vec3;
use std::error::Error;
use std::fs::File;

fn gen_ppm(path: &str, data: &Vec<Color>, width: u32, height: u32) -> Result<(), Box<dyn Error>> {
    let data = data.iter().map(|color| color.to_bytes()).collect();
    let file = File::create(path)?;
    let mut ppmstream = PpmWriter::new(file);
    ppmstream.write(data, width, height, 255)?;
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let width = 256;
    let height = 256;

    let mut data: Vec<Color> = Vec::new();
    for row in 0..height {
        for col in 0..width {
            data.push(Color::new(
                (col as f64) / (width as f64 - 1.0),
                (row as f64) / (height as f64 - 1.0),
                0.0,
            ));
        }
    }

    gen_ppm("test.ppm", &data, width, height)?;

    Ok(())
}
