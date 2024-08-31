use std::fmt;
use std::ops;

use crate::Interval;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Color {
    channels: [f32; 3],
}

impl Color {
    const INTENSITY: Interval = Interval::new(0.0, 0.999);

    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            channels: [r, g, b],
        }
    }

    pub fn r(&self) -> f32 {
        self[0]
    }

    pub fn g(&self) -> f32 {
        self[1]
    }

    pub fn b(&self) -> f32 {
        self[2]
    }

    pub fn to_rgb24(&self) -> [u8; 3] {
        [
            Self::make_byte(self.r()),
            Self::make_byte(self.g()),
            Self::make_byte(self.b()),
        ]
    }

    fn make_byte(channel: f32) -> u8 {
        f64::floor(Self::INTENSITY.clamp(channel as f64) * 255.0) as u8
    }

    fn linear_to_gamma(channel: f32) -> f32 {
        if channel > 0.0 {
            f32::sqrt(channel)
        } else {
            0.0
        }
    }

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

    fn f32_to_fixed(f: f32) -> u64 {
        f32::round(f * 1000000.0) as u64
    }

    fn color_to_fixed(c: [f32; 3]) -> [u64; 3] {
        [f32_to_fixed(c[0]), f32_to_fixed(c[1]), f32_to_fixed(c[2])]
    }

    #[test]
    fn color_general() {
        let c = Color::new(0.1, 0.2, 0.3);
        let d = Color::new(0.4, 0.5, 0.6);

        assert_eq!(f32_to_fixed(c[0]), f32_to_fixed(0.1));
        assert_eq!(f32_to_fixed(c[1]), f32_to_fixed(0.2));
        assert_eq!(f32_to_fixed(c[2]), f32_to_fixed(0.3));
        assert_eq!(c.to_rgb24(), [25, 51, 76]);

        assert_eq!(f32_to_fixed(d[0]), f32_to_fixed(0.4));
        assert_eq!(f32_to_fixed(d[1]), f32_to_fixed(0.5));
        assert_eq!(f32_to_fixed(d[2]), f32_to_fixed(0.6));
        assert_eq!(d.to_rgb24(), [102, 127, 153]);

        let u = c + d;
        assert_eq!(
            color_to_fixed([u[0], u[1], u[2]]),
            color_to_fixed([0.5, 0.7, 0.9])
        );
        let u = d + c;
        assert_eq!(
            color_to_fixed([u[0], u[1], u[2]]),
            color_to_fixed([0.5, 0.7, 0.9])
        );
        let mut u = c;
        assert_eq!(
            color_to_fixed([u[0], u[1], u[2]]),
            color_to_fixed([0.1, 0.2, 0.3])
        );
        u += d;
        assert_eq!(
            color_to_fixed([u[0], u[1], u[2]]),
            color_to_fixed([0.5, 0.7, 0.9])
        );
        u += c;
        assert_eq!(
            color_to_fixed([u[0], u[1], u[2]]),
            color_to_fixed([0.6, 0.9, 1.2])
        );

        let u = c - d;
        assert_eq!(
            color_to_fixed([u[0], u[1], u[2]]),
            color_to_fixed([0.0, 0.0, 0.0])
        );
        let u = d - c;
        assert_eq!(
            color_to_fixed([u[0], u[1], u[2]]),
            color_to_fixed([0.3, 0.3, 0.3])
        );
        let mut u = Color::new(0.0, 0.0, 0.0);
        u += c + d;
        assert_eq!(
            color_to_fixed([u[0], u[1], u[2]]),
            color_to_fixed([0.5, 0.7, 0.9])
        );
        u -= c;
        assert_eq!(
            color_to_fixed([u[0], u[1], u[2]]),
            color_to_fixed([0.4, 0.5, 0.6])
        );

        let u = c * d;
        assert_eq!(
            color_to_fixed([u[0], u[1], u[2]]),
            color_to_fixed([0.04, 0.1, 0.18])
        );
        let u = d * c;
        assert_eq!(
            color_to_fixed([u[0], u[1], u[2]]),
            color_to_fixed([0.04, 0.1, 0.18])
        );
        let u = c / d;
        assert_eq!(
            color_to_fixed([u[0], u[1], u[2]]),
            color_to_fixed([0.1 / 0.4, 0.2 / 0.5, 0.3 / 0.6])
        );
        let u = d / c;
        assert_eq!(
            color_to_fixed([u[0], u[1], u[2]]),
            color_to_fixed([4.0, 2.5, 2.0])
        );
        let mut u = c;
        u *= d;
        assert_eq!(
            color_to_fixed([u[0], u[1], u[2]]),
            color_to_fixed([0.04, 0.1, 0.18])
        );
        u /= d;
        assert_eq!(
            color_to_fixed([u[0], u[1], u[2]]),
            color_to_fixed([0.1, 0.2, 0.3])
        );
        u /= d;
        assert_eq!(
            [f32_to_fixed(u[0]), f32_to_fixed(u[1]), f32_to_fixed(u[2])],
            [
                f32_to_fixed(1.0 / 4.0),
                f32_to_fixed(2.0 / 5.0),
                f32_to_fixed(3.0 / 6.0)
            ]
        );

        let u = 2.0 * c;
        assert_eq!(
            color_to_fixed([u[0], u[1], u[2]]),
            color_to_fixed([0.2, 0.4, 0.6])
        );
        let u = c * 2.0;
        assert_eq!(
            color_to_fixed([u[0], u[1], u[2]]),
            color_to_fixed([0.2, 0.4, 0.6])
        );
        let mut u = 3.0 * c;
        assert_eq!(
            color_to_fixed([u[0], u[1], u[2]]),
            color_to_fixed([0.3, 0.6, 0.9])
        );
        u *= 1.1;
        assert_eq!(
            color_to_fixed([u[0], u[1], u[2]]),
            color_to_fixed([0.33, 0.66, 0.99])
        );
        u /= 2.0;
        assert_eq!(
            color_to_fixed([u[0], u[1], u[2]]),
            color_to_fixed([0.33 / 2.0, 0.66 / 2.0, 0.99 / 2.0])
        );
        u /= 3.0;
        assert_eq!(
            color_to_fixed([u[0], u[1], u[2]]),
            color_to_fixed([0.33 / 6.0, 0.66 / 6.0, 0.99 / 6.0])
        );
    }
}
