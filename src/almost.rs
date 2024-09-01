// use std::cmp::Ordering;

/// Tolerance for approximate comparisons betweens `f64` values.
const F64_TOLERANCE: f64 = 1e-8;

/// Tolerance for approximate comparisons between `f32` values.
const F32_TOLERANCE: f32 = 1e-6;

/// Naive approximate partial equality.
pub trait AlmostPartialEq<Rhs = Self>
where
    Rhs: Sized,
{
    /// Value is almost equal to another value.
    fn almost_eq(&self, other: Rhs) -> bool;

    /// Value is almost equal to zero.
    fn almost_zero(&self) -> bool;

    /// Value is almost not equal to another value.
    fn almost_ne(&self, other: Rhs) -> bool {
        !self.almost_eq(other)
    }
}

// pub trait AlmostPartialOrd<Rhs = Self>: AlmostPartialEq<Rhs>
// where
//     Rhs: Sized,
// {
//     /// Value is almost greater than another value.
//     fn almost_partial_cmp(&self, other: Rhs) -> Ordering;

//     /// Value is almost greater than another value.
//     fn almost_gt(&self, other: Rhs) -> bool {
//         self.almost_partial_cmp(other) == Ordering::Greater
//     }

//     /// Value is almost greater than or equal to another value.
//     fn almost_gte(&self, other: Rhs) -> bool {
//         let cmp = self.almost_partial_cmp(other);
//         cmp == Ordering::Greater || cmp == Ordering::Equal
//     }

//     /// Value is almost less than to another value.
//     fn almost_lt(&self, other: Rhs) -> bool {
//         self.almost_partial_cmp(other) == Ordering::Less
//     }

//     /// Value is almost less than or equal another value.
//     fn almost_lte(&self, other: Rhs) -> bool {
//         let cmp = self.almost_partial_cmp(other);
//         cmp == Ordering::Less || cmp == Ordering::Equal
//     }
// }

impl AlmostPartialEq for f64 {
    fn almost_eq(&self, other: Self) -> bool {
        (self - other) < F64_TOLERANCE
    }

    fn almost_zero(&self) -> bool {
        f64::abs(*self) < F64_TOLERANCE
    }
}

impl AlmostPartialEq for f32 {
    fn almost_eq(&self, other: Self) -> bool {
        (self - other) < F32_TOLERANCE
    }

    fn almost_zero(&self) -> bool {
        f32::abs(*self) < F32_TOLERANCE
    }
}

// impl AlmostPartialOrd for f64 {
//     fn almost_partial_cmp(&self, other: Self) -> Ordering {
//         if self.almost_eq(other) {
//             Ordering::Equal
//         } else if *self > other {
//             Ordering::Greater
//         } else {
//             Ordering::Less
//         }
//     }
// }

// impl AlmostPartialOrd for f32 {
//     fn almost_partial_cmp(&self, other: Self) -> Ordering {
//         if self.almost_eq(other) {
//             Ordering::Equal
//         } else if *self > other {
//             Ordering::Greater
//         } else {
//             Ordering::Less
//         }
//     }
// }
