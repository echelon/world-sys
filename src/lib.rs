#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(safe_extern_statics)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
//include!("bindgen.rs");

pub mod extras;

#[cfg(test)]
mod tests {
  use super::*;
  use std::mem;

  // Adapted from /examples/codec_test/f0analysis.cpp
  #[test]
  fn test_harvest() {
    let mut option = HarvestOption::default();

    unsafe {
      InitializeHarvestOption(&mut option);
    }

    option.frame_period = 5.0;
    option.f0_floor = world_kFloorF0;
    option.f0_ceil = world_kCeilF0;

    //HarvestOption option = { 0 };
    /*InitializeHarvestOption(&option);
    option.frame_period = 5.0;
    option.f0_ceil = world::kCeilF0;*/
    // TODO
    //
    let fs = 0;
    let x_length = 100;

    unsafe {
      // F0 analysis
      let number_of_frames = 
        GetSamplesForHarvest(fs, x_length, option.frame_period);
    }
  }

  #[test]
  fn test_fft() {
    let mut fft = fft_plan::default();

    fft.n = 12;

    unsafe {
      //let mut fft = fft_plan_dft_c2r_1d();
      //fft_destroy_plan(fft); // FIXME: Broken
    }
  }

  #[test]
  fn test_cheap_trick() {
    let mut option = CheapTrickOption::default();

    unsafe {
      InitializeCheapTrickOption(1, &mut option);
    }

    option.q1 = 0.0f64;
    option.f0_floor= 0.0f64;
    option.fft_size= 1;

    unsafe {
      //let mut fft = fft_plan_dft_c2r_1d();
      //fft_destroy_plan(fft); // FIXME: Broken
    }
  }
}

