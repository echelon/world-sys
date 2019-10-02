//! Core Builtins
//!
//! These are things that are built into WORLD or provided by
//! the 'pyworld' bindings.
//!

use super::GetSamplesForHarvest;
use super::Harvest;
use super::HarvestOption;
use super::InitializeHarvestOption;

use std::os::raw::c_int;

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
 * Harvest
 * - wav: input audio signal
 * - fs: input sample rate in Hz
 * - frame_period: period between consecutive frames in milliseconds.
 */
pub fn harvest(wav: Vec<f64>,
               fs: i32,
               f0_floor: Option<f64>,
               f0_ceil: Option<f64>,
               frame_period: Option<f64>) -> HarvestResult {

  // Pyworld Defaults
  let f0_floor = f0_floor.unwrap_or(71.0f64);
  let f0_ceil = f0_ceil.unwrap_or(800.0f64);
  let frame_period = frame_period.unwrap_or(5.0f64);

  let mut option = HarvestOption::default();

  println!("HarvestOption default: {:?}", option);

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
  pub fn test_world_decompose() {
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
