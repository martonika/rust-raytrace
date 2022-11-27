use crate::vec3;

pub use vec3::Vec3 as Color;

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min };
    if x > max { return max };
    x
}

impl Color {
    pub fn write_color(&self, samples_per_pixel: u32) {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        // Divide the color by the number of samples and gamma-correct for gamma=2.0.
        let scale = 1.0 / (samples_per_pixel as f64);
        r = (scale * r).sqrt();
        g = (scale * g).sqrt();
        b = (scale * b).sqrt();

        println!("{} {} {}",
            ((256.0*clamp(r, 0.0, 0.999)) as i16),
            ((256.0*clamp(g, 0.0, 0.999)) as i16),
            ((256.0*clamp(b, 0.0, 0.999)) as i16),
        )
    }
    pub fn new_black() -> Self { Self::new(0.0, 0.0, 0.0) }
}