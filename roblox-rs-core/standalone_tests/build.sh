#!/bin/bash
# Build and run the standalone tests

echo "Building standalone runtime tests..."
cd "$(dirname "$0")"
rustc main.rs -o runtime_tests

echo "Running tests..."
./runtime_tests
