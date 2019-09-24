world-sys
=========
Rust bindings for [WORLD vocoder](https://github.com/mmorise/World).

WIP.

Building
--------
Currently WORLD is cloned into the top level directory and `build.rs` refers to it.

I also tried moving the includes and lib (statically linked) into `/usr/local/{lib,include}`, 
which didn't seem to work as well.

TODO / WIP
----------
- [ ] Test entire API surface area
- [ ] Convert end-to-end example
- [ ] Create a _safe_ idiomatic wrapper

