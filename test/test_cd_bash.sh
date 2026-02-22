#!/bin/bash
# Test script for cd command with -L and -P options

echo "=== Testing cd command with -L and -P options ==="

# Create test directory structure
TEST_DIR="test_cd_$$"
mkdir -p "$TEST_DIR/real"
ln -sf "$TEST_DIR/real" "$TEST_DIR/link"

echo "Created test directory: $TEST_DIR"
echo "  real: $TEST_DIR/real"
echo "  link: $TEST_DIR/link -> $TEST_DIR/real"

# Test 1: cd without options (should default to -L)
echo -e "\n=== Test 1: cd without options (default -L) ==="
cd "$TEST_DIR/link" && pwd
cd - >/dev/null

# Test 2: cd -L (logical mode)
echo -e "\n=== Test 2: cd -L (logical mode) ==="
cd -L "$TEST_DIR/link" && pwd
cd - >/dev/null

# Test 3: cd -P (physical mode)
echo -e "\n=== Test 3: cd -P (physical mode) ==="
cd -P "$TEST_DIR/link" && pwd
cd - >/dev/null

# Test 4: cd with - and options
echo -e "\n=== Test 4: cd with - (previous directory) ==="
cd "$TEST_DIR/real"
cd -L "$TEST_DIR/link"
cd -  # Should go back to real
pwd

# Test 5: cd with multiple options (last one wins)
echo -e "\n=== Test 5: cd with multiple options ==="
cd -L -P "$TEST_DIR/link" && pwd  # Should use -P
cd - >/dev/null

# Test 6: cd with -- to separate options
echo -e "\n=== Test 6: cd with -- separator ==="
cd -- -L "$TEST_DIR/link" 2>/dev/null || echo "cd: -L: No such file or directory"

# Test 7: cd to home directory
echo -e "\n=== Test 7: cd to home directory ==="
cd
pwd

# Test 8: cd with invalid option
echo -e "\n=== Test 8: cd with invalid option ==="
cd -X "$TEST_DIR/link" 2>&1 | head -1

# Cleanup
cd
rm -rf "$TEST_DIR"
echo -e "\n=== Cleanup completed ==="