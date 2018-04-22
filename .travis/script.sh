#!/bin/sh
set -ex

# Build and test with no features.
cargo build --verbose
cargo test --verbose

# Build with all features.
cargo build --verbose --all-features

# Doc with all features.
cargo doc --verbose --all-features
cp .travis/index.html target/doc/

# Test with all possible combinations of all features.
cd vapoursynth; python3 ../.travis/run-tests.py
cd ..

# Run sample plugin tests.
cd sample-plugin
cargo build --verbose
cargo run --verbose --bin test \
    --features "cfg-if vapoursynth/vapoursynth-functions vapoursynth/vsscript-functions"
