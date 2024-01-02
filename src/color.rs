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
    //
}
