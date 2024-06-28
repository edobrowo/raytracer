use crate::Color;
use std::error::Error;
use std::fs::File;
use std::path::Path;

use netpbmr::{ppm, EncodingType};

pub fn create_ppm<P>(path: P, data: &[Color], w: u32, h: u32) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let file = File::create(path)?;
    let mut encoder = ppm::Encoder::new(file);

    let data: Vec<u8> = data.iter().flat_map(|color| color.to_rgb24()).collect();

    encoder.write(EncodingType::Raw, w, h, 255, &data)?;

    Ok(())
}
