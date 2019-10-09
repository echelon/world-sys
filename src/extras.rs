//! Extras
//!
//! These are things found in CycleGAN and not present in WORLD
//! or its Python bindings.
//!

use super::GetSamplesForHarvest;
use super::Harvest;
use super::HarvestOption;
use super::InitializeHarvestOption;

use std::os::raw::c_int;

use core::*;

/*
TODO: ALGORITHM TO GET VOICE TRANSFORMATION:

- [mono]
- [librosa.resample (downsample)]
- core convert()
    - wav_padding()
        - [maths]
        - [np.pad]
    - world_decompose()
        - pyworld.harvest
        - pyworld.cheaptrick
        - pyworld.d4c
    - world_encode_spectral_envelope()
        - pyworld.code_spectral_envelope
    - [matrix transpose]
    - pitch_conversion()
        - [maths]
    - [maths : coded_sp_norm]
    - << evaluate ML model >>
    - [maths : coded_sp_converted]
    - [matrix transpose]
    - [np.ascontiguousarray]
    - world_decode_spectral_envelop
        - pyworld.get_cheaptrick_fft_size
        - pyworld.decode_spectral_envelope
    - world_speech_synthesis
        - pyworld.synthesize
    - [librosa.resample (upsample)]
    - [wav things]

================================================
Our bindings,

- cheaptrick()
    - InitializeCheapTrickOptions()
    - GetFFTSizeForCheapTrick() - branch case, untested
    - CheapTrick() - TEST WORKS (VECVEC!)

- code_spectral_envelope()
    - CodeSpectralEnvelope() - TEST WORKS

- d4c()
    - get_cheaptrick_fft_size() - branch case, untested
    - InitializeD4COption
    - D4C() - TESTS WORK! (REALLY WORK W/ VEC<VEC<F64>>)

- decode_spectral_envelope()
    - TEST WORKS

- get_cheaptrick_fft_size() - TESTS WORK!
    - GetFFTSizeForCheapTrick() - TESTS WORK!

- harvest() - TESTS WORK!
    - Harvest() - TESTS WORK!

- synthesize()
    - INCOMPLETE
    - Synthesis() - UNTESTED

================================================

Vec<Vec<T>> is not tightly packed:
https://stackoverflow.com/a/37316739

FFI multidimensional array:
https://users.rust-lang.org/t/solved-ffi-convert-pointer-to-a-multidimensional-array/10740

Hmm, in/out params and null ptrs,
https://www.reddit.com/r/rust/comments/da0szc/how_to_represent_a_c_double_pointer_in_rust_ffi/
*/

pub struct DecomposeResult {
}


/*
def world_decompose(wav, fs, frame_period = 5.0):

    # Decompose speech signal into f0, spectral envelope and aperiodicity using WORLD
    wav = wav.astype(np.float64)
    f0, timeaxis = pyworld.harvest(wav, fs, frame_period = frame_period, f0_floor = 71.0, f0_ceil = 800.0)
    sp = pyworld.cheaptrick(wav, f0, timeaxis, fs)
    ap = pyworld.d4c(wav, f0, timeaxis, fs)

    return f0, timeaxis, sp, ap
*/

/**
 * WORLD decompose.
 * - wav: input audio signal
 * - fs: input sample rate in Hz
 * - frame_period: period between consecutive frames in milliseconds.
 */
pub fn world_decompose(wav: Vec<f64>, fs: i32, frame_period: f64) -> DecomposeResult {
  /*// https://s3.amazonaws.com/temp.michaelfbryan.com/arrays/index.html
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

  // Nothing appears to get allocated if I use 'with_capacity':
  // let mut estimated_f0_contour : Vec<f64> = Vec::with_capacity(wav.len());
  // let mut temporal_positions : Vec<f64> = Vec::with_capacity(wav.len());

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

  DecomposeResult {
    estimated_f0_contour,
    temporal_positions,
  }*/
  DecomposeResult {}
}

/*
def wav_padding(wav, sr, frame_period, multiple = 4):
    assert wav.ndim == 1
    num_frames = len(wav)
    num_frames_padded = int((np.ceil((np.floor(num_frames / (sr * frame_period / 1000)) + 1) / multiple + 1) * multiple - 1) * (sr * frame_period / 1000))
    num_frames_diff = num_frames_padded - num_frames
    num_pad_left = num_frames_diff // 2
    num_pad_right = num_frames_diff - num_pad_left
    wav_padded = np.pad(wav, (num_pad_left, num_pad_right), 'constant', constant_values = 0)

    return wav_padded
*/
/**
 * wav padding
 - NB: Guessing that frame_period is "frame_period: period between consecutive frames in milliseconds."
 - Multiple '4' default
*/

pub fn wav_padding(wav: Vec<f64>, frame_period: f64, multiple: Option<i64>) {
  let multiple = multiple.unwrap_or(4);

  // TODO
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::mem;

  #[test]
  pub fn test_world_decompose() {
    // TODO
  }
}
