//
// Laptop has an old clang that bindgen doesn't like for C++,
// clang version 3.8.0-2ubuntu4 (tags/RELEASE_380/final)
// root root 25 Jun 21  2016 /usr/bin/clang -> ../lib/llvm-3.8/bin/clang
//
// But now,
// After installing the things from http://apt.llvm.org/
// And fixing up some symlinks
// root root 27 Sep 24 21:47 /usr/bin/clang -> /usr/lib/llvm-3.9/bin/clang
// Actually, that doesn't work.
// `LIBCLANG_PATH=/usr/lib/llvm-3.9/lib cargo test` DOES.
//
extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;
use std::path::Path;

fn main() {
  // Write the bindings to the $OUT_DIR/bindings.rs file.
  // The compiled C++ will also go here.
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

  cc::Build::new()
    .cpp(true)
    .include(Path::new("/usr/local/include/world"))
    .include(Path::new("World/src/world"))
    .file("World/src/cheaptrick.cpp")
    .file("World/src/codec.cpp")
    .file("World/src/common.cpp")
    .file("World/src/d4c.cpp")
    .file("World/src/dio.cpp")
    .file("World/src/fft.cpp")
    .file("World/src/harvest.cpp")
    .file("World/src/matlabfunctions.cpp")
    .file("World/src/stonemask.cpp")
    .file("World/src/synthesis.cpp")
    .file("World/src/synthesisrealtime.cpp")
    .warnings(false) // NB: warnings spam the build
    .compile("world");
    //.shared_flag(true)
    //.compile("libworld.a");
    //.compile("libworld.so");

  // dylib,
  //println!("cargo:rustc-link-lib=world");

  // static,
  //println!("cargo:rustc-link-search=native=/usr/local/lib");
  //println!("cargo:rustc-link-search=native={}",
  //         out_path.as_path().to_str().clone().unwrap());
  //println!("cargo:rustc-link-lib=static=world");

  let bindings = bindgen::Builder::default()
    .header("wrapper.hpp")
    .header("World/src/world/constantnumbers.h")
    //.trust_clang_mangling(false)
    //.enable_cxx_namespaces()
    .derive_debug(true)
    .derive_default(true)
    .generate_comments(true)
    .clang_arg("-x")
    .clang_arg("c++")
    .clang_arg("-std=c++14")
    .generate()
    .expect("Unable to generate bindings");

  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");

  // static, built with makefile instead of Rust's bindgen:
  println!("cargo:rustc-link-search=native=/usr/local/lib");

  let lib_location = out_path.as_path().to_str().clone().unwrap();

  //println!("cargo:rustc-link-search=native={}", lib_location);
  //println!("cargo:rustc-link-lib=static=world");
  //println!("cargo:rustc-link-search={}", lib_location); 
  //println!("cargo:rustc-link-lib=world");
}

