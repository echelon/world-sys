world-sys
=========
Rust bindings for [WORLD vocoder](https://github.com/mmorise/World).

WIP.

Building
--------
Currently WORLD is cloned into the top level directory and `build.rs` refers to it.

I also tried moving the includes and lib (statically linked) into `/usr/local/{lib,include}`, 
which didn't seem to work as well.

The rust file generated can live in `src/bindgen.rs` (preferred) as generated by `generate_bindgen.sh`,
but `build.rs` puts them under the build dir. Note the difference in imports in `lib.rs` to accommodate
both configurations.

NB: `LIBCLANG_PATH` is critical to set!

- `LIBCLANG_PATH=/usr/lib/llvm-3.9/lib cargo test` (laptop)
- `LIBCLANG_PATH=/usr/lib/llvm-6.0/lib/ cargo test` (desktop)

Also appears to work if I build the library independent of Rust bindgen, then statically link 
from build.rs:

`println!("cargo:rustc-link-search=native=/usr/local/lib");`

Debugging
---------

- Don't forget to set `LIBCLANG_PATH` !
- `nm libfoo.a` to list symbol names
- `nm -C libfoo.a` to list symbol names without mangling
- `ar -t libfoo.a` to list object files

To use GDB to debug, follow these instructions:

http://smallcultfollowing.com/babysteps/blog/2018/09/21/office-hours-0-debugging-with-gdb/

List the binary in test build/exec,
> `LIBCLANG_PATH=/usr/lib/llvm-6.0/lib/ cargo test -v`

Run under GDB,
> `RUST_TEST_THREADS=1 rust-gdb {the binary}`

TODO / WIP
----------
- [ ] Test entire API surface area
- [ ] Convert end-to-end example
- [ ] Create a _safe_ idiomatic wrapper

