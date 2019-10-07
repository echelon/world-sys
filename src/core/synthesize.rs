
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

