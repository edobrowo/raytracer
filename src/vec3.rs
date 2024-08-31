use rand::{self, Rng};
use std::fmt;
use std::ops;

/// 3-D vector.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    /// Array of vector components.
    components: [f64; 3],
}

pub type Point3 = Vec3;

impl Vec3 {
    /// Minimum error for vector operations.
    const ERROR: f64 = 1e-8;

    /// Creates a new 3-D vector.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            components: [x, y, z],
        }
    }

    /// Retrieves x component.
    pub fn x(&self) -> f64 {
        self[0]
    }

    /// Retrieves y component.
    pub fn y(&self) -> f64 {
        self[1]
    }

    /// Retrieves z component.
    pub fn z(&self) -> f64 {
        self[2]
    }

    /// Dot product of two vectors.
    pub fn dot(&self, other: &Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    /// Square of the length of the vector.
    pub fn len_sqr(&self) -> f64 {
        self.dot(self)
    }

    /// Length of the vector.
    pub fn len(&self) -> f64 {
        f64::sqrt(self.len_sqr())
    }

    /// Cross product of two vectors.
    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    /// Creates a unit vector from the given vector.
    pub fn unit(&self) -> Self {
        self / self.len()
    }

    // TODO: test
    /// Determines whether the given vector is approximately the zero vector.
    pub fn is_almost_zero(&self) -> bool {
        self.components.iter().all(|&v| f64::abs(v) < Self::ERROR)
    }

    // TODO: test
    /// Reflects the vector in the given normal.
    pub fn reflect(&self, normal: &Self) -> Self {
        self - 2.0 * self.dot(normal) * normal
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &f64 {
        &self.components[i]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.components[i]
    }
}

macro_rules! negate {
    ( $exp:ty ) => {
        impl ops::Neg for $exp {
            type Output = Vec3;
            fn neg(self) -> Vec3 {
                Vec3::new(-self.x(), -self.y(), -self.z())
            }
        }
    };
}

negate!(Vec3);
negate!(&Vec3);

macro_rules! add {
    ( $lhs:ty , $rhs:ty ) => {
        impl ops::Add<$rhs> for $lhs {
            type Output = Vec3;
            fn add(self, rhs: $rhs) -> Vec3 {
                Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
            }
        }
    };
}

add!(Vec3, Vec3);
add!(&Vec3, Vec3);
add!(Vec3, &Vec3);
add!(&Vec3, &Vec3);

macro_rules! subtract {
    ( $lhs:ty , $rhs:ty ) => {
        impl ops::Sub<$rhs> for $lhs {
            type Output = Vec3;
            fn sub(self, rhs: $rhs) -> Vec3 {
                Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
            }
        }
    };
}

subtract!(Vec3, Vec3);
subtract!(&Vec3, Vec3);
subtract!(Vec3, &Vec3);
subtract!(&Vec3, &Vec3);

macro_rules! scalar_multiply_rhs {
    ( $lhs:ty , $rhs:ty ) => {
        impl ops::Mul<$rhs> for $lhs {
            type Output = Vec3;
            fn mul(self, rhs: $rhs) -> Vec3 {
                Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
            }
        }
    };
}

scalar_multiply_rhs!(Vec3, f64);
scalar_multiply_rhs!(&Vec3, f64);
scalar_multiply_rhs!(Vec3, &f64);
scalar_multiply_rhs!(&Vec3, &f64);

macro_rules! scalar_multiply_lhs {
    ( $lhs:ty , $rhs:ty ) => {
        impl ops::Mul<$rhs> for $lhs {
            type Output = Vec3;
            fn mul(self, rhs: $rhs) -> Vec3 {
                Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
            }
        }
    };
}

scalar_multiply_lhs!(f64, Vec3);
scalar_multiply_lhs!(&f64, Vec3);
scalar_multiply_lhs!(f64, &Vec3);
scalar_multiply_lhs!(&f64, &Vec3);

macro_rules! hadamard_multiply {
    ( $lhs:ty , $rhs:ty ) => {
        impl ops::Mul<$rhs> for $lhs {
            type Output = Vec3;
            fn mul(self, rhs: $rhs) -> Vec3 {
                Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
            }
        }
    };
}

hadamard_multiply!(Vec3, Vec3);
hadamard_multiply!(&Vec3, Vec3);
hadamard_multiply!(Vec3, &Vec3);
hadamard_multiply!(&Vec3, &Vec3);

macro_rules! scalar_divide {
    ( $lhs:ty , $rhs:ty ) => {
        impl ops::Div<$rhs> for $lhs {
            type Output = Vec3;
            fn div(self, rhs: $rhs) -> Vec3 {
                self * (1.0 / rhs)
            }
        }
    };
}

scalar_divide!(Vec3, f64);
scalar_divide!(&Vec3, f64);
scalar_divide!(Vec3, &f64);
scalar_divide!(&Vec3, &f64);

macro_rules! hadamard_divide {
    ( $lhs:ty , $rhs:ty ) => {
        impl ops::Div<$rhs> for $lhs {
            type Output = Vec3;
            fn div(self, rhs: $rhs) -> Vec3 {
                Vec3::new(self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z())
            }
        }
    };
}

hadamard_divide!(Vec3, Vec3);
hadamard_divide!(&Vec3, Vec3);
hadamard_divide!(Vec3, &Vec3);
hadamard_divide!(&Vec3, &Vec3);

macro_rules! add_assign {
    ( $rhs:ty ) => {
        impl ops::AddAssign<$rhs> for Vec3 {
            fn add_assign(&mut self, rhs: $rhs) {
                self.components[0] = self.x() + rhs.x();
                self.components[1] = self.y() + rhs.y();
                self.components[2] = self.z() + rhs.z()
            }
        }
    };
}

add_assign!(Vec3);
add_assign!(&Vec3);

macro_rules! subtract_assign {
    ( $rhs:ty ) => {
        impl ops::SubAssign<$rhs> for Vec3 {
            fn sub_assign(&mut self, rhs: $rhs) {
                self.components[0] = self.x() - rhs.x();
                self.components[1] = self.y() - rhs.y();
                self.components[2] = self.z() - rhs.z()
            }
        }
    };
}

subtract_assign!(Vec3);
subtract_assign!(&Vec3);

macro_rules! scalar_multiply_assign {
    ( $rhs:ty ) => {
        impl ops::MulAssign<$rhs> for Vec3 {
            fn mul_assign(&mut self, rhs: $rhs) {
                self.components[0] = self.x() * rhs;
                self.components[1] = self.y() * rhs;
                self.components[2] = self.z() * rhs
            }
        }
    };
}

scalar_multiply_assign!(f64);
scalar_multiply_assign!(&f64);

macro_rules! hadamard_multiply_assign {
    ( $rhs:ty ) => {
        impl ops::MulAssign<$rhs> for Vec3 {
            fn mul_assign(&mut self, rhs: $rhs) {
                self.components[0] = self.x() * rhs.x();
                self.components[1] = self.y() * rhs.y();
                self.components[2] = self.z() * rhs.z()
            }
        }
    };
}

hadamard_multiply_assign!(Vec3);
hadamard_multiply_assign!(&Vec3);

macro_rules! scalar_divide_assign {
    ( $rhs:ty ) => {
        impl ops::DivAssign<$rhs> for Vec3 {
            fn div_assign(&mut self, rhs: $rhs) {
                self.components[0] = self.x() / rhs;
                self.components[1] = self.y() / rhs;
                self.components[2] = self.z() / rhs
            }
        }
    };
}

scalar_divide_assign!(f64);
scalar_divide_assign!(&f64);

macro_rules! hadamard_divide_assign {
    ( $rhs:ty ) => {
        impl ops::DivAssign<$rhs> for Vec3 {
            fn div_assign(&mut self, rhs: $rhs) {
                self.components[0] = self.x() / rhs.x();
                self.components[1] = self.y() / rhs.y();
                self.components[2] = self.z() / rhs.z()
            }
        }
    };
}

hadamard_divide_assign!(Vec3);
hadamard_divide_assign!(&Vec3);

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

    pub fn random_unit() -> Self {
        Self::random_in_unit_sphere().unit()
    }

    pub fn random_on_hemisphere(normal: &Self) -> Self {
        let u = Self::random_unit();
        if u.dot(normal) > 0.0 {
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

        let u = u.unit();
        assert_eq!(vec3_to_fixed([u[0], u[1], u[2]]), [267261, 534522, 801784]);

        let u = v.dot(&w);
        assert_eq!(u, 32.0);
        let u = w.dot(&v);
        assert_eq!(u, 32.0);

        let u = v.cross(&w);
        assert_eq!([u[0], u[1], u[2]], [-3.0, 6.0, -3.0]);
        let u = w.cross(&v);
        assert_eq!([u[0], u[1], u[2]], [3.0, -6.0, 3.0]);
    }
}
