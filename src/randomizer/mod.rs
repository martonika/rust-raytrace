use rand::Rng;

pub fn rand_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn rand_f64_range(min: f64, max: f64) -> f64 {
    min + (max-min)*rand_f64()
}