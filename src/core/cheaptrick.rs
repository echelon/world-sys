use std::os::raw::c_int;
use std::ptr;

// Constants
use super::super::world_kCeilF0;
use super::super::world_kFloorF0;
use super::super::world_kFloorF0D4C; // TODO: Used?
use super::super::world_kThreshold;

// CheapTrick
use super::super::CheapTrick;
use super::super::CheapTrickOption;
use super::super::GetF0FloorForCheapTrick;
use super::super::GetFFTSizeForCheapTrick;
use super::super::InitializeCheapTrickOption;

pub struct CheapTrickResult {
  pub spectrogram: Vec<Vec<f64>>
}

/**
 * CheapTrick: harmonic spectral envelope estimation algorithm
 * - wav: input waveform signal
 * - f0: input f0 contour
 * - temporal_positions: temporal positions of each frame
 * - fs: sample rate of input signal in Hz
 * - q1: spectral recovery parameter (already tuned and does not normally need adjustment!)
 * - f0_floor: lower F0 limit in Hz. Not used in case 'fft_size' is specified
 * - fft_size: FFT size to be used. When null (default), computed automatically
 */
pub fn cheaptrick(wav: Vec<f64>,
  f0: Vec<f64>,
  temporal_postions: Vec<f64>,
  fs: i32,
  q1: Option<f64>,
  f0_floor: Option<f64>,
  fft_size: Option<i32>) -> CheapTrickResult {

  // Defaults
  let q1 = q1.unwrap_or(-0.15f64);
  let f0_floor = f0_floor.unwrap_or(world_kFloorF0); // default: 71.0

  let mut option = CheapTrickOption::default();
  unsafe {
    InitializeCheapTrickOption(fs, &mut option);
  }

  option.q1 = q1;

  let fft_size = match fft_size {
    Some(fft_size) => {
      // NB: From pyworld --
      // the f0_floor used by CheapTrick() will be re-compute from this given fft_size
      fft_size
    },
    None => {
      // NB: From pyworld --
      // CheapTrickOption.f0_floor is only used in GetFFTSizeForCheapTrick()
      option.f0_floor = f0_floor;
      unsafe {
        GetFFTSizeForCheapTrick(fs, &option)
      }
    },
  };

  option.fft_size = fft_size;

  // FIXME -- Not sure this is correct allocation!
  // But I'm not sure these are the correct lengths...
  // pyworld shape is: (f0_length, option.fft_size//2 + 1)
  let n = f0.len();
  let m = (fft_size/2 + 1) as usize;

  let mut results: Vec<Vec<f64>> = Vec::new();
  let mut outer : Vec<*mut f64> = Vec::new();

  for i in 0 .. n {
    let mut inner : Vec<f64> = Vec::new();
    for i in 0 .. m {
      inner.push(0.0f64);
    }
    outer.push(inner.as_mut_ptr());
    results.push(inner);
  }

  unsafe {
    CheapTrick(
      wav.as_ptr(),
      wav.len() as c_int,
      fs as c_int,
      temporal_postions.as_ptr(),
      f0.as_ptr(),
      f0.len() as c_int,
      &mut option,
      outer.as_mut_ptr(),
    );
  }

  CheapTrickResult {
    spectrogram: results,
  }
}

pub struct GetCheaptrickFftSizeResult {
  pub fft_size: i32,
}

pub fn get_cheaptrick_fft_size(fs: i32,
  f0_floor: Option<f64>) -> GetCheaptrickFftSizeResult {

  let f0_floor = f0_floor.unwrap_or(world_kFloorF0); // default: 71.0

  let mut option = CheapTrickOption::default();
  option.f0_floor = f0_floor;

  let fft_size = unsafe {
    GetFFTSizeForCheapTrick(fs, &option)
  };

  GetCheaptrickFftSizeResult {
    fft_size,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::mem;

  #[test]
  pub fn test_cheaptrick() {
    let mut audio = Vec::new();

    for i in 0..500 {
      let v = (i % 100) as f64;
      audio.push(v);
    }

    let f0 = audio.clone();
    let temporal = audio.clone();

    let result = cheaptrick(audio, f0, temporal, 16000, None, None, None);

    assert!(result.spectrogram.len() > 0);

    // NB: Just spot checking the array for now.
    // Should improve this to do an actual calculation.
    assert_ne!(0.0f64, result.spectrogram[0][0]);
    assert_ne!(0.0f64, result.spectrogram[1][0]);
    assert_ne!(0.0f64, result.spectrogram[1][1]);
  }

  #[test]
  pub fn test_get_cheaptrick_fft_size() {
    let result = get_cheaptrick_fft_size(16000, Some(world_kFloorF0));
    assert_eq!(1024, result.fft_size);
    let result = get_cheaptrick_fft_size(44100, Some(world_kFloorF0));
    assert_eq!(2048, result.fft_size);

    // NB: We don't use the ceiling, but here's an example anyway:
    let result = get_cheaptrick_fft_size(16000, Some(world_kCeilF0));
    assert_eq!(64, result.fft_size);
  }
}
