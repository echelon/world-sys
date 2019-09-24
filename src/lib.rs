#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
  use super::*;
  use std::mem;

  // Adapted from /examples/codec_test/f0analysis.cpp
  #[test]
  fn test_library() {
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
      let number_of_frames = GetSamplesForHarvest(fs, x_length, option.frame_period);
    }
  }
}

