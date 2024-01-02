use std::error::Error;
use std::fmt;
use std::io::{BufWriter, Write};

#[derive(Debug, Clone)]
struct PpmError {
    message: String,
}

impl PpmError {
    pub fn from(message: &str) -> PpmError {
        PpmError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for PpmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PPM error: {}", self.message)
    }
}

impl Error for PpmError {}

const PPM_BITDEPTH_MIN: u32 = 1;
const PPM_BITDEPTH_MAX: u32 = 65535;

struct PpmBitDepth(u32);

impl PpmBitDepth {
    pub fn new(val: u32) -> Result<PpmBitDepth, PpmError> {
        if PPM_BITDEPTH_MIN <= val && val <= PPM_BITDEPTH_MAX {
            Ok(PpmBitDepth(val))
        } else {
            Err(PpmError::from(
                format!(
                    "bitdepth must fall within the range [{},{}]",
                    PPM_BITDEPTH_MIN, PPM_BITDEPTH_MAX
                )
                .as_str(),
            ))
        }
    }
}

impl fmt::Display for PpmBitDepth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct PpmDim(u32);

impl PpmDim {
    pub fn new(val: u32) -> Result<PpmDim, PpmError> {
        if val > 0 {
            Ok(PpmDim(val))
        } else {
            Err(PpmError::from("image width must be greater than 0"))
        }
    }
}

impl fmt::Display for PpmDim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct PpmImage {
    data: Vec<[u8; 3]>,
    width: PpmDim,
    height: PpmDim,
    bitdepth: PpmBitDepth,
}

impl PpmImage {
    const MAGIC_NUMBER: &'static [u8; 2] = b"P6";

    pub fn from(
        data: Vec<[u8; 3]>,
        width: u32,
        height: u32,
        bitdepth: u32,
    ) -> Result<PpmImage, PpmError> {
        let width = PpmDim::new(width)?;
        let height = PpmDim::new(height)?;
        let bitdepth = PpmBitDepth::new(bitdepth)?;

        if data.len() as u64 != width.0 as u64 * height.0 as u64 {
            return Err(PpmError::from(
                format!(
                    "color vector size ({}) does not match dimensions ({}*{}={})",
                    data.len(),
                    width,
                    height,
                    width.0 as u64 * height.0 as u64
                )
                .as_str(),
            ));
        }

        for color in data.iter() {
            if let Some(chan) = color.iter().find(|&&chan| chan as u32 > bitdepth.0) {
                return Err(PpmError::from(
                    format!("channel value {chan} is invalid, expected channel<={bitdepth}")
                        .as_str(),
                ));
            }
        }

        Ok(PpmImage {
            data,
            width,
            height,
            bitdepth,
        })
    }
}

pub struct PpmWriter<W: Write> {
    stream: BufWriter<W>,
}

impl<W: Write> PpmWriter<W> {
    pub fn new(inner: W) -> PpmWriter<W> {
        let stream = BufWriter::new(inner);
        PpmWriter { stream }
    }

    pub fn write(
        &mut self,
        data: Vec<[u8; 3]>,
        width: u32,
        height: u32,
        bitdepth: u32,
    ) -> Result<usize, Box<dyn Error>> {
        let image = PpmImage::from(data, width, height, bitdepth)?;

        self.stream.write(PpmImage::MAGIC_NUMBER)?;
        self.stream.write(b"\n")?;
        self.stream.write(image.width.to_string().as_bytes())?;
        self.stream.write(b" ")?;
        self.stream.write(image.height.to_string().as_bytes())?;
        self.stream.write(b" ")?;
        self.stream.write(image.bitdepth.to_string().as_bytes())?;
        self.stream.write(b"\n")?;

        for color in image.data {
            // TODO: If bit depth is less than 256, 1 byte is used per channel. Otherwise 2 bytes is used, MSB first.
            self.stream.write(&color[..])?;
        }

        self.stream.flush()?;

        Ok(0)
    }
}
