/*
------------------------------

LUFS_NORMALIZER

lib.rs

------------------------------
*/
pub mod audio;
pub mod lufs;
pub mod k_filter;
pub mod resampler;

use audio::load_wav;
use lufs::calculate_lufs;

#[unsafe(no_mangle)]
pub extern "C" fn calculate_lufs_from_file(path: *const i8) -> f64 {
    let c_str = unsafe { std::ffi::CStr::from_ptr(path) };

    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return -999.0,
    };

    let audio = match load_wav(path_str) {
        Ok(a) => a,
        Err(_) => return -999.0,
    };

    match calculate_lufs(&audio) {
        Ok(v) => v,
        Err(_) => -999.0,
    }
}