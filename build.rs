extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("World/src/cheaptrick.cpp")
        .file("World/src/codec.cpp")
        .file("World/src/fft.cpp")
        .file("World/src/harvest.cpp")
        .compile("libworld.a");

    // dylib,
    //println!("cargo:rustc-link-lib=world");

    // static,
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-lib=static=world");

    let bindings = bindgen::Builder::default()
        .header("wrapper.hpp")
        //.trust_clang_mangling(false)
        //.enable_cxx_namespaces()
        .derive_debug(true)
        .derive_default(true)
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++14")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

