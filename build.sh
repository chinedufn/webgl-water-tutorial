#!/bin/bash

# Change to the directory that this script is in.
# This allows you to run this script from anywhere and have it still work.
cd $(dirname $0)

WASM_TARGET=wasm32-unknown-unknown
NIGHTLY=nightly-2021-02-11

rustup override set ${NIGHTLY}
rustup target add ${WASM_TARGET}

# ./build.sh
if [ -z "$RELEASE"  ]; then
  # --------------------------------------------------
  # DEVELOPMENT BUILD
  # --------------------------------------------------

  # Build the webgl_water_tutorial.wasm file
  RUST_BACKTRACE=1 cargo build --target ${WASM_TARGET}

  # # Process the webgl_water_tutorial.wasm file and generate the necessary
  # # JavaScript glue code to run it in the browser.
  wasm-bindgen ./target/${WASM_TARGET}/debug/webgl_water_tutorial.wasm --out-dir . --no-typescript --no-modules

# RELEASE=1 ./build.sh
else

  # --------------------------------------------------
  # RELEASE BUILD
  # --------------------------------------------------

  # Build the webgl_water_tutorial.wasm file
  cargo build --target ${WASM_TARGET} --release &&
  wasm-bindgen ./target/${WASM_TARGET}/release/webgl_water_tutorial.wasm --out-dir . --no-typescript --no-modules &&
  wasm-opt -O3 -o optimized.wasm webgl_water_tutorial_bg.wasm  &&
  mv optimized.wasm webgl_water_tutorial_bg.wasm
fi
