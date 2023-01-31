use crate::noise_fns::NoiseFn;

/// Noise function that outputs a vein pattern.
///
/// This function takes
///
#[derive(Clone, Copy, Debug)]
pub struct Vein {
    size: usize,
    power: f64,
}

impl Vein {
    const DEFAULT_SIZE: usize = 50;
    const DEFAULT_POWER: f64 = 2.0;

    pub fn new(size: usize, power: f64) -> Self {
        Self { size, power }
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }

    pub fn set_power(&mut self, power: f64) {
        self.power = power;
    }

    pub fn size(self) -> usize {
        self.size
    }

    pub fn power(self) -> f64 {
        self.power
    }
}

impl Default for Vein {
    fn default() -> Self {
        Self {
            size: Vein::DEFAULT_SIZE,
            power: Vein::DEFAULT_POWER,
        }
    }
}

impl NoiseFn<f64, 2> for Vein {
    fn get(&self, point: [f64; 2]) -> f64 {
        let x = (point[0] / self.size as f64).fract().abs();
        let y = (point[1] / self.size as f64).fract();

        let mut amount: f64 = 0.0;

        amount = (2.0 * x - 1.0).powf(self.power);

        amount
    }
}
