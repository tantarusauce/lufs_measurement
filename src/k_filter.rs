/*
------------------------------

LUFS_NORMALIZER

k_filter.rs

------------------------------
*/
#[derive(Clone, Copy)]
pub struct Biquad {
    a0: f64,
    a1: f64,
    a2: f64,
    b1: f64,
    b2: f64,
    z1: f64,
    z2: f64,
}

impl Biquad {
    pub fn new(a0: f64, a1: f64, a2: f64, b1: f64, b2: f64) -> Self {
        Self {
            a0, a1, a2, b1, b2,
            z1: 0.0,
            z2: 0.0,
        }
    }

    pub fn process(&mut self, x: f64) -> f64 {
        let y = self.a0 * x + self.z1;
        self.z1 = self.a1 * x - self.b1 * y + self.z2;
        self.z2 = self.a2 * x - self.b2 * y;
        y
    }
}

pub struct KWeighting {
    hp: Biquad,
    shelf: Biquad,
}

impl KWeighting {
    pub fn new(_sample_rate: f64) -> Self {
        let hp = Biquad::new(1.0, -2.0, 1.0, -1.99004745483398, 0.99007225036621);
        let shelf = Biquad::new(
            1.53512485958697,
            -2.69169618940638,
            1.19839281085285,
            -1.69065929318241,
            0.73248077421585,
        );

        Self { hp, shelf }
    }

    pub fn process(&mut self, x: f64) -> f64 {
        let x = self.hp.process(x);
        self.shelf.process(x)
    }
}