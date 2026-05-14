/*
------------------------------

LUFS_NORMALIZER

lufs.rs

------------------------------
*/
use crate::audio::AudioBuffer;
use crate::k_filter::KWeighting;

const ABS_GATE: f64 = -70.0;
const RELATIVE_GATE_OFFSET: f64 = -10.0;

fn channel_weight(idx: usize, total_channels: usize) -> f64 {

    match total_channels {

        1 => 1.0, // mono

        2 => 1.0,

        6 => match idx {
            0 => 1.0, // L
            1 => 1.0, // R
            2 => 1.0, // C
            3 => 0.0, // LFE
            4 => 1.41421356237, // Ls
            5 => 1.41421356237, // Rs
            _ => 1.0,
        },

        _ => 1.0, 

        
    }
}

fn mean_energy(blocks: &[f64]) -> f64 {
    blocks.iter().sum::<f64>() / blocks.len() as f64
}

pub fn calculate_lufs(audio: &AudioBuffer) -> Result<f64, &'static str> {
    if audio.channels.is_empty() {
        return Err("no channels");
    }

    let sr = audio.sample_rate as usize;

    let block_size = (sr * 400) / 1000;
    let hop_size = block_size / 4;

    if block_size == 0 || hop_size == 0 {
        return Err("invalid sample rate");
    }

    let weighted: Vec<Vec<f64>> = audio.channels
        .iter()
        .map(|ch| {
            let mut k = KWeighting::new(audio.sample_rate as f64);
            ch.iter().copied().map(|s| k.process(s)).collect()
        })
        .collect();

    let block_energy = make_block_energy(&weighted, block_size, hop_size);

    if block_energy.is_empty() {
        return Err("no full blocks");
    }

    let abs_kept: Vec<f64> = block_energy
        .iter()
        .copied()
        .filter(|&e| to_lufs(e) > ABS_GATE)
        .collect();

    if abs_kept.is_empty() {
        return Err("all blocks below absolute gate");
    }

    let abs_mean = mean_energy(&abs_kept);
    let rel_gate = abs_mean * 10_f64.powf(RELATIVE_GATE_OFFSET / 10.0);

    let gated: Vec<f64> = block_energy
        .iter()
        .copied()
        .filter(|&e| e > rel_gate)
        .collect();

    if gated.is_empty() {
        return Err("all blocks removed by gating");
    }

    let integrated = mean_energy(&gated);
    Ok(to_lufs(integrated))
}

fn make_block_energy(
    channels: &[Vec<f64>],
    block_size: usize,
    hop_size: usize,
) -> Vec<f64> {
    let len = channels.iter().map(|c| c.len()).min().unwrap_or(0);
    let mut out = Vec::new();
    let mut start = 0;

    while start + block_size <= len {
        let mut sum = 0.0;

        for (ch_idx, ch) in channels.iter().enumerate() {
            let block = &ch[start..start + block_size];
            let energy = block.iter().map(|x| x * x).sum::<f64>() / block_size as f64;
            sum += energy * channel_weight(ch_idx, channels.len());
        }

        if sum > 0.0 {
            out.push(sum);
        }

        start += hop_size;
    }

    out
}

fn to_lufs(x: f64) -> f64 {
    -0.691 + 10.0 * x.log10()
}
