use std::fmt;
use std::ops;

use crate::Interval;

/// RGB color.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Color {
    channels: [f32; 3],
}

impl Color {
    /// Used to clamp color values when converting to byte representations
    const INTENSITY: Interval = Interval::new(0.0, 0.999999);

    /// Minimum error for color operations.
    const ERROR: f32 = 1e-6;

    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            channels: [r, g, b],
        }
    }

    /// Retrieve the red channel.
    pub fn r(&self) -> f32 {
        self[0]
    }

    /// Retrieve the green channel.
    pub fn g(&self) -> f32 {
        self[1]
    }

    /// Retrieve the blue channel.
    pub fn b(&self) -> f32 {
        self[2]
    }

    /// Determines whether the given color is approximately all zero (black in color).
    pub fn is_almost_zero(&self) -> bool {
        self.channels
            .iter()
            .all(|&channel| f32::abs(channel) < Self::ERROR)
    }

    /// Determines whether two colors are approximately equal.
    pub fn is_almost_equal(&self, color: &Self) -> bool {
        Self::is_almost_zero(&(self - color))
    }
}

impl Color {
    /// Convert to RGB24 byte representation.
    pub fn to_rgb24(&self) -> [u8; 3] {
        [
            Self::make_byte(self.r()),
            Self::make_byte(self.g()),
            Self::make_byte(self.b()),
        ]
    }

    /// Make byte from a channel value.
    fn make_byte(channel: f32) -> u8 {
        f64::floor(Self::INTENSITY.clamp(channel as f64) * 255.0) as u8
    }
}

impl Color {
    /// Gamma correct a channel value.
    fn linear_to_gamma(channel: f32) -> f32 {
        if channel > 0.0 {
            f32::sqrt(channel)
        } else {
            0.0
        }
    }

    /// Gamma correct the RGB color.
    pub fn gamma_correct(&self) -> Self {
        Self::new(
            Self::linear_to_gamma(self.r()),
            Self::linear_to_gamma(self.g()),
            Self::linear_to_gamma(self.b()),
        )
    }
}

impl fmt::Display for Color {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "[{}, {}, {}]", self.r(), self.g(), self.b())
    }
}

impl ops::Index<usize> for Color {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        &self.channels[i]
    }
}

impl ops::IndexMut<usize> for Color {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        &mut self.channels[i]
    }
}

macro_rules! add {
    ( $lhs:ty , $rhs:ty ) => {
        impl ops::Add<$rhs> for $lhs {
            type Output = Color;
            fn add(self, rhs: $rhs) -> Color {
                Color::new(self.r() + rhs.r(), self.g() + rhs.g(), self.b() + rhs.b())
            }
        }
    };
}

add!(Color, Color);
add!(Color, &Color);
add!(&Color, Color);
add!(&Color, &Color);

macro_rules! subtract {
    ( $lhs:ty , $rhs:ty ) => {
        impl ops::Sub<$rhs> for $lhs {
            type Output = Color;
            fn sub(self, rhs: $rhs) -> Color {
                Color::new(self.r() - rhs.r(), self.g() - rhs.g(), self.b() - rhs.b())
            }
        }
    };
}

subtract!(Color, Color);
subtract!(Color, &Color);
subtract!(&Color, Color);
subtract!(&Color, &Color);

macro_rules! scalar_multiply_rhs {
    ( $lhs:ty , $rhs:ty ) => {
        impl ops::Mul<$rhs> for $lhs {
            type Output = Color;
            fn mul(self, rhs: $rhs) -> Color {
                Color::new(self.r() * rhs, self.g() * rhs, self.b() * rhs)
            }
        }
    };
}

scalar_multiply_rhs!(Color, f32);
scalar_multiply_rhs!(&Color, f32);
scalar_multiply_rhs!(Color, &f32);
scalar_multiply_rhs!(&Color, &f32);

macro_rules! scalar_multiply_lhs {
    ( $lhs:ty , $rhs:ty ) => {
        impl ops::Mul<$rhs> for $lhs {
            type Output = Color;
            fn mul(self, rhs: $rhs) -> Color {
                Color::new(self * rhs.r(), self * rhs.g(), self * rhs.b())
            }
        }
    };
}

scalar_multiply_lhs!(f32, Color);
scalar_multiply_lhs!(&f32, Color);
scalar_multiply_lhs!(f32, &Color);
scalar_multiply_lhs!(&f32, &Color);

macro_rules! hadamard_multiply {
    ( $lhs:ty , $rhs:ty ) => {
        impl ops::Mul<$rhs> for $lhs {
            type Output = Color;
            fn mul(self, rhs: $rhs) -> Color {
                Color::new(self.r() * rhs.r(), self.g() * rhs.g(), self.b() * rhs.b())
            }
        }
    };
}

hadamard_multiply!(Color, Color);
hadamard_multiply!(&Color, Color);
hadamard_multiply!(Color, &Color);
hadamard_multiply!(&Color, &Color);

macro_rules! scalar_divide {
    ( $lhs:ty , $rhs:ty ) => {
        impl ops::Div<$rhs> for $lhs {
            type Output = Color;
            fn div(self, rhs: $rhs) -> Color {
                self * (1.0 / rhs)
            }
        }
    };
}

scalar_divide!(Color, f32);
scalar_divide!(&Color, f32);
scalar_divide!(Color, &f32);
scalar_divide!(&Color, &f32);

macro_rules! hadamard_divide {
    ( $lhs:ty , $rhs:ty ) => {
        impl ops::Div<$rhs> for $lhs {
            type Output = Color;
            fn div(self, rhs: $rhs) -> Color {
                Color::new(self.r() / rhs.r(), self.g() / rhs.g(), self.b() / rhs.b())
            }
        }
    };
}

hadamard_divide!(Color, Color);
hadamard_divide!(&Color, Color);
hadamard_divide!(Color, &Color);
hadamard_divide!(&Color, &Color);

macro_rules! add_assign {
    ( $rhs:ty ) => {
        impl ops::AddAssign<$rhs> for Color {
            fn add_assign(&mut self, rhs: $rhs) {
                *self = *self + rhs
            }
        }
    };
}

add_assign!(Color);
add_assign!(&Color);

macro_rules! subtract_assign {
    ( $rhs:ty ) => {
        impl ops::SubAssign<$rhs> for Color {
            fn sub_assign(&mut self, rhs: $rhs) {
                *self = *self - rhs
            }
        }
    };
}

subtract_assign!(Color);
subtract_assign!(&Color);

macro_rules! scalar_multiply_assign {
    ( $rhs:ty ) => {
        impl ops::MulAssign<$rhs> for Color {
            fn mul_assign(&mut self, rhs: $rhs) {
                self.channels[0] = self.r() * rhs;
                self.channels[1] = self.g() * rhs;
                self.channels[2] = self.b() * rhs
            }
        }
    };
}

scalar_multiply_assign!(f32);
scalar_multiply_assign!(&f32);

macro_rules! hadamard_multiply_assign {
    ( $rhs:ty ) => {
        impl ops::MulAssign<$rhs> for Color {
            fn mul_assign(&mut self, rhs: $rhs) {
                self.channels[0] = self.r() * rhs.r();
                self.channels[1] = self.g() * rhs.g();
                self.channels[2] = self.b() * rhs.b()
            }
        }
    };
}

hadamard_multiply_assign!(Color);
hadamard_multiply_assign!(&Color);

macro_rules! scalar_divide_assign {
    ( $rhs:ty ) => {
        impl ops::DivAssign<$rhs> for Color {
            fn div_assign(&mut self, rhs: $rhs) {
                self.channels[0] = self.r() / rhs;
                self.channels[1] = self.g() / rhs;
                self.channels[2] = self.b() / rhs
            }
        }
    };
}

scalar_divide_assign!(f32);
scalar_divide_assign!(&f32);

macro_rules! hadamard_divide_assign {
    ( $rhs:ty ) => {
        impl ops::DivAssign<$rhs> for Color {
            fn div_assign(&mut self, rhs: $rhs) {
                self.channels[0] = self.r() / rhs.r();
                self.channels[1] = self.g() / rhs.g();
                self.channels[2] = self.b() / rhs.b()
            }
        }
    };
}

hadamard_divide_assign!(Color);
hadamard_divide_assign!(&Color);

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn color_channels() {
        let c = Color::new(0.1, 0.2, 0.3);
        assert_eq!(c[0], 0.1);
        assert_eq!(c[1], 0.2);
        assert_eq!(c[2], 0.3);

        let c = Color::new(0.4, 0.5, 0.6);
        assert_eq!(c[0], 0.4);
        assert_eq!(c[1], 0.5);
        assert_eq!(c[2], 0.6);
    }

    #[test]
    fn _almost_zero() {
        let c = Color::new(0.0, 0.0, 0.0);
        assert!(c.is_almost_zero());

        let c = Color::new(0.0, 0.001, 0.0);
        assert!(!c.is_almost_zero());

        let c = Color::new(0.0, 1e-7, 0.0);
        assert!(c.is_almost_zero());
    }

    #[test]
    fn color_almost_equal() {
        let c = Color::new(0.1, 0.2, 0.3);
        let d = Color::new(0.4, 0.5, 0.6);
        let e = Color::new(1.0, 0.0, 1.0);

        assert!(c.is_almost_equal(&c));
        assert!(!c.is_almost_equal(&d));
        assert!(!c.is_almost_equal(&e));

        assert!(d.is_almost_equal(&d));
        assert!(!d.is_almost_equal(&c));
        assert!(!d.is_almost_equal(&e));

        assert!(e.is_almost_equal(&e));
        assert!(!e.is_almost_equal(&c));
        assert!(!e.is_almost_equal(&d));

        let d = Color::new(0.1 + 1e-5, 0.2, 0.3);
        let e = Color::new(0.1 + 1e-8, 0.2, 0.3);

        assert!(c.is_almost_equal(&c));
        assert!(!c.is_almost_equal(&d));
        assert!(c.is_almost_equal(&e));
    }

    #[test]
    fn color_arithmetic() {
        let c = Color::new(0.1, 0.2, 0.3);
        let d = Color::new(0.4, 0.5, 0.6);

        let e = c + d;
        assert!(e.is_almost_equal(&Color::new(0.5, 0.7, 0.9)));
        let e = d + c;
        assert!(e.is_almost_equal(&Color::new(0.5, 0.7, 0.9)));
        let mut e = c;
        assert!(e.is_almost_equal(&Color::new(0.1, 0.2, 0.3)));
        e += d;
        assert!(e.is_almost_equal(&Color::new(0.5, 0.7, 0.9)));
        e += c;
        assert!(e.is_almost_equal(&Color::new(0.6, 0.9, 1.2)));

        let e = c - d;
        assert!(e.is_almost_equal(&Color::new(-0.3, -0.3, -0.3)));
        let e = d - c;
        assert!(e.is_almost_equal(&Color::new(0.3, 0.3, 0.3)));
        let mut e = Color::new(0.0, 0.0, 0.0);
        e += c + d;
        assert!(e.is_almost_equal(&Color::new(0.5, 0.7, 0.9)));
        e -= c;
        assert!(e.is_almost_equal(&Color::new(0.4, 0.5, 0.6)));

        let e = 2.0 * c;
        assert!(e.is_almost_equal(&Color::new(0.2, 0.4, 0.6)));
        let e = c * 2.0;
        assert!(e.is_almost_equal(&Color::new(0.2, 0.4, 0.6)));
        let mut e = 3.0 * c;
        assert!(e.is_almost_equal(&Color::new(0.3, 0.6, 0.9)));
        e *= 1.1;
        assert!(e.is_almost_equal(&Color::new(0.33, 0.66, 0.99)));
        e /= 2.0;
        assert!(e.is_almost_equal(&Color::new(0.33 / 2.0, 0.66 / 2.0, 0.99 / 2.0)));
        e /= 3.0;
        assert!(e.is_almost_equal(&Color::new(0.33 / 6.0, 0.66 / 6.0, 0.99 / 6.0)));

        let e = c * d;
        assert!(e.is_almost_equal(&Color::new(0.04, 0.1, 0.18)));
        let e = d * c;
        assert!(e.is_almost_equal(&Color::new(0.04, 0.1, 0.18)));
        let e = c / d;
        assert!(e.is_almost_equal(&Color::new(0.1 / 0.4, 0.2 / 0.5, 0.3 / 0.6)));
        let e = d / c;
        assert!(e.is_almost_equal(&Color::new(4.0, 2.5, 2.0)));
        let mut e = c;
        e *= d;
        assert!(e.is_almost_equal(&Color::new(0.04, 0.1, 0.18)));
        e /= d;
        assert!(e.is_almost_equal(&Color::new(0.1, 0.2, 0.3)));
        e /= d;
        assert!(e.is_almost_equal(&Color::new(1.0 / 4.0, 2.0 / 5.0, 3.0 / 6.0)));
    }

    #[test]
    fn color_bytes() {
        let c = Color::new(0.1, 0.2, 0.3);
        assert_eq!(c.to_rgb24(), [25, 51, 76]);

        let c = Color::new(0.4, 0.5, 0.6);
        assert_eq!(c.to_rgb24(), [102, 127, 153]);
    }

    #[test]
    fn color_gamma_correct() {
        let c = Color::new(0.1, 0.2, 0.3);
        assert!(c.gamma_correct().is_almost_equal(&Color::new(
            f32::sqrt(0.1),
            f32::sqrt(0.2),
            f32::sqrt(0.3)
        )));

        let c = Color::new(0.0, 1.0, 0.0);
        assert!(c
            .gamma_correct()
            .is_almost_equal(&Color::new(0.0, 1.0, 0.0)));
    }
}
