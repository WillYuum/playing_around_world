#!/bin/bash
set -e

# Define the output directory
OUTPUT_DIR="./builds/windows/"

# Ensure the output directory exists
mkdir -p $OUTPUT_DIR

# Build the project
cargo build --target x86_64-pc-windows-msvc --release

# Move the built executable to the output directory with the game assets
cp target/x86_64-pc-windows-msvc/release/playing_around_world.exe $OUTPUT_DIR
cp -r assets $OUTPUT_DIR

echo "Build completed. The Windows build is in the $OUTPUT_DIR directory."
