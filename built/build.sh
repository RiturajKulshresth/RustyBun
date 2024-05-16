#!/bin/bash

# Build RustyBun Clipboard
echo "Building RustyBun"
cd ../backend
cargo build --release

# Copy Rust output to shared folder
echo "Copying Rust output to built folder..."
cp target/release/clip_cpy ../gui/

# Build Electron code
echo "Building Electron code..."
cd ../gui
npm install
npm run dist:linux



echo "The deb can be found in dist/<>.deb"

