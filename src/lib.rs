#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(safe_extern_statics)]

//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
include!("bindgen.rs");

//pub mod extras;

/*use GetSamplesForHarvest;
use Harvest;
use HarvestOption;
use InitializeHarvestOption;*/

use std::os::raw::c_int;

/*

def world_decompose(wav, fs, frame_period = 5.0):

    # Decompose speech signal into f0, spectral envelope and aperiodicity using WORLD
    wav = wav.astype(np.float64)
    f0, timeaxis = pyworld.harvest(wav, fs, frame_period = frame_period, f0_floor = 71.0, f0_ceil = 800.0)
    sp = pyworld.cheaptrick(wav, f0, timeaxis, fs)
    ap = pyworld.d4c(wav, f0, timeaxis, fs)

    return f0, timeaxis, sp, ap


    // pyworld.harvest --
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
*/

/**
 * Decompose
 * fs: sample rate in Hz
 */

pub struct DecomposeResult {
  pub estimated_f0_contour: Vec<f64>,
  pub temporal_positions: Vec<f64>,
}


/**
 * WORLD decompose.
 * - wav: input audio signal
 * - fs: input sample rate in Hz
 * - frame_period: period between consecutive frames in milliseconds.
 */
pub fn world_decompose(wav: Vec<f64>, fs: i32, frame_period: f64) -> DecomposeResult {
  // https://s3.amazonaws.com/temp.michaelfbryan.com/arrays/index.html
  /*
        x: *const f64,
        x_length: ::std::os::raw::c_int,
        fs: ::std::os::raw::c_int,
        option: *const HarvestOption,
        temporal_positions: *mut f64,
        f0: *mut f64,
  */

  let mut option = HarvestOption::default();

  println!("HarvestOption default: {:?}", option);

  option.frame_period = frame_period;
  option.f0_floor = 71.0f64; // NB: pyworld default
  option.f0_ceil = 800.0f64; // NB: pyworld default

  let mut estimated_f0_contour : Vec<f64> = Vec::new();
  let mut temporal_positions : Vec<f64> = Vec::new();

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

  DecomposeResult {
    estimated_f0_contour,
    temporal_positions,
  }
}


#[cfg(test)]
mod tests {
  use super::*;
  use std::mem;

  // Adapted from /examples/codec_test/f0analysis.cpp
  #[test]
  fn test_harvest() {
    let mut option = HarvestOption::default();

    unsafe {
      InitializeHarvestOption(&mut option);
    }

    option.frame_period = 5.0;
    option.f0_floor = world_kFloorF0;
    option.f0_ceil = world_kCeilF0;

    //HarvestOption option = { 0 };
    /*InitializeHarvestOption(&option);
    option.frame_period = 5.0;
    option.f0_ceil = world::kCeilF0;*/
    // TODO
    //
    let fs = 0;
    let x_length = 100;

    unsafe {
      // F0 analysis
      let number_of_frames = 
        GetSamplesForHarvest(fs, x_length, option.frame_period);
    }
  }

  #[test]
  fn test_fft() {
    let mut fft = fft_plan::default();

    fft.n = 12;

    unsafe {
      //let mut fft = fft_plan_dft_c2r_1d();
      //fft_destroy_plan(fft); // FIXME: Broken
    }
  }

  #[test]
  fn test_cheap_trick() {
    let mut option = CheapTrickOption::default();

    unsafe {
      InitializeCheapTrickOption(1, &mut option);
    }

    option.q1 = 0.0f64;
    option.f0_floor= 0.0f64;
    option.fft_size= 1;

    unsafe {
      //let mut fft = fft_plan_dft_c2r_1d();
      //fft_destroy_plan(fft); // FIXME: Broken
    }
  }
}

