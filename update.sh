#!/bin/bash

echo "Running auto_index (Rust)..."
cargo run --manifest-path scripts/auto_index/Cargo.toml --release -- --inject-nav

if [ $? -eq 0 ]; then
    echo "Building mdbook..."
    mdbook build
else
    echo "auto_index failed."
fi
