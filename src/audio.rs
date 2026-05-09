/*
------------------------------

LUFS_NORMALIZER

audio.rs

------------------------------
*/

use std::error::Error;

use crate::resampler::PolyphaseResampler;

pub struct AudioBuffer {
    pub sample_rate: u32,
    pub channels: Vec<Vec<f64>>,
}

pub fn load_wav(path: &str) -> Result<AudioBuffer, Box<dyn Error>> {
    let mut reader = hound::WavReader::open(path)?;
    let spec = reader.spec();
    let ch_count = spec.channels as usize;

    let mut channels = vec![Vec::<f64>::new(); ch_count];

    match spec.sample_format {
        hound::SampleFormat::Int => {
            let max = (1_i64 << (spec.bits_per_sample - 1)) as f64 - 1.0;

            for (i, s) in reader.samples::<i32>().enumerate() {
                let s = s? as f64 / max;
                channels[i % ch_count].push(s);
            }
        }
        hound::SampleFormat::Float => {
            for (i, s) in reader.samples::<f32>().enumerate() {
                channels[i % ch_count].push(s? as f64);
            }
        }
    }

    let resampler = PolyphaseResampler::new(
        spec.sample_rate,
        48_000,
    );

    //let fir = PolyphaseResampler::make_lowpass_fir(256, PolyphaseResampler::cutoff(spec.sample_rate as f64, 48_000 as f64));
    //let filter = PolyphaseResampler::make_polyphase(fir, 32);

    let channels: Vec<Vec<f64>> = channels
        .into_iter()
        .map(|ch| resampler.resample(&ch))
        .collect();

    Ok(AudioBuffer {
        sample_rate: 48_000,
        channels,
    })
}