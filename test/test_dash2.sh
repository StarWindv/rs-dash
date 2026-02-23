#!/bin/bash
# Save output to file
{
echo "=== Testing with dash ==="
echo ""
echo "Test 1: No quotes"
echo "Command: echo C:\\ProgramData\\chocolatey\\bin\\tree.exe"
echo -n "Output: "
echo C:\ProgramData\chocolatey\bin\tree.exe
echo ""
echo "Test 2: Double quotes"
echo "Command: echo \"C:\\ProgramData\\chocolatey\\bin\\tree.exe\""
echo -n "Output: "
echo "C:\ProgramData\chocolatey\bin\tree.exe"
echo ""
echo "Test 3: Single quotes"
echo "Command: echo 'C:\\ProgramData\\chocolatey\\bin\\tree.exe'"
echo -n "Output: "
echo 'C:\ProgramData\chocolatey\bin\tree.exe'
} > dash_test_output.txt 2>&1

cat dash_test_output.txt