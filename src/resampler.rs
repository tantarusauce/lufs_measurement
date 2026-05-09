/*
------------------------------

LUFS_NORMALIZER

resampler.rs

------------------------------
*/

pub struct PolyphaseResampler {
    src_rate: f64,
    dst_rate: f64,
}

impl PolyphaseResampler {
    pub fn new(src: u32, dst: u32) -> Self {
        let src = src as f64;
        let dst = dst as f64;

        Self {
            src_rate: src,
            dst_rate: dst,
        }
    }

    pub fn resample(&self, input: &[f64]) -> Vec<f64> {
        let ratio = self.dst_rate / self.src_rate;
        let out_len = (input.len() as f64 * ratio) as usize;

        let mut out = vec![0.0; out_len];

        for i in 0..out_len {
            let pos = i as f64 / ratio;

            let i0 = pos.floor() as isize;
            let i1 = i0 + 1;

            let frac = pos - i0 as f64;

            let v0 = if i0 >= 0 && (i0 as usize) < input.len() {
                input[i0 as usize]
            } else {
                0.0
            };

            let v1 = if i1 >= 0 && (i1 as usize) < input.len() {
                input[i1 as usize]
            } else {
                0.0
            };
            
            //線形補完
            out[i] = v0 * (1.0 - frac) + v1 * frac;
        }
        out
    }
}