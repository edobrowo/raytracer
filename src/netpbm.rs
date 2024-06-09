use std::error::Error;
use std::fmt;
use std::io::{BufWriter, Write};

#[derive(Debug, Clone)]
struct NetpbmError {
    message: String,
}

impl NetpbmError {
    pub fn from(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for NetpbmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "netpbm error: {}", self.message)
    }
}

impl Error for NetpbmError {}

const PPM_BITDEPTH_MIN: u32 = 1;
const PPM_BITDEPTH_MAX: u32 = 65535;

struct PpmBitDepth(u32);

impl PpmBitDepth {
    pub fn new(val: u32) -> Result<Self, NetpbmError> {
        if (PPM_BITDEPTH_MIN..=PPM_BITDEPTH_MAX).contains(&val) {
            Ok(Self(val))
        } else {
            Err(NetpbmError::from(
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
    pub fn new(val: u32) -> Result<Self, NetpbmError> {
        if val > 0 {
            Ok(Self(val))
        } else {
            Err(NetpbmError::from("image dimension must be greater than 0"))
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
    ) -> Result<PpmImage, NetpbmError> {
        let width = PpmDim::new(width)?;
        let height = PpmDim::new(height)?;
        let bitdepth = PpmBitDepth::new(bitdepth)?;

        if data.len() as u64 != width.0 as u64 * height.0 as u64 {
            return Err(NetpbmError::from(
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
                return Err(NetpbmError::from(
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

#[derive(Debug)]
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

        self.stream.write_all(PpmImage::MAGIC_NUMBER)?;
        self.stream.write_all(b"\n")?;
        self.stream.write_all(image.width.to_string().as_bytes())?;
        self.stream.write_all(b" ")?;
        self.stream.write_all(image.height.to_string().as_bytes())?;
        self.stream.write_all(b" ")?;
        self.stream
            .write_all(image.bitdepth.to_string().as_bytes())?;
        self.stream.write_all(b"\n")?;

        for color in image.data {
            // TODO: If bit depth is less than 256, 1 byte is used per channel. Otherwise 2 bytes is used, MSB first.
            self.stream.write_all(&color[..])?;
        }

        self.stream.flush()?;

        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::PpmWriter;
    use std::io;

    // Dummy buffer used to validate successful writes
    #[derive(Debug)]
    struct ImageBuffer {
        buffer: Vec<u8>,
    }

    impl ImageBuffer {
        fn new() -> Self {
            ImageBuffer { buffer: Vec::new() }
        }
    }

    impl io::Write for ImageBuffer {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn invalid_images() {
        let data: Vec<[u8; 3]> = vec![
            [255, 0, 0],
            [0, 255, 0],
            [0, 0, 255],
            [255, 255, 0],
            [255, 255, 255],
            [0, 0, 0],
        ];

        let buffer = ImageBuffer::new();
        let mut stream = PpmWriter::new(buffer);

        assert!(!stream.write(data.clone(), 3, 0, 255).is_ok());
        assert!(!stream.write(data.clone(), 0, 2, 255).is_ok());
        assert!(!stream.write(data.clone(), 3, 3, 255).is_ok());
        assert!(!stream.write(data.clone(), 2, 2, 255).is_ok());
        assert!(!stream.write(data.clone(), 3, 2, 0).is_ok());
        assert!(!stream.write(data.clone(), 3, 2, 65536).is_ok());
        assert!(!stream.write(data, u32::MAX, u32::MAX, 255).is_ok());
    }

    #[test]
    fn valid_images() {
        let data: Vec<[u8; 3]> = vec![
            [255, 0, 0],
            [0, 255, 0],
            [0, 0, 255],
            [255, 255, 0],
            [255, 255, 255],
            [0, 0, 0],
        ];

        let mut ppmwriter = PpmWriter::new(ImageBuffer::new());
        let expected = [
            80, 54, 10, 51, 32, 50, 32, 50, 53, 53, 10, 255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255,
            0, 255, 255, 255, 0, 0, 0,
        ];

        assert!(ppmwriter.write(data, 3, 2, 255).is_ok());

        let inner = ppmwriter.stream.into_inner().unwrap().buffer;
        assert_eq!(inner[..], expected[..]);
    }
}
