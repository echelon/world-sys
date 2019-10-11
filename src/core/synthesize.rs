
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

#[derive(Debug)]
pub enum SynthesizeError {
  WrongOuterDimension {
    f0_len: usize,
    aperiodicity_len: usize,
    spectrogram_len: usize,
  },
  WrongInnerDimension {
    aperiodicity_len: usize,
    spectrogram_len: usize,
  }
}

pub fn synthesize(f0: &Vec<f64>,
  spectrogram: &Vec<Vec<f64>>, // 2D
  aperiodicity: &Vec<Vec<f64>>, // 2D
  fs: i32,
  frame_period: Option<f64>) -> Result<SynthesizeResult,SynthesizeError> {

  if f0.len() != spectrogram.len()
      || f0.len() != aperiodicity.len() {
    return Err(SynthesizeError::WrongOuterDimension {
      f0_len: f0.len(),
      aperiodicity_len: aperiodicity.len(),
      spectrogram_len: spectrogram.len(),
    });
  }

  // TODO: Unsafe.
  if aperiodicity[0].len() != spectrogram[0].len() {
    return Err(SynthesizeError::WrongInnerDimension {
      aperiodicity_len: aperiodicity[0].len(),
      spectrogram_len: spectrogram[0].len(),
    });
  }

  // Defaults
  let frame_period = frame_period.unwrap_or(5.0f64);

  /*
    cdef np.ndarray[double, ndim=1, mode="c"] y = \
        np.zeros(y_length, dtype=np.dtype('float64'))
  */
  let f0_length = f0.len();

  // NB(from pyworld):
  // y_length = int(f0_length * frame_period * fs / 1000)
  let y_length = f0_length as f64 * frame_period * fs as f64;
  let y_length = (y_length / 1000.0).floor() as usize;

  // NB(from pyworld):
  // cdef int fft_size = (<int>spectrogram.shape[1] - 1)*2
  let fft_size = (spectrogram[0].len() - 1) * 2; // FIXME UNSAFE

  /*
    The simplest data layout might be a C contiguous array. This is the
    default layout in NumPy and Cython arrays. C contiguous means that
    the array data is continuous in memory (see below) and that
    neighboring elements in the first dimension of the array are
    furthest apart in memory, whereas neighboring elements in the last
    dimension are closest together.

    # This array is C contiguous
    c_contig = np.arange(24).reshape((2,3,4))
    cdef int[:, :, ::1] c_contiguous = c_contig

    # This view is C contiguous
    cdef int[:, :, ::1] c_contiguous = myview.copy()

    # This view is Fortran contiguous
    cdef int[::1, :] f_contiguous_slice = myview.copy_fortran()

    -------------------------------------------------

    cdef double[:, ::1] spectrogram0 = spectrogram
    cdef double[:, ::1] aperiodicity0 = aperiodicity

  */

  /*

    cdef np.intp_t[:] tmp = np.zeros(f0_length, dtype=np.intp)
    cdef np.intp_t[:] tmp2 = np.zeros(f0_length, dtype=np.intp)

  */

  //let tmp = Vec::with_capacity(f0_length);
  //let tmp2 = Vec::with_capacity(f0_length);

  /*
    cdef double **cpp_spectrogram = <double**> (<void*> &tmp[0])
    cdef double **cpp_aperiodicity = <double**> (<void*> &tmp2[0])
    cdef np.intp_t i

    # Outer dim becomes f0_length
    for i in range(f0_length):
        cpp_spectrogram[i] = &spectrogram0[i, 0]
        cpp_aperiodicity[i] = &aperiodicity0[i, 0]
  */

  // TODO: This could be more efficient.
  let mut spectrogram2 : Vec<*const f64> = Vec::new();
  for x in spectrogram.iter() {
    spectrogram2.push(x.as_ptr());
  }

  let mut aperiodicity2 : Vec<*const f64> = Vec::new();
  for x in aperiodicity.iter() {
    aperiodicity2.push(x.as_ptr());
  }

  /*
      Synthesis(&f0[0], f0_length, cpp_spectrogram,
        cpp_aperiodicity, fft_size, frame_period, fs, y_length, &y[0])
  */

  /*
    cdef np.ndarray[double, ndim=1, mode="c"] y = \
        np.zeros(y_length, dtype=np.dtype('float64'))
  */

  let mut wav: Vec<f64> = Vec::new();
  for i in 0 .. y_length {
    wav.push(0.0f64);
  }

  println!("Wav (y) length: {}", wav.len());
  println!("y_length: {}", y_length);

  unsafe {
    Synthesis(
      f0.as_ptr(),
      f0_length as c_int,
      spectrogram2.as_ptr(),
      aperiodicity2.as_ptr(),
      fft_size as c_int,
      frame_period,
      fs as c_int,
      wav.len() as c_int,
      wav.as_mut_ptr(),
    );
  }

  Ok(SynthesizeResult {
    output_waveform: wav,
  })
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::mem;

  #[test]
  pub fn test_synthesize() {
    let mut f0 = Vec::new();

    for i in 0..100 {
      let v = (i % 100) as f64;
      f0.push(v);
    }

    let mut spectrogram = Vec::new();
    let mut aperiodicity = Vec::new();

    for i in 0..100 {
      let mut inner = Vec::new();

      for j in 0..50 {
        let v = (i % 100) as f64;
        inner.push(v);
      }

      spectrogram.push(inner.clone());
      aperiodicity.push(inner.clone());
    }

    let result = synthesize(
      &f0,
      &spectrogram,
      &aperiodicity,
      16_000,
      None,
    ).unwrap();

    // Check dimensions
    assert!(result.output_waveform.len() > 0);

    // NB: Just spot checking the array for now.
    // Should improve this to do an actual calculation.
    //assert_ne!(0.0f64, result.coded_spectral_envelope[0][0]);
    //assert_ne!(0.0f64, result.coded_spectral_envelope[1][0]);
    //assert_ne!(0.0f64, result.coded_spectral_envelope[1][1]);
  }
}
