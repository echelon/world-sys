use std::os::raw::c_int;
use std::ptr;

// Constants
use super::super::world_kCeilF0;
use super::super::world_kFloorF0;
use super::super::world_kFloorF0D4C; // TODO: Used?
use super::super::world_kThreshold;

// CheapTrick
use super::super::CheapTrick;
use super::super::CheapTrickOption;
use super::super::GetF0FloorForCheapTrick;
use super::super::GetFFTSizeForCheapTrick;
use super::super::InitializeCheapTrickOption;

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
  let n = f0.len();
  let m = (fft_size/2 + 1) as usize;

  println!("Fft size: {}", fft_size);
  println!("m: {}", m);
  //unimplemented!("test");

  let size = n * m;
  let mut spectrogram: Vec<f64> = vec![0.0f64; size];
  let mut spectrogram: Vec<f64> = vec![0.0f64; 10241 * 5130];
  //let mut spectrogram = vec![vec![0.0f64; 1024] ; 550513]; // SEGFAULT
  //let mut spectrogram = Box::new([0.0f64; size]); // DOES NOT COMPILE

  //let mut spectrogram: Vec<f64> = vec![0.0f64; size].into_boxed_slice(); SEGFAULT (rewrote code)

  // NB: This yields a stack overflow instead of a segfault!
  let mut spectrogram = [[0.0f64; 1024] ; 513];

  //let mut spectrogram : *mut f64 = ptr::null();
  // https://github.com/neithanmo/csound-rs/blob/46b50fa94ebb869d051b7d7f74555c76ecd4cbe9/src/callbacks.rs
  let mut ptr = ::std::ptr::null_mut();
  let spectrogram: *mut *mut f64 = &mut ptr as *mut *mut _;

  //println!("Spectrogram length: {}", spectrogram.len());

  /*
    CheapTrick(&x[0], x_length, fs, &temporal_positions[0],
        &f0[0], f0_length, &option, cpp_spectrogram)
    return np.array(spectrogram, dtype=np.float64)
  */

  unsafe {
    CheapTrick(
      wav.as_ptr(),
      wav.len() as c_int,
      fs as c_int,
      temporal_postions.as_ptr(),
      f0.as_ptr(),
      f0.len() as c_int,
      &mut option,
      spectrogram,
    );
  }

  CheapTrickResult {
    //spectrogram,
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

#[cfg(test)]
mod tests {
  use super::*;
  use std::mem;

  // TODO: I think this is stack overflowing instead of segfaulting now.
  /*#[test]
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
  }*/

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
}
