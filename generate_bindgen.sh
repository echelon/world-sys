#!/bin/bash

if ! which bindgen > /dev/null; then
    echo "ERROR: Please install 'bindgen' using cargo:"
    echo "    cargo install bindgen"
    echo "See https://github.com/servo/rust-bindgen for more information."
    exit 1
fi

bindgen_options=""
#header="./World/src/world/harvest.h"
#header="/usr/local/include/world/harvest.h"
header="wrapper.hpp"

bindgen ${bindgen_options} \
  ${header} \
  --with-derive-default \
  --output src/bindgen.rs
  #--distrust-clang-mangling \
  #--enable-cxx-namespaces \



