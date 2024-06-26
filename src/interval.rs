#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    const EMPTY: Self = Self {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };
    const UNIVERSE: Self = Self {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
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
