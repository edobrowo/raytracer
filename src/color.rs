use std::fmt;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    e: [f64; 3],
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { e: [r, g, b] }
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
}

impl fmt::Display for Color {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "[{}, {}, {}]", self[0], self[1], self[2])
    }
}

impl ops::Index<usize> for Color {
    type Output = f64;
    fn index<'a>(&'a self, i: usize) -> &'a f64 {
        &self.e[i]
    }
}

impl ops::IndexMut<usize> for Color {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f64 {
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
        f64::round(f * 1000000.0) as u64
    }

    #[test]
    fn color_general() {
        let c = Color::new(1.0, 2.0, 3.0);
        let d = Color::new(4.0, 5.0, 6.0);

        assert_eq!(c[0], 1.0);
        assert_eq!(c[1], 2.0);
        assert_eq!(c[2], 3.0);

        assert_eq!(d[0], 4.0);
        assert_eq!(d[1], 5.0);
        assert_eq!(d[2], 6.0);

        let u = c + d;
        assert_eq!([u[0], u[1], u[2]], [5.0, 7.0, 9.0]);
        let u = d + c;
        assert_eq!([u[0], u[1], u[2]], [5.0, 7.0, 9.0]);
        let mut u = c;
        assert_eq!([u[0], u[1], u[2]], [1.0, 2.0, 3.0]);
        u += d;
        assert_eq!([u[0], u[1], u[2]], [5.0, 7.0, 9.0]);
        u += c;
        assert_eq!([u[0], u[1], u[2]], [6.0, 9.0, 12.0]);

        let u = c - d;
        assert_eq!([u[0], u[1], u[2]], [-3.0, -3.0, -3.0]);
        let u = d - c;
        assert_eq!([u[0], u[1], u[2]], [3.0, 3.0, 3.0]);
        let mut u = Color::new(0.0, 0.0, 0.0);
        u += c + d;
        assert_eq!([u[0], u[1], u[2]], [5.0, 7.0, 9.0]);
        u -= c;
        assert_eq!([u[0], u[1], u[2]], [4.0, 5.0, 6.0]);

        let u = c * d;
        assert_eq!([u[0], u[1], u[2]], [4.0, 10.0, 18.0]);
        let u = d * c;
        assert_eq!([u[0], u[1], u[2]], [4.0, 10.0, 18.0]);
        let u = c / d;
        assert_eq!(
            [to_p7(u[0]), to_p7(u[1]), to_p7(u[2])],
            [to_p7(1.0 / 4.0), to_p7(2.0 / 5.0), to_p7(3.0 / 6.0)]
        );
        let u = d / c;
        assert_eq!(
            [to_p7(u[0]), to_p7(u[1]), to_p7(u[2])],
            [to_p7(4.0 / 1.0), to_p7(5.0 / 2.0), to_p7(6.0 / 3.0)]
        );
        let mut u = c;
        u *= d;
        assert_eq!([u[0], u[1], u[2]], [4.0, 10.0, 18.0]);
        u /= d;
        assert_eq!([u[0], u[1], u[2]], [1.0, 2.0, 3.0]);
        u /= d;
        assert_eq!(
            [to_p7(u[0]), to_p7(u[1]), to_p7(u[2])],
            [to_p7(1.0 / 4.0), to_p7(2.0 / 5.0), to_p7(3.0 / 6.0)]
        );

        let u = 5.0 * c;
        assert_eq!([u[0], u[1], u[2]], [5.0, 10.0, 15.0]);
        let u = c * 5.0;
        assert_eq!([u[0], u[1], u[2]], [5.0, 10.0, 15.0]);
        let mut u = 5.0 * c;
        assert_eq!([u[0], u[1], u[2]], [5.0, 10.0, 15.0]);
        u *= 2.0;
        assert_eq!([u[0], u[1], u[2]], [10.0, 20.0, 30.0]);
        u /= 2.0;
        assert_eq!([u[0], u[1], u[2]], [5.0, 10.0, 15.0]);
        u /= 3.0;
        assert_eq!(
            [to_p7(u[0]), to_p7(u[1]), to_p7(u[2])],
            [to_p7(5.0 / 3.0), to_p7(10.0 / 3.0), to_p7(15.0 / 3.0)]
        );
    }
}
