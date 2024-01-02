use std::fmt;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
    len: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        let len = f64::sqrt(x * x + y * y + z * z);
        Vec3 { e: [x, y, z], len }
    }

    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
    }

    pub fn len(&self) -> f64 {
        self.len
    }

    pub fn len_sqr(&self) -> f64 {
        self.len * self.len
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self[0] * other[0] + self[1] + other[1] + self[2] + other[2]
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        )
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.len
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "[{}, {}, {}]", self[0], self[1], self[2])
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index<'a>(&'a self, i: usize) -> &'a f64 {
        &self.e[i]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut f64 {
        &mut self.e[i]
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(self[0], self[1], self[2])
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self[0] + other[0], self[1] + other[1], self[2] + other[2])
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        self + -other
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = *self + other;
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = *self - other;
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, f: f64) -> Vec3 {
        Vec3::new(self[0] * f, self[1] * f, self[2] * f)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self[0] * other[0], self[1] * other[1], self[2] * other[2])
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, f: f64) {
        *self = *self * f;
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        *self = *self * other;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, f: f64) -> Vec3 {
        self * (1.0 / f)
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        Vec3::new(self[0] / other[0], self[1] / other[1], self[2] / other[2])
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, f: f64) {
        *self = *self * (1.0 / f)
    }
}

impl ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        *self = *self / other
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;
    //
}
