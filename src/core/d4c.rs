use std::os::raw::c_int;
use std::mem;
use std::ptr;
use std::alloc::{alloc, dealloc, Layout};

use super::cheaptrick::get_cheaptrick_fft_size;

// Constants
use super::super::world_kCeilF0;
use super::super::world_kFloorF0;
use super::super::world_kFloorF0D4C; // TODO: Used?
use super::super::world_kThreshold;

// D4C
use super::super::D4C;
use super::super::D4COption;
use super::super::InitializeD4COption;
pub struct D4CResult {
  pub aperiodicity: Vec<Vec<f64>>,
}

/**
 * D4C: aperiodicity estimation algorithm.
 */
pub fn d4c(wav: Vec<f64>,
  f0: Vec<f64>,
  temporal_postions: Vec<f64>,
  fs: i32,
  q1: Option<f64>,
  threshold: Option<f64>,
  fft_size: Option<i32>) -> D4CResult {

  // Pyworld Defaults
  let q1 = q1.unwrap_or(-0.15f64);
  let threshold = threshold.unwrap_or(world_kThreshold); // default: 0.85

  let fft_size = match fft_size {
    Some(fft_size) => fft_size,
    None => {
      let default_f0_floor = world_kFloorF0; // default: 71.0
      let result = get_cheaptrick_fft_size(fs, Some(default_f0_floor));
      result.fft_size
    },
  };

  let mut option = D4COption::default();
  unsafe {
    InitializeD4COption(&mut option);
  }

  option.threshold = threshold;

  // Shape is: (f0_length, fft_size0//2 + 1)
  let i = f0.len();
  let j = (fft_size / 2) + 1;
  // https://github.com/neithanmo/csound-rs/blob/46b50fa94ebb869d051b7d7f74555c76ecd4cbe9/src/callbacks.rs

  let mut results: Vec<Vec<f64>> = Vec::new();
  let mut outer : Vec<*mut f64> = Vec::new();

  for i in 0 .. i {
    let mut inner : Vec<f64> = Vec::new();
    for i in 0 .. j {
      inner.push(0.0f64);
    }
    outer.push(inner.as_mut_ptr());
    results.push(inner);
  }

  unsafe {
    D4C(
      wav.as_ptr(),
      wav.len() as c_int,
      fs as c_int,
      temporal_postions.as_ptr(),
      f0.as_ptr(),
      f0.len() as c_int,
      fft_size as c_int,
      &option,
      outer.as_mut_ptr(),
    );
  }

  D4CResult {
    aperiodicity: results,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::mem;

  #[test]
  pub fn test_d4c() {
    let mut audio = Vec::new();

    for i in 0..500 {
      let v = (i % 100) as f64;
      audio.push(v);
    }

    let f0 = audio.clone();
    let temporal = audio.clone();

    let result = d4c(audio, f0, temporal, 16000, None, None, Some(8));

    assert!(result.aperiodicity.len() > 0);

    // NB: Just spot checking the array for now.
    // Should improve this to do an actual calculation.
    assert_ne!(0.0f64, result.aperiodicity[0][0]);
    assert_ne!(0.0f64, result.aperiodicity[1][0]);
    assert_ne!(0.0f64, result.aperiodicity[1][1]);
  }
}
