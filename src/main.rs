use raytracer_ow::PpmWriter;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let data: Vec<[u8; 3]> = vec![
        [255, 0, 0],
        [0, 255, 0],
        [0, 0, 255],
        [255, 255, 0],
        [255, 255, 255],
        [0, 0, 0],
    ];

    let file = File::create("test.ppm")?;
    let mut ppmstream = PpmWriter::new(file);

    ppmstream.write(data, 3, 2, 255)?;

    Ok(())
}
