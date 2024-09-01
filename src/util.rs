pub mod random {
    use rand::{self, Rng};

    pub fn gen_unit() -> f64 {
        rand::thread_rng().gen()
    }

    pub fn gen_range(min: f64, max: f64) -> f64 {
        rand::thread_rng().gen_range(min..=max)
    }
}
