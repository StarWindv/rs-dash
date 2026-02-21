#!/bin/bash
# Test script for rs-dash

echo "Testing basic functionality..."
./target/debug/rs-dash -c "echo hello"
echo "Exit code: $?"

echo -e "\nTesting variable assignment and expansion..."
./target/debug/rs-dash -c "MYVAR=test; echo \$MYVAR"
echo "Exit code: $?"

echo -e "\nTesting exit status..."
./target/debug/rs-dash -c "false; echo \$?"
echo "Exit code: $?"

echo -e "\nTesting command substitution..."
./target/debug/rs-dash -c "echo \$(echo nested)"
echo "Exit code: $?"

echo -e "\nTesting arithmetic expansion..."
./target/debug/rs-dash -c "echo \$((1 + 2 * 3))"
echo "Exit code: $?"

echo -e "\nTesting parameter expansion..."
./target/debug/rs-dash -c "VAR=hello; echo \${VAR}"
echo "Exit code: $?"

echo -e "\nTesting positional parameters..."
./target/debug/rs-dash -c "echo \$1 \$2" arg1 arg2
echo "Exit code: $?"