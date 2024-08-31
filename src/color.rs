use std::fmt;
use std::ops;

use crate::Interval;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Color {
    e: [f32; 3],
}

impl Color {
    const INTENSITY: Interval = Interval::new(0.0, 0.999);

    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { e: [r, g, b] }
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
        write!(fmt, "[{}, {}, {}]", self[0], self[1], self[2])
    }
}

impl ops::Index<usize> for Color {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        &self.e[i]
    }
}

impl ops::IndexMut<usize> for Color {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        &mut self.e[i]
    }
}

impl ops::Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color::new(self[0] + other[0], self[1] + other[1], self[2] + other[2])
    }
}

impl ops::Sub for Color {
    type Output = Color;
    fn sub(self, other: Color) -> Color {
        Color::new(self[0] - other[0], self[1] - other[1], self[2] - other[2])
    }
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        *self = *self + other;
    }
}

impl ops::SubAssign for Color {
    fn sub_assign(&mut self, other: Color) {
        *self = *self - other;
    }
}

impl ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self, f: f32) -> Color {
        Color::new(self[0] * f, self[1] * f, self[2] * f)
    }
}

impl ops::Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, c: Color) -> Color {
        Color::new(self * c[0], self * c[1], self * c[2])
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        Color::new(self[0] * other[0], self[1] * other[1], self[2] * other[2])
    }
}

impl ops::MulAssign<f32> for Color {
    fn mul_assign(&mut self, f: f32) {
        *self = *self * f;
    }
}

impl ops::MulAssign<Color> for Color {
    fn mul_assign(&mut self, other: Color) {
        *self = *self * other;
    }
}

impl ops::Div<f32> for Color {
    type Output = Color;
    fn div(self, f: f32) -> Color {
        self * (1.0 / f)
    }
}

impl ops::Div<Color> for Color {
    type Output = Color;
    fn div(self, other: Color) -> Color {
        Color::new(self[0] / other[0], self[1] / other[1], self[2] / other[2])
    }
}

impl ops::DivAssign<f32> for Color {
    fn div_assign(&mut self, f: f32) {
        *self *= 1.0 / f
    }
}

impl ops::DivAssign<Color> for Color {
    fn div_assign(&mut self, other: Color) {
        *self = *self / other
    }
}

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
