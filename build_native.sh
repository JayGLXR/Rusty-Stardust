#!/bin/bash
set -e

# This script can be used for local testing with a native Rust installation
# It won't generate the actual cross-compiled Windows shellcode but helps validate code

echo "Building Rust code with native compiler..."
cargo build

echo "Testing code structure..."
cargo check --all-features

echo "Checking for errors..."
cargo clippy

echo "All tests passed!"
echo "Note: This is only a code quality test - for actual Windows shellcode generation,"
echo "you need to use the Docker-based build with cross-compilation."