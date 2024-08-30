use core::f64;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub const EMPTY: Self = Self::new(f64::INFINITY, f64::NEG_INFINITY);
    pub const UNIVERSE: Self = Self::new(f64::NEG_INFINITY, f64::INFINITY);

    pub const NONNEGATIVE: Self = Self::new(0.0, f64::INFINITY);
    pub const NONPOSITIVE: Self = Self::new(f64::NEG_INFINITY, 0.0);

    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn surrounds_or_min(&self, x: f64) -> bool {
        self.min <= x && x < self.max
    }

    pub fn surrounds_or_max(&self, x: f64) -> bool {
        self.min < x && x <= self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Interval;

    #[test]
    fn interval_general() {
        let int = Interval::new(-2.0, 5.0);

        assert!(int.contains(0.0));
        assert!(int.contains(-2.0));
        assert!(int.contains(5.0));
        assert!(!int.contains(-2.1));
        assert!(!int.contains(5.1));

        assert!(int.surrounds(0.0));
        assert!(!int.surrounds(-2.0));
        assert!(!int.surrounds(5.0));
        assert!(!int.surrounds(-2.1));
        assert!(!int.surrounds(5.1));

        assert!(int.surrounds_or_min(0.0));
        assert!(int.surrounds_or_min(-2.0));
        assert!(!int.surrounds_or_min(5.0));
        assert!(!int.surrounds_or_min(-2.1));
        assert!(!int.surrounds_or_min(5.1));

        assert!(int.surrounds_or_max(0.0));
        assert!(!int.surrounds_or_max(-2.0));
        assert!(int.surrounds_or_max(5.0));
        assert!(!int.surrounds_or_max(-2.1));
        assert!(!int.surrounds_or_max(5.1));

        assert_eq!(int.clamp(3.0), 3.0);
        assert_eq!(int.clamp(-2.1), -2.0);
        assert_eq!(int.clamp(5.1), 5.0);

        assert!(!Interval::EMPTY.contains(0.0));
        assert!(!Interval::EMPTY.contains(1000000.0));

        assert!(Interval::UNIVERSE.contains(0.0));
        assert!(Interval::UNIVERSE.contains(1000000.0));
    }

    #[test]
    fn min_greater_than_max() {
        let int = Interval::new(10.0, 9.0);

        assert!(!int.contains(0.0));
        assert!(!int.contains(10.0));
        assert!(!int.contains(9.0));
    }
}
