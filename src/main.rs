/*
------------------------------

LUFS_NORMALIZER

main.rs

------------------------------
*/
mod audio;
mod lufs;
mod k_filter;
mod resampler;

use audio::load_wav;
use lufs::calculate_lufs;

fn main() {
    let path = "audiofile/song2.wav";

    let result = load_wav(path)
        .and_then(|audio| calculate_lufs(&audio).map_err(|e| e.into()));

    match result {
        Ok(lufs) => println!("LUFS: {:.5}", lufs),
        Err(e) => eprintln!("error: {e}"),
    }
}