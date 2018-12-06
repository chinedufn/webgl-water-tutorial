#!/bin/bash

# Change to the directory that this script is in.
# This allows you to run this script from anywhere and have it still work.
cd $(dirname $0)

# Build the webgl_water_tutorial.wasm file
cargo build --target wasm32-unknown-unknown

# Process the webgl_water_tutorial.wasm file and generate the necessary
# JavaScript glue code to run it in the browser.
wasm-bindgen ./target/wasm32-unknown-unknown/debug/webgl_water_tutorial.wasm --out-dir . --no-typescript --no-modules
