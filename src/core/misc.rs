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

#[cfg(test)]
mod tests {
  use super::*;
  use std::mem;
}
