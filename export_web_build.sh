#!/bin/bash
set -e

# Define the output directory
OUTPUT_DIR="./builds/web/"

# Build the project
cargo build --target wasm32-unknown-unknown --release

# Generate the bindings with wasm-bindgen
wasm-bindgen --out-dir $OUTPUT_DIR --target web target/wasm32-unknown-unknown/release/playing_around_world.wasm

# Copy the assets folder to the output directory
cp -r assets $OUTPUT_DIR

# Copy index template
cp ./build-template/index.html $OUTPUT_DIR

echo "Build completed. The web build is in the $OUTPUT_DIR directory."
