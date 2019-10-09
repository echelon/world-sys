
use std::os::raw::c_int;
use std::ptr;

// Constants
use super::super::world_kCeilF0;
use super::super::world_kFloorF0;
use super::super::world_kFloorF0D4C; // TODO: Used?
use super::super::world_kThreshold;

// Synthesis
use super::super::Synthesis2; // TODO: Used?
use super::super::Synthesis;
use super::super::WorldSynthesizer; // TODO: Used? Looks complicated.
use super::super::InitializeSynthesizer;

pub struct SynthesizeResult {
  output_waveform: Vec<f64>,
}

pub fn synthesize(f0: &Vec<f64>,
  spectrogram: &Vec<Vec<f64>>, // 2D
  aperiodicity: Vec<Vec<f64>>, // 2D
  fs: i32,
  frame_period: Option<f64>) -> SynthesizeResult {

  // Defaults
  let frame_period = frame_period.unwrap_or(5.0f64);

  // NB(from pyworld):
  let y_length = f0.len() * frame_period * fs / 1000;

  /*
  cdef int f0_length = <int>len(f0)
  y_length = int(f0_length * frame_period * fs / 1000)
  cdef int fft_size = (<int>spectrogram.shape[1] - 1)*2
  cdef np.ndarray[double, ndim=1, mode="c"] y = \
  np.zeros(y_length, dtype=np.dtype('float64'))
  */
  let fft_size = (spectrogram.len() - 1) * 2; // TODO: Incorrect wrt multidimensional

  // FIXME -- Not sure this is correct allocation!
  // But I'm not sure these are the correct lengths...

  /*
      Synthesis(&f0[0], f0_length, cpp_spectrogram,
        cpp_aperiodicity, fft_size, frame_period, fs, y_length, &y[0])
  */
  let mut wav: Vec<f64> = Vec::new();

  for i in 0 .. y_length {
    wav.push(0.0f64);
  }

  unsafe {
    Synthesis(
      f0.as_ptr(),
      f0.len() as c_int,
      spectrogram.as_ptr() as *const _, // TODO: Does this work? I invented this! ----- ??????
      aperiodicity.as_ptr() as *const _, // TODO: Does this work? I invented this! ----- ??????
      fft_size as c_int,
      frame_period,
      fs as c_int,
      wav.len() as c_int,
      //wav.as_mut_ptr() as *mut _,
      wav.as_mut_ptr(),
    );
  }

  SynthesizeResult {
    output_waveform: wav,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::mem;

  #[test]
  pub fn test_encode_spectral_envelope() {
    let mut spectrogram = Vec::new();

    for i in 0..500 {
      let mut inner = Vec::new();

      for j in 0..500 {
        let v = (i % 100) as f64;
        inner.push(v);
      }
      spectrogram.push(inner);
    }

    let result = code_spectral_envelope(
      &spectrogram,
      16_000,
      128
    );

    // Check dimensions
    assert!(result.coded_spectral_envelope.len() > 0);
    assert!(result.coded_spectral_envelope[0].len() > 0);

    // NB: Just spot checking the array for now.
    // Should improve this to do an actual calculation.
    //assert_ne!(0.0f64, result.coded_spectral_envelope[0][0]);
    //assert_ne!(0.0f64, result.coded_spectral_envelope[1][0]);
    //assert_ne!(0.0f64, result.coded_spectral_envelope[1][1]);
  }
}
