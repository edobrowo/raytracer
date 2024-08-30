use rand::{self, Rng};
use std::fmt;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
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

    pub fn len_sqr(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn len(&self) -> f64 {
        f64::sqrt(self.len_sqr())
    }

    pub fn dot(v: &Self, w: &Self) -> f64 {
        v.x() * w.x() + v.y() * w.y() + v.z() * w.z()
    }

    pub fn cross(v: &Self, w: &Self) -> Self {
        Self::new(
            v.y() * w.z() - v.z() * w.y(),
            v.z() * w.x() - v.x() * w.z(),
            v.x() * w.y() - v.y() * w.x(),
        )
    }

    pub fn unit(v: &Self) -> Self {
        *v / v.len()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "({}, {}, {})", self[0], self[1], self[2])
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self[0], -self[1], -self[2])
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

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v[0], self * v[1], self * v[2])
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
        *self *= 1.0 / f
    }
}

impl ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        *self = *self / other
    }
}

impl Vec3 {
    fn random() -> Self {
        Self::new(
            rand::thread_rng().gen::<f64>(),
            rand::thread_rng().gen::<f64>(),
            rand::thread_rng().gen::<f64>(),
        )
    }

    fn random_in_range(min: f64, max: f64) -> Self {
        Self::new(min, min, min) + (max - min) * Self::random()
    }

    fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_in_range(-1.0, 1.0);
            if p.len_sqr() < 1.0 {
                return p;
            }
        }
    }

    fn random_unit() -> Self {
        Self::unit(&Self::random_in_unit_sphere())
    }

    pub fn random_on_hemisphere(normal: &Self) -> Self {
        let u = Self::random_unit();
        if Self::dot(&u, normal) > 0.0 {
            u
        } else {
            -u
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    fn f64_to_fixed(f: f64) -> u64 {
        f64::round(f * 1000000.0) as u64
    }

    fn vec3_to_fixed(c: [f64; 3]) -> [u64; 3] {
        [f64_to_fixed(c[0]), f64_to_fixed(c[1]), f64_to_fixed(c[2])]
    }

    #[test]
    fn vec3_general() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let w = Vec3::new(4.0, 5.0, 6.0);

        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
        assert_eq!(f64_to_fixed(v.len()), 3741657);

        assert_eq!(w[0], 4.0);
        assert_eq!(w[1], 5.0);
        assert_eq!(w[2], 6.0);
        assert_eq!(f64_to_fixed(w.len()), 8774964);

        let u = v + w;
        assert_eq!([u[0], u[1], u[2]], [5.0, 7.0, 9.0]);
        let u = w + v;
        assert_eq!([u[0], u[1], u[2]], [5.0, 7.0, 9.0]);
        let mut u = v;
        assert_eq!([u[0], u[1], u[2]], [1.0, 2.0, 3.0]);
        u += w;
        assert_eq!([u[0], u[1], u[2]], [5.0, 7.0, 9.0]);
        u += v;
        assert_eq!([u[0], u[1], u[2]], [6.0, 9.0, 12.0]);

        let u = -v;
        assert_eq!([u[0], u[1], u[2]], [-1.0, -2.0, -3.0]);
        let u = v - w;
        assert_eq!([u[0], u[1], u[2]], [-3.0, -3.0, -3.0]);
        let u = w - v;
        assert_eq!([u[0], u[1], u[2]], [3.0, 3.0, 3.0]);
        let mut u = -v;
        assert_eq!([u[0], u[1], u[2]], [-1.0, -2.0, -3.0]);
        u += v;
        assert_eq!([u[0], u[1], u[2]], [0.0, 0.0, 0.0]);
        u += v + w;
        assert_eq!([u[0], u[1], u[2]], [5.0, 7.0, 9.0]);
        u -= v;
        assert_eq!([u[0], u[1], u[2]], [4.0, 5.0, 6.0]);

        let u = v * w;
        assert_eq!([u[0], u[1], u[2]], [4.0, 10.0, 18.0]);
        let u = w * v;
        assert_eq!([u[0], u[1], u[2]], [4.0, 10.0, 18.0]);
        let u = v / w;
        assert_eq!(
            vec3_to_fixed([u[0], u[1], u[2]]),
            vec3_to_fixed([1.0 / 4.0, 2.0 / 5.0, 3.0 / 6.0])
        );
        let u = w / v;
        assert_eq!(
            vec3_to_fixed([u[0], u[1], u[2]]),
            vec3_to_fixed([4.0 / 1.0, 5.0 / 2.0, 6.0 / 3.0])
        );
        let mut u = v;
        u *= w;
        assert_eq!([u[0], u[1], u[2]], [4.0, 10.0, 18.0]);
        u /= w;
        assert_eq!([u[0], u[1], u[2]], [1.0, 2.0, 3.0]);
        u /= w;
        assert_eq!(
            vec3_to_fixed([u[0], u[1], u[2]]),
            vec3_to_fixed([1.0 / 4.0, 2.0 / 5.0, 3.0 / 6.0])
        );

        let u = 5.0 * v;
        assert_eq!([u[0], u[1], u[2]], [5.0, 10.0, 15.0]);
        let u = v * 5.0;
        assert_eq!([u[0], u[1], u[2]], [5.0, 10.0, 15.0]);
        let mut u = 5.0 * v;
        assert_eq!([u[0], u[1], u[2]], [5.0, 10.0, 15.0]);
        u *= 2.0;
        assert_eq!([u[0], u[1], u[2]], [10.0, 20.0, 30.0]);
        u /= 2.0;
        assert_eq!([u[0], u[1], u[2]], [5.0, 10.0, 15.0]);
        u /= 3.0;
        assert_eq!(
            vec3_to_fixed([u[0], u[1], u[2]]),
            vec3_to_fixed([5.0 / 3.0, 10.0 / 3.0, 15.0 / 3.0])
        );

        let u = Vec3::unit(&u);
        assert_eq!(vec3_to_fixed([u[0], u[1], u[2]]), [267261, 534522, 801784]);

        let u = Vec3::dot(&v, &w);
        assert_eq!(u, 32.0);
        let u = Vec3::dot(&w, &v);
        assert_eq!(u, 32.0);

        let u = Vec3::cross(&v, &w);
        assert_eq!([u[0], u[1], u[2]], [-3.0, 6.0, -3.0]);
        let u = Vec3::cross(&w, &v);
        assert_eq!([u[0], u[1], u[2]], [3.0, -6.0, 3.0]);
    }
}
