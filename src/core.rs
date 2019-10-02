//! Core Builtins
//!
//! These are things that are built into WORLD or provided by
//! the 'pyworld' bindings.
//!

use std::os::raw::c_int;

// Constants
use super::world_kCeilF0;
use super::world_kFloorF0;

// CheapTrick
use super::CheapTrick;
use super::CheapTrickOption;
use super::GetF0FloorForCheapTrick;
use super::GetFFTSizeForCheapTrick;
use super::InitializeCheapTrickOption;

// Harvest
use super::GetSamplesForHarvest;
use super::Harvest;
use super::HarvestOption;
use super::InitializeHarvestOption;

/*
// pyworld.cheaptrick
def cheaptrick(*args, **kwargs): # real signature unknown
    """
    CheapTrick harmonic spectral envelope estimation algorithm.

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
        f0_floor : float, None
            Lower F0 limit in Hz. Not used in case `fft_size` is specified.
            Default: 71.0
        fft_size : int, None
            FFT size to be used. When `None` (default) is used, the FFT size is computed
            automatically as a function of the given input sample rate and F0 floor.
            When `fft_size` is specified, the given `f0_floor` parameter is ignored.
            Default: None

        Returns
        -------
        spectrogram : ndarray
            Spectral envelope (squared magnitude).
    """
    pass


extern "C" {
    pub fn CheapTrick(
        x: *const f64,
        x_length: ::std::os::raw::c_int,
        fs: ::std::os::raw::c_int,
        temporal_positions: *const f64,
        f0: *const f64,
        f0_length: ::std::os::raw::c_int,
        option: *const CheapTrickOption,
        spectrogram: *mut *mut f64,
    );
}

*/

pub struct CheapTrickResult {
  spectrogram: Vec<f64>
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
  option.q1 = q1;
  option.f0_floor = f0_floor;
  //option.fft_size = fft_size; // TODO: How to make this optional?

  // FIXME -- Not sure this is correct allocation!
  // But I'm not sure these are the correct lengths...
  let mut spectrogram: Vec<f64> = vec![0.0f64; wav.len()];

  /*unsafe {
    CheapTrick(
      wav.as_ptr(),
      wav.len() as c_int,
      fs as c_int,
      temporal_postions.as_ptr(),
      f0.as_ptr(),
      f0.len() as c_int,
      &mut option,
      spectrogram.as_mut_ptr() as *mut _,
    );
  }*/

  CheapTrickResult {
    spectrogram,
  }
}

/*
// pyworld.harvest
def harvest(*args, **kwargs): # real signature unknown
    """
    Harvest F0 extraction algorithm.

        Parameters
        ----------
        x : ndarray
            Input waveform signal.
        fs : int
            Sample rate of input signal in Hz.
        f0_floor : float
            Lower F0 limit in Hz.
            Default: 71.0
        f0_ceil : float
            Upper F0 limit in Hz.
            Default: 800.0
        frame_period : float
            Period between consecutive frames in milliseconds.
            Default: 5.0

        Returns
        -------
        f0 : ndarray
            Estimated F0 contour.
        temporal_positions : ndarray
            Temporal position of each frame.
    """
    pass
*/

pub struct HarvestResult {
  pub estimated_f0_contour: Vec<f64>,
  pub temporal_positions: Vec<f64>,
}

/**
 * Harvest: F0 extraction algorithm
 * - wav: input audio signal
 * - fs: sample rate of input signal in Hz
 * - f0_floor: lower F0 limit in Hz
 * - f0_ceil: upper F0 limit in Hz
 * - frame_period: period between consecutive frames in milliseconds.
 */
pub fn harvest(wav: Vec<f64>,
               fs: i32,
               f0_floor: Option<f64>,
               f0_ceil: Option<f64>,
               frame_period: Option<f64>) -> HarvestResult {

  // Pyworld Defaults
  let f0_floor = f0_floor.unwrap_or(world_kFloorF0); // default: 71.0
  let f0_ceil = f0_ceil.unwrap_or(world_kCeilF0); // default: 800.0
  let frame_period = frame_period.unwrap_or(5.0f64);

  let mut option = HarvestOption::default();
  option.frame_period = frame_period;
  option.f0_floor = f0_floor;
  option.f0_ceil = f0_ceil;

  // Nothing appears to get allocated if I use 'with_capacity':
  // let mut estimated_f0_contour : Vec<f64> = Vec::with_capacity(wav.len());
  // let mut temporal_positions : Vec<f64> = Vec::with_capacity(wav.len());

  // FIXME -- Not sure this is correct allocation!
  // But I'm not sure these are the correct lengths...
  let mut estimated_f0_contour : Vec<f64> = vec![0.0f64; wav.len()];
  let mut temporal_positions : Vec<f64> = vec![0.0f64; wav.len()];

  unsafe {
    Harvest(
      wav.as_ptr(),
      wav.len() as c_int,
      fs as c_int,
      &mut option,
      temporal_positions.as_mut_ptr() as *mut _,
      estimated_f0_contour.as_mut_ptr() as *mut _,
    );
  }

  HarvestResult {
    estimated_f0_contour,
    temporal_positions,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::mem;

  #[test]
  pub fn test_cheaptrick() {
    let mut audio = Vec::new();

    for i in 0..10000 {
      let v = (i % 100) as f64;
      audio.push(v);
    }

    let f0 = audio.clone();
    let temporal = audio.clone();

    let result = cheaptrick(audio, f0, temporal, 16000, None, None, None);

    println!("Result spectrogram: {:?}", result.spectrogram);
  }

  #[test]
  pub fn test_harvest() {
    let mut audio = Vec::new();

    for i in 0..10000 {
      let v = (i % 100) as f64;
      audio.push(v);
    }

    let result = harvest(audio, 16000, None, None, Some(10.0));

    println!("Result a: {:?}", result.temporal_positions);
    println!("Result b: {:?}", result.estimated_f0_contour);

    assert_eq!(false, result.temporal_positions.is_empty());
    assert_eq!(false, result.estimated_f0_contour.is_empty());

    // NB: This may not be correct
    assert_ne!(0.0f64, result.estimated_f0_contour[0]);
  }
}
