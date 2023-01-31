use crate::noise_fns::NoiseFn;

/// Noise function that outputs a vein pattern.
///
/// This function takes
///
#[derive(Clone, Copy, Debug)]
pub struct Vein {
    size: usize,
    power: f32,
}

impl Vein {
    const DEFAULT_SIZE: usize = 50;
    const DEFAULT_POWER: f32 = 2.0;

    pub fn new(size: usize, power: usize) -> Size {
        Self { size, power }
    }

    pub fn set_size(&self, size: usize) -> Self {
        self.size = size;
    }

    pub fn set_power(&self, size: usize) -> Self {
        self.power = power;
    }

    pub fn size(self) -> usize {
        self.size
    }

    pub fn power(self) -> f32 {
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
        let x = (point[0] / self.size).fract();
        let y = (point[1] / self.size).fract();

        let mut amount: f64 = 0.0;

        if (x < 0.5) {
            amount = (2.0 * x).powf(self.power);
        } else {
            amount = (1.0 - (2.0 * (x - 0.5))).powf(self.power);
        }

        amount
    }
}
