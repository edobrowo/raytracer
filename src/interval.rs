use core::f64;

/// Defines an interval defined along [`min`, `max`].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Interval {
    /// Minimum of the interval.
    min: f64,

    /// Maximum of the interval.
    max: f64,
}

impl Interval {
    pub const EMPTY: Self = Self::new(f64::INFINITY, f64::NEG_INFINITY);
    pub const UNIVERSE: Self = Self::new(f64::NEG_INFINITY, f64::INFINITY);

    /// Creates a new interval.
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// Retrieves the minimum of the interval.
    pub fn min(&self) -> f64 {
        self.min
    }

    /// Retrieves the maximum of the interval.
    pub fn max(&self) -> f64 {
        self.max
    }

    /// Checks whether `x` is in `[min, max]` (bounds-inclusive).
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// Checks whether `x` is in `(min, max)` (bounds-exclusive).
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    /// Checks whether `x` is in `[min, max)` (lower bound-inclusive, upper bound-exclusive).
    pub fn surrounds_or_min(&self, x: f64) -> bool {
        self.min <= x && x < self.max
    }

    /// Checks whether `x` is in `(min, max]` (lower bound-exclusive, upper bound-inclusive).
    pub fn surrounds_or_max(&self, x: f64) -> bool {
        self.min < x && x <= self.max
    }

    /// Clamps `x` within the inclusive bounds of the interval.
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
