use crate::almost::AlmostPartialEq;
use crate::util::random;
use std::fmt;
use std::ops;

/// 3-D vector.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    /// Array of vector components.
    components: [f64; 3],
}

pub type Point3 = Vec3;

/// Basic component functions.
impl Vec3 {
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

    /// Determines whether the given vector is approximately the zero vector.
    pub fn almost_zero(&self) -> bool {
        self.components.iter().all(|&ui| ui.almost_zero())
    }

    /// Determines whether two vectors are approximately equal.
    pub fn almost_eq(&self, v: &Self) -> bool {
        (self - v).almost_zero()
    }
}

/// Geometry operations.
impl Vec3 {
    /// Dot product of two vectors.
    pub fn dot(u: &Self, v: &Self) -> f64 {
        u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
    }

    /// Square of the length of the vector.
    pub fn len_sqr(&self) -> f64 {
        Self::dot(self, self)
    }

    /// Length of the vector.
    pub fn len(&self) -> f64 {
        f64::sqrt(self.len_sqr())
    }

    /// Cross product of two vectors.
    pub fn cross(u: &Self, v: &Self) -> Self {
        Self::new(
            u.y() * v.z() - u.z() * v.y(),
            u.z() * v.x() - u.x() * v.z(),
            u.x() * v.y() - u.y() * v.x(),
        )
    }

    /// Creates a unit vector from the given vector.
    pub fn unit(&self) -> Self {
        self / self.len()
    }

    /// Reflects the vector in the given normal.
    pub fn reflect(v: &Self, normal: &Self) -> Self {
        v - 2.0 * Self::dot(v, normal) * normal
    }

    /// Refracts the vector across the given normal with in and target refractive index.
    pub fn refract(uv: &Self, normal: &Self, eta_i_over_eta_t: f64) -> Self {
        let cos_theta = f64::min(Self::dot(&-uv, normal), 1.0);

        // Snell's law
        let ray_out_perp = eta_i_over_eta_t * (uv + cos_theta * normal);
        let ray_out_para = -f64::sqrt(f64::abs(1.0 - ray_out_perp.len_sqr())) * normal;

        ray_out_perp + ray_out_para
    }
}

/// Random generation.
impl Vec3 {
    /// Generate a random unit vector.
    pub fn random_unit() -> Self {
        Self::random_in_unit_sphere().unit()
    }

    /// Generate a random unit vector on the same hemisphere as a surface normal.
    pub fn random_on_hemisphere(normal: &Self) -> Self {
        let u = Self::random_unit();
        if Vec3::dot(&u, normal) > 0.0 {
            u
        } else {
            -u
        }
    }

    /// Generates a random vector on the unit disk.
    pub fn random_on_unit_disk() -> Self {
        loop {
            let x = random::gen_range(-1.0, 1.0);
            let y = random::gen_range(-1.0, 1.0);
            let p = Self::new(x, y, 0.0);
            if p.len_sqr() < 1.0 {
                return p;
            }
        }
    }

    /// Generate a random vector where each component has value between 0 and 1.
    pub fn random() -> Self {
        Self::new(random::gen_unit(), random::gen_unit(), random::gen_unit())
    }

    /// Generate a random vector scaled to within the given range.
    fn random_in_range(min: f64, max: f64) -> Self {
        Self::new(
            random::gen_range(min, max),
            random::gen_range(min, max),
            random::gen_range(min, max),
        )
    }

    /// Generate a random vector in the unit sphere.
    fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_in_range(-1.0, 1.0);
            if p.len_sqr() < 1.0 {
                return p;
            }
        }
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

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn vec3_components() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);

        let v = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v[0], 4.0);
        assert_eq!(v[1], 5.0);
        assert_eq!(v[2], 6.0);
    }

    #[test]
    fn vec3_almost_zero() {
        let u = Vec3::new(0.0, 0.0, 0.0);
        assert!(u.almost_zero());

        let u = Vec3::new(0.0, 0.001, 0.0);
        assert!(!u.almost_zero());

        let u = Vec3::new(0.0, 1e-10, 0.0);
        assert!(u.almost_zero());
    }

    #[test]
    fn vec3_almost_equal() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let w = Vec3::new(4.0, 5.0, 6.0);
        let x = Vec3::new(-1.0, -2.0, -3.0);

        assert!(v.almost_eq(&v));
        assert!(!v.almost_eq(&w));
        assert!(!v.almost_eq(&x));

        assert!(w.almost_eq(&w));
        assert!(!w.almost_eq(&v));
        assert!(!w.almost_eq(&x));

        assert!(x.almost_eq(&x));
        assert!(!x.almost_eq(&v));
        assert!(!x.almost_eq(&w));

        let w = Vec3::new(1.0 + 1e-7, 2.0, 3.0);
        let x = Vec3::new(1.0 + 1e-11, 2.0, 3.0);

        assert!(v.almost_eq(&v));
        assert!(!v.almost_eq(&w));
        assert!(v.almost_eq(&x));
    }

    #[test]
    fn vec3_arithmetic() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let w = Vec3::new(4.0, 5.0, 6.0);

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
        assert!(u.almost_eq(&Vec3::new(1.0 / 4.0, 2.0 / 5.0, 3.0 / 6.0)));

        let u = w / v;
        assert!(u.almost_eq(&Vec3::new(4.0 / 1.0, 5.0 / 2.0, 6.0 / 3.0)));
        let mut u = v;
        u *= w;
        assert_eq!([u[0], u[1], u[2]], [4.0, 10.0, 18.0]);
        u /= w;
        assert_eq!([u[0], u[1], u[2]], [1.0, 2.0, 3.0]);
        u /= w;
        assert!(u.almost_eq(&Vec3::new(1.0 / 4.0, 2.0 / 5.0, 3.0 / 6.0)));

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
        assert!(u.almost_eq(&Vec3::new(5.0 / 3.0, 10.0 / 3.0, 15.0 / 3.0)));
    }

    #[test]
    fn vec3_dot() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let w = Vec3::new(4.0, 5.0, 6.0);

        let u = Vec3::dot(&v, &w);
        assert_eq!(u, 32.0);
        let u = Vec3::dot(&w, &v);
        assert_eq!(u, 32.0);
    }

    #[test]
    fn vec3_len() {
        let v = Vec3::new(1.0, 2.0, 3.0);

        assert_eq!(v.len_sqr(), 14.0);
        assert_eq!(v.len_sqr(), Vec3::dot(&v, &v));
        assert_eq!(v.len(), f64::sqrt(14.0));
    }

    #[test]
    fn vec3_cross() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let w = Vec3::new(4.0, 5.0, 6.0);

        let u = Vec3::cross(&v, &w);
        assert_eq!([u[0], u[1], u[2]], [-3.0, 6.0, -3.0]);
        let u = Vec3::cross(&w, &v);
        assert_eq!([u[0], u[1], u[2]], [3.0, -6.0, 3.0]);
    }

    #[test]
    fn vec3_unit() {
        let u = Vec3::new(5.0 / 3.0, 10.0 / 3.0, 15.0 / 3.0);
        let u = u.unit();
        assert!(u.almost_eq(&Vec3::new(0.26726124, 0.53452248, 0.80178372)));
    }

    #[test]
    fn vec3_reflect() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let normal = Vec3::new(4.0, 5.0, 6.0);

        assert!(Vec3::reflect(&v, &normal).almost_eq(&Vec3::new(-255.0, -318.0, -381.0)));
    }

    #[test]
    fn vec3_refract() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let normal = Vec3::new(4.0, 5.0, 6.0);

        assert!(Vec3::refract(&v, &normal, 1.0 / 1.5).almost_eq(&Vec3::new(
            -823.73154128,
            -1029.16442660,
            -1234.59731192
        )));
    }
}
