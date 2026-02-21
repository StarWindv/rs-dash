#!/bin/sh
# Test script for rs-dash

echo "Testing rs-dash basic functionality..."

# Test echo command
echo "Test 1: echo command"
./target/debug/rs-dash -c "echo hello world"

# Test pwd command
echo "Test 2: pwd command"
./target/debug/rs-dash -c "pwd"

# Test cd and pwd
echo "Test 3: cd command"
./target/debug/rs-dash -c "cd .. && pwd"

# Test external command (ls/dir)
echo "Test 4: external command"
if command -v ls >/dev/null 2>&1; then
    ./target/debug/rs-dash -c "ls -la" | head -5
elif command -v dir >/dev/null 2>&1; then
    ./target/debug/rs-dash -c "dir" | head -5
fi

# Test help command
echo "Test 5: help command"
./target/debug/rs-dash -c "help"

echo "Testing complete!"