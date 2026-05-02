#!/bin/bash

echo "Running auto_index (Rust)..."
cargo run --manifest-path scripts/auto_index/Cargo.toml --release -- --inject-nav
rc=$?
if [ $rc -ne 0 ]; then
    echo "auto_index failed."
    exit $rc
fi

echo "Linting Markdown..."
npx markdownlint-cli2 "src/**/*.md" --config .markdownlint.yml --fix
if [ $? -ne 0 ]; then
    echo "markdownlint failed."
    exit 1
fi

echo "Building mdbook..."
mdbook build
