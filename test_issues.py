#!/usr/bin/env python3
"""
Test specific issues mentioned by user:
1. Variable expansion issue: echo $? not working
2. Command substitution: echo $(nproc) not working
3. Pipeline issue: echo hello | grep ll hangs
"""

import subprocess
import os
import time

def run_rs_dash(cmd):
    """Run a command in rs-dash"""
    rs_dash_path = os.path.join(os.path.dirname(__file__), "target", "debug", "rs-dash")
    if os.name == 'nt':
        rs_dash_path += ".exe"
    
    return subprocess.run([rs_dash_path, "-c", cmd],
                         capture_output=True, text=True, timeout=5)

def test_variable_expansion():
    """Test variable expansion issue"""
    print("\n=== Test 1: Variable expansion ===")
    
    # Test 1.1: $? (exit code)
    print("Test: true; echo $?; false; echo $?")
    result = run_rs_dash("true; echo $?; false; echo $?")
    print(f"Exit code: {result.returncode}")
    print(f"Stdout: {result.stdout}")
    print(f"Stderr: {result.stderr}")
    
    # Test 1.2: Command substitution
    print("\nTest: echo $(nproc)")
    result = run_rs_dash("echo $(nproc)")
    print(f"Exit code: {result.returncode}")
    print(f"Stdout: {result.stdout}")
    print(f"Stderr: {result.stderr}")

def test_pipeline():
    """Test pipeline issue"""
    print("\n=== Test 2: Pipeline ===")
    
    # Test 2.1: Simple pipeline
    print("Test: echo hello | grep ll")
    try:
        result = run_rs_dash("echo hello | grep ll")
        print(f"Exit code: {result.returncode}")
        print(f"Stdout: {result.stdout}")
        print(f"Stderr: {result.stderr}")
    except subprocess.TimeoutExpired:
        print("TIMEOUT: Pipeline command hung (this is the bug!)")
    
    # Test 2.2: Pipeline with output
    print("\nTest: echo hello | cat")
    try:
        result = run_rs_dash("echo hello | cat")
        print(f"Exit code: {result.returncode}")
        print(f"Stdout: {result.stdout}")
        print(f"Stderr: {result.stderr}")
    except subprocess.TimeoutExpired:
        print("TIMEOUT: Pipeline command hung")

def test_special_variables():
    """Test special shell variables"""
    print("\n=== Test 3: Special variables ===")
    
    tests = [
        ("$?", "Exit status of last command"),
        ("$$", "PID of shell"),
        ("$0", "Name of shell"),
        ("$1", "First positional parameter"),
    ]
    
    for var, desc in tests:
        print(f"\nTest: echo {var} ({desc})")
        result = run_rs_dash(f"echo {var}")
        print(f"Result: {result.stdout.strip()}")

def main():
    print("Testing specific issues mentioned by user")
    
    test_variable_expansion()
    test_pipeline()
    test_special_variables()

if __name__ == "__main__":
    main()