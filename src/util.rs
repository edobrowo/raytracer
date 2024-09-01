use rand::{self, Rng};

pub fn gen_unit() -> f64 {
    rand::thread_rng().gen::<f64>()
}

pub fn gen_between(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::thread_rng().gen::<f64>()
}
