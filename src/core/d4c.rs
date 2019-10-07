
use std::os::raw::c_int;
use std::ptr;

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

/*
def d4c(*args, **kwargs): # real signature unknown
    """
    D4C aperiodicity estimation algorithm.

        Parameters
        ----------
        x : ndarray
            Input waveform signal.
        f0 : ndarray
            Input F0 contour.
        temporal_positions : ndarray
            Temporal positions of each frame.
        fs : int
            Sample rate of input signal in Hz.
        q1 : float
            Spectral recovery parameter.
            Default: -0.15 (this value was tuned and normally does not need adjustment)
        threshold : float
            Threshold for aperiodicity-based voiced/unvoiced decision, in range 0 to 1.
            If a value of 0 is used, voiced frames will be kept voiced. If a value > 0 is
            used some voiced frames can be considered unvoiced by setting their aperiodicity
            to 1 (thus synthesizing them with white noise). Using `threshold=0` will result
            in the behavior of older versions of D4C. The current default of 0.85 is meant
            to be used in combination with the Harvest F0 estimator, which was designed to have
            a high voiced/unvoiced threshold (i.e. most frames will be considered voiced).
            Default: 0.85
        fft_size : int, None
            FFT size to be used. When `None` (default) is used, the FFT size is computed
            automatically as a function of the given input sample rate and the default F0 floor.
            When `fft_size` is specified, it should match the FFT size used to compute
            the spectral envelope (i.e. `fft_size=2*(sp.shape[1] - 1)`) in order to get the
            desired results when resynthesizing.
            Default: None

        Returns
        -------
        aperiodicity : ndarray
            Aperiodicity (envelope, linear magnitude relative to spectral envelope).
    """
    pass

pub struct D4COption {
    pub threshold: f64,
}

    pub fn D4C(
        x: *const f64,
        x_length: ::std::os::raw::c_int,
        fs: ::std::os::raw::c_int,
        temporal_positions: *const f64,
        f0: *const f64,
        f0_length: ::std::os::raw::c_int,
        fft_size: ::std::os::raw::c_int,
        option: *const D4COption,
        aperiodicity: *mut *mut f64,
    );
*/

pub struct D4CResult {
  pub aperiodicity: Vec<f64>,
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

  // FIXME -- Not sure this is correct allocation!
  // But I'm not sure these are the correct lengths...
  // Shape is: (f0_length, fft_size0//2 + 1)
  //let size = f0.len() * (fft_size / 2 + 1) as usize;
  //let mut aperiodicity: Vec<f64> = vec![0.0f64; size];
  // https://github.com/neithanmo/csound-rs/blob/46b50fa94ebb869d051b7d7f74555c76ecd4cbe9/src/callbacks.rs
  let mut ptr = ::std::ptr::null_mut();
  let aperiodicity: *mut *mut f64 = &mut ptr as *mut *mut _;

  unsafe {
    /*
        D4C(&x[0], x_length, fs, &temporal_positions[0],
        &f0[0], f0_length, fft_size0, &option,
        cpp_aperiodicity)
    */
    D4C(
      wav.as_ptr(),
      wav.len() as c_int,
      fs as c_int,
      temporal_postions.as_ptr(),
      f0.as_ptr(),
      f0.len() as c_int,
      fft_size as c_int,
      &option,
      aperiodicity,
    );
  }

  D4CResult {
    //aperiodicity,
    aperiodicity: vec![],
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

    println!("Result aperiod len: {:?}", result.aperiodicity.len());
    println!("Result aperiod first item: {:?}", result.aperiodicity[0]);
  }
}
