
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

  // NB(from pyworld):
  // y_length = int(f0_length * frame_period * fs / 1000)
  let y_length = f0.len() as f64 * frame_period * fs as f64;
  let y_length = (y_length / 1000.0) as usize;

  // NB(from pyworld):
  // cdef int fft_size = (<int>spectrogram.shape[1] - 1)*2
  let fft_size = (spectrogram[0].len() - 1) * 2; // FIXME UNSAFE

  /*
    cdef int f0_length = <int>len(f0)
    cdef np.ndarray[double, ndim=1, mode="c"] y = \
        np.zeros(y_length, dtype=np.dtype('float64'))

    cdef double[:, ::1] spectrogram0 = spectrogram
    cdef double[:, ::1] aperiodicity0 = aperiodicity
    cdef np.intp_t[:] tmp = np.zeros(f0_length, dtype=np.intp)
    cdef np.intp_t[:] tmp2 = np.zeros(f0_length, dtype=np.intp)
    cdef double **cpp_spectrogram = <double**> (<void*> &tmp[0])
    cdef double **cpp_aperiodicity = <double**> (<void*> &tmp2[0])
    cdef np.intp_t i
    for i in range(f0_length):
        cpp_spectrogram[i] = &spectrogram0[i, 0]
        cpp_aperiodicity[i] = &aperiodicity0[i, 0]
  */

  /*
      Synthesis(&f0[0], f0_length, cpp_spectrogram,
        cpp_aperiodicity, fft_size, frame_period, fs, y_length, &y[0])
  */

  let mut spectrogram2 : Vec<*const f64> = Vec::new();
  for x in spectrogram.iter() {
    spectrogram2.push(x.as_ptr());
  }


  let mut wav: Vec<f64> = Vec::new();

  for i in 0 .. y_length {
    wav.push(0.0f64);
  }

  unsafe {
    Synthesis(
      f0.as_ptr(),
      f0.len() as c_int,
      spectrogram2.as_ptr(),
      aperiodicity.as_ptr() as *const _, // TODO: Does this work? I invented this! ----- ??????
      fft_size as c_int,
      frame_period,
      fs as c_int,
      wav.len() as c_int,
      //wav.as_mut_ptr() as *mut _,
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

    for i in 0..500 {
      let v = (i % 100) as f64;
      f0.push(v);
    }

    let mut spectrogram = Vec::new();
    let mut aperiodicity = Vec::new();

    for i in 0..500 {
      let mut inner = Vec::new();

      for j in 0..500 {
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
