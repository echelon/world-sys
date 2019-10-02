//! Core Builtins
//!
//! These are things that are built into WORLD or provided by
//! the 'pyworld' bindings.
//!
//! See:
//! https://github.com/JeremyCCHsu/Python-Wrapper-for-World-Vocoder/blob/master/pyworld/pyworld.pyx
//!

use std::os::raw::c_int;

// Constants
use super::world_kCeilF0;
use super::world_kFloorF0;
use super::world_kFloorF0D4C; // TODO: Used?
use super::world_kThreshold;

// CheapTrick
use super::CheapTrick;
use super::CheapTrickOption;
use super::GetF0FloorForCheapTrick;
use super::GetFFTSizeForCheapTrick;
use super::InitializeCheapTrickOption;

// CodeSpectralEnvelope
use super::CodeSpectralEnvelope;

// D4C
use super::D4C;
use super::D4COption;
use super::InitializeD4COption;

// DecodeSpectralEnvelope
use super::DecodeSpectralEnvelope;

// Harvest
use super::GetSamplesForHarvest;
use super::Harvest;
use super::HarvestOption;
use super::InitializeHarvestOption;

// Synthesis
use super::Synthesis2; // TODO: Used?
use super::Synthesis;
use super::WorldSynthesizer; // TODO: Used? Looks complicated.
use super::InitializeSynthesizer;

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
  pub spectrogram: Vec<f64>
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
  unsafe {
    InitializeCheapTrickOption(fs, &mut option);
  }

  option.q1 = q1;

  let fft_size = match fft_size {
    Some(fft_size) => {
      // NB: From pyworld --
      // the f0_floor used by CheapTrick() will be re-compute from this given fft_size
      fft_size
    },
    None => {
      // NB: From pyworld --
      // CheapTrickOption.f0_floor is only used in GetFFTSizeForCheapTrick()
      option.f0_floor = f0_floor;
      unsafe {
        GetFFTSizeForCheapTrick(fs, &option)
      }
    },
  };

  option.fft_size = fft_size;

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

  // FIXME -- Not sure this is correct allocation!
  // But I'm not sure these are the correct lengths...
  // pyworld shape is: (f0_length, option.fft_size//2 + 1)
  let size = f0.len() * (fft_size/2 + 1) as usize;
  let mut spectrogram: Vec<f64> = vec![0.0f64; size];

  /*
    CheapTrick(&x[0], x_length, fs, &temporal_positions[0],
        &f0[0], f0_length, &option, cpp_spectrogram)
    return np.array(spectrogram, dtype=np.float64)
  */
  /*unsafe {
    CheapTrick(
      wav.as_ptr(),
      wav.len() as c_int,
      fs as c_int,
      temporal_postions.as_ptr(),
      f0.as_ptr(),
      f0.len() as c_int,
      &option,
      spectrogram.as_mut_ptr() as *mut _,
    );
  }*/

  CheapTrickResult {
    spectrogram,
  }
}

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
    Some(f) => f,
    None => {
      /*
        if fft_size is None:
            fft_size0 = get_cheaptrick_fft_size(fs, default_f0_floor)
        else:
            fft_size0 = fft_size
      */
      let default_f0_floor = world_kFloorF0; // default: 71.0
      let result = get_cheaptrick_fft_size(fs, Some(default_f0_floor));
      result.fft_size
    },
  };

  let mut option = D4COption::default();
  option.threshold = threshold;

  // FIXME -- Not sure this is correct allocation!
  // But I'm not sure these are the correct lengths...
  let mut aperiodicity: Vec<f64> = vec![0.0f64; wav.len()];

  unsafe {
    D4C(
      wav.as_ptr(),
      wav.len() as c_int,
      fs as c_int,
      temporal_postions.as_ptr(),
      f0.as_ptr(),
      f0.len() as c_int,
      fft_size as c_int,
      &option,
      aperiodicity.as_mut_ptr() as *mut _,
    );
  }

  D4CResult {
    aperiodicity,
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

pub fn decode_spectral_envelope_result() -> DecodeSpectralEnvelopeResult {
  DecodeSpectralEnvelopeResult {
    spectrogram: vec![],
  }
}

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

pub struct GetCheaptrickFftSizeResult {
  pub fft_size: i32,
}

pub fn get_cheaptrick_fft_size(fs: i32,
                               f0_floor: Option<f64>) -> GetCheaptrickFftSizeResult {

  let f0_floor = f0_floor.unwrap_or(world_kFloorF0); // default: 71.0

  let mut option = CheapTrickOption::default();
  option.f0_floor = f0_floor;

  let fft_size = unsafe {
    GetFFTSizeForCheapTrick(fs, &option)
  };

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
  GetCheaptrickFftSizeResult {
    fft_size,
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
  pub fn test_cheaptrick() {
    let mut audio = Vec::new();

    for i in 0..500 {
      let v = (i % 100) as f64;
      audio.push(v);
    }

    let f0 = audio.clone();
    let temporal = audio.clone();

    let result = cheaptrick(audio, f0, temporal, 16000, None, None, None);

    println!("Result spectrogram: {:?}", result.spectrogram);
  }

  #[test]
  pub fn test_d4c() {
    let mut audio = Vec::new();

    for i in 0..500 {
      let v = (i % 100) as f64;
      audio.push(v);
    }

    let f0 = audio.clone();
    let temporal = audio.clone();

    /*let result = d4c(audio, f0, temporal, 16000, None, None, None);

    println!("Result aperiod len: {:?}", result.aperiodicity.len());
    println!("Result aperiod first item: {:?}", result.aperiodicity[0]);*/
  }

  #[test]
  pub fn test_get_cheaptrick_fft_size() {
    let result = get_cheaptrick_fft_size(16000, Some(world_kFloorF0));
    assert_eq!(1024, result.fft_size);
    let result = get_cheaptrick_fft_size(44100, Some(world_kFloorF0));
    assert_eq!(2048, result.fft_size);

    // NB: We don't use the ceiling, but here's an example anyway:
    let result = get_cheaptrick_fft_size(16000, Some(world_kCeilF0));
    assert_eq!(64, result.fft_size);
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
