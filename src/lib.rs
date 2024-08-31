pub mod camera;
pub mod color;
pub mod hittable;
pub mod image;
pub mod interval;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;

pub use color::Color;
pub use interval::Interval;
pub use ray::Ray;
pub use vec3::{Point3, Vec3};

#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }

    pub fn new_camera(msg: &str) -> Self {
        Self {
            kind: ErrorKind::Camera(msg.to_string()),
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Camera(String),
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::Camera(_) => "camera error",
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ErrorKind::Camera(ref s) => write!(f, "{}", s),
        }
    }
}
