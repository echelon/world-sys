//! Core Builtins
//!
//! These are things that are built into WORLD or provided by
//! the 'pyworld' bindings.
//!
//! See:
//! https://github.com/JeremyCCHsu/Python-Wrapper-for-World-Vocoder/blob/master/pyworld/pyworld.pyx
//!

use std::os::raw::c_int;
use std::ptr;

use super::cheaptrick::get_cheaptrick_fft_size;

// Constants
use super::super::world_kCeilF0;
use super::super::world_kFloorF0;
use super::super::world_kFloorF0D4C; // TODO: Used?
use super::super::world_kThreshold;

// CodeSpectralEnvelope
use super::super::CodeSpectralEnvelope;

// DecodeSpectralEnvelope
use super::super::DecodeSpectralEnvelope;

// Harvest
use super::super::GetSamplesForHarvest;
use super::super::Harvest;
use super::super::HarvestOption;
use super::super::InitializeHarvestOption;

// Synthesis
use super::super::Synthesis2; // TODO: Used?
use super::super::Synthesis;
use super::super::WorldSynthesizer; // TODO: Used? Looks complicated.
use super::super::InitializeSynthesizer;

/*
def code_spectral_envelope(*args, **kwargs): # real signature unknown
    """
    Reduce dimensionality of spectral envelope.

        Parameters
        ----------
        spectrogram : ndarray
            Spectral envelope.
        fs : int
            Sample rate of input signal in Hz.
        number_of_dimensions : int
            Number of dimentions of coded spectral envelope

        Returns
        -------
        coded_spectral_envelope : ndarray
            Coded spectral envelope.
    """
    pass

    pub fn CodeSpectralEnvelope(
        spectrogram: *const *const f64,
        f0_length: ::std::os::raw::c_int,
        fs: ::std::os::raw::c_int,
        fft_size: ::std::os::raw::c_int,
        number_of_dimensions: ::std::os::raw::c_int,
        coded_spectral_envelope: *mut *mut f64,
    );
*/

pub struct CodeSpectralEnvelopeResult {
  pub coded_spectral_envelope: Vec<Vec<f64>>,
}

/**
 * CodeSpectralEnvelope: reduce dimensionality of spectral envelope
 * - spectrogram
 * - fs: sample rate of input signal in Hz
 * - number_of_dimensions: number of dimensions of coded spectral envelope
 */
pub fn code_spectral_envelope(spectrogram: Vec<Vec<f64>>,
                              fs: i32,
                              number_of_dimensions: u32) -> CodeSpectralEnvelopeResult {

  let fft_size = 1; // TODO

  // FIXME -- Not sure this is correct allocation!
  // But I'm not sure these are the correct lengths...
  //let mut coded_spectral_envelope: Vec<Vec<f64>> = vec![0.0f64; spectrogram.len()];

  /*unsafe {
    CodeSpectralEnvelope(
      spectrogram.as_ptr(),
      spectrogram.len() as c_int, //FIXME: WAT f0_length!??!
      fs as c_int,
      fft_size as c_int,
      number_of_dimensions as c_int,
      coded_spectral_envelope.as_mut_ptr() as *mut _,
    );
  }*/

  CodeSpectralEnvelopeResult {
    coded_spectral_envelope: vec![],
  }
}

/*
def decode_spectral_envelope(*args, **kwargs): # real signature unknown
    """
    Restore full dimensionality of coded spectral envelope.

        Parameters
        ----------
        coded_spectral_envelope : ndarray
            Coded spectral envelope.
        fs : int
            Sample rate of input signal in Hz.
        fft_size : int
            FFT size corresponding to the full dimensional spectral envelope.

        Returns
        -------
        spectrogram : ndarray
            Spectral envelope.
    """
    pass

    pub fn DecodeSpectralEnvelope(
        coded_spectral_envelope: *const *const f64,
        f0_length: ::std::os::raw::c_int,
        fs: ::std::os::raw::c_int,
        fft_size: ::std::os::raw::c_int,
        number_of_dimensions: ::std::os::raw::c_int,
        spectrogram: *mut *mut f64,
    );
*/

pub struct DecodeSpectralEnvelopeResult {
  pub spectrogram: Vec<f64>,
}

pub fn decode_spectral_envelope() -> DecodeSpectralEnvelopeResult {
  DecodeSpectralEnvelopeResult {
    spectrogram: vec![],
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

/*
def synthesize(*args, **kwargs): # real signature unknown
    """
    WORLD synthesis from parametric representation.

        Parameters
        ----------
        f0 : ndarray
            Input F0 contour.
        spectrogram : ndarray
            Spectral envelope.
        aperiodicity : ndarray
            Aperodicity envelope.
        fs : int
            Sample rate of input signal in Hz.
        frame_period : float
            Period between consecutive frames in milliseconds.
            Default: 5.0

        Returns
        -------
        y : ndarray
            Output waveform signal.
    """
    pass

    pub fn Synthesis(
        f0: *const f64,
        f0_length: ::std::os::raw::c_int,
        spectrogram: *const *const f64,
        aperiodicity: *const *const f64,
        fft_size: ::std::os::raw::c_int,
        frame_period: f64,
        fs: ::std::os::raw::c_int,
        y_length: ::std::os::raw::c_int,
        y: *mut f64,
    );

    pub fn Synthesis2(synth: *mut WorldSynthesizer) -> ::std::os::raw::c_int;
*/

pub struct SynthesizeResult {
  output_waveform: Vec<f64>,
}

pub fn synthesize(f0: Vec<f64>,
                  spectrogram: Vec<f64>, // 2D
                  aperiodicity: Vec<f64>, // 2D
                  fs: i32,
                  frame_period: Option<f64>) -> SynthesizeResult {

  // Defaults
  let frame_period = frame_period.unwrap_or(5.0f64);

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
  let mut wav: Vec<f64> = vec![0.0f64; f0.len()];

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
      wav.as_mut_ptr() as *mut _,
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
