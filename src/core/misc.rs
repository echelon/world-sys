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
def get_cheaptrick_fft_size(fs, f0_floor=default_f0_floor):
  """Calculate suitable FFT size for CheapTrick given F0 floor.
  Parameters
  ----------
  fs : int
      Sample rate of input signal in Hz.
  f0_floor : float
      Lower F0 limit in Hz. The required FFT size is a direct
      consequence of the F0 floor used.
      Default: 71.0
  Returns
  -------
  fft_size : int
      Resulting FFT size.
  """
  cdef CheapTrickOption option
  option.f0_floor = f0_floor
  cdef int fft_size = GetFFTSizeForCheapTrick(fs, &option)
  return fft_size
*/
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

void CheapTrick(const double *x, int x_length, int fs,
    const double *temporal_positions, const double *f0, int f0_length,
    const CheapTrickOption *option, double **spectrogram) {

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


/*
  cdef CheapTrickOption option
  InitializeCheapTrickOption(fs, &option)
  option.q1 = q1
  if fft_size is None:
      option.f0_floor = f0_floor  # CheapTrickOption.f0_floor is only used in GetFFTSizeForCheapTrick()
      option.fft_size = GetFFTSizeForCheapTrick(fs, &option)
  else:
      option.fft_size = fft_size
      # the f0_floor used by CheapTrick() will be re-compute from this given fft_size
  cdef int x_length = <int>len(x)
  cdef int f0_length = <int>len(f0)

  cdef double[:, ::1] spectrogram = np.zeros((f0_length, option.fft_size//2 + 1),
                                             dtype=np.dtype('float64'))
  cdef np.intp_t[:] tmp = np.zeros(f0_length, dtype=np.intp)
  cdef double **cpp_spectrogram = <double**> (<void*> &tmp[0])
  cdef np.intp_t i
  for i in range(f0_length):
      cpp_spectrogram[i] = &spectrogram[i, 0]

*/

/*
  CheapTrick(&x[0], x_length, fs, &temporal_positions[0],
      &f0[0], f0_length, &option, cpp_spectrogram)
  return np.array(spectrogram, dtype=np.float64)
*/

/*
def get_cheaptrick_fft_size(*args, **kwargs): # real signature unknown
    """
    Calculate suitable FFT size for CheapTrick given F0 floor.

        Parameters
        ----------
        fs : int
            Sample rate of input signal in Hz.
        f0_floor : float
            Lower F0 limit in Hz. The required FFT size is a direct
            consequence of the F0 floor used.
            Default: 71.0

        Returns
        -------
        fft_size : int
            Resulting FFT size.
    """
    pass

    pub fn GetFFTSizeForCheapTrick(
        fs: ::std::os::raw::c_int,
        option: *const CheapTrickOption,
    ) -> ::std::os::raw::c_int;
*/

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

D4C(&x[0], x_length, fs, &temporal_positions[0],
&f0[0], f0_length, fft_size0, &option,
cpp_aperiodicity)

*/




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
