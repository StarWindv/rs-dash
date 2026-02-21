#!/bin/bash
# Simple test script for rs-dash (Linux/macOS)

set -e

echo "=== Testing rs-dash ==="
echo ""

# Build rs-dash
echo "Building rs-dash..."
cargo build

if [ $? -ne 0 ]; then
    echo "Build failed!"
    exit 1
fi

echo "Build successful!"
echo ""

# Run the main test suite
echo "Running test suite..."
python3 test/run_all_tests.py

exit $?