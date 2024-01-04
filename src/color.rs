use std::fmt;
use std::ops;

#[derive(Debug, Clone, Copy)]
struct Channel(f64);

impl Channel {
    // Clamp instead of using Option
    pub fn new(val: f64) -> Channel {
        if 0.0 <= val && val < 1.0 {
            Channel(val)
        } else if val < 0.0 {
            Channel(0.0)
        } else {
            Channel(1.0 - f64::EPSILON)
        }
    }

    pub fn to_byte(&self) -> u8 {
        let val = f64::round(self.0 * 256.0) as u16;
        let val = if val > 255 { 255 } else { val };
        val as u8
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    e: [Channel; 3],
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {
            e: [Channel::new(r), Channel::new(g), Channel::new(b)],
        }
    }

    pub fn r(&self) -> f64 {
        self[0]
    }

    pub fn g(&self) -> f64 {
        self[1]
    }

    pub fn b(&self) -> f64 {
        self[2]
    }

    pub fn to_bytes(&self) -> [u8; 3] {
        [
            self.e[0].to_byte(),
            self.e[1].to_byte(),
            self.e[2].to_byte(),
        ]
    }
}

impl fmt::Display for Color {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "[{}, {}, {}]", self[0], self[1], self[2])
    }
}

impl ops::Index<usize> for Color {
    type Output = f64;
    fn index<'a>(&'a self, i: usize) -> &'a f64 {
        &self.e[i].0
    }
}

impl ops::IndexMut<usize> for Color {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f64 {
        &mut self.e[i].0
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

impl ops::Mul<f64> for Color {
    type Output = Color;
    fn mul(self, f: f64) -> Color {
        Color::new(self[0] * f, self[1] * f, self[2] * f)
    }
}

impl ops::Mul<Color> for f64 {
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

impl ops::MulAssign<f64> for Color {
    fn mul_assign(&mut self, f: f64) {
        *self = *self * f;
    }
}

impl ops::MulAssign<Color> for Color {
    fn mul_assign(&mut self, other: Color) {
        *self = *self * other;
    }
}

impl ops::Div<f64> for Color {
    type Output = Color;
    fn div(self, f: f64) -> Color {
        self * (1.0 / f)
    }
}

impl ops::Div<Color> for Color {
    type Output = Color;
    fn div(self, other: Color) -> Color {
        Color::new(self[0] / other[0], self[1] / other[1], self[2] / other[2])
    }
}

impl ops::DivAssign<f64> for Color {
    fn div_assign(&mut self, f: f64) {
        *self = *self * (1.0 / f)
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

    fn to_p7(f: f64) -> u64 {
        f64::round(f * 10000000.0) as u64
    }

    fn to_3p7(c: [f64; 3]) -> [u64; 3] {
        [to_p7(c[0]), to_p7(c[1]), to_p7(c[2])]
    }

    #[test]
    fn color_general() {
        let c = Color::new(0.1, 0.2, 0.3);
        let d = Color::new(0.4, 0.5, 0.6);

        assert_eq!(to_p7(c[0]), to_p7(0.1));
        assert_eq!(to_p7(c[1]), to_p7(0.2));
        assert_eq!(to_p7(c[2]), to_p7(0.3));
        assert_eq!(c.to_bytes(), [26, 51, 77]);

        assert_eq!(to_p7(d[0]), to_p7(0.4));
        assert_eq!(to_p7(d[1]), to_p7(0.5));
        assert_eq!(to_p7(d[2]), to_p7(0.6));
        assert_eq!(d.to_bytes(), [102, 128, 154]);

        let u = c + d;
        assert_eq!(to_3p7([u[0], u[1], u[2]]), to_3p7([0.5, 0.7, 0.9]));
        let u = d + c;
        assert_eq!(to_3p7([u[0], u[1], u[2]]), to_3p7([0.5, 0.7, 0.9]));
        let mut u = c;
        assert_eq!(to_3p7([u[0], u[1], u[2]]), to_3p7([0.1, 0.2, 0.3]));
        u += d;
        assert_eq!(to_3p7([u[0], u[1], u[2]]), to_3p7([0.5, 0.7, 0.9]));
        u += c;
        assert_eq!(
            to_3p7([u[0], u[1], u[2]]),
            to_3p7([0.6, 0.9, 1.0 - f64::EPSILON])
        );

        let u = c - d;
        assert_eq!(to_3p7([u[0], u[1], u[2]]), to_3p7([0.0, 0.0, 0.0]));
        let u = d - c;
        assert_eq!(to_3p7([u[0], u[1], u[2]]), to_3p7([0.3, 0.3, 0.3]));
        let mut u = Color::new(0.0, 0.0, 0.0);
        u += c + d;
        assert_eq!(to_3p7([u[0], u[1], u[2]]), to_3p7([0.5, 0.7, 0.9]));
        u -= c;
        assert_eq!(to_3p7([u[0], u[1], u[2]]), to_3p7([0.4, 0.5, 0.6]));

        let u = c * d;
        assert_eq!(to_3p7([u[0], u[1], u[2]]), to_3p7([0.04, 0.1, 0.18]));
        let u = d * c;
        assert_eq!(to_3p7([u[0], u[1], u[2]]), to_3p7([0.04, 0.1, 0.18]));
        let u = c / d;
        assert_eq!(
            to_3p7([u[0], u[1], u[2]]),
            to_3p7([0.1 / 0.4, 0.2 / 0.5, 0.3 / 0.6])
        );
        let u = d / c;
        assert_eq!(
            to_3p7([u[0], u[1], u[2]]),
            to_3p7([1.0 - f64::EPSILON, 1.0 - f64::EPSILON, 1.0 - f64::EPSILON])
        );
        let mut u = c;
        u *= d;
        assert_eq!(to_3p7([u[0], u[1], u[2]]), to_3p7([0.04, 0.1, 0.18]));
        u /= d;
        assert_eq!(to_3p7([u[0], u[1], u[2]]), to_3p7([0.1, 0.2, 0.3]));
        u /= d;
        assert_eq!(
            [to_p7(u[0]), to_p7(u[1]), to_p7(u[2])],
            [to_p7(1.0 / 4.0), to_p7(2.0 / 5.0), to_p7(3.0 / 6.0)]
        );

        let u = 2.0 * c;
        assert_eq!(to_3p7([u[0], u[1], u[2]]), to_3p7([0.2, 0.4, 0.6]));
        let u = c * 2.0;
        assert_eq!(to_3p7([u[0], u[1], u[2]]), to_3p7([0.2, 0.4, 0.6]));
        let mut u = 3.0 * c;
        assert_eq!(to_3p7([u[0], u[1], u[2]]), to_3p7([0.3, 0.6, 0.9]));
        u *= 1.1;
        assert_eq!(to_3p7([u[0], u[1], u[2]]), to_3p7([0.33, 0.66, 0.99]));
        u /= 2.0;
        assert_eq!(
            to_3p7([u[0], u[1], u[2]]),
            to_3p7([0.33 / 2.0, 0.66 / 2.0, 0.99 / 2.0])
        );
        u /= 3.0;
        assert_eq!(
            to_3p7([u[0], u[1], u[2]]),
            to_3p7([0.33 / 6.0, 0.66 / 6.0, 0.99 / 6.0])
        );
    }
}
